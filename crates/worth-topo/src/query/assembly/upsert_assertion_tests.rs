use forge_query::facade::{
    ForgeQueryEntity, ForgeQueryExistingTruthAssertionDenialKind, ForgeQueryRuntimeError,
};
use forge_relational::facade::identity::{EntityId, RelationId};
use worth_schema::facade::{
    seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase, WorthTopologyRelationKind,
};
use worth_schema::facade::{
    RawWorthTopologyIntent, WorthEntityKind, WorthMutationOrigin, WorthRelationKind,
    WorthTopologyEntityKind, WorthTopologyMutation,
};

use super::*;
use crate::facade::worth_milestone_one_runtime_builder;
use crate::query::assembly::authority_support::mutation_evidence_for_intent;
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
fn query_native_assembly_applies_topology_entity_upsert_with_backend_verified_assertion() {
    let (mut workspace, assembly, verified) =
        seeded_current_head_workspace("query-native-assembly-apply-upsert-entity");
    let entity_row = workspace.read(assembly.entities())[0].clone();
    let entity_id = decode_entity_id(&entity_row);
    let entity_kind = decode_entity_kind(&entity_row);

    let applied = assembly
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
        .expect("entity upsert should execute through backend verification");

    assert_eq!(applied.receipt.write_count(), 1);
    assert_eq!(
        applied.receipt.write_receipts()[0]
            .existing_truth_assertion_evidence()
            .expect("entity upsert should retain assertion evidence")
            .mode()
            .as_str(),
        "backend_verified_assertion"
    );
}

#[test]
fn query_native_assembly_applies_topology_relation_upsert_with_backend_verified_assertion() {
    let (mut workspace, assembly, verified) =
        seeded_current_head_workspace("query-native-assembly-apply-upsert-relation");
    let relation_row = workspace.read(assembly.relations())[0].clone();
    let relation_id = decode_relation_id(&relation_row);
    let relation_kind = decode_relation_kind(&relation_row);
    let entity_rows = workspace.read(assembly.entities());
    let (source_id, target_id) = decode_relation_endpoints(&relation_row, &entity_rows);

    let applied = assembly
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
        .expect("relation upsert should execute through backend verification");

    assert_eq!(applied.receipt.write_count(), 1);
    assert_eq!(
        applied.receipt.write_receipts()[0]
            .existing_truth_assertion_evidence()
            .expect("relation upsert should retain assertion evidence")
            .mode()
            .as_str(),
        "backend_verified_assertion"
    );
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
                ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch
            );
            assert_eq!(denial.asserted_aspect_path(), Some("topology.kind"));
        }
        other => panic!("expected query assertion denial, got {other:?}"),
    }
}

#[test]
fn query_native_assembly_lowers_changed_topology_relation_kind_upserts_into_verified_update_before_invariant_denial(
) {
    let (mut workspace, assembly, verified) =
        seeded_current_head_workspace("query-native-assembly-upsert-relation-mismatch");
    let relation_row = workspace.read(assembly.relations())[0].clone();
    let relation_id = decode_relation_id(&relation_row);
    let live_relation_kind = decode_relation_kind(&relation_row);
    let entity_rows = workspace.read(assembly.entities());
    let (source_id, target_id) = decode_relation_endpoints(&relation_row, &entity_rows);
    let mismatched_kind = if live_relation_kind
        == WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeStartsAtVertex)
    {
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeEndsAtVertex)
    } else {
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeStartsAtVertex)
    };

    let error = assembly
        .apply_raw_intent(
            &mut workspace,
            RawWorthTopologyIntent::new(
                vec![WorthTopologyMutation::UpsertRelation {
                    relation_id,
                    kind: mismatched_kind,
                    source: source_id,
                    target: target_id,
                }],
                WorthMutationOrigin::LocalEdit,
            ),
            &verified.read_basis,
        )
        .expect_err("changed relation kind should reach the verified update lane and then fail closed on the runtime's kind-preserving relation update substrate");

    match error {
        super::authority::WorthTopologyQueryApplyError::Query(
            ForgeQueryRuntimeError::Workspace(workspace_error),
        ) => {
            let message = workspace_error.to_string();
            assert!(
                message.contains("worth.m1.topology.ownership_surface"),
                "expected runtime ownership-surface invariant denial after verified relation update lowering, got: {message}"
            );
            assert!(
                !message.contains(
                    "relation endpoint update intent kind does not match authoritative relation kind"
                ),
                "kind-changing upsert should now reach the verified update lane instead of failing at the old substrate guard: {message}"
            );
        }
        other => panic!("expected workspace-backed relation update denial, got {other:?}"),
    }
}

#[test]
fn query_native_assembly_applies_changed_topology_relation_endpoint_upserts_through_verified_update(
) {
    let (mut workspace, assembly, verified) =
        seeded_current_head_workspace("query-native-assembly-upsert-relation-update-lane");
    let entity_rows = workspace.read(assembly.entities());
    let relation_row = workspace
        .read(assembly.relations())
        .into_iter()
        .find(|row| {
            decode_relation_kind(row)
                == WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeEndsAtVertex)
        })
        .expect("seeded topology should contain half-edge endpoint relation");
    let relation_id = decode_relation_id(&relation_row);
    let relation_kind = decode_relation_kind(&relation_row);
    let (source_id, current_target_id) = decode_relation_endpoints(&relation_row, &entity_rows);
    let alternate_target_id = entity_rows
        .iter()
        .filter(|row| {
            decode_entity_kind(row) == WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex)
        })
        .map(decode_entity_id)
        .find(|entity_id| *entity_id != current_target_id && *entity_id != source_id)
        .expect("seeded topology should expose an alternate vertex target");

    let applied = assembly
        .apply_raw_intent(
            &mut workspace,
            RawWorthTopologyIntent::new(
                vec![WorthTopologyMutation::UpsertRelation {
                    relation_id,
                    kind: relation_kind,
                    source: source_id,
                    target: alternate_target_id,
                }],
                WorthMutationOrigin::LocalEdit,
            ),
            &verified.read_basis,
        )
        .expect("changed relation endpoints should now execute through the verified update lane");

    assert_eq!(applied.receipt.write_count(), 1);
    assert_eq!(
        applied
            .receipt
            .batch_mutation_evidence()
            .backend_verified_update_count(),
        1
    );
}

#[test]
fn query_native_assembly_retains_both_live_and_desired_relation_aspect_families_for_kind_updates() {
    let (workspace, assembly, verified) =
        seeded_current_head_workspace("query-native-assembly-upsert-relation-evidence");
    let entities =
        super::authority_support::index_imported_entities(workspace.read(assembly.entities()))
            .expect("imported entities");
    let relations =
        super::authority_support::index_imported_relations(workspace.read(assembly.relations()))
            .expect("imported relations");
    let relation_row = workspace
        .read(assembly.relations())
        .into_iter()
        .find(|row| {
            decode_relation_kind(row)
                == WorthRelationKind::Topology(WorthTopologyRelationKind::ModelOwnsBody)
        })
        .expect("seeded topology should contain an ownership relation");
    let relation_id = decode_relation_id(&relation_row);
    let imported = relations
        .get(&relation_id)
        .expect("imported relation should exist");
    let entity_rows = workspace.read(assembly.entities());
    let source_id = decode_entity_id(
        entity_rows
            .iter()
            .find(|row| row.identity == imported.source_query_identity)
            .expect("source row should exist"),
    );
    let target_id = decode_entity_id(
        entity_rows
            .iter()
            .find(|row| row.identity == imported.target_query_identity)
            .expect("target row should exist"),
    );
    let desired_kind =
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeStartsAtVertex);

    let evidence = mutation_evidence_for_intent(
        &verified.read_basis,
        &RawWorthTopologyIntent::new(
            vec![WorthTopologyMutation::UpsertRelation {
                relation_id,
                kind: desired_kind,
                source: source_id,
                target: target_id,
            }],
            WorthMutationOrigin::LocalEdit,
        ),
        &entities,
        &relations,
    )
    .expect("mutation evidence should build");

    assert!(
        evidence
            .touched_aspect_paths
            .iter()
            .any(|path| path == "topology.ownership"),
        "expected touched aspect evidence to include the live ownership family"
    );
    assert!(
        evidence
            .touched_aspect_paths
            .iter()
            .any(|path| path == "topology.boundary"),
        "expected touched aspect evidence to include the desired relation family"
    );
}
