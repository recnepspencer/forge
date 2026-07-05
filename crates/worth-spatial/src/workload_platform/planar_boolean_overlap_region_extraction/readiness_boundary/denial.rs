use super::counters::{
    PlanarBooleanOverlapReadinessLoopLedgerBindingCounters,
    PlanarBooleanOverlapRegionExtractionRequestCounters,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapReadinessLoopLedgerBindingDenialKind {
    MissingLoopLedgerReceiptIdentity,
    MissingLoopLedgerRequestIdentity,
    MissingLoopDecisionLogIdentity,
    MissingLoopIdentityMapIdentity,
    MissingPersistentNameMapIdentity,
    MissingSubshapeSignatureMapIdentity,
    MissingLoopLedgerRows,
    SelectedPlanDigestMismatch,
    SelectedRouteIdentityMismatch,
    SelectedFamilyIdentityMismatch,
    SelectedProductIdentityMismatch,
    SelectedWitnessIdentityMismatch,
    TouchedClosureMismatch,
    OverlapIdentityMismatch,
    TopologyQueryPostureMismatch,
    SpatialQueryPostureMismatch,
    ResidueMismatch,
    SourceFirewallMismatch,
    ArchitectureClaimMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapReadinessLoopLedgerBindingDenial {
    kind: PlanarBooleanOverlapReadinessLoopLedgerBindingDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanOverlapReadinessLoopLedgerBindingCounters,
    human_reason: &'static str,
}

impl PlanarBooleanOverlapReadinessLoopLedgerBindingDenial {
    pub(crate) fn new(
        kind: PlanarBooleanOverlapReadinessLoopLedgerBindingDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanOverlapReadinessLoopLedgerBindingCounters,
        human_reason: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            human_reason,
        }
    }

    pub fn kind(&self) -> PlanarBooleanOverlapReadinessLoopLedgerBindingDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanOverlapReadinessLoopLedgerBindingCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &'static str {
        self.human_reason
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRegionExtractionRequestDenialKind {
    ReadinessLoopLedgerBindingRejected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionExtractionRequestDenial {
    kind: PlanarBooleanOverlapRegionExtractionRequestDenialKind,
    binding_denial: PlanarBooleanOverlapReadinessLoopLedgerBindingDenial,
    counters: PlanarBooleanOverlapRegionExtractionRequestCounters,
}

impl PlanarBooleanOverlapRegionExtractionRequestDenial {
    pub(crate) fn from_binding_denial(
        binding_denial: PlanarBooleanOverlapReadinessLoopLedgerBindingDenial,
        counters: PlanarBooleanOverlapRegionExtractionRequestCounters,
    ) -> Self {
        Self {
            kind: PlanarBooleanOverlapRegionExtractionRequestDenialKind::ReadinessLoopLedgerBindingRejected,
            binding_denial,
            counters,
        }
    }

    pub fn kind(&self) -> PlanarBooleanOverlapRegionExtractionRequestDenialKind {
        self.kind
    }

    pub fn binding_denial(&self) -> &PlanarBooleanOverlapReadinessLoopLedgerBindingDenial {
        &self.binding_denial
    }

    pub fn counters(&self) -> PlanarBooleanOverlapRegionExtractionRequestCounters {
        self.counters
    }
}
