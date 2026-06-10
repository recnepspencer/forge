use super::error::ForgeQuerySessionLabelError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ForgeQuerySessionLabelSegment(String);

impl ForgeQuerySessionLabelSegment {
    pub fn new(value: impl Into<String>) -> Result<Self, ForgeQuerySessionLabelError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ForgeQuerySessionLabelError::EmptyNameSegment);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ForgeQuerySessionLabelSegment {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ForgeQuerySessionLabelSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
