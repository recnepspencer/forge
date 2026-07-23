use super::WorthQueryCertificationCounters;

/// Earliest authority boundary at which a hostile scenario may be rejected.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryCertificationDenialBoundary {
    OperatingWorldEntry,
    FamilyLookup,
    OperationBinding,
    GraphParticipation,
    ConditionalInstallation,
    ExecutionAdmission,
    PublicationAdmission,
    ConsumptionAdmission,
    CompatibilityAdmission,
    SharingAdmission,
    InvalidationAdmission,
    CollectionAdmission,
    LifecycleAdmission,
    ReplayAdmission,
    ReversalAdmission,
}

/// Typed denial plus exact structural work performed before rejection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCertificationDenialEvidence {
    boundary: WorthQueryCertificationDenialBoundary,
    counters: WorthQueryCertificationCounters,
}

impl WorthQueryCertificationDenialEvidence {
    pub fn observed(
        boundary: WorthQueryCertificationDenialBoundary,
        counters: WorthQueryCertificationCounters,
    ) -> Self {
        Self { boundary, counters }
    }

    pub fn boundary(&self) -> WorthQueryCertificationDenialBoundary {
        self.boundary
    }

    pub fn counters(&self) -> &WorthQueryCertificationCounters {
        &self.counters
    }
}
