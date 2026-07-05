use super::counters::PlanarBooleanBoundaryContactClassificationCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanBoundaryContactClassificationDenialKind {
    InputIdentityMismatchDenied,
    ContradictoryIslandComponentMembershipDenied,
    MixedBoundaryAreaRequiresCellDecompositionDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanBoundaryContactClassificationDenial {
    kind: PlanarBooleanBoundaryContactClassificationDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanBoundaryContactClassificationCounters,
    human_reason: &'static str,
}

impl PlanarBooleanBoundaryContactClassificationDenial {
    pub(crate) fn new(
        kind: PlanarBooleanBoundaryContactClassificationDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanBoundaryContactClassificationCounters,
        human_reason: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            human_reason,
        }
    }

    pub fn kind(&self) -> PlanarBooleanBoundaryContactClassificationDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanBoundaryContactClassificationCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
