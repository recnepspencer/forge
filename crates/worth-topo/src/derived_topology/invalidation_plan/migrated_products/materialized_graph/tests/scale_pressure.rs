use super::super::{
    MaterializedGraphExecutionInput, MaterializedGraphReadSource,
    MaterializedGraphReadStageExecutor,
};
use super::support::{selected_materialized_graph_plan, selected_materialized_receipt};

#[test]
fn sparse_materialized_graph_execution_counts_selected_rows_not_available_rows() {
    let plan = selected_materialized_graph_plan();
    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_wire_chain_view(
            64,
        );
    let read_source =
        MaterializedGraphReadSource::from_topology_view_with_selected_prefix(&topology, 2, 1)
            .unwrap();
    let available_entity_count = read_source.available_entity_count();
    let available_relation_count = read_source.available_relation_count();
    let receipt = MaterializedGraphReadStageExecutor::execute(&plan, read_source).unwrap();

    let closeout = super::support::close_materialized_graph_slice(receipt);

    assert_eq!(closeout.counters().selected_entity_count(), 2);
    assert_eq!(closeout.counters().selected_relation_count(), 1);
    assert_eq!(
        closeout.counters().available_entity_count(),
        available_entity_count
    );
    assert_eq!(
        closeout.counters().available_relation_count(),
        available_relation_count
    );
    assert!(closeout.counters().available_entity_count() > 2);
    assert!(closeout.counters().available_relation_count() > 1);
    assert_eq!(closeout.counters().execution_work_count(), 3);
    assert_eq!(closeout.counters().whole_view_fallback_count(), 0);
}

#[test]
fn selected_read_rows_cannot_exceed_available_read_rows() {
    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_wire_chain_view(
            4,
        );
    let available_entities =
        MaterializedGraphReadSource::from_topology_view_with_selected_prefix(&topology, 1, 1)
            .unwrap()
            .available_entity_count();

    let error = MaterializedGraphReadSource::from_topology_view_with_selected_prefix(
        &topology,
        available_entities + 1,
        1,
    )
    .unwrap_err();

    assert_eq!(
        error,
        super::super::MaterializedGraphMigrationError::ReadStageSelectedRowsExceedAvailableRows
    );
}

#[test]
fn execution_input_requires_read_stage_to_match_selected_plan() {
    let plan = selected_materialized_graph_plan();
    let unrelated_receipt =
        selected_materialized_receipt(&plan).with_selected_plan_digest_for_tests("forged-plan");

    let error = MaterializedGraphExecutionInput::from_selected_plan_and_read_stage(
        &plan,
        unrelated_receipt,
    )
    .unwrap_err();

    assert_eq!(
        error,
        super::super::MaterializedGraphMigrationError::ReadStageReceiptNotBoundToSelectedPlan
    );
}

#[test]
fn execution_input_requires_read_stage_query_receipt_to_match_selected_row() {
    let plan = selected_materialized_graph_plan();
    let forged_receipt = selected_materialized_receipt(&plan)
        .with_native_query_read_receipt_digest_for_tests("forged-query-read");

    let error =
        MaterializedGraphExecutionInput::from_selected_plan_and_read_stage(&plan, forged_receipt)
            .unwrap_err();

    assert_eq!(
        error,
        super::super::MaterializedGraphMigrationError::ReadStageReceiptNotBoundToSelectedPlan
    );
}

#[test]
fn missing_query_read_support_denies_before_materialized_read_stage() {
    let plan = super::support::materialized_graph_plan_missing_native_read();

    let error = MaterializedGraphReadStageExecutor::execute(
        &plan,
        super::support::selected_materialized_read_source(),
    )
    .unwrap_err();

    assert_eq!(
        error,
        super::super::MaterializedGraphMigrationError::SelectedPlanMissingMaterializedGraphRow
    );
}

#[test]
fn missing_legality_support_denies_before_materialized_read_stage() {
    let plan = super::support::materialized_graph_plan_missing_legality();

    let error = MaterializedGraphReadStageExecutor::execute(
        &plan,
        super::support::selected_materialized_read_source(),
    )
    .unwrap_err();

    assert_eq!(
        error,
        super::super::MaterializedGraphMigrationError::SelectedPlanMissingMaterializedGraphRow
    );
}
