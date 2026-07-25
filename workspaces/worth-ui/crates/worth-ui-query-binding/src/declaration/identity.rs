use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryViewIdentityError {
    Empty,
    ContainsWhitespace,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorthUiQueryViewIdentity(Arc<str>);

impl WorthUiQueryViewIdentity {
    pub fn new(value: impl Into<String>) -> Result<Self, WorthUiQueryViewIdentityError> {
        let value = value.into();
        if value.is_empty() {
            return Err(WorthUiQueryViewIdentityError::Empty);
        }
        if value.chars().any(char::is_whitespace) {
            return Err(WorthUiQueryViewIdentityError::ContainsWhitespace);
        }
        Ok(Self(Arc::from(value)))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
