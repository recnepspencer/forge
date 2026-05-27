mod artifacts;
mod execution;
mod outcome;
mod request;
mod transcript;

pub use artifacts::{
    ForgeQueryContinuationBasisPosture, ForgeQueryContinuationRuntimeContract,
    ForgeQueryContinuationTruthContext, ForgeQueryContinuationWorkspaceContract,
    ForgeQueryPreparedContinuation, ForgeQueryPreparedContinuationExecutionMode,
    ForgeQueryPreparedContinuationFamily, ForgeQueryPreparedContinuationSignalPosture,
};
pub use execution::ForgeQueryContinuationExecution;
pub use outcome::{
    ordinary_outcome_from_continuation_checked, ordinary_outcome_from_execution_checked,
    ForgeQueryContinuationExecutionChecked, ForgeQueryContinuationExecutionOutcome,
    ForgeQueryPreparedContinuationChecked, ForgeQueryPreparedContinuationOutcome,
};
pub use request::{
    ForgeQueryExecutePreparedContinuationRequest, ForgeQueryPreparedContinuationRequest,
};
pub use transcript::{
    ForgeQueryContinuationExecutionTranscript, ForgeQueryPreparedContinuationTranscript,
};

pub(crate) use execution::{
    execute_prepared_continuation_on_handle, prepare_continuation_from_context_on_handle,
    prepare_continuation_from_signal_checked_on_handle, prepare_continuation_from_target_on_handle,
};

#[cfg(test)]
mod tests;
