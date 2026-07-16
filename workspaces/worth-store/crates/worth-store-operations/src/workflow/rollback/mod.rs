mod execution;
mod intent;
mod lowering;

pub use execution::{
    ExecutedRollback, ExecutionReadyRollback, RollbackExecutionDenial, RollbackOperationReceipt,
    RollbackReadinessDenial,
};
pub use intent::{
    AdmittedRollbackSourceOperation, EvidenceBoundRollbackPlan, ResolvedRollbackOperation,
    RollbackIntent, RollbackResolutionDenial, RollbackSourceAdmissionDenial,
};
pub use lowering::{AuthorizedRollbackPlan, LoweredRollbackPlanDag, RollbackLoweringDenial};

#[derive(Debug, Clone, Copy)]
pub(crate) struct RollbackOperation;
