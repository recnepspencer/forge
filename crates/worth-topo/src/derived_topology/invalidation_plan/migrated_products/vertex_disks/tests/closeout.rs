use super::support::{
    close_vertex_disk_slice_from_read_source, query_native_vertex_disk_read_source,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;

#[test]
fn query_native_shared_vertex_read_closes_full_migration_slice() {
    let fixture = query_native_vertex_disk_read_source(
        "vertex-disk.phase-15.query-closeout",
        "vertex-disk-query-closeout",
    );

    let closeout =
        close_vertex_disk_slice_from_read_source("vertex-disk-query-closeout", fixture.read_source);

    assert_eq!(closeout.counters().output_row_count(), 1);
    assert_eq!(closeout.counters().selected_source_row_count(), 1);
    assert_eq!(closeout.counters().whole_view_fallback_count(), 0);
    assert_eq!(
        closeout.migrated_family_closeout().family_identity(),
        DerivedTopologyProductFamilyIdentity::VertexDisks
    );
}

#[test]
fn closeout_harness_binds_query_read_source_to_product_output() {
    let fixture = query_native_vertex_disk_read_source(
        "vertex-disk.phase-15.harness-closeout",
        "vertex-disk-harness-closeout",
    );

    let closeout = close_vertex_disk_slice_from_read_source(
        "vertex-disk-harness-closeout",
        fixture.read_source,
    );

    assert!(!closeout.vertex_disk_output_digest().is_empty());
    assert!(!closeout.vertex_disk_executed_row_digest().is_empty());
    assert_eq!(closeout.counters().execution_work_count(), 1);
}
