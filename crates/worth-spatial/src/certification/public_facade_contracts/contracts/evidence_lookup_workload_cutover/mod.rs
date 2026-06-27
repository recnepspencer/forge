use worth_spatial::facade::evidence_lookup_workload_cutover::{
    EvidenceLookupConsumedWorkloadHandoff, EvidenceLookupMilestoneTwelveReplayReadinessPosture,
    EvidenceLookupMilestoneTwelveSeed, EvidenceLookupWorkloadCutoverCounters,
    EvidenceLookupWorkloadCutoverError, EvidenceLookupWorkloadCutoverErrorKind,
};

#[test]
fn spatial_public_api_exports_lookup_workload_cutover_contract() {
    let _: fn(&EvidenceLookupConsumedWorkloadHandoff) -> &str =
        EvidenceLookupConsumedWorkloadHandoff::stage_receipt_identity;
    let _: fn(&EvidenceLookupConsumedWorkloadHandoff) -> &str =
        EvidenceLookupConsumedWorkloadHandoff::workload_stage_index_identity;
    let _: fn(&EvidenceLookupConsumedWorkloadHandoff) -> &str =
        EvidenceLookupConsumedWorkloadHandoff::selected_lookup_plan_digest;
    let _: fn(&EvidenceLookupConsumedWorkloadHandoff) -> &str =
        EvidenceLookupConsumedWorkloadHandoff::lookup_execution_receipt_digest;
    let _: fn(&EvidenceLookupConsumedWorkloadHandoff) -> &str =
        EvidenceLookupConsumedWorkloadHandoff::lookup_product_output_digest;
    let _: fn(&EvidenceLookupConsumedWorkloadHandoff) -> &[String] =
        EvidenceLookupConsumedWorkloadHandoff::covered_family_identities;
    let _: fn(&EvidenceLookupConsumedWorkloadHandoff) -> &EvidenceLookupMilestoneTwelveSeed =
        EvidenceLookupConsumedWorkloadHandoff::milestone_twelve_seed;
    let _: fn(&EvidenceLookupConsumedWorkloadHandoff) -> &EvidenceLookupWorkloadCutoverCounters =
        EvidenceLookupConsumedWorkloadHandoff::counters;
}

#[test]
fn spatial_public_api_exports_lookup_workload_cutover_support_types() {
    let _: fn(&EvidenceLookupMilestoneTwelveSeed) -> &str =
        EvidenceLookupMilestoneTwelveSeed::milestone_eleven_closeout_digest;
    let _: fn(&EvidenceLookupMilestoneTwelveSeed) -> &str =
        EvidenceLookupMilestoneTwelveSeed::selected_lookup_plan_digest;
    let _: fn(&EvidenceLookupMilestoneTwelveSeed) -> &str =
        EvidenceLookupMilestoneTwelveSeed::query_surface_matrix_digest;
    let _: fn(&EvidenceLookupMilestoneTwelveSeed) -> &str =
        EvidenceLookupMilestoneTwelveSeed::query_consumer_kit_closeout_digest;
    let _: fn(&EvidenceLookupMilestoneTwelveSeed) -> &str =
        EvidenceLookupMilestoneTwelveSeed::source_firewall_digest;
    let _: fn(&EvidenceLookupMilestoneTwelveSeed) -> &str =
        EvidenceLookupMilestoneTwelveSeed::residue_audit_digest;
    let _: fn(&EvidenceLookupMilestoneTwelveSeed) -> &str =
        EvidenceLookupMilestoneTwelveSeed::family_coverage_digest;
    let _: fn(&EvidenceLookupMilestoneTwelveSeed) -> &str =
        EvidenceLookupMilestoneTwelveSeed::lookup_execution_receipt_digest;
    let _: fn(&EvidenceLookupMilestoneTwelveSeed) -> &str =
        EvidenceLookupMilestoneTwelveSeed::lookup_product_output_digest;
    let _: fn(
        &EvidenceLookupMilestoneTwelveSeed,
    ) -> EvidenceLookupMilestoneTwelveReplayReadinessPosture =
        EvidenceLookupMilestoneTwelveSeed::replay_readiness_posture;
    let _: fn(&EvidenceLookupWorkloadCutoverCounters) -> usize =
        EvidenceLookupWorkloadCutoverCounters::indexed_lookup_count;
    let _: fn(&EvidenceLookupWorkloadCutoverCounters) -> usize =
        EvidenceLookupWorkloadCutoverCounters::raw_row_scan_count;
    let _: fn(&EvidenceLookupWorkloadCutoverError) -> EvidenceLookupWorkloadCutoverErrorKind =
        EvidenceLookupWorkloadCutoverError::kind;
    let _: fn(&EvidenceLookupWorkloadCutoverError) -> &str =
        EvidenceLookupWorkloadCutoverError::detail;
}
