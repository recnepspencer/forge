use super::counters::PlanarBooleanPreRegionNormalizationCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanPreRegionNormalizationDenialKind {
    InputIdentityMismatchDenied,
    MissingChainLineageForSharedAreaOutcomeDenied,
    AmbiguousOppositeSenseOverlapOrderingDenied,
    UnstableOrientationTieBreakerDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanPreRegionNormalizationDenial {
    kind: PlanarBooleanPreRegionNormalizationDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanPreRegionNormalizationCounters,
    message: &'static str,
}

impl PlanarBooleanPreRegionNormalizationDenial {
    pub(crate) fn new(
        kind: PlanarBooleanPreRegionNormalizationDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanPreRegionNormalizationCounters,
        message: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            message,
        }
    }

    pub fn kind(&self) -> PlanarBooleanPreRegionNormalizationDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanPreRegionNormalizationCounters {
        self.counters
    }

    pub fn message(&self) -> &'static str {
        self.message
    }
}
