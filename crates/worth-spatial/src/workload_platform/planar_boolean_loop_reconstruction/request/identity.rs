use super::counters::PlanarBooleanLoopReconstructionRequestCounters;

pub(crate) fn loop_reconstruction_request_identity(
    split_consumption_identity: &str,
    split_ledger_receipt_identity: &str,
    split_request_identity: &str,
    workload_stage_index_identity: &str,
    counters: PlanarBooleanLoopReconstructionRequestCounters,
) -> String {
    format!(
        "planar-boolean-loop-reconstruction-request:{}:{}:{}:{}:{}:{}",
        split_consumption_identity,
        split_ledger_receipt_identity,
        split_request_identity,
        workload_stage_index_identity,
        counters.split_consumption_products_consumed(),
        counters.split_chain_rows_bound()
    )
}
