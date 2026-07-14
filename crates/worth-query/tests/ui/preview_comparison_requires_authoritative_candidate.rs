use worth_query::facade::foundation::{ExecutionPreflightBundle, ExecutionResultEnvelope};
use worth_query::facade::policy::PreviewComparisonError;
use worth_query::facade::{admit_preview_execution_comparison, PreviewExecutionComparisonAdmission, PreviewExecutionEnvelope};

fn main() {
    let _: fn(
        &PreviewExecutionEnvelope,
        &ExecutionPreflightBundle,
        &ExecutionResultEnvelope,
    ) -> Result<PreviewExecutionComparisonAdmission, PreviewComparisonError> =
        admit_preview_execution_comparison;
}
