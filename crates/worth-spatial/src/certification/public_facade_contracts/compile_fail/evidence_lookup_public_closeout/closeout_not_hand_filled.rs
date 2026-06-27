use worth_spatial::facade::evidence_lookup_public_closeout::EvidenceLookupPublicCloseout;

fn requires_public_closeout(_: EvidenceLookupPublicCloseout) {}
fn fake<T>() -> T {
    panic!("compile-fail placeholder")
}

fn main() {
    requires_public_closeout(EvidenceLookupPublicCloseout {
        family_stage_rows: Vec::new(),
        query_surface_matrix: fake(),
        query_consumer_kit: fake(),
        source_firewall_report: fake(),
        spatial_deletion_ledger_rows: Vec::new(),
        counters: fake(),
        family_coverage_digest: String::new(),
        spatial_deletion_ledger_digest: String::new(),
        residue_audit_digest: String::new(),
        milestone_twelve_seed: fake(),
        closeout_digest: String::new(),
    });
}
