use super::counters::PlanarBooleanOverlapIslandComponentCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapIslandComponentDenialKind {
    InputIdentityMismatchDenied,
    MissingCellContainmentEvidenceDenied,
    MissingCellWindingEvidenceDenied,
    UnsupportedCellOverlapSignalDenied,
    ContradictoryComponentMembershipDenied,
    MixedIslandPartitionDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapIslandComponentDenial {
    kind: PlanarBooleanOverlapIslandComponentDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanOverlapIslandComponentCounters,
    human_reason: &'static str,
}

impl PlanarBooleanOverlapIslandComponentDenial {
    pub(crate) fn new(
        kind: PlanarBooleanOverlapIslandComponentDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanOverlapIslandComponentCounters,
        human_reason: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            human_reason,
        }
    }

    pub fn kind(&self) -> PlanarBooleanOverlapIslandComponentDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanOverlapIslandComponentCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &str {
        self.human_reason
    }
}
