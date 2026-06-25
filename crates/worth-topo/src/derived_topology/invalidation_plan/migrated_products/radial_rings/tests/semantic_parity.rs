use super::super::RadialRingReadSource;
use super::support::{
    close_radial_ring_slice_from_topology, selected_radial_ring_topology_with_unrelated_shells,
    selected_radial_ring_touched_closure, selected_radial_rings_plan, source_row,
};

#[test]
fn migrated_radial_rings_preserve_ring_summary_semantics_for_touched_roots() {
    let plan = selected_radial_rings_plan("loop-touch");
    let touched_closure = selected_radial_ring_touched_closure("loop-touch");
    let topology = selected_radial_ring_topology_with_unrelated_shells();
    let closeout = close_radial_ring_slice_from_topology(&plan, &touched_closure, &topology);
    let output = closeout.family_closeout_seed();

    assert_eq!(output.migrated_family(), "radial_rings");
    assert_eq!(closeout.counters().output_row_count(), 1);
    assert_eq!(closeout.counters().selected_source_row_count(), 1);
    assert_eq!(closeout.counters().whole_view_fallback_count(), 0);
    assert!(closeout.counters().non_loop_placeholder_execution_count() > 0);

    let read_source =
        RadialRingReadSource::select_from_touched_closure(&plan, &touched_closure, &topology)
            .unwrap();
    let row = read_source
        .selected_rows()
        .first()
        .expect("read source should expose selected radial row");
    assert_eq!(
        row.source_half_edge_identity(),
        source_row(24, 240, 25, 2, false, false).source_half_edge_identity()
    );
    assert_eq!(row.ring_half_edge_count(), 2);
    assert!(!row.boundary_half_edge());
    assert!(!row.non_manifold_edge());
}
