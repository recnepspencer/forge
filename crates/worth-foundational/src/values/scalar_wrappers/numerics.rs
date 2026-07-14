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
}

fn canonical_big_int_string_is_zero(value: &str) -> bool {
    let unsigned_digits = value
        .strip_prefix('+')
        .or_else(|| value.strip_prefix('-'))
        .unwrap_or(value);

    !unsigned_digits.is_empty() && unsigned_digits.chars().all(|digit| digit == '0')
}
