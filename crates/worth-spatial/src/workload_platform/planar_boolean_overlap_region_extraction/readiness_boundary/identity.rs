use super::binding::PlanarBooleanOverlapReadinessLoopLedgerBinding;
use super::counters::{
    PlanarBooleanOverlapReadinessLoopLedgerBindingCounters,
    PlanarBooleanOverlapRegionExtractionRequestCounters,
};

pub(crate) fn overlap_readiness_loop_ledger_binding_identity(
    selected_route_identity_digest: &str,
    selected_plan_digest: &str,
    touched_closure_digest: &str,
    overlap_identity_digests: &[String],
    topology_query_posture_digest: &str,
    spatial_query_posture_digest: &str,
    residue_digest: &str,
    source_firewall_digest: &str,
    architecture_claim_digest: &str,
    loop_ledger_receipt_identity: &str,
    loop_ledger_request_identity: &str,
    counters: PlanarBooleanOverlapReadinessLoopLedgerBindingCounters,
) -> String {
    format!(
        "planar-boolean-overlap-readiness-loop-ledger-binding:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
        selected_route_identity_digest,
        selected_plan_digest,
        touched_closure_digest,
        overlap_identity_digests.join("|"),
        topology_query_posture_digest,
        spatial_query_posture_digest,
        residue_digest,
        source_firewall_digest,
        architecture_claim_digest,
        loop_ledger_receipt_identity,
        loop_ledger_request_identity,
        counters.readiness_consumers_consumed(),
        counters.loop_ledger_receipts_consumed(),
        counters.provenance_mismatches_rejected(),
    )
}

pub(crate) fn overlap_region_extraction_request_identity(
    binding: &PlanarBooleanOverlapReadinessLoopLedgerBinding,
    counters: PlanarBooleanOverlapRegionExtractionRequestCounters,
) -> String {
    format!(
        "planar-boolean-overlap-region-extraction-request:{}:{}:{}:{}",
        binding.binding_identity(),
        binding.loop_ledger_receipt_identity(),
        counters.readiness_bindings_consumed(),
        counters.loop_ledger_rows_bound(),
    )
}
