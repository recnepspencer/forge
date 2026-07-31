mod command;
mod joined_outcome;
mod outcome;
pub(super) mod settlement;

pub use command::{
    PhysicalExecutorCommand, PhysicalExecutorCommandDenial, PhysicalPublicationEffect,
    PhysicalRetryCommand,
};
pub(in crate::physical_runtime) use command::{
    PhysicalMetadataExecutorCommand, PhysicalPublicationExecutorCommand,
    PhysicalReadExecutorCommand, PhysicalResidencyWritebackExecutorCommand, PhysicalRetryPayload,
    PhysicalWalAppendExecutorCommand, PhysicalWalBarrierExecutorCommand,
    PhysicalWriteExecutorCommand,
};
pub use joined_outcome::{
    PhysicalSignalSettlementOutcome, PhysicalWorkBatchDenial, PhysicalWorkExecutionBatchOutcome,
    PhysicalWorkExecutionOutcome,
};
pub use outcome::{CompletedPhysicalPublicationEffect, CompletedPhysicalWalBarrier};
pub(in crate::physical_runtime) use outcome::{
    IndeterminatePhysicalPublicationEffect, IndeterminatePhysicalWalBarrier,
    PhysicalEffectRecoveryObligation, PhysicalExecutorDispatch, PhysicalExecutorOutcome,
    PhysicalResidencyWritebackCompletion,
};
pub(in crate::physical_runtime) use settlement::PhysicalWorkSettlement;
pub use settlement::{
    PhysicalWorkEffectFate, PhysicalWorkHealthRevocation, PhysicalWorkNoEffectEvidence,
    PhysicalWorkPublicationResiduePosture, PhysicalWorkSchedulerPosture,
    PhysicalWorkSettlementEvidence, PhysicalWorkTerminalCause, PhysicalWorkTerminalFailure,
};
