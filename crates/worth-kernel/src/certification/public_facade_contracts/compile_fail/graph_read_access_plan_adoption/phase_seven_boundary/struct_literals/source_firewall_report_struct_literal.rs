use worth_kernel::graph_read_access_plan_adoption::WorthGraphReadAccessHardDeletionSourceFirewallReport;

fn main() {
    let _ = WorthGraphReadAccessHardDeletionSourceFirewallReport {
        region_rows: Vec::new(),
        scanned_region_count: 0,
        scanned_source_count: 0,
        forbidden_pattern_count: 0,
        violation_count: 0,
        report_digest: String::new(),
    };
}
