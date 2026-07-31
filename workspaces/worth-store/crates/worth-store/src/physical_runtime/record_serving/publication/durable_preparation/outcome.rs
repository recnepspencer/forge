use worth_proof::ProofOutcome;

use super::PreparedPhysicalMutation;
use crate::physical_runtime::{RecordAppendDenial, RecordStreamFailure};

pub type PhysicalMutationPreparationOutcome = ProofOutcome<
    PreparedPhysicalMutation,
    PhysicalMutationPreparationDenial,
    PhysicalMutationPreparationDeferred,
    PhysicalMutationPreparationStale,
    PhysicalMutationPreparationRebindRequired,
    PhysicalMutationPreparationFailure,
>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalMutationPreparationDenial {
    RecordAppend(RecordAppendDenial),
    IdempotencyConflict,
    IdempotencyExpired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalMutationPreparationDeferred {
    PreparedRecordSlots { required_records: u32 },
    PreparedPayloadBytes { required_bytes: u64 },
    PendingUnresolvedLimitReached,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalMutationPreparationStale {
    PublicationAuthorityReleased,
    DurabilityAuthorityReleased,
    WorkOwnerReleased,
    LifecycleGenerationAdvanced,
    AdmissionStopped,
    SignalOwnerUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalMutationPreparationRebindRequired {
    ForeignStore,
    ForeignRuntime,
    ForeignDurabilityPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalMutationPreparationFailure {
    Stream(RecordStreamFailure),
    CanonicalRequestRejected,
    OperationIdentityExhausted,
}
