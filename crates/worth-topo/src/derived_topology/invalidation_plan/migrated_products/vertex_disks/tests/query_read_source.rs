use super::support::query_native_vertex_disk_read_source;

#[test]
fn read_stage_consumes_query_native_shared_vertex_facts() {
    let fixture = query_native_vertex_disk_read_source(
        "vertex-disk.phase-15.query-read",
        "vertex-disk-query-read",
    );
    let read_source = fixture.read_source;

    assert_eq!(read_source.selected_rows().len(), 1);
    assert_eq!(
        read_source.selected_rows()[0].source_half_edge_identity(),
        fixture.source_identity
    );
    assert!(!read_source.selected_rows()[0]
        .touched_vertex_identities()
        .is_empty());
    assert_eq!(read_source.counters().touched_half_edge_lookup_count(), 1);
    assert_eq!(read_source.counters().selected_source_row_count(), 1);
    assert_eq!(read_source.counters().whole_view_fallback_count(), 0);
    assert_eq!(read_source.query_report_digests().len(), 1);
}
