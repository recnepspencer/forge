#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialSelectedEquivalenceFamilyErrorKind {
    MissingDeclaredFamily,
    SchemaVocabularyAdmissionFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialSelectedEquivalenceFamilyError {
    kind: SpatialSelectedEquivalenceFamilyErrorKind,
    detail: String,
}

impl SpatialSelectedEquivalenceFamilyError {
    pub(crate) fn new(
        kind: SpatialSelectedEquivalenceFamilyErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> SpatialSelectedEquivalenceFamilyErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
