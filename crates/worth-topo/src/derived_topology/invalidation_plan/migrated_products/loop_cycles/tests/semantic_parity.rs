use super::super::LoopCycleReadSource;
use super::support::{
    close_loop_cycle_slice_from_topology, selected_loop_cycle_topology_with_unrelated_shells,
    selected_loop_cycle_touched_closure, selected_loop_cycles_plan, source_row,
};

#[test]
fn migrated_loop_cycles_preserve_boundary_summary_semantics_for_touched_shells() {
    let plan = selected_loop_cycles_plan("loop-touch");
    let touched_closure = selected_loop_cycle_touched_closure("loop-touch");
    let topology = selected_loop_cycle_topology_with_unrelated_shells();
    let closeout = close_loop_cycle_slice_from_topology(&plan, &touched_closure, &topology);
    let output = closeout.phase_six_seed();

    assert_eq!(output.migrated_family(), "loop_cycles");
    assert_eq!(closeout.counters().output_row_count(), 1);
    assert_eq!(closeout.counters().selected_source_row_count(), 1);
    assert_eq!(closeout.counters().whole_view_fallback_count(), 0);
    assert!(closeout.counters().non_loop_placeholder_execution_count() > 0);

    let read_source =
        LoopCycleReadSource::select_from_touched_closure(&plan, &touched_closure, &topology)
            .unwrap();
    let row = read_source
        .selected_rows()
        .first()
        .expect("read source should expose selected boundary row");
    assert_eq!(row.shell_id(), source_row(24, 1, 5).shell_id());
    assert_eq!(row.boundary_component_count(), 1);
    assert_eq!(row.boundary_half_edge_count(), 5);
}
