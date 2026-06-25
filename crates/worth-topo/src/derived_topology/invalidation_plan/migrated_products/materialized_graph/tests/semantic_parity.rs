use super::super::MaterializedGraphOldAuthorityResidue;
use super::support::{
    assert_plan_selects_materialized_graph, close_materialized_graph_slice,
    selected_materialized_graph_plan, selected_materialized_read_source,
    selected_materialized_receipt,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::materialized_graph::MaterializedTopologyView;

#[test]
fn migrated_materialized_graph_closes_from_selected_read_stage_receipt() {
    let plan = selected_materialized_graph_plan();
    assert_plan_selects_materialized_graph(&plan);
    let receipt = selected_materialized_receipt(&plan);

    let closeout = close_materialized_graph_slice(receipt);

    assert_eq!(
        closeout.phase_ten_seed().migrated_family(),
        "materialized_graph"
    );
    assert!(!closeout.phase_ten_seed().seed_digest().is_empty());
    assert!(!closeout.execution_receipt_digest().is_empty());
    assert!(!closeout.materialized_graph_output_digest().is_empty());
    assert!(!closeout.diagnostic_projection_digest().is_empty());
    assert!(!closeout.old_authority_residue_digest().is_empty());
    assert_eq!(
        closeout.migrated_family_closeout().family_identity(),
        DerivedTopologyProductFamilyIdentity::MaterializedGraph
    );
    assert!(!closeout.closeout_digest().is_empty());
    assert_eq!(closeout.counters().selected_entity_count(), 2);
    assert_eq!(closeout.counters().selected_relation_count(), 1);
    assert_eq!(closeout.counters().available_entity_count(), 14);
    assert_eq!(closeout.counters().available_relation_count(), 30);
    assert_eq!(closeout.counters().execution_work_count(), 3);
    assert_eq!(closeout.counters().whole_view_fallback_count(), 0);
    assert_eq!(
        closeout.counters().old_authority_residue_count(),
        MaterializedGraphOldAuthorityResidue::required_capped_callers().len()
    );
    assert!(
        closeout
            .counters()
            .non_materialized_placeholder_execution_count()
            > 0
    );
    assert_eq!(
        closeout.phase_ten_seed().closeout_digest(),
        closeout.closeout_digest()
    );
    assert_eq!(
        closeout.phase_ten_seed().counters_digest(),
        closeout.counters().counters_digest()
    );
    assert_eq!(
        closeout.phase_ten_seed().old_authority_residue_digest(),
        closeout.old_authority_residue_digest()
    );
}

#[test]
fn migrated_materialized_graph_breadth_matches_old_materialized_view_breadth() {
    let old_topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_wire_chain_view(
            4,
        );
    let old_materialized = MaterializedTopologyView::whole_view(old_topology);
    let old_report = old_materialized.report();
    let plan = selected_materialized_graph_plan();
    let read_source = selected_materialized_read_source();
    let expected_entity_rows = read_source.selected_entities().to_vec();
    let expected_relation_rows = read_source.selected_relations().to_vec();
    let receipt =
        super::super::MaterializedGraphReadStageExecutor::execute(&plan, read_source).unwrap();

    let closeout = close_materialized_graph_slice(receipt);

    assert_eq!(
        closeout.counters().topology_entity_count(),
        old_report.breadth.topology_entity_count
    );
    assert_eq!(closeout.counters().topology_relation_count(), 30);
    assert_eq!(closeout.counters().whole_view_fallback_count(), 0);

    let output_input =
        super::super::MaterializedGraphExecutionInput::from_selected_plan_and_read_stage(
            &plan,
            super::super::MaterializedGraphReadStageExecutor::execute(
                &plan,
                selected_materialized_read_source(),
            )
            .unwrap(),
        )
        .unwrap();
    let output =
        super::super::MaterializedGraphDerivedProductOutput::from_execution_input(&output_input);
    assert_eq!(output.entity_rows().len(), expected_entity_rows.len());
    assert_eq!(output.relation_rows().len(), expected_relation_rows.len());
    assert_eq!(
        output.entity_rows()[0].source_entity_id(),
        expected_entity_rows[0].entity_id()
    );
    assert_eq!(
        output.relation_rows()[0].source_entity_id(),
        expected_relation_rows[0].source_entity_id()
    );
    assert_eq!(
        output.relation_rows()[0].target_entity_id(),
        expected_relation_rows[0].target_entity_id()
    );
}
