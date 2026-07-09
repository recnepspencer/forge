use worth_query::facade::{
    admit_preview_execution_comparison, ExecutionPreflightBundle, ExecutionResultEnvelope,
    PreviewComparisonError, PreviewExecutionComparisonAdmission, PreviewExecutionEnvelope,
};

fn main() {
    let _: fn(
        &PreviewExecutionEnvelope,
        &ExecutionPreflightBundle,
        &ExecutionResultEnvelope,
    ) -> Result<PreviewExecutionComparisonAdmission, PreviewComparisonError> =
        admit_preview_execution_comparison;
}
