use std::fmt;

/// Represents a currency unit in the Mu-Bank system.
/// #[ptbr] Moeda com suporte a MUB (aprendizado) e moedas fiduciárias.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    MUB,
    USD,
    EUR,
    BRL,
}

impl Currency {
    /// ISO 4217 numeric code (MUB uses 999 as a private range).
    pub fn numeric_code(&self) -> u16 {
        match self {
            Currency::MUB => 999,
            Currency::USD => 840,
            Currency::EUR => 978,
            Currency::BRL => 986,
        }
    }

    /// Alphabetic currency code.
    pub fn code(&self) -> &'static str {
        match self {
            Currency::MUB => "MUB",
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::BRL => "BRL",
        }
    }

    /// Common symbol for the currency.
    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::MUB => "\u{00b5}",
            Currency::USD => "$",
            Currency::EUR => "\u{20ac}",
            Currency::BRL => "R$",
        }
    }

    /// Number of decimal places for this currency.
    pub fn decimals(&self) -> u8 {
        2
    }

    /// Full name of the currency.
    pub fn name(&self) -> &'static str {
        match self {
            Currency::MUB => "Mu-Bank",
            Currency::USD => "US Dollar",
            Currency::EUR => "Euro",
            Currency::BRL => "Brazilian Real",
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl TryFrom<&str> for Currency {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "MUB" => Ok(Currency::MUB),
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            "BRL" => Ok(Currency::BRL),
            _ => Err(format!("Unknown currency code: {}", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_code() {
        assert_eq!(Currency::MUB.code(), "MUB");
        assert_eq!(Currency::USD.code(), "USD");
        assert_eq!(Currency::EUR.code(), "EUR");
        assert_eq!(Currency::BRL.code(), "BRL");
    }

    #[test]
    fn test_currency_symbol() {
        assert_eq!(Currency::MUB.symbol(), "\u{00b5}");
        assert_eq!(Currency::USD.symbol(), "$");
    }

    #[test]
    fn test_currency_try_from_str() {
        assert_eq!(Currency::try_from("usd"), Ok(Currency::USD));
        assert_eq!(Currency::try_from("BRL"), Ok(Currency::BRL));
        assert!(Currency::try_from("GBP").is_err());
    }

    #[test]
    fn test_currency_display() {
        assert_eq!(Currency::MUB.to_string(), "MUB");
        assert_eq!(Currency::EUR.to_string(), "EUR");
    }
}
