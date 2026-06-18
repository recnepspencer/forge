use forge_query::facade::consumer_kit::ForgeQuerySupportPinReport;

fn main() {
    let _ = ForgeQuerySupportPinReport {
        consumer_name: String::new(),
        contract_digest: String::new(),
        observed_schema_identity: String::new(),
        observed_source_matrix_digest: String::new(),
        observed_snapshot_digest: String::new(),
        requirement_count: 0,
        observed_count: 0,
        matched_required_count: 0,
        snapshot_row_count: 0,
        finding_count: 0,
        blocking_finding_count: 0,
        findings: Vec::new(),
        report_digest: String::new(),
    };
}
