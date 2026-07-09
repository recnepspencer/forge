mod artifacts;
mod execution;
mod outcome;
mod readmission;
mod request;
mod transcript;

pub use artifacts::{
    WorthQueryContinuationBasisPosture, WorthQueryContinuationRuntimeContract,
    WorthQueryContinuationTruthContext, WorthQueryContinuationWorkspaceContract,
    WorthQueryPreparedContinuation, WorthQueryPreparedContinuationExecutionMode,
    WorthQueryPreparedContinuationFamily, WorthQueryPreparedContinuationSignalPosture,
};
pub use execution::WorthQueryContinuationExecution;
pub use outcome::{
    ordinary_outcome_from_continuation_checked, ordinary_outcome_from_execution_checked,
    WorthQueryContinuationExecutionChecked, WorthQueryContinuationExecutionOutcome,
    WorthQueryPreparedContinuationChecked, WorthQueryPreparedContinuationOutcome,
};
pub use readmission::{
    WorthQueryPreparedContinuationAuthorityWitness, WorthQueryPreparedContinuationBasisKind,
    WorthQueryPreparedContinuationBasisWitness, WorthQueryPreparedContinuationDriftKind,
    WorthQueryPreparedContinuationExecutionReadmission,
    WorthQueryPreparedContinuationFreshnessPosture,
};
pub use request::{
    WorthQueryExecutePreparedContinuationRequest, WorthQueryPreparedContinuationRequest,
};
pub use transcript::{
    WorthQueryContinuationExecutionTranscript, WorthQueryPreparedContinuationTranscript,
};

pub(crate) use execution::{
    execute_prepared_continuation_on_handle, prepare_continuation_from_context_on_handle,
    prepare_continuation_from_signal_checked_on_handle, prepare_continuation_from_target_on_handle,
};

#[cfg(test)]
mod tests;

#[cfg(test)]
pub(crate) use tests::runtime_backed_continuation_closure_summary;
