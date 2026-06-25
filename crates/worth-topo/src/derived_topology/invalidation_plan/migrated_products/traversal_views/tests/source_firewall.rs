use super::super::{
    TraversalViewsDerivedProductOutput, TraversalViewsDiagnosticProjection,
    TraversalViewsExecutionInput, TraversalViewsMigrationCloseout,
    TraversalViewsOldAuthorityResidue,
};
use super::support::{selected_traversal_receipt, selected_traversal_views_plan};

#[test]
fn old_traversal_authority_residue_is_empty_after_hard_cutover() {
    let residue = TraversalViewsOldAuthorityResidue::current_source_scan();

    assert_eq!(residue.capped_traversal_authority_count(), 0);
    assert_eq!(
        TraversalViewsOldAuthorityResidue::required_capped_callers(),
        &[] as &[&str]
    );
    assert!(residue.contains_required_caps());
    assert!(residue.capped_rows().is_empty());
}

#[test]
fn traversal_closeout_accepts_deleted_old_authority_residue() {
    let plan = selected_traversal_views_plan();
    let read_stage_receipt = selected_traversal_receipt();
    let input = TraversalViewsExecutionInput::from_selected_plan_and_read_stage(
        &plan,
        read_stage_receipt.clone(),
    )
    .unwrap();
    let executor = super::super::TraversalViewsDerivedProductExecutor::new(input.clone());
    let receipt =
        crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(
            &plan,
            &executor,
        )
        .unwrap();
    let output = TraversalViewsDerivedProductOutput::from_execution_input(&input);
    let diagnostic_projection = TraversalViewsDiagnosticProjection::from_read_stage_and_output(
        &read_stage_receipt,
        &output,
    );
    let deleted_residue = TraversalViewsOldAuthorityResidue::uncapped_for_tests();

    let closeout = TraversalViewsMigrationCloseout::close(
        &receipt,
        &output,
        &diagnostic_projection,
        &deleted_residue,
    )
    .expect("deleted old authority should close with zero residue");

    assert_eq!(closeout.counters().old_authority_residue_count(), 0);
}
