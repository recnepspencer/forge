#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapEdgeChainDenialKind {
    ForeignFragmentSet,
    MissingFragmentReference,
    MissingSubdivisionReference,
    MismatchedFragmentAuthority,
    AmbiguousOverlapChainBasis,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapEdgeChainDenial {
    denial_kind: PlanarBooleanOverlapEdgeChainDenialKind,
    evidence_identity: String,
    message: &'static str,
}

impl PlanarBooleanOverlapEdgeChainDenial {
    pub(crate) fn new(
        denial_kind: PlanarBooleanOverlapEdgeChainDenialKind,
        evidence_identity: impl Into<String>,
        message: &'static str,
    ) -> Self {
        Self {
            denial_kind,
            evidence_identity: evidence_identity.into(),
            message,
        }
    }

    pub fn denial_kind(&self) -> PlanarBooleanOverlapEdgeChainDenialKind {
        self.denial_kind
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn message(&self) -> &'static str {
        self.message
    }
}
