use super::*;

#[test]
fn interpreted_surface_rejects_malformed_materialized_rows() {
    let error = interpreted_topology_from_materialized_rows(&[json!({
        "not": "a materialized topology row",
    })])
    .expect_err("malformed materialized rows must fail closed");

    assert!(error.to_string().contains("failed to decode"));
}

#[test]
fn validation_surface_rejects_malformed_interpreted_rows() {
    let materialized =
        MaterializedTopologyView::whole_view(crate::brep::topology_graph::TopologyView::default());
    let materialized_rows =
        vec![serde_json::to_value(materialized).expect("materialized topology should serialize")];

    let error = validation_report_from_query_rows(
        &materialized_rows,
        &[json!({ "not": "an interpreted topology row" })],
    )
    .expect_err("malformed interpreted rows must fail closed");

    assert!(error.to_string().contains("failed to decode"));
}
