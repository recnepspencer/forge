use forge_query::facade::{
    ForgeQueryEntity, ForgeQueryExistingTruthAssertionDenialKind, ForgeQueryRuntimeError,
};
use forge_relational::facade::identity::{EntityId, RelationId};
use worth_schema::facade::{seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase};
use worth_schema::facade::{
    RawWorthTopologyIntent, WorthEntityKind, WorthMutationOrigin, WorthRelationKind,
    WorthTopologyEntityKind, WorthTopologyMutation,
};

use super::*;
use crate::facade::worth_milestone_one_runtime_builder;
use crate::query::{worth_topology_runtime, WorthTopologyRuntimeAdapters};

fn seeded_current_head_workspace(
    stem: &str,
) -> (
    forge_query::facade::ForgeQueryWorkspace,
    WorthTopologyQueryAssembly,
    worth_schema::facade::VerifiedTopologyCommit,
) {
    let mut runtime = worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        stem,
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, stem).expect("query workspace should build");
    let assembly =
        WorthTopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");
    (workspace, assembly, verified)
}

fn decode_entity_id(row: &ForgeQueryEntity) -> EntityId {
    serde_json::from_value(
        row.payload
            .get("lineage")
            .and_then(|value| value.get("provenance"))
            .cloned()
            .expect("entity provenance"),
    )
    .expect("entity provenance should decode")
}

fn decode_relation_id(row: &ForgeQueryEntity) -> RelationId {
    serde_json::from_value(
        row.payload
            .get("lineage")
            .and_then(|value| value.get("provenance"))
            .cloned()
            .expect("relation provenance"),
    )
    .expect("relation provenance should decode")
}

fn decode_entity_kind(row: &ForgeQueryEntity) -> WorthEntityKind {
    let kind_name = row
        .payload
        .get("topology")
        .and_then(|value| value.get("kind"))
        .and_then(serde_json::Value::as_str)
        .expect("entity kind");
    WorthEntityKind::ALL
        .into_iter()
        .find(|kind| kind.kind_name() == kind_name)
        .expect("entity kind should decode")
}

fn decode_relation_kind(row: &ForgeQueryEntity) -> WorthRelationKind {
    let kind_name = row
        .payload
        .get("topology")
        .and_then(|value| value.get("kind"))
        .and_then(serde_json::Value::as_str)
        .expect("relation kind");
    WorthRelationKind::ALL
        .into_iter()
        .find(|kind| kind.kind_name() == kind_name)
        .expect("relation kind should decode")
}

fn decode_relation_endpoints(
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

#[test]
fn query_native_assembly_denies_topology_entity_upsert_until_backend_verified_assertions_exist() {
    let (mut workspace, assembly, verified) =
        seeded_current_head_workspace("query-native-assembly-apply-upsert-entity");
    let entity_row = workspace.read(assembly.entities())[0].clone();
    let entity_id = decode_entity_id(&entity_row);
    let entity_kind = decode_entity_kind(&entity_row);

    let error = assembly
        .apply_raw_intent(
            &mut workspace,
            RawWorthTopologyIntent::new(
                vec![WorthTopologyMutation::UpsertEntity {
                    entity_id,
                    kind: entity_kind,
                }],
                WorthMutationOrigin::LocalEdit,
            ),
            &verified.read_basis,
        )
        .expect_err("entity upsert should fail closed until backend verification support exists");

    match error {
        super::authority::WorthTopologyQueryApplyError::Query(
            ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial),
        ) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
            );
            assert_eq!(denial.asserted_aspect_path(), None);
        }
        other => panic!("expected backend-verification denial, got {other:?}"),
    }
}

#[test]
fn query_native_assembly_denies_topology_relation_upsert_until_backend_verified_assertions_exist() {
    let (mut workspace, assembly, verified) =
        seeded_current_head_workspace("query-native-assembly-apply-upsert-relation");
    let relation_row = workspace.read(assembly.relations())[0].clone();
    let relation_id = decode_relation_id(&relation_row);
    let relation_kind = decode_relation_kind(&relation_row);
    let entity_rows = workspace.read(assembly.entities());
    let (source_id, target_id) = decode_relation_endpoints(&relation_row, &entity_rows);

    let error = assembly
        .apply_raw_intent(
            &mut workspace,
            RawWorthTopologyIntent::new(
                vec![WorthTopologyMutation::UpsertRelation {
                    relation_id,
                    kind: relation_kind,
                    source: source_id,
                    target: target_id,
                }],
                WorthMutationOrigin::LocalEdit,
            ),
            &verified.read_basis,
        )
        .expect_err("relation upsert should fail closed until backend verification support exists");

    match error {
        super::authority::WorthTopologyQueryApplyError::Query(
            ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial),
        ) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
            );
            assert_eq!(denial.asserted_aspect_path(), None);
        }
        other => panic!("expected backend-verification denial, got {other:?}"),
    }
}

#[test]
fn query_native_assembly_rejects_topology_entity_upsert_when_live_kind_mismatches() {
    let (mut workspace, assembly, verified) =
        seeded_current_head_workspace("query-native-assembly-upsert-entity-mismatch");
    let entity_row = workspace.read(assembly.entities())[0].clone();
    let entity_id = decode_entity_id(&entity_row);
    let live_kind = decode_entity_kind(&entity_row);
    let mismatched_kind = if live_kind == WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex)
    {
        WorthEntityKind::Topology(WorthTopologyEntityKind::Face)
    } else {
        WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex)
    };

    let error = assembly
        .apply_raw_intent(
            &mut workspace,
            RawWorthTopologyIntent::new(
                vec![WorthTopologyMutation::UpsertEntity {
                    entity_id,
                    kind: mismatched_kind,
                }],
                WorthMutationOrigin::LocalEdit,
            ),
            &verified.read_basis,
        )
        .expect_err("mismatched entity kind should fail closed");

    match error {
        super::authority::WorthTopologyQueryApplyError::Query(
            ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial),
        ) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
            );
            assert_eq!(denial.asserted_aspect_path(), None);
        }
        other => panic!("expected query assertion denial, got {other:?}"),
    }
}

#[test]
fn query_native_assembly_rejects_topology_relation_upsert_when_live_shape_mismatches() {
    let (mut workspace, assembly, verified) =
        seeded_current_head_workspace("query-native-assembly-upsert-relation-mismatch");
    let relation_row = workspace.read(assembly.relations())[0].clone();
    let relation_id = decode_relation_id(&relation_row);
    let relation_kind = decode_relation_kind(&relation_row);
    let entity_rows = workspace.read(assembly.entities());
    let (source_id, target_id) = decode_relation_endpoints(&relation_row, &entity_rows);

    let error = assembly
        .apply_raw_intent(
            &mut workspace,
            RawWorthTopologyIntent::new(
                vec![WorthTopologyMutation::UpsertRelation {
                    relation_id,
                    kind: relation_kind,
                    source: target_id,
                    target: source_id,
                }],
                WorthMutationOrigin::LocalEdit,
            ),
            &verified.read_basis,
        )
        .expect_err("mismatched relation shape should fail closed");

    match error {
        super::authority::WorthTopologyQueryApplyError::Query(
            ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial),
        ) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported
            );
            assert_eq!(denial.asserted_aspect_path(), None);
        }
        other => panic!("expected query assertion denial, got {other:?}"),
    }
}
