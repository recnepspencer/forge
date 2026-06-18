use forge_query::facade::{ForgeQueryEntity, ForgeQueryEntityIdentity, RelationName};
use forge_runtime_bridge::facade::RelationalBridgeRecordIdentityParts;
use serde_json::json;

use super::{
    decode_local_rewire_neighborhood, decode_radial_neighborhood, decode_shared_vertex_neighborhood,
};

fn relation(name: &str) -> RelationName {
    RelationName::new(name).expect("test relation names should be valid")
}

fn entity_identity(partition: u32, slot: u64, generation: u32) -> ForgeQueryEntityIdentity {
    ForgeQueryEntityIdentity::from_relational_record(RelationalBridgeRecordIdentityParts::entity(
        partition, slot, generation,
    ))
}

fn entity_label(partition: u32, slot: u64, generation: u32) -> String {
    format!("entity:{partition}:{slot}:{generation}")
}

fn row(
    partition: u32,
    slot: u64,
    generation: u32,
    relations: serde_json::Value,
    relation_ids: serde_json::Value,
) -> ForgeQueryEntity {
    ForgeQueryEntity::from_external_projection(
        entity_identity(partition, slot, generation),
        json!({
            "relations": relations,
            "relation_identities": relation_ids,
        }),
    )
}

#[test]
fn shared_vertex_decode_fails_closed_when_edge_relation_is_missing() {
    let anchor = entity_label(0, 1, 1);
    let rows = vec![row(
        0,
        1,
        1,
        json!({
            "starts_at_vertex": entity_label(0, 3, 1),
            "ends_at_vertex": entity_label(0, 4, 1),
        }),
        json!({}),
    )];

    let error = decode_shared_vertex_neighborhood(
        &rows,
        &anchor,
        &relation("uses_edge"),
        &[relation("starts_at_vertex"), relation("ends_at_vertex")],
        "shared-vertex neighborhood",
    )
    .expect_err("missing edge materialization should fail closed");

    assert!(error.to_string().contains("uses_edge"));
}

#[test]
fn radial_decode_fails_closed_when_relation_record_id_is_missing() {
    let anchor = entity_label(0, 1, 1);
    let rows = vec![
        row(
            0,
            1,
            1,
            json!({
                "uses_edge": entity_label(0, 5, 1),
                "radial_next": entity_label(0, 2, 1),
            }),
            json!({}),
        ),
        row(
            0,
            2,
            1,
            json!({
                "uses_edge": entity_label(0, 5, 1),
            }),
            json!({}),
        ),
    ];

    let error = decode_radial_neighborhood(
        &rows,
        &anchor,
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
    let anchor = entity_label(0, 1, 1);
    let rows = vec![row(
        0,
        1,
        1,
        json!({
            "next": entity_label(0, 2, 1),
        }),
        json!({
            "next": "rel:next:1",
        }),
    )];

    let error = decode_local_rewire_neighborhood(
        &rows,
        &anchor,
        1,
        &relation("next"),
        &relation("prev"),
        "local rewire neighborhood",
    )
    .expect_err("missing previous relation should fail closed");

    assert!(error.to_string().contains("prev"));
}
