use super::support::vertex_disk_source_row;

#[test]
fn source_row_names_touched_vertices_and_incident_edges() {
    let row = vertex_disk_source_row(
        &[10, 11],
        24,
        100,
        &[24, 25],
        &[25, 26, 27],
        &[100, 101, 102],
    );

    assert_eq!(row.touched_vertex_identities().len(), 2);
    assert_eq!(row.incident_half_edge_identities().len(), 2);
    assert_eq!(row.touched_incident_edge_count(), 3);
    assert!(row.branch_vertex_disk());
    assert!(!row.row_digest().is_empty());
}

#[test]
fn source_row_does_not_call_two_incident_edges_a_branch_disk() {
    let row = vertex_disk_source_row(&[10], 24, 100, &[24], &[25, 26], &[100, 101]);

    assert_eq!(row.touched_incident_edge_count(), 2);
    assert!(!row.branch_vertex_disk());
}
