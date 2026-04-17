use forge_query::facade::{admit_historical_evaluation_path, HistoricalEvaluationRequest};

type WrongFn = fn(
    HistoricalEvaluationRequest,
    forge_runtime_bridge::facade::ResolvedTruthViewPolicy,
) -> Result<
    forge_query::facade::HistoricalEvaluationAdmission,
    forge_query::facade::HistoricalEvaluationError,
>;

fn _expects_query_owned_policy(_: WrongFn) {}

fn main() {
    _expects_query_owned_policy(admit_historical_evaluation_path);
}
