use crate::{
    BackendCapabilityKind, BackendCapabilitySupportPosture, BackendRebindTriggers,
    BackendTargetProfile, CapabilityEvidenceClass, WalDurabilityBarrier,
};

use super::{StoreDurabilityCounterSnapshot, StoreDurabilityOperation, StoreDurabilityState};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreDurabilityDenialKind {
    EvidenceClassTooWeak,
    ExternallyGuaranteedCannotSatisfyCertifiedApi,
    UnsupportedDurabilityCapability,
    UnknownDurabilityPosture,
    StaleDurabilityPosture,
    RebindRequired,
    MissingMediaAssumption,
    MissingRequiredBarrier,
    ExecutionBindingMismatch,
    DelayedSync,
    FailedSync,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StoreDurabilityDenial {
    kind: StoreDurabilityDenialKind,
    state: StoreDurabilityState,
    operation: StoreDurabilityOperation,
    profile: BackendTargetProfile,
    required_evidence: CapabilityEvidenceClass,
    actual_evidence: CapabilityEvidenceClass,
    capability: Option<BackendCapabilityKind>,
    support_posture: Option<BackendCapabilitySupportPosture>,
    missing_barrier: Option<WalDurabilityBarrier>,
    rebind_triggers: Option<BackendRebindTriggers>,
    counters: StoreDurabilityCounterSnapshot,
}

impl StoreDurabilityDenial {
    pub(crate) const fn new(
        kind: StoreDurabilityDenialKind,
        state: StoreDurabilityState,
        operation: StoreDurabilityOperation,
        profile: BackendTargetProfile,
        required_evidence: CapabilityEvidenceClass,
        actual_evidence: CapabilityEvidenceClass,
        counters: StoreDurabilityCounterSnapshot,
    ) -> Self {
        Self {
            kind,
            state,
            operation,
            profile,
            required_evidence,
            actual_evidence,
            capability: None,
            support_posture: None,
            missing_barrier: None,
            rebind_triggers: None,
            counters,
        }
    }

    pub(crate) const fn with_capability(
        mut self,
        capability: BackendCapabilityKind,
        support_posture: BackendCapabilitySupportPosture,
    ) -> Self {
        self.capability = Some(capability);
        self.support_posture = Some(support_posture);
        self
    }

    pub(crate) const fn with_missing_barrier(mut self, barrier: WalDurabilityBarrier) -> Self {
        self.missing_barrier = Some(barrier);
        self
    }

    pub(crate) const fn with_rebind_triggers(mut self, triggers: BackendRebindTriggers) -> Self {
        self.rebind_triggers = Some(triggers);
        self
    }

    pub const fn kind(self) -> StoreDurabilityDenialKind {
        self.kind
    }

    pub const fn state(self) -> StoreDurabilityState {
        self.state
    }

    pub const fn operation(self) -> StoreDurabilityOperation {
        self.operation
    }

    pub const fn profile(self) -> BackendTargetProfile {
        self.profile
    }

    pub const fn required_evidence(self) -> CapabilityEvidenceClass {
        self.required_evidence
    }

    pub const fn actual_evidence(self) -> CapabilityEvidenceClass {
        self.actual_evidence
    }

    pub const fn capability(self) -> Option<BackendCapabilityKind> {
        self.capability
    }

    pub const fn support_posture(self) -> Option<BackendCapabilitySupportPosture> {
        self.support_posture
    }

    pub const fn missing_barrier(self) -> Option<WalDurabilityBarrier> {
        self.missing_barrier
    }

    pub const fn rebind_triggers(self) -> Option<BackendRebindTriggers> {
        self.rebind_triggers
    }

    pub const fn counters(self) -> StoreDurabilityCounterSnapshot {
        self.counters
    }
}
