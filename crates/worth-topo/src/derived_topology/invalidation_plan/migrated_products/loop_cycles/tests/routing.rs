use super::support::{
    close_loop_cycle_slice_from_topology, selected_loop_cycle_topology_with_unrelated_shells,
    selected_loop_cycle_touched_closure_for_shell, selected_loop_cycles_plan_for_shell,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;

#[test]
fn same_family_declaration_routes_multiple_operator_touches_without_executor_edits() {
    let topology = selected_loop_cycle_topology_with_unrelated_shells();
    let first_plan = selected_loop_cycles_plan_for_shell("operator-a", 24);
    let second_plan = selected_loop_cycles_plan_for_shell("operator-b", 99);
    let first_closure = selected_loop_cycle_touched_closure_for_shell("operator-a", 24);
    let second_closure = selected_loop_cycle_touched_closure_for_shell("operator-b", 99);
    let first = close_loop_cycle_slice_from_topology(&first_plan, &first_closure, &topology);
    let second = close_loop_cycle_slice_from_topology(&second_plan, &second_closure, &topology);

    assert_eq!(
        first.phase_six_seed().migrated_family(),
        second.phase_six_seed().migrated_family()
    );
    assert_eq!(
        first_plan
            .selected_rows()
            .iter()
            .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::LoopCycles)
            .unwrap()
            .family_digest(),
        second_plan
            .selected_rows()
            .iter()
            .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::LoopCycles)
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
    assert_eq!(first.counters().read_stage_shell_lookup_count(), 1);
    assert_eq!(second.counters().read_stage_shell_lookup_count(), 1);
}
