use super::counters::PlanarBooleanLoopReconstructionSplitConsumptionCounters;

pub(crate) fn loop_reconstruction_split_consumption_identity(
    downstream_consumption_identity: &str,
    split_ledger_receipt_identity: &str,
    split_ledger_downstream_identity: &str,
    split_request_identity: &str,
    workload_stage_index_identity: &str,
    counters: PlanarBooleanLoopReconstructionSplitConsumptionCounters,
) -> String {
    format!(
        "planar-boolean-loop-reconstruction-split-consumption:{}:{}:{}:{}:{}:{}:{}",
        downstream_consumption_identity,
        split_ledger_receipt_identity,
        split_ledger_downstream_identity,
        split_request_identity,
        workload_stage_index_identity,
        counters.receipts_consumed(),
        counters.spatial_lookup_indexed_lookups()
    )
}
