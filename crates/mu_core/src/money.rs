use crate::currency::Currency;
use std::fmt;
use std::ops::{Add, Sub};

/// A monetary value composed of an amount and a currency.
/// #[ptbr] Amount armazenado na menor unidade (centavos) para evitar erros de ponto flutuante.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    amount: i64,
    currency: Currency,
}

impl Money {
    /// Creates a new Money value.
    pub fn new(amount: i64, currency: Currency) -> Self {
        Money { amount, currency }
    }

    /// Zero value for a given currency.
    pub fn zero(currency: Currency) -> Self {
        Money { amount: 0, currency }
    }

    /// Returns the amount in the smallest unit (cents).
    pub fn amount(&self) -> i64 {
        self.amount
    }

    /// Returns the currency of this money.
    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// Returns the amount as a formatted decimal string (e.g. "1234.56").
    pub fn display_amount(&self) -> String {
        let decimals = self.currency.decimals() as u32;
        let divisor = 10i64.pow(decimals);
        let whole = self.amount / divisor;
        let fraction = self.amount.abs() % divisor;
        format!("{}.{:02}", whole, fraction)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.display_amount(), self.currency.code())
    }
}

/// Errors that can occur during Money arithmetic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoneyError {
    /// #[ptbr] Tentativa de operar entre moedas diferentes (ex: USD + BRL)
    CurrencyMismatch { left: Currency, right: Currency },
    /// #[ptbr] Estouro aritmético (overflow/underflow)
    Overflow,
}

impl fmt::Display for MoneyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoneyError::CurrencyMismatch { left, right } => {
                write!(f, "currency mismatch: cannot mix {} with {}", left, right)
            }
            MoneyError::Overflow => write!(f, "arithmetic overflow"),
        }
    }
}

impl Add for Money {
    type Output = Result<Money, MoneyError>;

    fn add(self, rhs: Self) -> Result<Money, MoneyError> {
        if self.currency != rhs.currency {
            return Err(MoneyError::CurrencyMismatch {
                left: self.currency,
                right: rhs.currency,
            });
        }
        self.amount
            .checked_add(rhs.amount)
            .map(|amount| Money::new(amount, self.currency))
            .ok_or(MoneyError::Overflow)
    }
}

impl Sub for Money {
    type Output = Result<Money, MoneyError>;

    fn sub(self, rhs: Self) -> Result<Money, MoneyError> {
        if self.currency != rhs.currency {
            return Err(MoneyError::CurrencyMismatch {
                left: self.currency,
                right: rhs.currency,
            });
        }
        self.amount
            .checked_sub(rhs.amount)
            .map(|amount| Money::new(amount, self.currency))
            .ok_or(MoneyError::Overflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mub(amount: i64) -> Money {
        Money::new(amount, Currency::MUB)
    }

    fn usd(amount: i64) -> Money {
        Money::new(amount, Currency::USD)
    }

    #[test]
    fn test_money_new() {
        let m = Money::new(100, Currency::MUB);
        assert_eq!(m.amount(), 100);
        assert_eq!(m.currency(), Currency::MUB);
    }

    #[test]
    fn test_money_zero() {
        let m = Money::zero(Currency::BRL);
        assert_eq!(m.amount(), 0);
        assert_eq!(m.currency(), Currency::BRL);
    }

    #[test]
    fn test_money_display() {
        assert_eq!(mub(10050).to_string(), "100.50 MUB");
        assert_eq!(usd(0).to_string(), "0.00 USD");
    }

    #[test]
    fn test_money_add_same_currency() {
        let result = mub(100) + mub(200);
        assert_eq!(result, Ok(mub(300)));
    }

    #[test]
    fn test_money_add_different_currency() {
        let result = mub(100) + usd(200);
        assert_eq!(
            result,
            Err(MoneyError::CurrencyMismatch {
                left: Currency::MUB,
                right: Currency::USD,
            })
        );
    }

    #[test]
    fn test_money_sub_same_currency() {
        let result = mub(500) - mub(200);
        assert_eq!(result, Ok(mub(300)));
    }

    #[test]
    fn test_money_sub_negative() {
        let result = mub(100) - mub(500);
        assert_eq!(result, Ok(mub(-400)));
    }

    #[test]
    fn test_money_add_overflow() {
        let result = mub(i64::MAX) - mub(-1);
        assert_eq!(result, Err(MoneyError::Overflow));
    }

    #[test]
    fn test_money_display_amount() {
        assert_eq!(mub(12345).display_amount(), "123.45");
        assert_eq!(mub(0).display_amount(), "0.00");
        assert_eq!(mub(1).display_amount(), "0.01");
        assert_eq!(mub(-500).display_amount(), "-5.00");
    }
}
