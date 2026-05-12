#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalString(String);

impl CanonicalString {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for CanonicalString {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for CanonicalString {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
