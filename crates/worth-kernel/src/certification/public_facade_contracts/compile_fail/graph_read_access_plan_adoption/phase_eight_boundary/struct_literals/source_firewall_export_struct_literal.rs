use worth_kernel::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionSourceFirewallExport;

fn main() {
    let _ = WorthGraphReadAccessPlanAdoptionSourceFirewallExport {
        report: panic!("public callers cannot fabricate source firewall reports"),
        export_digest: String::new(),
    };
}
