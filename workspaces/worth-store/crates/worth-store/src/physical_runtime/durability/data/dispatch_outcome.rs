use crate::physical_runtime::durability::{
    DataDispatchedPhysicalMutation, PhysicalDataEffectSettlement,
};
use crate::physical_runtime::{
    CandidateFrameContractViolation, PhysicalRecordMutationFailureEvidence,
    PhysicalRecordWritebackFailureEvidence, RecordAppendDenial, WalDurablePhysicalMutation,
};

pub enum PhysicalDataDispatchOutcome {
    Dispatched(DataDispatchedPhysicalMutation),
    NotStarted {
        durable: WalDurablePhysicalMutation,
        cause: PhysicalDataDispatchFailureCause,
    },
    Indeterminate(IndeterminatePhysicalDataDispatch),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalDataDispatchFailureCause {
    PublicationAuthorityReleased,
    ForeignStore,
    StaleRuntime,
    SignalProfileMismatch,
    Residency(RecordAppendDenial),
    CandidateFrameContract(CandidateFrameContractViolation),
    Canonical(PhysicalRecordMutationFailureEvidence),
    C6Writeback(PhysicalRecordWritebackFailureEvidence),
    IncompleteFrameSet,
    MissingEffectSettlement,
}

pub struct IndeterminatePhysicalDataDispatch {
    durable: WalDurablePhysicalMutation,
    effects: Vec<PhysicalDataEffectSettlement>,
    cause: PhysicalDataDispatchFailureCause,
}

impl IndeterminatePhysicalDataDispatch {
    pub(in crate::physical_runtime) fn new(
        durable: WalDurablePhysicalMutation,
        effects: Vec<PhysicalDataEffectSettlement>,
        cause: PhysicalDataDispatchFailureCause,
    ) -> Self {
        Self {
            durable,
            effects,
            cause,
        }
    }

    pub const fn mutation_identity(&self) -> crate::physical_runtime::PhysicalMutationIdentity {
        self.durable.mutation_identity()
    }

    pub fn completed_frames(&self) -> usize {
        self.effects.len()
    }

    pub const fn durable(&self) -> &WalDurablePhysicalMutation {
        &self.durable
    }

    pub fn effects(&self) -> &[PhysicalDataEffectSettlement] {
        &self.effects
    }

    pub const fn cause(&self) -> &PhysicalDataDispatchFailureCause {
        &self.cause
    }
}
