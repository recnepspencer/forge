use super::counters::PlanarBooleanDownstreamSplitConsumptionCounters;

pub(crate) fn downstream_split_consumption_identity(
    split_ledger_receipt_identity: &str,
    decision_log_receipt_identity: &str,
    persistent_naming_receipt_identity: &str,
    replay_parity_receipt_identity: &str,
    spatial_lookup_key: &str,
    spatial_lookup_product_digest: &str,
    counters: PlanarBooleanDownstreamSplitConsumptionCounters,
) -> String {
    format!(
        "planar-boolean-downstream-split-consumption:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
        split_ledger_receipt_identity,
        decision_log_receipt_identity,
        persistent_naming_receipt_identity,
        replay_parity_receipt_identity,
        spatial_lookup_key,
        spatial_lookup_product_digest,
        counters.split_chains_consumed(),
        counters.fragment_rows_consumed(),
        counters.vertex_rows_consumed(),
        counters.persistent_name_rows_consumed(),
        counters.spatial_lookup_indexed_lookups()
    )
}
