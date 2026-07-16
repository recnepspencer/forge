use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CanonicalDecimal(pub String);

impl CanonicalDecimal {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn is_canonical(&self) -> bool {
        canonical_decimal_text(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CanonicalBigInt(pub String);

impl CanonicalBigInt {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn is_canonical(&self) -> bool {
        canonical_integer_text(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CanonicalRational {
    pub numerator: CanonicalBigInt,
    pub denominator: CanonicalBigInt,
}

impl CanonicalRational {
    pub fn new(numerator: CanonicalBigInt, denominator: CanonicalBigInt) -> Option<Self> {
        if canonical_big_int_string_is_zero(denominator.as_str()) {
            None
        } else {
            Some(Self {
                numerator,
                denominator,
            })
        }
    }

    pub(crate) fn is_canonical(&self) -> bool {
        self.numerator.is_canonical()
            && self.denominator.is_canonical()
            && !canonical_big_int_string_is_zero(self.denominator.as_str())
    }
}

fn canonical_decimal_text(value: &str) -> bool {
    let unsigned = unsigned_numeric_text(value);
    let mut segments = unsigned.split('.');
    let integer = segments.next().unwrap_or_default();
    let fraction = segments.next();
    integer.chars().all(|digit| digit.is_ascii_digit())
        && !integer.is_empty()
        && fraction.is_none_or(|digits| {
            !digits.is_empty() && digits.chars().all(|digit| digit.is_ascii_digit())
        })
        && segments.next().is_none()
}

fn canonical_integer_text(value: &str) -> bool {
    let unsigned = unsigned_numeric_text(value);
    !unsigned.is_empty() && unsigned.chars().all(|digit| digit.is_ascii_digit())
}

fn unsigned_numeric_text(value: &str) -> &str {
    value
        .strip_prefix('+')
        .or_else(|| value.strip_prefix('-'))
        .unwrap_or(value)
}

fn canonical_big_int_string_is_zero(value: &str) -> bool {
    let unsigned_digits = value
        .strip_prefix('+')
        .or_else(|| value.strip_prefix('-'))
        .unwrap_or(value);

    !unsigned_digits.is_empty() && unsigned_digits.chars().all(|digit| digit == '0')
}
