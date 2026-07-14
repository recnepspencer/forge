use worth_query::facade::foundation::{admit_historical_evaluation_path, HistoricalEvaluationRequest};

type WrongFn = fn(
    HistoricalEvaluationRequest,
    worth_runtime_bridge::facade::ResolvedTruthViewPolicy,
) -> Result<
    worth_query::facade::foundation::HistoricalEvaluationAdmission,
    worth_query::facade::foundation::HistoricalEvaluationError,
>;

fn _expects_query_owned_policy(_: WrongFn) {}

fn main() {
    _expects_query_owned_policy(admit_historical_evaluation_path);
}
