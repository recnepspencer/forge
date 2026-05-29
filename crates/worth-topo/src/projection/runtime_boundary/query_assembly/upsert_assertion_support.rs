use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use schema::facade::{EntityKind, RelationKind};

use super::*;
use crate::facade::milestone_one_runtime_builder;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};

pub(super) fn seeded_current_head_workspace(
    stem: &str,
) -> (
    forge_query::facade::ForgeQueryWorkspace,
    TopologyQueryAssembly,
    schema::facade::VerifiedTopologyCommit,
) {
    let mut runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        stem,
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, stem).expect("query workspace should build");
    let assembly =
        TopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");
    (workspace, assembly, verified)
}

pub(super) fn decode_entity_id(row: &ForgeQueryEntity) -> EntityId {
    serde_json::from_value(
        row.payload
            .get("lineage")
            .and_then(|value| value.get("provenance"))
            .cloned()
            .expect("entity provenance"),
    )
    .expect("entity provenance should decode")
}

pub(super) fn decode_relation_id(row: &ForgeQueryEntity) -> RelationId {
    serde_json::from_value(
        row.payload
            .get("lineage")
            .and_then(|value| value.get("provenance"))
            .cloned()
            .expect("relation provenance"),
    )
    .expect("relation provenance should decode")
}

pub(super) fn decode_entity_kind(row: &ForgeQueryEntity) -> EntityKind {
    let kind_name = row
        .payload
        .get("topology")
        .and_then(|value| value.get("kind"))
        .and_then(serde_json::Value::as_str)
        .expect("entity kind");
    EntityKind::ALL
        .into_iter()
        .find(|kind| kind.kind_name() == kind_name)
        .expect("entity kind should decode")
}

pub(super) fn decode_relation_kind(row: &ForgeQueryEntity) -> RelationKind {
    let kind_name = row
        .payload
        .get("topology")
        .and_then(|value| value.get("kind"))
        .and_then(serde_json::Value::as_str)
        .expect("relation kind");
    RelationKind::ALL
        .into_iter()
        .find(|kind| kind.kind_name() == kind_name)
        .expect("relation kind should decode")
}

pub(super) fn decode_relation_endpoints(
    relation_row: &ForgeQueryEntity,
    entity_rows: &[ForgeQueryEntity],
) -> (EntityId, EntityId) {
    let source_identity = relation_row
        .payload
        .get("topology")
        .and_then(|value| value.get("source_identity"))
        .and_then(serde_json::Value::as_str)
        .expect("source identity");
    let target_identity = relation_row
        .payload
        .get("topology")
        .and_then(|value| value.get("target_identity"))
        .and_then(serde_json::Value::as_str)
        .expect("target identity");
    let source_id = decode_entity_id(
        entity_rows
            .iter()
            .find(|row| row.identity == source_identity)
            .expect("source row should exist"),
    );
    let target_id = decode_entity_id(
        entity_rows
            .iter()
            .find(|row| row.identity == target_identity)
            .expect("target row should exist"),
    );
    (source_id, target_id)
}
