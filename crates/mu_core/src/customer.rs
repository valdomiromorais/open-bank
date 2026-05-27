use std::fmt;
use uuid::Uuid;

/// Unique identifier for a customer.
/// #[ptbr] Novo tipo sobre Uuid para evitar confusão com outros IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomerId(Uuid);

impl CustomerId {
    pub fn new() -> Self {
        CustomerId(Uuid::new_v4())
    }
}

impl fmt::Display for CustomerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A customer of the bank.
/// #[ptbr] Pessoa física titular de contas no μBank.
#[derive(Debug, Clone)]
pub struct Customer {
    id: CustomerId,
    name: String,
}

impl Customer {
    pub fn new(id: CustomerId, name: String) -> Self {
        Customer { id, name }
    }

    pub fn id(&self) -> CustomerId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer_new() {
        let id = CustomerId::new();
        let c = Customer::new(id, "Alice".into());
        assert_eq!(c.id(), id);
        assert_eq!(c.name(), "Alice");
    }

    #[test]
    fn test_customer_id_display() {
        let id = CustomerId::new();
        let s = id.to_string();
        assert_eq!(s.len(), 36); // UUID format
        assert_eq!(s.chars().filter(|&c| c == '-').count(), 4);
    }
}
