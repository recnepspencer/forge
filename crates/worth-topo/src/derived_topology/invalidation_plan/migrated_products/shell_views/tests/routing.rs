use super::support::{
    close_shell_view_slice_from_topology, selected_shell_view_topology_with_unrelated_shells,
    selected_shell_view_touched_closure_for_shell, selected_shell_views_plan_for_shell,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;

#[test]
fn same_family_declaration_routes_multiple_operator_touches_without_executor_edits() {
    let topology = selected_shell_view_topology_with_unrelated_shells();
    let first_plan = selected_shell_views_plan_for_shell("operator-a", 24);
    let second_plan = selected_shell_views_plan_for_shell("operator-b", 99);
    let first_closure = selected_shell_view_touched_closure_for_shell("operator-a", 24);
    let second_closure = selected_shell_view_touched_closure_for_shell("operator-b", 99);
    let first = close_shell_view_slice_from_topology(&first_plan, &first_closure, &topology);
    let second = close_shell_view_slice_from_topology(&second_plan, &second_closure, &topology);

    assert_eq!(
        first.family_closeout_seed().migrated_family(),
        second.family_closeout_seed().migrated_family()
    );
    assert_eq!(
        first_plan
            .selected_rows()
            .iter()
            .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::ShellViews)
            .unwrap()
            .family_digest(),
        second_plan
            .selected_rows()
            .iter()
            .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::ShellViews)
            .unwrap()
            .family_digest()
    );
    assert_eq!(
        first.counters().selected_source_row_count(),
        second.counters().selected_source_row_count()
    );
    assert_ne!(
        first_plan.touched_closure_digest(),
        second_plan.touched_closure_digest()
    );
    assert_eq!(first.counters().read_stage_half_edge_lookup_count(), 1);
    assert_eq!(second.counters().read_stage_half_edge_lookup_count(), 1);
}
