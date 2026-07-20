use super::error::WorthQuerySessionLabelError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQuerySessionNamespace(String);

impl WorthQuerySessionNamespace {
    pub fn new(value: impl Into<String>) -> Result<Self, WorthQuerySessionLabelError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(WorthQuerySessionLabelError::EmptyNamespace);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for WorthQuerySessionNamespace {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for WorthQuerySessionNamespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
