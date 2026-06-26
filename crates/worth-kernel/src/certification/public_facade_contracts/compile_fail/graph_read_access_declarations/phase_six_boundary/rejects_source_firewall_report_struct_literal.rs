use worth_kernel::graph_read_access_declarations::WorthGraphReadDeclarationSourceFirewallReport;

fn main() {
    let _ = WorthGraphReadDeclarationSourceFirewallReport {
        region_reports: Vec::new(),
        scanned_source_count: 0,
        violation_count: 0,
        violations: Vec::new(),
        report_digest: String::new(),
    };
}
