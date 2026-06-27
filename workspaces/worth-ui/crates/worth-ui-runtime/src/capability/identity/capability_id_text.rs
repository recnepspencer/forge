use core::fmt;

use super::capability_id_error::CapabilityIdError;
use super::capability_id_validation::validate_capability_id_text;

/// Canonical validated text shared by all capability identity families.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(super) struct CapabilityIdText {
    canonical_text: String,
}

impl CapabilityIdText {
    pub(super) fn new(raw_text: impl AsRef<str>) -> Result<Self, CapabilityIdError> {
        let raw_text = raw_text.as_ref();
        validate_capability_id_text(raw_text)?;
        Ok(Self {
            canonical_text: raw_text.to_owned(),
        })
    }

    pub(super) fn as_str(&self) -> &str {
        &self.canonical_text
    }
}

impl fmt::Debug for CapabilityIdText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CapabilityIdText")
            .field(&self.as_str())
            .finish()
    }
}
