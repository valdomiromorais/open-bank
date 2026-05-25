use crate::currency::Currency;
use rust_decimal::Decimal;
use std::fmt;
use std::ops::{Add, Sub};

/// A monetary value composed of a decimal amount and a currency.
/// #[ptbr] Amount usa rust_decimal para representação exata sem ponto flutuante.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    pub amount: Decimal,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        Money { amount, currency }
    }

    pub fn zero(currency: Currency) -> Self {
        Money {
            amount: Decimal::ZERO,
            currency,
        }
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = self.currency.decimals() as u32;
        let amount = self.amount.round_dp(d);
        let s = amount.to_string();
        match s.split_once('.') {
            Some((int, frac)) => {
                let padding = (d as usize).saturating_sub(frac.len());
                write!(f, "{}.{}{} {}", int, frac, "0".repeat(padding), self.currency.code())
            }
            None => {
                write!(f, "{}.{} {}", s, "0".repeat(d as usize), self.currency.code())
            }
        }
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
    use rust_decimal_macros::dec;

    fn mub(amount: Decimal) -> Money {
        Money::new(amount, Currency::MUB)
    }

    fn usd(amount: Decimal) -> Money {
        Money::new(amount, Currency::USD)
    }

    #[test]
    fn test_money_new() {
        let m = Money::new(dec!(10.50), Currency::MUB);
        assert_eq!(m.amount, dec!(10.50));
        assert_eq!(m.currency, Currency::MUB);
    }

    #[test]
    fn test_money_zero() {
        let m = Money::zero(Currency::BRL);
        assert_eq!(m.amount, Decimal::ZERO);
        assert_eq!(m.currency, Currency::BRL);
    }

    #[test]
    fn test_money_display() {
        assert_eq!(mub(dec!(100.50)).to_string(), "100.50 MUB");
        assert_eq!(usd(dec!(0)).to_string(), "0.00 USD");
    }

    #[test]
    fn test_money_add_same_currency() {
        let result = mub(dec!(100)) + mub(dec!(200));
        assert_eq!(result, Ok(mub(dec!(300))));
    }

    #[test]
    fn test_money_add_different_currency() {
        let result = mub(dec!(100)) + usd(dec!(200));
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
        let result = mub(dec!(500)) - mub(dec!(200));
        assert_eq!(result, Ok(mub(dec!(300))));
    }

    #[test]
    fn test_money_sub_negative() {
        let result = mub(dec!(100)) - mub(dec!(500));
        assert_eq!(result, Ok(mub(dec!(-400))));
    }

    #[test]
    fn test_money_add_overflow() {
        let max = Money::new(Decimal::MAX, Currency::MUB);
        let one = mub(dec!(1));
        let result = max + one;
        assert_eq!(result, Err(MoneyError::Overflow));
    }

    #[test]
    fn test_money_dec_literal() {
        let m = Money {
            amount: dec!(10.50),
            currency: Currency::BRL,
        };
        assert_eq!(m.to_string(), "10.50 BRL");
    }
}
