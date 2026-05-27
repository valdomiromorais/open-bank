use crate::currency::Currency;
use crate::customer::CustomerId;
use std::fmt;
use uuid::Uuid;

/// Unique identifier for an account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccountId(Uuid);

impl AccountId {
    pub fn new() -> Self {
        AccountId(Uuid::new_v4())
    }
}

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.0.to_string();
        // short display: first 8 chars
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
    pub fn new(id: AccountId, holder: CustomerId, currency: Currency) -> Self {
        Account {
            id,
            holder,
            status: AccountStatus::PendingVerification,
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
        let id = AccountId::new();
        let holder = CustomerId::new();
        let acc = Account::new(id, holder, Currency::BRL);
        assert_eq!(acc.id(), id);
        assert_eq!(acc.holder(), holder);
        assert_eq!(acc.currency(), Currency::BRL);
        assert_eq!(acc.status(), AccountStatus::PendingVerification);
    }

    #[test]
    fn test_account_lifecycle() {
        let mut acc = Account::new(AccountId::new(), CustomerId::new(), Currency::MUB);
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
