use worth_kernel::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionResidueExport;

fn main() {
    let _ = WorthGraphReadAccessPlanAdoptionResidueExport {
        report: panic!("public callers cannot fabricate residue reports"),
        export_digest: String::new(),
    };
}
