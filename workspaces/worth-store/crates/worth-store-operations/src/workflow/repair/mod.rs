mod authority_affecting;
mod authority_affecting_execution;
mod authority_owner_dag;
mod authority_receipt_persistence;
mod authority_staging_artifacts;
mod execution;
mod execution_control;
mod intent;
mod journal;
mod journal_replay;
mod lowering;
mod region_projection;
mod resolved_region;

#[cfg(test)]
mod authority_affecting_tests;
#[cfg(test)]
mod crash_recovery_tests;
#[cfg(test)]
mod derived_maintenance_tests;

pub use execution::{
    ExecutedRepair, ExecutedRepairOwnerReceipt, ExecutedRepairOwnerReceiptDag,
    ExecutionReadyRepair, RepairExecutionDenial, RepairReadinessDenial,
};
pub use execution_control::{
    RepairExecutionBoundary, RepairExecutionBoundaryMoment, RepairExecutionControlPort,
    RepairExecutionInterrupted, UninterruptedRepairExecution,
};
pub use intent::{
    CurrentAuthorityPreservingMaintenancePlan, EvidenceBoundRepairPlan, RepairCandidateSet,
    RepairIntent, RepairPlanExplanation, RepairResolutionDenial, UnrecoverableDamageReport,
};
pub use journal::{RepairExecutionDisposition, RepairJournalDenial};
pub use lowering::{AuthorizedRepairPlan, LoweredRepairOwnerPlanDag, RepairLoweringDenial};

#[derive(Debug, Clone, Copy)]
pub(crate) struct DerivedRepairOperation;

#[derive(Debug, Clone, Copy)]
pub(crate) struct AuthorityAffectingRepairOperation;
pub use authority_affecting::{
    AuthorityAffectingRepairLoweringDenial, AuthorityAffectingRepairReadinessDenial,
    AuthorityAffectingStagedRepairPlan, AuthorizedAuthorityAffectingRepairPlan,
    ExecutionReadyAuthorityAffectingRepair, LoweredAuthorityAffectingRepairOwnerPlanDag,
};
pub use authority_affecting_execution::{
    AuthorityAffectingRepairExecutionDenial, ExecutedAuthorityAffectingRepair,
};
