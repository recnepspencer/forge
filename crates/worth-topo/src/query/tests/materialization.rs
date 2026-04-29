use super::support::seed_sheet_disk_topology;
use super::*;

#[test]
fn query_materializer_rebuilds_minimal_topology_from_retained_rows() {
    let mut workspace = worth_topology_query_workspace("worth-query-materializer-minimal")
        .expect("query workspace should build");
    let assembly =
        WorthTopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");

    let last_receipt = seed_sheet_disk_topology(&mut workspace);
    let rows = workspace.materialize(assembly.materialized());
    if let Some(message) = rows[0]
        .get(QUERY_SURFACE_FAILURE_ROW_KEY)
        .and_then(Value::as_str)
    {
        panic!("materialized topology error row: {message}");
    }
    let materialized_view: MaterializedTopologyView =
        serde_json::from_value(rows[0].clone()).expect("materialized topology row");

    assert!(last_receipt
        .affected_derived_view_ids()
        .contains(&MATERIALIZED_TOPOLOGY_SURFACE.to_string()));
    assert_eq!(rows.len(), 1);
    assert_eq!(materialized_view.topology().models.len(), 1);
    assert_eq!(materialized_view.topology().faces.len(), 1);
    assert_eq!(materialized_view.topology().vertices.len(), 1);
    assert_eq!(
        materialized_view.report().breadth.topology_relation_count,
        14
    );
}
