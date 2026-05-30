// #[pptbr] Módulo revisado:

use std::fmt;
use uuid::Uuid;
//#[ptbr] Um Universally Unique Identifier (UUID) - Identificador Universalmente Único -
// é um número de 128 bit (16 bytes) usado para identificar informações ou recursos em
// sistemas computacionais de forma única.

/// Unique identifier for a customer. \
/// #[ptbr]: Novo tipo sobre Uuid para evitar confusão com outros IDs.

//#[ptbr] struct CustomerId(Uuid): _Newtype pattern_ — um _wrapper_ (envelopador) sobre um tipo
// existente (Uuid) para ganhar segurança de tipo em tempo de compilação.
// UUID format: 32 hex (dígitos hexadecimais) e 4 '-': 8-4-4-4-12 (exemplo:123e4567-e89b-12d3-a456-426614174000)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomerId(Uuid); //#[ptbr] struct CustomerId(Uuid): _Newtype pattern_

impl CustomerId {
    pub fn new() -> Self {
        CustomerId(Uuid::new_v4())
    } // #[ptbr] Cria um UUID (com 32 hex + 4 '-') aleatoriamente
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
    id:   CustomerId,
    name: String,
}

impl Customer {
    /// Creates a new customer with an auto-generated ID.
    pub fn new(name: String) -> Self {
        Customer {
            id: CustomerId::new(),
            name,
        }
    }

    /// Creates a customer with a specific ID (for persistence restoration).
    pub fn with_id(id: CustomerId, name: String) -> Self {
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
        let c = Customer::new("Alice".into());
        assert_eq!(c.name(), "Alice");
    }

    #[test]
    fn test_customer_with_id() {
        let id = CustomerId::new();
        let c = Customer::with_id(id, "Bob".into());
        assert_eq!(c.id(), id);
        assert_eq!(c.name(), "Bob");
    }

    #[test]
    fn test_customer_id_display() {
        let id = CustomerId::new(); //#[ptbr] cria uma UUID aleatóriamente
        let s = id.to_string();
        assert_eq!(s.len(), 36); // #[ptbr] 36 porque o UUID format tem 32 dígitos Hexadecimais e 4 '-': 8-4-4-4-12 (exemplo:123e4567-e89b-12d3-a456-426614174000)
        assert_eq!(s.chars().filter(|&c| c == '-').count(), 4); // #[ptbr] verificar se existem os 4 '-' do UUID format
    }
}
