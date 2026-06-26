use super::support::vertex_disk_source_row;

#[test]
fn product_rows_preserve_branch_vertex_disk_semantics() {
    let row = vertex_disk_source_row(&[10], 24, 100, &[24, 25, 26], &[25, 26], &[100, 101, 102]);
    let product = super::super::VertexDiskProductRow::from_source_row(&row);

    assert_eq!(
        product.touched_vertex_identities(),
        row.touched_vertex_identities()
    );
    assert_eq!(product.touched_incident_edge_count(), 3);
    assert!(product.branch_vertex_disk());
    assert_eq!(
        product.incident_different_edge_half_edge_identities(),
        row.incident_different_edge_half_edge_identities()
    );
}
