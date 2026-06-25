use super::super::{
    MaterializedGraphDerivedProductExecutor, MaterializedGraphDiagnosticProjection,
    MaterializedGraphExecutionInput, MaterializedGraphMigrationCloseout,
    MaterializedGraphMigrationError, MaterializedGraphOldAuthorityResidue,
};
use super::support::{selected_materialized_graph_plan, selected_materialized_receipt};
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;

#[test]
fn receipt_output_digest_must_bind_materialized_graph_product_output() {
    let plan = selected_materialized_graph_plan();
    let input = MaterializedGraphExecutionInput::from_selected_plan_and_read_stage(
        &plan,
        selected_materialized_receipt(&plan),
    )
    .unwrap();
    let executor = MaterializedGraphDerivedProductExecutor::new(input);
    let receipt =
        DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(&plan, &executor)
            .unwrap();

    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_wire_chain_view(
            4,
        );
    let unrelated_read_source =
        super::super::MaterializedGraphReadSource::from_topology_view_with_selected_prefix(
            &topology, 1, 1,
        )
        .unwrap();
    let unrelated_output_input =
        MaterializedGraphExecutionInput::from_selected_plan_and_read_stage(
            &plan,
            super::super::MaterializedGraphReadStageExecutor::execute(&plan, unrelated_read_source)
                .unwrap(),
        )
        .unwrap();
    let unrelated_output =
        super::super::MaterializedGraphDerivedProductOutput::from_execution_input(
            &unrelated_output_input,
        );
    let diagnostic_projection = MaterializedGraphDiagnosticProjection::from_read_stage_and_output(
        unrelated_output_input.read_stage_receipt(),
        &unrelated_output,
    );
    let residue = MaterializedGraphOldAuthorityResidue::current_source_scan();

    let error = MaterializedGraphMigrationCloseout::close(
        &receipt,
        &unrelated_output,
        &diagnostic_projection,
        &residue,
    )
    .unwrap_err();

    assert_eq!(
        error,
        MaterializedGraphMigrationError::OutputDigestNotBoundToReceipt
    );
}

#[test]
fn diagnostic_projection_exposes_read_stage_output_and_breadth_identity() {
    let plan = selected_materialized_graph_plan();
    let read_stage_receipt = selected_materialized_receipt(&plan);
    let input = MaterializedGraphExecutionInput::from_selected_plan_and_read_stage(
        &plan,
        read_stage_receipt.clone(),
    )
    .unwrap();
    let output = super::super::MaterializedGraphDerivedProductOutput::from_execution_input(&input);

    let diagnostic_projection = MaterializedGraphDiagnosticProjection::from_read_stage_and_output(
        &read_stage_receipt,
        &output,
    );

    assert_eq!(
        diagnostic_projection.selected_plan_digest(),
        plan.selected_plan_digest()
    );
    assert_eq!(
        diagnostic_projection.read_stage_receipt_digest(),
        read_stage_receipt.receipt_digest()
    );
    assert_eq!(
        diagnostic_projection.product_output_digest(),
        output.output_digest()
    );
    assert_eq!(
        diagnostic_projection.selected_entity_count(),
        output.selected_entity_count()
    );
    assert_eq!(
        diagnostic_projection.selected_relation_count(),
        output.selected_relation_count()
    );
    assert_eq!(
        diagnostic_projection.available_entity_count(),
        output.available_entity_count()
    );
    assert_eq!(
        diagnostic_projection.available_relation_count(),
        output.available_relation_count()
    );
}
