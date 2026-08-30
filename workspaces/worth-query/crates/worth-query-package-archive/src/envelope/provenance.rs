use crate::denial::WorthQueryPackageArchiveDenial as Denial;

use super::validate_descriptive_text;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPackageReleaseProvenance {
    source_repository: String,
    source_revision: String,
    source_reference: String,
}

impl WorthQueryPackageReleaseProvenance {
    pub fn new(
        source_repository: impl Into<String>,
        source_revision: impl Into<String>,
        source_reference: impl Into<String>,
    ) -> Result<Self, Denial> {
        let value = Self {
            source_repository: source_repository.into(),
            source_revision: source_revision.into(),
            source_reference: source_reference.into(),
        };
        validate_descriptive_text(&value.source_repository)?;
        validate_descriptive_text(&value.source_revision)?;
        validate_descriptive_text(&value.source_reference)?;
        Ok(value)
    }

    pub fn source_repository(&self) -> &str {
        &self.source_repository
    }
    pub fn source_revision(&self) -> &str {
        &self.source_revision
    }
    pub fn source_reference(&self) -> &str {
        &self.source_reference
    }
}
