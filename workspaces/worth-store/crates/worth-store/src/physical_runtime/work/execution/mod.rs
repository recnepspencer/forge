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
    PhysicalWriteExecutorCommand, PhysicalWritePosture,
};
pub use joined_outcome::{
    PhysicalSignalSettlementOutcome, PhysicalWorkBatchDenial, PhysicalWorkExecutionBatchOutcome,
    PhysicalWorkExecutionOutcome,
};
pub use outcome::CompletedPhysicalPublicationEffect;
pub(in crate::physical_runtime) use outcome::{
    IndeterminatePhysicalPublicationEffect, PhysicalEffectRecoveryObligation,
    PhysicalExecutorDispatch, PhysicalExecutorOutcome,
};
pub(in crate::physical_runtime) use settlement::PhysicalWorkSettlement;
pub use settlement::{
    PhysicalWorkEffectFate, PhysicalWorkHealthRevocation, PhysicalWorkNoEffectEvidence,
    PhysicalWorkPublicationResiduePosture, PhysicalWorkResidencyPosture,
    PhysicalWorkSchedulerPosture, PhysicalWorkSettlementEvidence, PhysicalWorkTerminalCause,
    PhysicalWorkTerminalFailure,
};
