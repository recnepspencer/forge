use super::super::{
    TraversalViewsDerivedProductOutput, TraversalViewsDiagnosticProjection,
    TraversalViewsExecutionInput, TraversalViewsMigrationCloseout, TraversalViewsMigrationError,
    TraversalViewsOldAuthorityResidue, TraversalViewsReadSource, TraversalViewsReadStageExecutor,
};
use super::support::{selected_traversal_receipt, selected_traversal_views_plan};

#[test]
fn receipt_output_digest_must_bind_traversal_views_product_output() {
    let plan = selected_traversal_views_plan();
    let input = TraversalViewsExecutionInput::from_selected_plan_and_read_stage(
        &plan,
        selected_traversal_receipt(),
    )
    .unwrap();
    let executor = super::super::TraversalViewsDerivedProductExecutor::new(input);
    let receipt =
        crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(
            &plan,
            &executor,
        )
        .unwrap();
    let unrelated_topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_wire_chain_view(
            8,
        );
    let unrelated_read_source =
        TraversalViewsReadSource::from_topology_view_with_selected_prefix(&unrelated_topology, 1)
            .unwrap();
    let unrelated_output_input = TraversalViewsExecutionInput::from_selected_plan_and_read_stage(
        &plan,
        TraversalViewsReadStageExecutor::execute(&plan, unrelated_read_source).unwrap(),
    )
    .unwrap();
    let unrelated_output =
        TraversalViewsDerivedProductOutput::from_execution_input(&unrelated_output_input);
    let diagnostic_projection = TraversalViewsDiagnosticProjection::from_read_stage_and_output(
        unrelated_output_input.read_stage_receipt(),
        &unrelated_output,
    );
    let residue = TraversalViewsOldAuthorityResidue::current_source_scan();

    let error = TraversalViewsMigrationCloseout::close(
        &receipt,
        &unrelated_output,
        &diagnostic_projection,
        &residue,
    )
    .unwrap_err();

    assert_eq!(
        error,
        TraversalViewsMigrationError::OutputDigestNotBoundToReceipt
    );
}

#[test]
fn diagnostic_projection_exposes_read_stage_output_and_breadth_identity() {
    let plan = selected_traversal_views_plan();
    let read_stage_receipt = selected_traversal_receipt();
    let input = TraversalViewsExecutionInput::from_selected_plan_and_read_stage(
        &plan,
        read_stage_receipt.clone(),
    )
    .unwrap();
    let output = TraversalViewsDerivedProductOutput::from_execution_input(&input);
    let diagnostic_projection = TraversalViewsDiagnosticProjection::from_read_stage_and_output(
        &read_stage_receipt,
        &output,
    );

    assert_eq!(
        diagnostic_projection.selected_plan_digest(),
        output.selected_plan_digest()
    );
    assert_eq!(
        diagnostic_projection.read_stage_receipt_digest(),
        read_stage_receipt.receipt_digest()
    );
    assert_eq!(
        diagnostic_projection.product_output_digest(),
        output.output_digest()
    );
    assert_eq!(diagnostic_projection.selected_traversal_count(), 2);
    assert_eq!(
        diagnostic_projection.available_traversal_count(),
        output.available_traversal_count()
    );
    assert!(!diagnostic_projection.projection_digest().is_empty());
}
