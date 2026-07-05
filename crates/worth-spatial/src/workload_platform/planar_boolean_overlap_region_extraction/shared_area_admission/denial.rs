use super::counters::PlanarBooleanSharedAreaAdmissionCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSharedAreaAdmissionDenialKind {
    InputIdentityMismatchDenied,
    ContradictoryIslandComponentMembershipDenied,
    AreaComponentMissingSupportingCellProofDenied,
    MixedBoundaryAreaRequiresCellDecompositionDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSharedAreaAdmissionDenial {
    kind: PlanarBooleanSharedAreaAdmissionDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanSharedAreaAdmissionCounters,
    human_reason: &'static str,
}

impl PlanarBooleanSharedAreaAdmissionDenial {
    pub(crate) fn new(
        kind: PlanarBooleanSharedAreaAdmissionDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanSharedAreaAdmissionCounters,
        human_reason: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            human_reason,
        }
    }

    pub fn kind(&self) -> PlanarBooleanSharedAreaAdmissionDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanSharedAreaAdmissionCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &str {
        self.human_reason
    }
}
