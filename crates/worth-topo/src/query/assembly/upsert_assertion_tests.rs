use worth_schema::facade::{seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase};
use worth_schema::facade::{
    RawWorthTopologyIntent, WorthEntityKind, WorthMutationOrigin, WorthRelationKind,
    WorthTopologyMutation,
};

use super::*;
use crate::facade::worth_milestone_one_runtime_builder;
use crate::reader::WorthTopologyReader;
use forge_query::facade::{
    ForgeQueryExistingTruthAssertionDenialKind, ForgeQueryExistingTruthAssertionMode,
    ForgeQueryRuntimeError,
};

#[test]
fn query_native_assembly_applies_topology_entity_upsert_through_existing_binding_assertion() {
    let mut runtime = worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native-assembly-apply-upsert-entity",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let reader = WorthTopologyReader::new(&runtime);
    let read_view = reader
        .read_view(&verified.read_basis)
        .expect("read view should open");
    let entity = read_view.entities()[0].clone();
    let entity_kind = WorthEntityKind::from_kind_id(entity.kind.kind_id)
        .expect("read view entity kind should decode into worth kind");

    let mut workspace = worth_topology_query_workspace("worth-topology-query-apply-upsert-entity")
        .expect("query workspace should build");
    let assembly =
        WorthTopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");
    assembly
        .import_read_view(&mut workspace, &read_view, &verified.read_basis)
        .expect("read view should import into query truth");

    let applied = assembly
        .apply_raw_intent(
            &mut workspace,
            RawWorthTopologyIntent::new(
                vec![WorthTopologyMutation::UpsertEntity {
                    entity_id: entity.entity_id,
                    kind: entity_kind,
                }],
                WorthMutationOrigin::LocalEdit,
            ),
            &verified.read_basis,
        )
        .expect("topology entity upsert should lower through imported binding assertion");

    assert_eq!(applied.receipt.write_count(), 1);
    assert_eq!(
        applied.receipt.write_receipts()[0].mutation_family(),
        forge_query::facade::ForgeQueryMutationFamily::Assertion
    );
    assert!(
        applied.receipt.write_receipts()[0].deltas().is_empty(),
        "topology entity assertion must not mint a write delta"
    );
    assert!(applied
        .mutation_evidence
        .touched_aspect_paths
        .contains(&"topology.structure".to_string()));
    let binding = applied.receipt.write_receipts()[0]
        .existing_truth_binding_evidence()
        .expect("upsert should retain existing-binding evidence");
    assert_eq!(
        binding.family().as_str(),
        "direct-entity-identity",
        "entity upsert should preserve the direct entity binding family"
    );
    assert_eq!(
        binding.authoritative_identity(),
        format!("{:?}", entity.entity_id)
    );
    assert_eq!(binding.target_collection(), Some("WorthTopologyEntity"));
    let assertion = applied.receipt.write_receipts()[0]
        .existing_truth_assertion_evidence()
        .expect("entity upsert should retain assertion evidence");
    assert_eq!(
        assertion.mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
}

#[test]
fn query_native_assembly_applies_topology_relation_upsert_through_existing_binding_assertion() {
    let mut runtime = worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native-assembly-apply-upsert-relation",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let reader = WorthTopologyReader::new(&runtime);
    let read_view = reader
        .read_view(&verified.read_basis)
        .expect("read view should open");
    let relation = read_view.relations()[0].clone();
    let relation_kind = WorthRelationKind::from_kind_id(relation.kind.kind_id)
        .expect("read view relation kind should decode into worth kind");

    let mut workspace =
        worth_topology_query_workspace("worth-topology-query-apply-upsert-relation")
            .expect("query workspace should build");
    let assembly =
        WorthTopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");
    assembly
        .import_read_view(&mut workspace, &read_view, &verified.read_basis)
        .expect("read view should import into query truth");

    let applied = assembly
        .apply_raw_intent(
            &mut workspace,
            RawWorthTopologyIntent::new(
                vec![WorthTopologyMutation::UpsertRelation {
                    relation_id: relation.relation_id,
                    kind: relation_kind,
                    source: relation.source,
                    target: relation.target,
                }],
                WorthMutationOrigin::LocalEdit,
            ),
            &verified.read_basis,
        )
        .expect("topology relation upsert should lower through imported binding assertion");

    assert_eq!(applied.receipt.write_count(), 1);
    assert_eq!(
        applied.receipt.write_receipts()[0].mutation_family(),
        forge_query::facade::ForgeQueryMutationFamily::Assertion
    );
    assert!(
        applied.receipt.write_receipts()[0].deltas().is_empty(),
        "topology relation assertion must not mint a write delta"
    );
    assert!(applied
        .mutation_evidence
        .touched_aspect_paths
        .iter()
        .any(|path| path.starts_with("topology.")));
    let binding = applied.receipt.write_receipts()[0]
        .existing_truth_binding_evidence()
        .expect("upsert should retain existing-binding evidence");
    assert_eq!(
        binding.family().as_str(),
        "direct-relation-identity",
        "relation upsert should preserve the direct relation binding family"
    );
    assert_eq!(
        binding.authoritative_identity(),
        format!("{:?}", relation.relation_id)
    );
    assert_eq!(binding.target_collection(), Some("WorthTopologyRelation"));
    let assertion = applied.receipt.write_receipts()[0]
        .existing_truth_assertion_evidence()
        .expect("relation upsert should retain assertion evidence");
    assert_eq!(
        assertion.mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
}

#[test]
fn query_native_assembly_rejects_topology_entity_upsert_when_imported_kind_mismatches() {
    let mut runtime = worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native-assembly-upsert-entity-mismatch",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let reader = WorthTopologyReader::new(&runtime);
    let read_view = reader
        .read_view(&verified.read_basis)
        .expect("read view should open");
    let entity = read_view.entities()[0].clone();
    let entity_kind = WorthEntityKind::from_kind_id(entity.kind.kind_id)
        .expect("read view entity kind should decode into worth kind");

    let mut workspace =
        worth_topology_query_workspace("worth-topology-query-upsert-entity-mismatch")
            .expect("query workspace should build");
    let assembly =
        WorthTopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");
    assembly
        .import_read_view(&mut workspace, &read_view, &verified.read_basis)
        .expect("read view should import into query truth");

    let error = assembly
        .apply_raw_intent(
            &mut workspace,
            RawWorthTopologyIntent::new(
                vec![WorthTopologyMutation::UpsertEntity {
                    entity_id: entity.entity_id,
                    kind: if entity_kind
                        == WorthEntityKind::Topology(
                            worth_schema::facade::WorthTopologyEntityKind::Vertex,
                        ) {
                        WorthEntityKind::Topology(
                            worth_schema::facade::WorthTopologyEntityKind::Face,
                        )
                    } else {
                        WorthEntityKind::Topology(
                            worth_schema::facade::WorthTopologyEntityKind::Vertex,
                        )
                    },
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
fn query_native_assembly_rejects_topology_relation_upsert_when_imported_shape_mismatches() {
    let mut runtime = worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native-assembly-upsert-relation-mismatch",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let reader = WorthTopologyReader::new(&runtime);
    let read_view = reader
        .read_view(&verified.read_basis)
        .expect("read view should open");
    let relation = read_view.relations()[0].clone();
    let relation_kind = WorthRelationKind::from_kind_id(relation.kind.kind_id)
        .expect("read view relation kind should decode into worth kind");

    let mut workspace =
        worth_topology_query_workspace("worth-topology-query-upsert-relation-mismatch")
            .expect("query workspace should build");
    let assembly =
        WorthTopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");
    assembly
        .import_read_view(&mut workspace, &read_view, &verified.read_basis)
        .expect("read view should import into query truth");

    let error = assembly
        .apply_raw_intent(
            &mut workspace,
            RawWorthTopologyIntent::new(
                vec![WorthTopologyMutation::UpsertRelation {
                    relation_id: relation.relation_id,
                    kind: relation_kind,
                    source: relation.target,
                    target: relation.source,
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
                ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch
            );
            assert!(
                matches!(
                    denial.asserted_aspect_path(),
                    Some("topology.kind")
                        | Some("topology.source_identity")
                        | Some("topology.target_identity")
                ),
                "relation mismatch should surface on one of the asserted relation-shape fields"
            );
        }
        other => panic!("expected query assertion denial, got {other:?}"),
    }
}
