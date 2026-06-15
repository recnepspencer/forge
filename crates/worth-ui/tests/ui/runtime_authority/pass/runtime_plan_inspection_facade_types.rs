use worth_ui::facade::{
    WorthUiArtifactToPlanProvenance, WorthUiExecutionPlanInspection, WorthUiLaneInspection,
    WorthUiPlanInspectionCounters, WorthUiPlanInspectionDenial,
    WorthUiPlanInspectionDenialReason, WorthUiPlanNodeInspection, WorthUiPlanProvenanceSource,
    WorthUiQueryInspectionLinks,
};

fn accepts_plan_inspection_types(
    _inspection: Option<WorthUiExecutionPlanInspection>,
    _node: Option<WorthUiPlanNodeInspection>,
    _provenance: Option<WorthUiArtifactToPlanProvenance>,
    _lane: Option<WorthUiLaneInspection>,
    _query_links: Option<WorthUiQueryInspectionLinks>,
    _counters: Option<WorthUiPlanInspectionCounters>,
    _denial: Option<WorthUiPlanInspectionDenial>,
) {
}

fn main() {
    let _ = WorthUiPlanInspectionDenialReason::PlanInputNodeCountMismatch;
    let _ = WorthUiPlanInspectionDenialReason::PlanInputReceiptMismatch;
    let _ = WorthUiPlanInspectionDenialReason::PlanNodeFamilyMismatch;
    let _ = WorthUiPlanInspectionDenialReason::RuntimeHandlePlanIndexMismatch;
    let _ = WorthUiPlanProvenanceSource::ReplacementClassification;
    let _ = WorthUiPlanProvenanceSource::QueryBinding;
    accepts_plan_inspection_types(None, None, None, None, None, None, None);
}
