use super::counters::PlanarBooleanDownstreamSplitConsumptionCounters;

pub(crate) fn downstream_split_consumption_identity(
    split_ledger_receipt_identity: &str,
    decision_log_receipt_identity: &str,
    persistent_naming_receipt_identity: &str,
    replay_parity_receipt_identity: &str,
    stage_index_identity: &str,
    counters: PlanarBooleanDownstreamSplitConsumptionCounters,
) -> String {
    format!(
        "planar-boolean-downstream-split-consumption:{}:{}:{}:{}:{}:{}:{}:{}:{}",
        split_ledger_receipt_identity,
        decision_log_receipt_identity,
        persistent_naming_receipt_identity,
        replay_parity_receipt_identity,
        stage_index_identity,
        counters.split_chains_consumed(),
        counters.fragment_rows_consumed(),
        counters.vertex_rows_consumed(),
        counters.persistent_name_rows_consumed()
    )
}
