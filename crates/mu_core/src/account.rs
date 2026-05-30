use crate::currency::Currency;
use crate::customer::CustomerId;
use std::fmt;
use uuid::Uuid;

/// Unique identifier for an account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccountId(Uuid); //#[ptbr] Newtype pattner

impl AccountId {
    pub fn new() -> Self {
        AccountId(Uuid::new_v4())
    }
}

// Uma Warning: código dublicado com TransactionId
impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.0.to_string(); // UUID de 128 bits ()
        // short display: first 8 chars (&s[..8]) .. and 4 last
        write!(f, "{}..{}", &s[..8], &s[s.len() - 4..])
    }
}

/// Status lifecycle of an account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountStatus {
    Active,
    Frozen,
    Closed,
    PendingVerification,
}

/// A bank account belonging to a customer.
/// #[ptbr] Saldo nunca é armazenado — é derivado do ledger.
#[derive(Debug, Clone)]
pub struct Account {
    id: AccountId,
    holder: CustomerId,
    status: AccountStatus,
    currency: Currency,
}

impl Account {
    /// Creates a new account with an auto-generated ID.
    pub fn new(holder: CustomerId, currency: Currency) -> Self {
        Account {
            id: AccountId::new(),
            holder,
            status: AccountStatus::PendingVerification,
            currency,
        }
    }

    /// Creates an account with a specific ID (for persistence restoration).
    pub fn with_id(
        id: AccountId,
        holder: CustomerId,
        currency: Currency,
        status: AccountStatus,
    ) -> Self {
        Account {
            id,
            holder,
            status,
            currency,
        }
    }

    pub fn id(&self) -> AccountId {
        self.id
    }

    pub fn holder(&self) -> CustomerId {
        self.holder
    }

    pub fn status(&self) -> AccountStatus {
        self.status
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn activate(&mut self) {
        self.status = AccountStatus::Active;
    }

    pub fn freeze(&mut self) {
        self.status = AccountStatus::Frozen;
    }

    pub fn close(&mut self) {
        self.status = AccountStatus::Closed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_new() {
        let holder = CustomerId::new();
        let acc = Account::new(holder, Currency::BRL);
        assert_eq!(acc.holder(), holder);
        assert_eq!(acc.currency(), Currency::BRL);
        assert_eq!(acc.status(), AccountStatus::PendingVerification);
    }

    #[test]
    fn test_account_with_id() {
        let id = AccountId::new();
        let holder = CustomerId::new();
        let acc = Account::with_id(id, holder, Currency::BRL, AccountStatus::Active);
        assert_eq!(acc.id(), id);
        assert_eq!(acc.status(), AccountStatus::Active);
    }

    #[test]
    fn test_account_lifecycle() {
        let mut acc = Account::new(CustomerId::new(), Currency::MUB);
        assert_eq!(acc.status(), AccountStatus::PendingVerification);
        acc.activate();
        assert_eq!(acc.status(), AccountStatus::Active);
        acc.freeze();
        assert_eq!(acc.status(), AccountStatus::Frozen);
        acc.close();
        assert_eq!(acc.status(), AccountStatus::Closed);
    }

    #[test]
    fn test_account_id_display() {
        let id = AccountId::new();
        let s = id.to_string();
        assert!(s.contains(".."));
    }
}
