mod batch_surface;
mod budget;
mod execution;
mod plan;

pub use batch_surface::{
    AdmittedNarrowBatchReceipt, BroadenedBatchReceipt, CaughtUpContinuationBatch,
    ContinuationBatchId, ContinuationBatchResult, ControlLaneBatchReceipt,
};
pub use budget::{
    ContinuationBatchBudget, FetchWidth, MaxBatchItems, MaxCoveredCommits,
    MaxMaterializedBytes, MaxSupportRowsPerBatch,
};
pub use plan::{ContinuationStrategy, CursorContinuationPlan};

pub(crate) use execution::{
    execute_cursor_continuation, verify_cursor_continuation_budget, ContinuationExecutionEffect,
};
