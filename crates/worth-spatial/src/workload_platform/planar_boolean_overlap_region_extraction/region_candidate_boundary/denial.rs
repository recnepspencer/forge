use super::counters::PlanarBooleanOverlapRegionCandidateBoundaryCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRegionCandidateBoundaryDenialKind {
    InputIdentityMismatchDenied,
    NormalizationSharedAreaMismatchDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanDeniedOverlapRegionCandidateKind {
    MissingNormalizationDenied,
    ContradictoryPromotionPostureDenied,
    MixedBoundaryAreaRequiresFurtherDecompositionDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionCandidateBoundaryDenial {
    kind: PlanarBooleanOverlapRegionCandidateBoundaryDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanOverlapRegionCandidateBoundaryCounters,
    message: &'static str,
}

impl PlanarBooleanOverlapRegionCandidateBoundaryDenial {
    pub(crate) fn new(
        kind: PlanarBooleanOverlapRegionCandidateBoundaryDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanOverlapRegionCandidateBoundaryCounters,
        message: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            message,
        }
    }

    pub fn kind(&self) -> PlanarBooleanOverlapRegionCandidateBoundaryDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanOverlapRegionCandidateBoundaryCounters {
        self.counters
    }

    pub fn message(&self) -> &'static str {
        self.message
    }
}
