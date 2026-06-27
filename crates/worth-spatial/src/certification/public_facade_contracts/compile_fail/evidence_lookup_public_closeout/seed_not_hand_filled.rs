use worth_spatial::facade::evidence_lookup_workload_cutover::EvidenceLookupMilestoneTwelveSeed;

fn requires_seed(_: EvidenceLookupMilestoneTwelveSeed) {}
fn fake<T>() -> T {
    panic!("compile-fail placeholder")
}

fn main() {
    requires_seed(EvidenceLookupMilestoneTwelveSeed {
        milestone_eleven_closeout_digest: String::new(),
        selected_lookup_plan_digest: String::new(),
        lookup_execution_receipt_digest: String::new(),
        lookup_product_output_digest: String::new(),
        covered_family_identities: Vec::new(),
        query_surface_matrix_digest: String::new(),
        query_consumer_kit_closeout_digest: String::new(),
        source_firewall_digest: String::new(),
        residue_audit_digest: String::new(),
        family_coverage_digest: String::new(),
        replay_readiness_posture: fake(),
    });
}
