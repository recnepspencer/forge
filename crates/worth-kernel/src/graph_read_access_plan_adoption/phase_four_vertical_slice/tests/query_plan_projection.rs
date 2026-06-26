use super::super::query_plan_projection::WorthGraphReadAccessSlicePlanProjectionStatus;
use super::production_phase_four_closeout;

#[test]
fn query_plan_projection_uses_real_query_plan_or_gap() {
    let closeout = production_phase_four_closeout();
    let plan_projection = closeout.plan_projection();

    assert_eq!(
        WorthGraphReadAccessSlicePlanProjectionStatus::MissingQueryReadFamilyArtifactForExecution,
        plan_projection.status()
    );
    assert_eq!(None, plan_projection.admitted_plan_digest());
    assert_eq!(None, plan_projection.query_admission_digest());
    assert_eq!(
        Some("ForgeQueryReadFamily"),
        plan_projection.required_worth_artifact()
    );
    assert!(plan_projection.blocker().is_some());
}

#[test]
fn raw_admitted_plan_is_not_receipt() {
    let closeout = production_phase_four_closeout();

    assert!(!closeout.plan_projection().claims_query_plan_admission());
    assert!(!closeout.claims_access_plan_consumption());
    assert!(!closeout.claims_graph_read_execution());
    assert!(!closeout.claims_graph_read_receipts());
}
