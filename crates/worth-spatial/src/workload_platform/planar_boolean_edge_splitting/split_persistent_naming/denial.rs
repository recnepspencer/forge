#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSplitPersistentNamingDenialKind {
    SplitChainValidationNotCertified,
    ForeignFragmentSet,
    ForeignOverlapChainSet,
    ForeignSplitVertexSet,
    MissingSplitArtifact,
    DuplicatePersistentName,
    DanglingPersistentNameReference,
    GeometryOrDisplayAuthorityRejected,
    AmbiguousIdentityEvolution,
    IdentityEvolutionBreak,
    DeniedIdentityEvolution,
    AdvisoryIdentityEvolutionNotAuthoritative,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitPersistentNamingDenial {
    kind: PlanarBooleanSplitPersistentNamingDenialKind,
    subject_identity: String,
    message: &'static str,
}

impl PlanarBooleanSplitPersistentNamingDenial {
    pub(crate) fn new(
        kind: PlanarBooleanSplitPersistentNamingDenialKind,
        subject_identity: impl Into<String>,
        message: &'static str,
    ) -> Self {
        Self {
            kind,
            subject_identity: subject_identity.into(),
            message,
        }
    }

    pub fn kind(&self) -> PlanarBooleanSplitPersistentNamingDenialKind {
        self.kind
    }
    pub fn subject_identity(&self) -> &str {
        &self.subject_identity
    }
    pub fn message(&self) -> &'static str {
        self.message
    }
}
