use super::BridgeExecutionBasisCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeExecutionBasisDenialKind {
    InvalidManagedExecutionIntent,
    SignalRuntimeThreadAffinityViolation,
    SignalDeclarationUnavailable,
    SignalAttemptAdmissionFailed,
    SignalAttemptMissing,
    SignalAttemptMismatch,
    SignalManagedQueueBindingFailed,
    TruthBasisMismatch,
    PreviewBasisUnsupported,
    ManagedExecutionIntentAlreadyReserved,
    TruthMaterializationFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeExecutionBasisDenial {
    kind: BridgeExecutionBasisDenialKind,
    detail: String,
    counters: BridgeExecutionBasisCounters,
}

impl BridgeExecutionBasisDenial {
    pub(crate) fn new(
        kind: BridgeExecutionBasisDenialKind,
        detail: impl Into<String>,
        counters: BridgeExecutionBasisCounters,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            counters,
        }
    }

    pub fn kind(&self) -> BridgeExecutionBasisDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn counters(&self) -> &BridgeExecutionBasisCounters {
        &self.counters
    }
}
