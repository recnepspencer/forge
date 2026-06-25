use super::support::{
    close_loop_cycle_slice_from_topology, selected_loop_cycle_read_source,
    selected_loop_cycle_topology_with_many_unrelated_shells,
    selected_loop_cycle_topology_with_unrelated_shells, selected_loop_cycle_touched_closure,
    selected_loop_cycles_plan, source_row,
};

#[test]
fn scale_pressure_counts_touched_rows_not_available_topology_rows() {
    let plan = selected_loop_cycles_plan("loop-touch");
    let touched_closure = selected_loop_cycle_touched_closure("loop-touch");
    let topology = selected_loop_cycle_topology_with_many_unrelated_shells(10_000);
    let closeout = close_loop_cycle_slice_from_topology(&plan, &touched_closure, &topology);

    assert_eq!(closeout.counters().available_source_row_count(), 1);
    assert_eq!(closeout.counters().selected_source_row_count(), 1);
    assert_eq!(closeout.counters().execution_work_count(), 1);
    assert_eq!(closeout.counters().read_stage_touched_anchor_count(), 1);
    assert_eq!(closeout.counters().read_stage_shell_lookup_count(), 1);
    assert_eq!(closeout.counters().read_stage_face_lookup_count(), 2);
    assert_eq!(
        closeout
            .counters()
            .read_stage_unrelated_source_breadth_count(),
        topology.shells.len() - 1
    );
}

#[test]
fn production_read_source_selects_only_touched_shell_boundary_rows() {
    let read_source = selected_loop_cycle_read_source();

    assert_eq!(read_source.available_source_row_count(), 1);
    assert_eq!(read_source.selected_rows().len(), 1);
    assert_eq!(read_source.counters().touched_anchor_count(), 1);
    assert_eq!(read_source.counters().shell_lookup_count(), 1);
    assert_eq!(read_source.counters().face_lookup_count(), 4);
    assert_eq!(read_source.counters().whole_view_fallback_count(), 0);
    assert_eq!(
        read_source.selected_rows()[0].shell_id(),
        source_row(24, 1, 12).shell_id()
    );
}

#[test]
fn production_read_source_does_not_count_unrelated_shells_as_available_work() {
    let plan = selected_loop_cycles_plan("loop-touch");
    let touched_closure = selected_loop_cycle_touched_closure("loop-touch");
    let topology = selected_loop_cycle_topology_with_unrelated_shells();

    let read_source = super::super::LoopCycleReadSource::select_from_touched_closure(
        &plan,
        &touched_closure,
        &topology,
    )
    .unwrap();

    assert_eq!(read_source.available_source_row_count(), 1);
    assert_eq!(read_source.selected_rows().len(), 1);
    assert_eq!(read_source.selected_rows()[0], source_row(24, 1, 5));
    assert_eq!(read_source.counters().unrelated_source_breadth_count(), 2);
}
