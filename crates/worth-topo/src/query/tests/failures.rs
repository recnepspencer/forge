use super::*;

#[test]
fn query_materializer_rejects_malformed_relation_endpoints() {
    let entity_rows = vec![forge_query::facade::ForgeQueryEntity {
        identity: "entity:0:1:0".to_string(),
        payload: json!({
            "topology": {
                "kind": WorthTopologyEntityKind::Model.kind_name(),
                "structure": "model-a",
            }
        }),
    }];
    let relation_rows = vec![forge_query::facade::ForgeQueryEntity {
        identity: "entity:0:9:0".to_string(),
        payload: json!({
            "topology": {
                "kind": WorthTopologyRelationKind::ModelOwnsBody.kind_name(),
                "source_identity": "bad-source",
                "target_identity": "entity:0:2:0",
            }
        }),
    }];

    let error = materialized_topology_from_query_rows(&entity_rows, &relation_rows)
        .expect_err("malformed query rows must fail closed");

    assert!(error
        .to_string()
        .contains("expected forge-query entity identity"));
}

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
    let materialized = MaterializedTopologyView::whole_view(
        crate::data::topology_view::WorthTopologyView::default(),
    );
    let materialized_rows =
        vec![serde_json::to_value(materialized).expect("materialized topology should serialize")];

    let error = validation_report_from_query_rows(
        &materialized_rows,
        &[json!({ "not": "an interpreted topology row" })],
    )
    .expect_err("malformed interpreted rows must fail closed");

    assert!(error.to_string().contains("failed to decode"));
}
