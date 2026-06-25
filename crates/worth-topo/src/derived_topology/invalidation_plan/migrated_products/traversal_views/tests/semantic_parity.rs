use super::super::TraversalViewsOldAuthorityResidue;
use super::support::{
    assert_plan_selects_traversal_views, close_traversal_views_slice, selected_traversal_receipt,
    selected_traversal_touched_closure, selected_traversal_views_plan,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::bootstrap_topology_interpretation;

#[test]
fn migrated_traversal_views_closes_from_selected_read_stage_receipt() {
    let plan = selected_traversal_views_plan();
    assert_plan_selects_traversal_views(&plan);
    let receipt = selected_traversal_receipt();

    let closeout = close_traversal_views_slice(receipt);

    assert_eq!(
        closeout.phase_eleven_seed().migrated_family(),
        "traversal_views"
    );
    assert!(!closeout.phase_eleven_seed().seed_digest().is_empty());
    assert!(!closeout.execution_receipt_digest().is_empty());
    assert!(!closeout.traversal_views_output_digest().is_empty());
    assert!(!closeout.diagnostic_projection_digest().is_empty());
    assert!(!closeout.old_authority_residue_digest().is_empty());
    assert_eq!(
        closeout.migrated_family_closeout().family_identity(),
        DerivedTopologyProductFamilyIdentity::TraversalViews
    );
    assert_eq!(closeout.counters().selected_traversal_count(), 2);
    assert_eq!(closeout.counters().execution_work_count(), 2);
    assert_eq!(closeout.counters().whole_view_fallback_count(), 0);
    assert_eq!(
        closeout.counters().old_authority_residue_count(),
        TraversalViewsOldAuthorityResidue::required_capped_callers().len()
    );
}

#[test]
fn migrated_traversal_views_breadth_matches_old_interpreter_breadth_without_using_it_as_output() {
    let old_topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_shell_nmt_fan_view(4);
    let old_materialized = MaterializedTopologyView::whole_view(old_topology.clone());
    let old_interpreted = bootstrap_topology_interpretation(&old_materialized);
    let old_report = old_interpreted.report();
    let plan = selected_traversal_views_plan();
    let touched_closure = selected_traversal_touched_closure();
    let read_source = super::super::TraversalViewsReadSource::select_from_touched_closure(
        &plan,
        &touched_closure,
        &old_topology,
    )
    .unwrap();
    let receipt =
        super::super::TraversalViewsReadStageExecutor::execute(&plan, read_source.clone()).unwrap();

    let closeout = close_traversal_views_slice(receipt);

    assert_eq!(closeout.counters().selected_traversal_count(), 2);
    assert_eq!(old_report.interpreted_shell_count, 1);
    assert_eq!(old_report.interpreted_wire_count, 0);
    assert!(read_source
        .selected_rows()
        .iter()
        .all(|row| row.traversal_kind() == "loop.half_edge_walk"));
    assert_eq!(
        read_source
            .selected_rows()
            .iter()
            .map(|row| row.reached_entity_count())
            .collect::<Vec<_>>(),
        vec![3, 3]
    );
    assert_eq!(
        closeout.counters().available_traversal_count(),
        read_source.available_traversal_count()
    );
    assert_eq!(closeout.counters().whole_view_fallback_count(), 0);
}
