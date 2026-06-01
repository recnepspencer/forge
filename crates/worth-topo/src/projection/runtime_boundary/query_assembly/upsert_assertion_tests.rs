use forge_query::facade::{ForgeQueryExistingTruthAssertionDenialKind, ForgeQueryRuntimeError};
use schema::facade::platform::authority::{MutationOrigin, RawTopologyIntent, TopologyMutation};
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};

use super::upsert_assertion_support::{
    decode_entity_id, decode_entity_kind, decode_relation_endpoints, decode_relation_id,
    decode_relation_kind, seeded_current_head_workspace,
};
use crate::projection::runtime_boundary::query_assembly::authority_support::mutation_evidence_for_intent;
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
            RawTopologyIntent::new(
                vec![TopologyMutation::UpsertEntity {
                    entity_id,
                    kind: entity_kind,
                }],
                MutationOrigin::LocalEdit,
            ),
            &verified.read_basis(),
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
            RawTopologyIntent::new(
                vec![TopologyMutation::UpsertRelation {
                    relation_id,
                    kind: relation_kind,
                    source: source_id,
                    target: target_id,
                }],
                MutationOrigin::LocalEdit,
            ),
            &verified.read_basis(),
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
    let mismatched_kind = if live_kind == EntityKind::Topology(TopologyEntityKind::Vertex) {
        EntityKind::Topology(TopologyEntityKind::Face)
    } else {
        EntityKind::Topology(TopologyEntityKind::Vertex)
    };

    let error = assembly
        .apply_raw_intent(
            &mut workspace,
            RawTopologyIntent::new(
                vec![TopologyMutation::UpsertEntity {
                    entity_id,
                    kind: mismatched_kind,
                }],
                MutationOrigin::LocalEdit,
            ),
            &verified.read_basis(),
        )
        .expect_err("mismatched entity kind should fail closed");

    match error {
        super::authority::TopologyQueryApplyError::Query(
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
        == RelationKind::Topology(TopologyRelationKind::HalfEdgeStartsAtVertex)
    {
        RelationKind::Topology(TopologyRelationKind::HalfEdgeEndsAtVertex)
    } else {
        RelationKind::Topology(TopologyRelationKind::HalfEdgeStartsAtVertex)
    };

    let error = assembly
        .apply_raw_intent(
            &mut workspace,
            RawTopologyIntent::new(
                vec![TopologyMutation::UpsertRelation {
                    relation_id,
                    kind: mismatched_kind,
                    source: source_id,
                    target: target_id,
                }],
                MutationOrigin::LocalEdit,
            ),
            &verified.read_basis(),
        )
        .expect_err("changed relation kind should reach the verified update lane and then fail closed on the runtime's kind-preserving relation update substrate");

    match error {
        super::authority::TopologyQueryApplyError::Query(ForgeQueryRuntimeError::Workspace(
            workspace_error,
        )) => {
            let message = workspace_error.to_string();
            assert!(
                message.contains(".m1.topology.ownership_surface"),
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
                == RelationKind::Topology(TopologyRelationKind::HalfEdgeEndsAtVertex)
        })
        .expect("seeded topology should contain half-edge endpoint relation");
    let relation_id = decode_relation_id(&relation_row);
    let relation_kind = decode_relation_kind(&relation_row);
    let (source_id, current_target_id) = decode_relation_endpoints(&relation_row, &entity_rows);
    let alternate_target_id = entity_rows
        .iter()
        .filter(|row| decode_entity_kind(row) == EntityKind::Topology(TopologyEntityKind::Vertex))
        .map(decode_entity_id)
        .find(|entity_id| *entity_id != current_target_id && *entity_id != source_id)
        .expect("seeded topology should expose an alternate vertex target");

    let applied = assembly
        .apply_raw_intent(
            &mut workspace,
            RawTopologyIntent::new(
                vec![TopologyMutation::UpsertRelation {
                    relation_id,
                    kind: relation_kind,
                    source: source_id,
                    target: alternate_target_id,
                }],
                MutationOrigin::LocalEdit,
            ),
            &verified.read_basis(),
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
    let (mut workspace, assembly, verified) =
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
            decode_relation_kind(row) == RelationKind::Topology(TopologyRelationKind::ModelOwnsBody)
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
            .find(|row| row.identity() == imported.source_query_identity)
            .expect("source row should exist"),
    );
    let target_id = decode_entity_id(
        entity_rows
            .iter()
            .find(|row| row.identity() == imported.target_query_identity)
            .expect("target row should exist"),
    );
    let desired_kind = RelationKind::Topology(TopologyRelationKind::HalfEdgeStartsAtVertex);

    let evidence = mutation_evidence_for_intent(
        &verified.read_basis(),
        &RawTopologyIntent::new(
            vec![TopologyMutation::UpsertRelation {
                relation_id,
                kind: desired_kind,
                source: source_id,
                target: target_id,
            }],
            MutationOrigin::LocalEdit,
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
