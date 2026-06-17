use super::counters::PlanarBooleanSplitDecisionLogCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSplitDecisionLogDenialKind {
    EmptyQueryDeclarationIdentity,
    GeometryOrDisplayAuthorityRejected,
    ForeignSplitRequestProduct,
    ForeignEndpointBoundaryProduct,
    ForeignIntervalSubdivisionProduct,
    ForeignSplitVertexProduct,
    ForeignSplitFragmentProduct,
    ForeignChainValidationProduct,
    ForeignPersistentNamingProduct,
    DuplicateDecisionIdentity,
    MissingDecisionCoverage,
    MissingDecisionIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitDecisionLogDenial {
    kind: PlanarBooleanSplitDecisionLogDenialKind,
    evidence_identity: String,
    counters: PlanarBooleanSplitDecisionLogCounters,
    human_reason: String,
}

impl PlanarBooleanSplitDecisionLogDenial {
    pub(crate) fn new(
        kind: PlanarBooleanSplitDecisionLogDenialKind,
        evidence_identity: impl Into<String>,
        counters: PlanarBooleanSplitDecisionLogCounters,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            counters,
            human_reason: human_reason.into(),
        }
    }
    pub fn kind(&self) -> PlanarBooleanSplitDecisionLogDenialKind {
        self.kind
    }
    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }
    pub fn counters(&self) -> PlanarBooleanSplitDecisionLogCounters {
        self.counters
    }
    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
