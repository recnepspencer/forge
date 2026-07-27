use std::marker::PhantomData;

pub trait Currency: Copy + std::fmt::Debug + Eq + 'static {
    const CODE: &'static str;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USD;

impl Currency for USD {
    const CODE: &'static str = "USD";
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Money<C: Currency> {
    minor_units: i64,
    _currency: PhantomData<C>,
}

impl<C: Currency> Money<C> {
    pub const fn from_minor(minor_units: i64) -> Result<Self, MoneyError> {
        if minor_units <= 0 {
            return Err(MoneyError::NotPositive);
        }
        Ok(Self {
            minor_units,
            _currency: PhantomData,
        })
    }

    pub const fn from_signed_minor(minor_units: i64) -> Self {
        Self {
            minor_units,
            _currency: PhantomData,
        }
    }

    pub const fn minor_units(self) -> i64 {
        self.minor_units
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoneyError {
    NotPositive,
}

impl std::fmt::Display for MoneyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("money movement must be positive")
    }
}

impl std::error::Error for MoneyError {}
