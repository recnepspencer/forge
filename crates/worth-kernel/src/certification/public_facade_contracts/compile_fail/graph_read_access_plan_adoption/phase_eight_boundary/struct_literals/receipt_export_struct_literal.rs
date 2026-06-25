use worth_kernel::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionReceiptExport;

fn main() {
    let _ = WorthGraphReadAccessPlanAdoptionReceiptExport {
        report: panic!("public callers cannot fabricate receipt accounting reports"),
        executed_receipt_count: 1,
        admitted_plan_requires_receipt_count: 0,
        required_posture_count: 0,
        denied_posture_count: 0,
        carried_gap_count: 0,
        export_digest: String::new(),
    };
}
