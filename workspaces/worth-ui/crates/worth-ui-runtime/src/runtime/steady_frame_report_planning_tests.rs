use forge_foundational::FoundationalPerformanceReportMaterializationBoundary;

use super::{
    WorthUiFrameReportMaterializationBoundary, WorthUiOrdinaryLaneCounters,
    WorthUiSteadyFrameCounterBoundary, WorthUiSteadyFrameFoundationalBridge,
    WorthUiSteadyFrameReportPlanner,
};

#[test]
fn steady_frame_report_planner_preserves_foundational_materialization_policy() {
    let evidence = foundational_evidence_for_single_ordinary_lane();

    let claim_only = WorthUiSteadyFrameReportPlanner::claim_inspection_only()
        .plan_from_foundational_receipts(&evidence)
        .expect("claim-only report planning succeeds");
    assert_eq!(
        claim_only.materialization_boundary(),
        WorthUiFrameReportMaterializationBoundary::ClaimInspectionOnly
    );
    assert_eq!(
        claim_only.foundational_boundaries(),
        &[FoundationalPerformanceReportMaterializationBoundary::ClaimInspectionOnly; 2]
    );

    let report_assembly = WorthUiSteadyFrameReportPlanner::report_assembly()
        .plan_from_foundational_receipts(&evidence)
        .expect("report assembly planning succeeds");
    assert_eq!(
        report_assembly.materialization_boundary(),
        WorthUiFrameReportMaterializationBoundary::ReportAssembly
    );
    assert_eq!(
        report_assembly.foundational_boundaries(),
        &[FoundationalPerformanceReportMaterializationBoundary::ReportAssembly; 2]
    );

    let support_request = WorthUiSteadyFrameReportPlanner::support_report()
        .plan_from_foundational_receipts(&evidence)
        .expect("support report planning succeeds");
    assert_eq!(
        support_request.materialization_boundary(),
        WorthUiFrameReportMaterializationBoundary::ReportAssembly
    );
    assert_eq!(
        support_request.foundational_boundaries(),
        &[FoundationalPerformanceReportMaterializationBoundary::ReportAssembly; 2]
    );
}

fn foundational_evidence_for_single_ordinary_lane() -> super::WorthUiSteadyFrameFoundationalEvidence
{
    let mut ordinary = WorthUiOrdinaryLaneCounters::default();
    ordinary.record_frame_row_touch();
    ordinary.record_text_shape();

    let certified = WorthUiSteadyFrameCounterBoundary::for_active_plan(41)
        .record_ordinary_counters_for_test(ordinary)
        .seal()
        .expect("steady frame receipt seals")
        .certify()
        .expect("steady frame receipt certifies");

    WorthUiSteadyFrameFoundationalBridge::lower_counter_receipts(&certified)
        .expect("steady frame evidence lowers")
}
