// "Apanhai-nos as raposas, as raposinhas, que fazem mal às vinhas,
//  porque as nossas vinhas estão em flor." — Cantares 2:15
// Cada validação aqui é uma raposinha caçada.

use crate::account::{Account, AccountId, AccountStatus};
use crate::currency::Currency;
use crate::customer::{Customer, CustomerId};
use crate::money::{Money, MoneyError};
use crate::transaction::{Transaction, TransactionId, TransactionKind};
use std::collections::HashMap;

/// Errors that can occur during ledger operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    AccountNotFound(AccountId),
    CustomerNotFound(CustomerId),
    AccountNotActive(AccountId, AccountStatus),
    InsufficientBalance {
        account: AccountId,
        balance: Money,
        requested: Money,
    },
    CurrencyMismatch {
        account: AccountId,
        account_currency: Currency,
        tx_currency: Currency,
    },
    MoneyError(MoneyError),
    TransferToSameAccount,
    TransactionNotFound(TransactionId),
    OriginalAlreadyReversed(TransactionId),
}

impl std::fmt::Display for LedgerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LedgerError::AccountNotFound(id) => write!(f, "account {} not found", id),
            LedgerError::CustomerNotFound(id) => write!(f, "customer {} not found", id),
            LedgerError::AccountNotActive(id, status) => {
                write!(f, "account {} is {:?}, must be Active", id, status)
            }
            LedgerError::InsufficientBalance {
                account,
                balance,
                requested,
            } => {
                write!(
                    f,
                    "account {} has {} but requested {}",
                    account, balance, requested
                )
            }
            LedgerError::CurrencyMismatch {
                account,
                account_currency,
                tx_currency,
            } => {
                write!(
                    f,
                    "account {} operates in {} but transaction is in {}",
                    account, account_currency, tx_currency
                )
            }
            LedgerError::MoneyError(e) => write!(f, "money error: {}", e),
            LedgerError::TransferToSameAccount => write!(f, "cannot transfer to the same account"),
            LedgerError::TransactionNotFound(id) => write!(f, "transaction {} not found", id),
            LedgerError::OriginalAlreadyReversed(id) => {
                write!(f, "transaction {} has already been reversed", id)
            }
        }
    }
}

/// In-memory ledger — the single source of truth for all balances.
/// #[ptbr] Saldo nunca é armazenado; é computado agregando transações (Event Sourcing).
#[derive(Debug, Clone)]
pub struct Ledger {
    customers: HashMap<CustomerId, Customer>,
    accounts: HashMap<AccountId, Account>,
    transactions: Vec<Transaction>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            customers: HashMap::new(),
            accounts: HashMap::new(),
            transactions: Vec::new(),
        }
    }

    // --- Customer operations ---

    pub fn create_customer(&mut self, name: String) -> CustomerId {
        let customer = Customer::new(name);
        let id = customer.id();
        self.customers.insert(id, customer);
        id
    }

    pub fn get_customer(&self, id: CustomerId) -> Option<&Customer> {
        self.customers.get(&id)
    }

    // --- Account operations ---

    pub fn create_account(
        &mut self,
        holder: CustomerId,
        currency: Currency,
    ) -> Result<AccountId, LedgerError> {
        if !self.customers.contains_key(&holder) {
            return Err(LedgerError::CustomerNotFound(holder));
        }
        let account = Account::new(holder, currency);
        let id = account.id();
        self.accounts.insert(id, account);
        Ok(id)
    }

    pub fn get_account(&self, id: AccountId) -> Option<&Account> {
        self.accounts.get(&id)
    }

    pub fn activate_account(&mut self, id: AccountId) -> Result<(), LedgerError> {
        let account = self
            .accounts
            .get_mut(&id)
            .ok_or(LedgerError::AccountNotFound(id))?;
        account.activate();
        Ok(())
    }

    // --- Balance (computed from transactions) ---

    /// Computes the current balance of an account by folding over all transactions.
    /// #[ptbr] Coração do Event Sourcing: saldo é função pura do histórico.
    pub fn balance(&self, account_id: AccountId) -> Result<Money, LedgerError> {
        let account = self
            .accounts
            .get(&account_id)
            .ok_or(LedgerError::AccountNotFound(account_id))?;
        let currency = account.currency();
        let mut balance = Money::zero(currency);

        for tx in &self.transactions {
            match tx.kind() {
                TransactionKind::Deposit => {
                    if tx.account_id() == account_id {
                        balance = (balance + tx.amount()).map_err(LedgerError::MoneyError)?;
                    }
                }
                TransactionKind::Withdraw => {
                    if tx.account_id() == account_id {
                        balance = (balance - tx.amount()).map_err(LedgerError::MoneyError)?;
                    }
                }
                TransactionKind::Transfer { from, to } => {
                    if *from == account_id || *to == account_id {
                        if *from == account_id {
                            balance =
                                (balance - tx.amount()).map_err(LedgerError::MoneyError)?;
                        }
                        if *to == account_id {
                            balance =
                                (balance + tx.amount()).map_err(LedgerError::MoneyError)?;
                        }
                    }
                }
                TransactionKind::Reversal { original_tx } => {
                    if let Some(orig) = self.find_transaction(*original_tx) {
                        let affects_this = orig.account_id() == account_id
                            || matches!(orig.kind(), TransactionKind::Transfer { from, to }
                                if *from == account_id || *to == account_id);
                        if affects_this {
                            match orig.kind() {
                                TransactionKind::Deposit => {
                                    balance = (balance - tx.amount())
                                        .map_err(LedgerError::MoneyError)?;
                                }
                                TransactionKind::Withdraw => {
                                    balance = (balance + tx.amount())
                                        .map_err(LedgerError::MoneyError)?;
                                }
                                TransactionKind::Transfer { from, to } => {
                                    if *from == account_id {
                                        balance = (balance + tx.amount())
                                            .map_err(LedgerError::MoneyError)?;
                                    }
                                    if *to == account_id {
                                        balance = (balance - tx.amount())
                                            .map_err(LedgerError::MoneyError)?;
                                    }
                                }
                                TransactionKind::Reversal { .. } => {}
                            }
                        }
                    }
                }
            }
        }

        Ok(balance)
    }

    // --- Transaction operations ---

    pub fn deposit(
        &mut self,
        account_id: AccountId,
        amount: Money,
        description: String,
    ) -> Result<TransactionId, LedgerError> {
        self.ensure_can_transact(account_id, &amount)?;

        let tx = Transaction::new(account_id, TransactionKind::Deposit, amount, description);
        let id = tx.id();
        self.transactions.push(tx);
        Ok(id)
    }

    pub fn withdraw(
        &mut self,
        account_id: AccountId,
        amount: Money,
        description: String,
    ) -> Result<TransactionId, LedgerError> {
        self.ensure_can_transact(account_id, &amount)?;

        let balance = self.balance(account_id)?;
        let available = (balance - amount).map_err(LedgerError::MoneyError)?;
        if available.amount().is_sign_negative() {
            return Err(LedgerError::InsufficientBalance {
                account: account_id,
                balance,
                requested: amount,
            });
        }

        let tx = Transaction::new(account_id, TransactionKind::Withdraw, amount, description);
        let id = tx.id();
        self.transactions.push(tx);
        Ok(id)
    }

    pub fn transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Money,
        description: String,
    ) -> Result<TransactionId, LedgerError> {
        if from == to {
            return Err(LedgerError::TransferToSameAccount);
        }

        self.ensure_can_transact(from, &amount)?;
        self.ensure_can_transact(to, &amount)?;

        let from_balance = self.balance(from)?;
        let available = (from_balance - amount).map_err(LedgerError::MoneyError)?;
        if available.amount().is_sign_negative() {
            return Err(LedgerError::InsufficientBalance {
                account: from,
                balance: from_balance,
                requested: amount,
            });
        }

        let tx = Transaction::new(from, TransactionKind::Transfer { from, to }, amount, description);
        let id = tx.id();
        self.transactions.push(tx);
        Ok(id)
    }

    /// Reverses a previous transaction.
    /// #[ptbr] Estorno: cria uma nova transação que desfaz o efeito da original.
    pub fn reversal(
        &mut self,
        original_tx: TransactionId,
        description: String,
    ) -> Result<TransactionId, LedgerError> {
        let original = self
            .find_transaction(original_tx)
            .ok_or(LedgerError::TransactionNotFound(original_tx))?;

        if self
            .transactions
            .iter()
            .any(|t| matches!(t.kind(), TransactionKind::Reversal { original_tx: ot } if *ot == original_tx))
        {
            return Err(LedgerError::OriginalAlreadyReversed(original_tx));
        }

        let account_id = original.account_id();
        self.ensure_can_transact(account_id, &original.amount())?;

        let tx = Transaction::new(
            account_id,
            TransactionKind::Reversal { original_tx },
            original.amount(),
            description,
        );
        let id = tx.id();
        self.transactions.push(tx);
        Ok(id)
    }

    pub fn all_transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    pub fn transactions_for(&self, account_id: AccountId) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|tx| tx.account_id() == account_id)
            .collect()
    }

    // --- Private helpers ---

    fn find_transaction(&self, id: TransactionId) -> Option<&Transaction> {
        self.transactions.iter().find(|tx| tx.id() == id)
    }

    fn ensure_can_transact(
        &self,
        account_id: AccountId,
        amount: &Money,
    ) -> Result<(), LedgerError> {
        let account = self
            .accounts
            .get(&account_id)
            .ok_or(LedgerError::AccountNotFound(account_id))?;

        if account.status() != AccountStatus::Active {
            return Err(LedgerError::AccountNotActive(account_id, account.status()));
        }

        if account.currency() != amount.currency() {
            return Err(LedgerError::CurrencyMismatch {
                account: account_id,
                account_currency: account.currency(),
                tx_currency: amount.currency(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn mub(amount: rust_decimal::Decimal) -> Money {
        Money::new(amount, Currency::MUB)
    }

    fn setup() -> (Ledger, CustomerId, AccountId) {
        let mut ledger = Ledger::new();
        let customer = ledger.create_customer("Alice".into());
        let account = ledger.create_account(customer, Currency::MUB).unwrap();
        ledger.activate_account(account).unwrap();
        (ledger, customer, account)
    }

    #[test]
    fn test_create_customer_and_account() {
        let mut ledger = Ledger::new();
        let cid = ledger.create_customer("Bob".into());
        assert!(ledger.get_customer(cid).is_some());

        let aid = ledger.create_account(cid, Currency::USD).unwrap();
        let acc = ledger.get_account(aid).unwrap();
        assert_eq!(acc.currency(), Currency::USD);
    }

    #[test]
    fn test_deposit_and_balance() {
        let (mut ledger, _customer, account) = setup();

        let tx_id = ledger
            .deposit(account, mub(dec!(1000)), "Salary".into())
            .unwrap();

        let balance = ledger.balance(account).unwrap();
        assert_eq!(balance, mub(dec!(1000)));

        let txs = ledger.all_transactions();
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].id(), tx_id);
    }

    #[test]
    fn test_withdraw() {
        let (mut ledger, _customer, account) = setup();
        ledger
            .deposit(account, mub(dec!(500)), "dep".into())
            .unwrap();
        ledger
            .withdraw(account, mub(dec!(200)), "atm".into())
            .unwrap();

        assert_eq!(ledger.balance(account).unwrap(), mub(dec!(300)));
    }

    #[test]
    fn test_insufficient_balance() {
        let (mut ledger, _customer, account) = setup();
        let result = ledger.withdraw(account, mub(dec!(100)), "empty".into());
        assert!(matches!(result, Err(LedgerError::InsufficientBalance { .. })));
    }

    #[test]
    fn test_transfer_between_accounts() {
        let mut ledger = Ledger::new();
        let cid = ledger.create_customer("Alice".into());

        let acc_a = ledger.create_account(cid, Currency::MUB).unwrap();
        let acc_b = ledger.create_account(cid, Currency::MUB).unwrap();
        ledger.activate_account(acc_a).unwrap();
        ledger.activate_account(acc_b).unwrap();

        ledger.deposit(acc_a, mub(dec!(1000)), "dep".into()).unwrap();
        ledger
            .transfer(acc_a, acc_b, mub(dec!(300)), "pix".into())
            .unwrap();

        assert_eq!(ledger.balance(acc_a).unwrap(), mub(dec!(700)));
        assert_eq!(ledger.balance(acc_b).unwrap(), mub(dec!(300)));
    }

    #[test]
    fn test_reject_transfer_to_same_account() {
        let (mut ledger, _customer, account) = setup();
        ledger.deposit(account, mub(dec!(100)), "dep".into()).unwrap();
        let result = ledger.transfer(account, account, mub(dec!(10)), "self".into());
        assert_eq!(result, Err(LedgerError::TransferToSameAccount));
    }

    #[test]
    fn test_currency_mismatch_rejected() {
        let (mut ledger, _customer, account) = setup();
        let result = ledger.deposit(account, Money::new(dec!(100), Currency::USD), "usd".into());
        assert!(matches!(result, Err(LedgerError::CurrencyMismatch { .. })));
    }

    #[test]
    fn test_balance_after_multiple_deposits() {
        let (mut ledger, _customer, account) = setup();
        ledger.deposit(account, mub(dec!(100)), "a".into()).unwrap();
        ledger.deposit(account, mub(dec!(200)), "b".into()).unwrap();
        ledger.deposit(account, mub(dec!(50)), "c".into()).unwrap();
        assert_eq!(ledger.balance(account).unwrap(), mub(dec!(350)));
    }

    #[test]
    fn test_reversal_of_deposit() {
        let (mut ledger, _customer, account) = setup();
        let dep = ledger.deposit(account, mub(dec!(500)), "dep".into()).unwrap();
        let rev = ledger.reversal(dep, "estorno".into()).unwrap();

        assert_eq!(ledger.balance(account).unwrap(), mub(dec!(0)));
        assert_eq!(ledger.all_transactions().len(), 2);

        let rev_tx = ledger.all_transactions().iter().find(|t| t.id() == rev).unwrap();
        assert_eq!(rev_tx.kind(), &TransactionKind::Reversal { original_tx: dep });
    }

    #[test]
    fn test_reversal_of_withdraw() {
        let (mut ledger, _customer, account) = setup();
        ledger.deposit(account, mub(dec!(300)), "dep".into()).unwrap();
        let wd = ledger.withdraw(account, mub(dec!(100)), "saque".into()).unwrap();
        let _rev = ledger.reversal(wd, "estorno saque".into()).unwrap();

        assert_eq!(ledger.balance(account).unwrap(), mub(dec!(300)));
    }

    #[test]
    fn test_reversal_of_transfer() {
        let mut ledger = Ledger::new();
        let cid = ledger.create_customer("Alice".into());
        let a = ledger.create_account(cid, Currency::MUB).unwrap();
        let b = ledger.create_account(cid, Currency::MUB).unwrap();
        ledger.activate_account(a).unwrap();
        ledger.activate_account(b).unwrap();

        ledger.deposit(a, mub(dec!(500)), "dep".into()).unwrap();
        let t = ledger.transfer(a, b, mub(dec!(200)), "pix".into()).unwrap();
        let _rev = ledger.reversal(t, "estorno pix".into()).unwrap();

        assert_eq!(ledger.balance(a).unwrap(), mub(dec!(500)));
        assert_eq!(ledger.balance(b).unwrap(), mub(dec!(0)));
    }

    #[test]
    fn test_reversal_twice_rejected() {
        let (mut ledger, _customer, account) = setup();
        let dep = ledger.deposit(account, mub(dec!(100)), "dep".into()).unwrap();
        ledger.reversal(dep, "rev".into()).unwrap();
        let result = ledger.reversal(dep, "rev2".into());
        assert_eq!(result, Err(LedgerError::OriginalAlreadyReversed(dep)));
    }

    #[test]
    fn test_reversal_nonexistent_tx() {
        let mut ledger = Ledger::new();
        let phantom = TransactionId::new();
        let result = ledger.reversal(phantom, "ghost".into());
        assert_eq!(result, Err(LedgerError::TransactionNotFound(phantom)));
    }

    #[test]
    fn test_account_not_found() {
        let ledger = Ledger::new();
        let phantom = AccountId::new();
        let result = ledger.balance(phantom);
        assert_eq!(result, Err(LedgerError::AccountNotFound(phantom)));
    }
}
