use crate::contracts::{MoneyValue, Quotation};

/// Wrapper for InvestAPI SDK MoneyValue and Quantity
#[derive(Clone, Copy)]
pub struct Currency(f64);

impl From<&Quotation> for Currency {
    fn from(value: &Quotation) -> Self {
        Currency(value.units as f64 + value.nano as f64 / 1_000_000_000.0)
    }
}

impl From<&MoneyValue> for Currency {
    fn from(value: &MoneyValue) -> Self {
        Currency(value.units as f64 + value.nano as f64 / 1_000_000_000.0)
    }
}

impl From<f64> for Currency {
    fn from(value: f64) -> Self {
        Currency(value)
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}
