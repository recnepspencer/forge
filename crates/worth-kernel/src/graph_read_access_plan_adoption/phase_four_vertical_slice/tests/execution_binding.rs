use super::super::cutover_proof::WorthGraphReadAccessSliceCutoverStatus;
use super::super::query_plan_projection::WorthGraphReadAccessSlicePlanProjectionStatus;
use super::super::receipt_boundary::WorthGraphReadAccessSliceReceiptStatus;
use super::production_phase_four_receipt_closeout;

#[test]
fn first_migrated_slice_receipt_matches_query_plan_identity() {
    let closeout = production_phase_four_receipt_closeout();
    let executed = closeout
        .executed_slice()
        .expect("receipt-backed closeout should retain executed slice");
    let plan = closeout.plan_projection();
    let receipt = closeout.receipt_projection();

    assert_eq!(
        WorthGraphReadAccessSlicePlanProjectionStatus::QueryPlanAdmitted,
        plan.status()
    );
    assert_eq!(
        Some(executed.executed_read_family_digest()),
        plan.executed_read_family_digest()
    );
    assert_eq!(
        Some(executed.admitted_plan_digest()),
        plan.admitted_plan_digest()
    );
    assert_eq!(
        Some(executed.query_admission_digest()),
        plan.query_admission_digest()
    );
    assert_eq!(
        Some(executed.query_requirement_set_digest()),
        plan.query_requirement_set_digest()
    );
    assert_eq!(
        WorthGraphReadAccessSliceReceiptStatus::QueryReceiptObserved,
        receipt.status()
    );
    assert_eq!(
        Some(executed.plan_consumption_digest()),
        receipt.plan_consumption_digest()
    );
    assert_eq!(
        Some(executed.requirement_row_digest()),
        receipt.requirement_row_digest()
    );
    assert_eq!(
        Some(executed.declared_read_family_identity_digest()),
        receipt.declared_read_family_identity_digest()
    );
    assert_eq!(
        Some(executed.executed_read_family_digest()),
        receipt.executed_read_family_digest()
    );
    assert_eq!(
        Some(executed.admitted_plan_digest()),
        receipt.admitted_plan_digest()
    );
}

#[test]
fn first_migrated_slice_receipt_names_execution_basis_and_counters() {
    let closeout = production_phase_four_receipt_closeout();
    let executed = closeout
        .executed_slice()
        .expect("receipt-backed closeout should retain executed slice");
    let receipt = closeout.receipt_projection();

    assert_eq!(executed.execution_basis_digest(), receipt.execution_basis());
    assert_eq!(1, receipt.executor_entry_count());
    assert_eq!(1, receipt.materialized_row_count());
    assert!(receipt.no_caller_owned_graph_work());
    assert!(executed.no_caller_owned_graph_work());
    assert!(closeout.claims_first_vertical_slice_migration());
    assert!(closeout.claims_access_plan_consumption());
    assert!(closeout.claims_graph_read_execution());
    assert!(closeout.claims_graph_read_receipts());
}

#[test]
fn old_slice_helper_is_capped_after_receipt_until_inventory_binding_exists() {
    let closeout = production_phase_four_receipt_closeout();
    let cutover = closeout.cutover_proof();

    assert_eq!(
        WorthGraphReadAccessSliceCutoverStatus::CappedUntilMigrationInventoryBindingExists,
        cutover.status()
    );
    assert!(cutover.old_path_is_deleted_or_capped());
    assert!(!cutover.displaced_evidence_identity().is_empty());
}
