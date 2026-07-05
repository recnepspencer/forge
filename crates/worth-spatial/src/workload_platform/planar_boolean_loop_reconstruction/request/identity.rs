use super::counters::PlanarBooleanLoopReconstructionRequestCounters;

pub(crate) fn loop_reconstruction_request_identity(
    split_consumption_identity: &str,
    split_ledger_receipt_identity: &str,
    split_request_identity: &str,
    workload_stage_index_identity: &str,
    selected_plan_digest: &str,
    selected_route_identity_digest: &str,
    touched_closure_digest: &str,
    overlap_identity_digest_count: usize,
    topology_query_posture_digest: &str,
    spatial_query_posture_digest: &str,
    residue_digest: &str,
    source_firewall_digest: &str,
    architecture_claim_digest: &str,
    counters: PlanarBooleanLoopReconstructionRequestCounters,
) -> String {
    format!(
        "planar-boolean-loop-reconstruction-request:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
        split_consumption_identity,
        split_ledger_receipt_identity,
        split_request_identity,
        workload_stage_index_identity,
        selected_plan_digest,
        selected_route_identity_digest,
        touched_closure_digest,
        overlap_identity_digest_count,
        topology_query_posture_digest,
        spatial_query_posture_digest,
        residue_digest,
        source_firewall_digest,
        architecture_claim_digest,
        counters.split_consumption_products_consumed(),
        counters.split_chain_rows_bound()
    )
}
