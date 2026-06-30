#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialCompiledProductFamilyErrorKind {
    DuplicateConsumerCoverage,
    MissingAuthorityBasis,
    MissingConsumerForDeclaration,
    MissingEvidenceSupportRole,
    MissingEquivalencePolicy,
    MissingFamilyIdentity,
    MissingLocalityBasis,
    MissingPriorProofRole,
    NoDeclaredFamilyForConsumer,
    SchemaVocabularyAdmissionFailed,
    UnsupportedConsumerBasis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialCompiledProductFamilyError {
    kind: SpatialCompiledProductFamilyErrorKind,
    detail: String,
}

impl SpatialCompiledProductFamilyError {
    pub fn new(kind: SpatialCompiledProductFamilyErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> SpatialCompiledProductFamilyErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
