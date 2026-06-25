use super::support::{
    close_vertex_disk_slice_from_topology, selected_vertex_disk_read_source,
    selected_vertex_disk_topology_with_unrelated_disks,
};

#[test]
fn touched_topology_selection_counts_unrelated_vertex_disks_without_executing_them() {
    let topology = selected_vertex_disk_topology_with_unrelated_disks();
    let read_source = selected_vertex_disk_read_source();

    assert!(
        topology.half_edges.len()
            > read_source.selected_rows()[0]
                .incident_half_edge_identities()
                .len()
    );
    assert_eq!(read_source.selected_rows().len(), 1);
    assert_eq!(read_source.counters().selected_vertex_disk_root_count(), 1);
    assert_eq!(read_source.counters().selected_source_row_count(), 1);
    assert!(read_source.counters().unrelated_vertex_disk_breadth_count() > 0);
    assert_eq!(read_source.counters().whole_view_fallback_count(), 0);
}

#[test]
fn topology_closeout_keeps_vertex_disk_work_bounded_to_touched_graph() {
    let closeout = close_vertex_disk_slice_from_topology("vertex-disk-locality-closeout");

    assert_eq!(closeout.counters().output_row_count(), 1);
    assert_eq!(
        closeout
            .counters()
            .read_stage_selected_vertex_disk_root_count(),
        1
    );
    assert!(
        closeout
            .counters()
            .read_stage_unrelated_vertex_disk_breadth_count()
            > 0
    );
    assert_eq!(closeout.counters().whole_view_fallback_count(), 0);
    assert_eq!(closeout.counters().execution_work_count(), 1);
}
