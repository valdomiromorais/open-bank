use crate::account::AccountId;
use crate::money::Money;
use chrono::{DateTime, Utc};
use std::fmt;
use uuid::Uuid;

/// Unique identifier for a transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransactionId(Uuid);

impl TransactionId {
    pub fn new() -> Self {
        TransactionId(Uuid::new_v4())
    }
}

// Uma Warning: código dublicado com AccountId
impl fmt::Display for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.0.to_string();
        // short display: first 8 chars (&s[..8]) .. and 4 last
        write!(f, "{}..{}", &s[..8], &s[s.len() - 4..])
    }
}

/// The nature of a financial transaction.
/// #[ptbr] Define se é depósito, saque ou transferência entre contas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionKind {
    Deposit,
    Withdraw,
    Transfer {
        from: AccountId,
        to: AccountId,
    },
}

/// A single financial transaction recorded in the ledger.
/// #[ptbr] Imutável após criada — correções são feitas com estorno (nova transação).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    id: TransactionId,
    account_id: AccountId,
    kind: TransactionKind,
    amount: Money,
    timestamp: DateTime<Utc>,
    description: String,
}

impl Transaction {
    /// Creates a new transaction with an auto-generated ID.
    pub fn new(
        account_id: AccountId,
        kind: TransactionKind,
        amount: Money,
        description: String,
    ) -> Self {
        Transaction {
            id: TransactionId::new(),
            account_id,
            kind,
            amount,
            timestamp: Utc::now(),
            description,
        }
    }

    /// Creates a transaction with a specific ID (for persistence restoration).
    pub fn with_id(
        id: TransactionId,
        account_id: AccountId,
        kind: TransactionKind,
        amount: Money,
        description: String,
    ) -> Self {
        Transaction {
            id,
            account_id,
            kind,
            amount,
            timestamp: Utc::now(),
            description,
        }
    }

    pub fn id(&self) -> TransactionId {
        self.id
    }

    pub fn account_id(&self) -> AccountId {
        self.account_id
    }

    pub fn kind(&self) -> &TransactionKind {
        &self.kind
    }

    pub fn amount(&self) -> Money {
        self.amount
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::Currency;
    use rust_decimal_macros::dec;

    fn mub(amount: rust_decimal::Decimal) -> Money {
        Money::new(amount, Currency::MUB)
    }

    #[test]
    fn test_deposit_transaction() {
        let acc = AccountId::new();
        let tx = Transaction::new(acc, TransactionKind::Deposit, mub(dec!(500)), "Initial deposit".into());
        assert_eq!(tx.account_id(), acc);
        assert_eq!(tx.amount(), mub(dec!(500)));
        assert_eq!(tx.kind(), &TransactionKind::Deposit);
    }

    #[test]
    fn test_transfer_transaction() {
        let from = AccountId::new();
        let to = AccountId::new();
        let tx = Transaction::new(from, TransactionKind::Transfer { from, to }, mub(dec!(100)), "Pix transfer".into());
        assert_eq!(tx.description(), "Pix transfer");
    }

    #[test]
    fn test_transaction_with_id() {
        let id = TransactionId::new();
        let acc = AccountId::new();
        let tx = Transaction::with_id(id, acc, TransactionKind::Deposit, mub(dec!(50)), "restored".into());
        assert_eq!(tx.id(), id);
    }
}
