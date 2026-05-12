#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalDecimal(pub String);

impl CanonicalDecimal {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalBigInt(pub String);

impl CanonicalBigInt {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalRational {
    pub numerator: CanonicalBigInt,
    pub denominator: CanonicalBigInt,
}

impl CanonicalRational {
    pub fn new(numerator: CanonicalBigInt, denominator: CanonicalBigInt) -> Option<Self> {
        if denominator.as_str() == "0" {
            None
        } else {
            Some(Self {
                numerator,
                denominator,
            })
        }
    }
}
