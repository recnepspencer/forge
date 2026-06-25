use worth_kernel::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionDeletionExport;

fn main() {
    let _ = WorthGraphReadAccessPlanAdoptionDeletionExport {
        report: panic!("public callers cannot fabricate deletion reports"),
        export_digest: String::new(),
    };
}
