use worth_query::facade::foundation::{resolve_historical_materialization_path, HistoricalEvaluationAdmission, HistoricalMaterializationDescriptor};

type WrongFn = fn(
    HistoricalEvaluationAdmission,
    worth_runtime_bridge::facade::BridgeHistoricalEvaluationDecisionLog,
) -> Result<
    worth_query::facade::foundation::HistoricalPathResolved,
    worth_query::facade::foundation::HistoricalEvaluationError,
>;

fn _expects_query_owned_materialization(_: WrongFn) {}

fn main() {
    _expects_query_owned_materialization(resolve_historical_materialization_path);
}
