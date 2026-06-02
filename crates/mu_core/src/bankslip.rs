use crate::account::AccountId;
use crate::money::Money;
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

/// #[ptbr] Status do boleto — ciclo de vida do instrumento.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlipStatus {
    Issued,
    Paid,
    Cancelled,
    Expired,
}

/// #[ptbr] Boleto bancário brasileiro — ordem de pagamento com código de barras.
/// Representa o instrumento em si, não a transação financeira.
#[derive(Debug, Clone)]
pub struct BankSlip {
    id: Uuid,
    account_id: AccountId,
    amount: Money,
    code: String,
    status: SlipStatus,
    due_date: NaiveDate,
    issued_at: DateTime<Utc>,
    paid_at: Option<DateTime<Utc>>,
}

impl BankSlip {
    pub fn new(account_id: AccountId, amount: Money, code: String, due_date: NaiveDate) -> Self {
        BankSlip {
            id: Uuid::new_v4(),
            account_id,
            amount,
            code,
            status: SlipStatus::Issued,
            due_date,
            issued_at: Utc::now(),
            paid_at: None,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn account_id(&self) -> AccountId {
        self.account_id
    }

    pub fn amount(&self) -> Money {
        self.amount
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn status(&self) -> SlipStatus {
        self.status
    }

    pub fn due_date(&self) -> NaiveDate {
        self.due_date
    }

    pub fn issued_at(&self) -> DateTime<Utc> {
        self.issued_at
    }

    pub fn paid_at(&self) -> Option<DateTime<Utc>> {
        self.paid_at
    }

    pub fn pay(&mut self) {
        self.status = SlipStatus::Paid;
        self.paid_at = Some(Utc::now());
    }

    pub fn cancel(&mut self) {
        self.status = SlipStatus::Cancelled;
    }

    pub fn expire(&mut self) {
        self.status = SlipStatus::Expired;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::Currency;
    use rust_decimal_macros::dec;

    fn sample_slip() -> BankSlip {
        let acc = AccountId::new();
        BankSlip::new(
            acc,
            Money::new(dec!(150.00), Currency::BRL),
            "00190.00009 01234.567890 12345.678901 1 12345678901234".into(),
            NaiveDate::from_ymd_opt(2026, 6, 15).unwrap(),
        )
    }

    #[test]
    fn test_bankslip_new() {
        let slip = sample_slip();
        assert_eq!(slip.status(), SlipStatus::Issued);
        assert_eq!(slip.amount(), Money::new(dec!(150.00), Currency::BRL));
        assert!(slip.paid_at().is_none());
    }

    #[test]
    fn test_bankslip_lifecycle() {
        let mut slip = sample_slip();
        assert_eq!(slip.status(), SlipStatus::Issued);

        slip.pay();
        assert_eq!(slip.status(), SlipStatus::Paid);
        assert!(slip.paid_at().is_some());
    }

    #[test]
    fn test_bankslip_cancel() {
        let mut slip = sample_slip();
        slip.cancel();
        assert_eq!(slip.status(), SlipStatus::Cancelled);
    }

    #[test]
    fn test_bankslip_expire() {
        let mut slip = sample_slip();
        slip.expire();
        assert_eq!(slip.status(), SlipStatus::Expired);
    }
}
