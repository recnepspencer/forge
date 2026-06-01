use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::platform::entities::EntityKind;
use schema::facade::platform::relations::RelationKind;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};

use super::*;
use crate::committed_artifact::TopologyCommittedArtifact;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::validation::reference_integrity::milestone_one_runtime_builder;

pub(super) fn seeded_current_head_workspace(
    stem: &str,
) -> (
    forge_query::facade::ForgeQueryWorkspace,
    TopologyQueryAssembly,
    TopologyCommittedArtifact,
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
    let verified = TopologyCommittedArtifact::from_parts(
        verified.canonical_batch().clone(),
        verified.branch_id().clone(),
        verified.commits().to_vec(),
        verified.persisted_truth().clone(),
        verified.read_basis().clone(),
    );
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, stem).expect("query workspace should build");
    let assembly =
        TopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");
    (workspace, assembly, verified)
}

pub(super) fn decode_entity_id(row: &ForgeQueryEntity) -> EntityId {
    serde_json::from_value(
        row.external_row()
            .get("lineage")
            .and_then(|value| value.get("provenance"))
            .cloned()
            .expect("entity provenance"),
    )
    .expect("entity provenance should decode")
}

pub(super) fn decode_relation_id(row: &ForgeQueryEntity) -> RelationId {
    serde_json::from_value(
        row.external_row()
            .get("lineage")
            .and_then(|value| value.get("provenance"))
            .cloned()
            .expect("relation provenance"),
    )
    .expect("relation provenance should decode")
}

pub(super) fn decode_entity_kind(row: &ForgeQueryEntity) -> EntityKind {
    let kind_name = row
        .external_row()
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
        .external_row()
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
        .external_row()
        .get("topology")
        .and_then(|value| value.get("source_identity"))
        .and_then(serde_json::Value::as_str)
        .expect("source identity");
    let target_identity = relation_row
        .external_row()
        .get("topology")
        .and_then(|value| value.get("target_identity"))
        .and_then(serde_json::Value::as_str)
        .expect("target identity");
    let source_id = decode_entity_id(
        entity_rows
            .iter()
            .find(|row| row.identity() == source_identity)
            .expect("source row should exist"),
    );
    let target_id = decode_entity_id(
        entity_rows
            .iter()
            .find(|row| row.identity() == target_identity)
            .expect("target row should exist"),
    );
    (source_id, target_id)
}
