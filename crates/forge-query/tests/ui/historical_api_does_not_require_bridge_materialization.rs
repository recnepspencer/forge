use forge_query::facade::{
    resolve_historical_materialization_path, HistoricalEvaluationAdmission,
    HistoricalMaterializationDescriptor,
};

type WrongFn = fn(
    HistoricalEvaluationAdmission,
    forge_runtime_bridge::facade::BridgeHistoricalEvaluationDecisionLog,
) -> Result<
    forge_query::facade::HistoricalPathResolved,
    forge_query::facade::HistoricalEvaluationError,
>;

fn _expects_query_owned_materialization(_: WrongFn) {}

fn main() {
    _expects_query_owned_materialization(resolve_historical_materialization_path);
}
