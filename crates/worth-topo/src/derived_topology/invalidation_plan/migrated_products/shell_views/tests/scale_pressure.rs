use super::support::{
    close_shell_view_slice_from_topology, selected_shell_view_read_source,
    selected_shell_view_topology_with_many_unrelated_shells,
    selected_shell_view_topology_with_unrelated_shells, selected_shell_view_touched_closure,
    selected_shell_views_plan, source_row,
};

#[test]
fn scale_pressure_counts_touched_rows_not_available_topology_rows() {
    let plan = selected_shell_views_plan("loop-touch");
    let touched_closure = selected_shell_view_touched_closure("loop-touch");
    let topology = selected_shell_view_topology_with_many_unrelated_shells(10_000);
    let closeout = close_shell_view_slice_from_topology(&plan, &touched_closure, &topology);

    assert_eq!(closeout.counters().available_source_row_count(), 1);
    assert_eq!(closeout.counters().selected_source_row_count(), 1);
    assert_eq!(closeout.counters().execution_work_count(), 1);
    assert_eq!(closeout.counters().read_stage_touched_anchor_count(), 1);
    assert_eq!(closeout.counters().read_stage_half_edge_lookup_count(), 1);
    assert_eq!(
        closeout
            .counters()
            .read_stage_radial_relation_lookup_count(),
        1
    );
    assert_eq!(
        closeout
            .counters()
            .read_stage_touched_neighborhood_breadth_count(),
        2
    );
    assert_eq!(
        closeout
            .counters()
            .read_stage_unrelated_source_breadth_count(),
        topology.half_edges.len() - 1
    );
}

#[test]
fn fixture_topology_read_source_selects_only_touched_shell_boundary_rows() {
    let read_source = selected_shell_view_read_source();

    assert_eq!(read_source.available_source_row_count(), 1);
    assert_eq!(read_source.selected_rows().len(), 1);
    assert_eq!(read_source.counters().touched_anchor_count(), 1);
    assert_eq!(read_source.counters().half_edge_lookup_count(), 1);
    assert_eq!(read_source.counters().radial_relation_lookup_count(), 1);
    assert_eq!(
        read_source.counters().touched_neighborhood_breadth_count(),
        2
    );
    assert_eq!(read_source.counters().whole_view_fallback_count(), 0);
    assert_eq!(
        read_source.selected_rows()[0].source_half_edge_identity(),
        source_row(24, 240, 25, 2, false, false).source_half_edge_identity()
    );
}

#[test]
fn fixture_topology_read_source_does_not_count_unrelated_shell_views_as_available_work() {
    let plan = selected_shell_views_plan("loop-touch");
    let touched_closure = selected_shell_view_touched_closure("loop-touch");
    let topology = selected_shell_view_topology_with_unrelated_shells();

    let read_source = super::super::ShellViewReadSource::select_from_touched_closure(
        &plan,
        &touched_closure,
        &topology,
    )
    .unwrap();

    assert_eq!(read_source.available_source_row_count(), 1);
    assert_eq!(read_source.selected_rows().len(), 1);
    assert_eq!(
        read_source.selected_rows()[0],
        source_row(24, 240, 25, 2, false, false)
    );
    assert_eq!(read_source.counters().unrelated_source_breadth_count(), 5);
}
