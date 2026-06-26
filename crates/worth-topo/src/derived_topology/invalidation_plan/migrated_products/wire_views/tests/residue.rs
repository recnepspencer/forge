use super::super::{
    WireViewDerivedProductExecutor, WireViewExecutionInput, WireViewMigrationCloseout,
    WireViewMigrationError, WireViewOldAuthorityResidue, WireViewReadStageExecutor,
};
use super::support::{
    selected_wire_view_plan, selected_wire_view_query_read_rows,
    selected_wire_view_read_source_fixture, unbound_wire_view_touched_closure,
};
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;

#[test]
fn wire_view_closeout_accepts_deleted_old_authority_residue() {
    let fixture = selected_wire_view_read_source_fixture("wire-view-uncapped-residue");
    let read_receipt =
        WireViewReadStageExecutor::execute(&fixture.plan, fixture.read_source).unwrap();
    let input =
        WireViewExecutionInput::from_selected_plan_and_read_stage(&fixture.plan, read_receipt)
            .unwrap();
    let executor = WireViewDerivedProductExecutor::new(input);
    let receipt = DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(
        &fixture.plan,
        &executor,
    )
    .unwrap();
    let output = executor.output().unwrap();

    let closeout = WireViewMigrationCloseout::close(
        &receipt,
        &output,
        &WireViewOldAuthorityResidue::uncapped_for_tests(),
    )
    .expect("deleted old wire-view authority should close with zero residue");

    assert_eq!(closeout.counters().old_authority_residue_count(), 0);
}

#[test]
fn old_wire_view_authority_residue_is_empty_after_hard_cutover() {
    let residue = WireViewOldAuthorityResidue::current_source_scan();

    assert!(residue.contains_required_caps());
    assert_eq!(residue.capped_direct_interpreter_count(), 0);
    assert_eq!(
        WireViewOldAuthorityResidue::required_capped_callers(),
        &[] as &[&str]
    );
    assert!(residue.capped_rows().is_empty());
}

#[test]
fn read_stage_rejects_touched_closure_from_a_different_selected_plan() {
    let plan = selected_wire_view_plan("wire-view-plan-a");
    let wrong_touched_closure = unbound_wire_view_touched_closure("wire-view-plan-b");
    let rows = selected_wire_view_query_read_rows();

    let error = super::super::WireViewReadSource::from_query_wire_views(
        &plan,
        &wrong_touched_closure,
        &rows,
    )
    .unwrap_err();

    assert_eq!(
        error,
        WireViewMigrationError::ReadStageTouchedClosureNotBoundToSelectedPlan
    );
}
