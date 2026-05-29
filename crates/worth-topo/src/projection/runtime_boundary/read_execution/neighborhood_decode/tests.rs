use forge_query::facade::{ForgeQueryEntity, RelationName};
use serde_json::json;

use super::{
    decode_local_rewire_neighborhood, decode_radial_neighborhood, decode_shared_vertex_neighborhood,
};

fn relation(name: &str) -> RelationName {
    RelationName::new(name).expect("test relation names should be valid")
}

fn row(
    identity: &str,
    relations: serde_json::Value,
    relation_ids: serde_json::Value,
) -> ForgeQueryEntity {
    ForgeQueryEntity {
        identity: identity.to_string(),
        payload: json!({
            "relations": relations,
            "relation_identities": relation_ids,
        }),
    }
}

#[test]
fn shared_vertex_decode_fails_closed_when_edge_relation_is_missing() {
    let rows = vec![row(
        "he:1",
        json!({
            "starts_at_vertex": "v:1",
            "ends_at_vertex": "v:2",
        }),
        json!({}),
    )];

    let error = decode_shared_vertex_neighborhood(
        &rows,
        "he:1",
        &relation("uses_edge"),
        &[relation("starts_at_vertex"), relation("ends_at_vertex")],
        "shared-vertex neighborhood",
    )
    .expect_err("missing edge materialization should fail closed");

    assert!(error.to_string().contains("uses_edge"));
}

#[test]
fn radial_decode_fails_closed_when_relation_record_id_is_missing() {
    let rows = vec![
        row(
            "he:1",
            json!({
                "uses_edge": "e:1",
                "radial_next": "he:2",
            }),
            json!({}),
        ),
        row(
            "he:2",
            json!({
                "uses_edge": "e:1",
            }),
            json!({}),
        ),
    ];

    let error = decode_radial_neighborhood(
        &rows,
        "he:1",
        &relation("uses_edge"),
        &relation("radial_next"),
        "radial neighborhood",
    )
    .expect_err("missing radial-next relation id should fail closed");

    assert!(error
        .to_string()
        .contains("relation identity materialization"));
}

#[test]
fn local_rewire_decode_fails_closed_when_previous_relation_is_missing() {
    let rows = vec![row(
        "he:1",
        json!({
            "next": "he:2",
        }),
        json!({
            "next": "rel:next:1",
        }),
    )];

    let error = decode_local_rewire_neighborhood(
        &rows,
        "he:1",
        1,
        &relation("next"),
        &relation("prev"),
        "local rewire neighborhood",
    )
    .expect_err("missing previous relation should fail closed");

    assert!(error.to_string().contains("prev"));
}
