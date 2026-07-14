//! Neutral occurrence identity shared by candidate rows and observed inventory.
//!
//! Not owned by snapshot truth or diagnostic projection. Both evidence sources
//! map into this fact so inventory does not depend on snapshot representation.

/// One retired-fragment hit at a governed path and 1-based line:column location.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct LegacyReferenceOccurrence {
    pub(super) path: String,
    pub(super) location: String,
    pub(super) fragment: String,
}

impl LegacyReferenceOccurrence {
    pub(super) fn new(
        path: impl Into<String>,
        location: impl Into<String>,
        fragment: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            location: location.into(),
            fragment: fragment.into(),
        }
    }

    pub(super) fn subject(&self) -> String {
        format!("{}:{}", self.path, self.location)
    }
}
