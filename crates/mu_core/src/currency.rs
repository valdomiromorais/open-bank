use std::fmt;

/// Represents a currency unit in the Mu-Bank system. \
/// #[ptbr] Moeda com suporte a MUB (aprendizado) e moedas fiduciárias. \
/// #[ptbr] ISO 4217 <https://en.wikipedia.org/wiki/ISO_4217> é um padrão internacional estabelecido pela ISO que
/// define códigos de três letras e de três dígitos para representar moedas e fundos no mundo todo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    MUB,
    USD, //#[ptbr] Dólar americano
    EUR, //#[ptbr] Euro
    BRL, //#[ptbr] Real Brasileiro
    CNY, //#[ptbr] Yuan/Renminbi Chinês
    GBP, //#[ptbr] Libra esterlina (Pound sterling)
    JPY, //#[ptbr] Yen Japonês
}

impl Currency {
    /// ISO 4217 (https://www.iso.org/standard/64758.html) numeric code
    /// (MUB uses 999 as a private range).
    pub fn numeric_code(&self) -> u16 {
        match self {
            Currency::MUB => 999, //No currency
            Currency::USD => 840, //ok
            Currency::EUR => 978, //ok
            Currency::BRL => 986, //ok
            Currency::CNY => 156, //ok
            Currency::GBP => 826, //ok
            Currency::JPY => 392, //ok
        }
    }

    /// Alphabetic currency code.
    pub fn code(&self) -> &'static str {
        match self {
            Currency::MUB => "MUB",
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::BRL => "BRL",
            Currency::CNY => "CNY",
            Currency::GBP => "GBP",
            Currency::JPY => "JPY",
        }
    }

    /// Common symbol for the currency.
    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::MUB => "\u{00b5}",
            Currency::USD => "$",
            Currency::EUR => "\u{20ac}",
            Currency::BRL => "R$",
            Currency::CNY => "CN\u{00A5}", // mesmo do JPY?
            Currency::GBP => "\u{00a3}",
            Currency::JPY => "JP\u{00A5}", //ok
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
            Currency::CNY => "Chinese Yuan (Renminbi)",
            Currency::GBP => "Pound Sterling",
            Currency::JPY => "Japanese Yen",
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
            "CNY" => Ok(Currency::CNY),
            "GBP" => Ok(Currency::GBP),
            "JPY" => Ok(Currency::JPY),
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
        assert_eq!(Currency::CNY.code(), "CNY");
        assert_eq!(Currency::JPY.code(), "JPY");
        assert_eq!(Currency::GBP.code(), "GBP");
        assert_eq!(Currency::JPY.code(), "JPY");
    }

    #[test]
    fn test_currency_symbol() {
        assert_eq!(Currency::MUB.symbol(), "\u{00b5}");
        assert_eq!(Currency::USD.symbol(), "$");
        assert_eq!(Currency::CNY.symbol(), "CN\u{00a5}");
        assert_eq!(Currency::JPY.symbol(), "JP\u{00a5}");
        assert_eq!(Currency::GBP.symbol(), "\u{00a3}");
    }

    #[test]
    fn test_currency_try_from_str() {
        assert_eq!(Currency::try_from("usd"), Ok(Currency::USD));
        assert_eq!(Currency::try_from("BRL"), Ok(Currency::BRL));
        assert_eq!(Currency::try_from("cny"), Ok(Currency::CNY));
        assert_eq!(Currency::try_from("GBP"), Ok(Currency::GBP));
        assert_eq!(Currency::try_from("JPY"), Ok(Currency::JPY));
        assert!(Currency::try_from("XYZ").is_err());
    }

    #[test]
    fn test_currency_display() {
        assert_eq!(Currency::MUB.to_string(), "MUB");
        assert_eq!(Currency::EUR.to_string(), "EUR");
        assert_eq!(Currency::GBP.to_string(), "GBP");
        assert_eq!(Currency::JPY.to_string(), "JPY");
    }
}
