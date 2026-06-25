use super::super::receipt_boundary::WorthGraphReadAccessSliceReceiptStatus;
use super::production_phase_four_closeout;

#[test]
fn receipt_projection_requires_query_consumption_surface() {
    let closeout = production_phase_four_closeout();
    let receipt = closeout.receipt_projection();

    assert_eq!(
        WorthGraphReadAccessSliceReceiptStatus::QueryExecutionCapabilityGap,
        receipt.status()
    );
    assert_eq!(None, receipt.plan_consumption_digest());
    assert_eq!(
        Some("ForgeQueryReadResult::receipt().graph_read_access_plan_consumption()"),
        receipt.required_query_surface()
    );
    assert_eq!(
        Some("WorthGraphReadAccessSelectedVerticalSlice -> ForgeQueryReadFamily execution binding"),
        receipt.required_worth_surface()
    );
    assert_eq!(
        Some(
            "crate::construction::query_access_planning::execute_planned_construction_query_access"
        ),
        receipt.existing_worth_execution_surface()
    );
    assert!(receipt
        .blocker()
        .expect("missing binding should be explicit")
        .contains("ForgeQueryWorkspace::execute_read_family_with_access_plan"));
    assert!(receipt.blocker().is_some());
}

#[test]
fn phase_four_closeout_does_not_claim_validator_selection() {
    let closeout = production_phase_four_closeout();

    assert!(!closeout.claims_first_vertical_slice_migration());
    assert!(!closeout.claims_validator_selection());
    assert_eq!(0, closeout.counters().local_graph_traversal_attempt_count());
    assert_eq!(
        0,
        closeout.counters().local_adjacency_lookup_attempt_count()
    );
    assert_eq!(0, closeout.counters().local_broad_scan_attempt_count());
    assert_eq!(
        0,
        closeout
            .counters()
            .local_receipt_fabrication_attempt_count()
    );
}
