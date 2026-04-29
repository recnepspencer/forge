use serde_json::Value;
use worth_schema::facade::{seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase};
use worth_schema::facade::{
    RawWorthTopologyIntent, WorthCreateKey, WorthEntityKind, WorthEntityReference,
    WorthMutationOrigin, WorthTopologyEntityKind, WorthTopologyMutation, WorthTopologyRelationKind,
};

use super::import::import_topology_relation_records;
use super::*;
use crate::facade::{certify_milestone_one_read_view_traced, worth_milestone_one_runtime_builder};
use crate::reader::WorthTopologyReader;

#[test]
fn query_native_assembly_imports_real_read_view_and_matches_reader_outputs() {
    let mut runtime = worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native-assembly-sheet-disk",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let reader = WorthTopologyReader::new(&runtime);
    let read_view = reader
        .read_view(&verified.read_basis)
        .expect("read view should open");

    let mut workspace = worth_topology_query_workspace("worth-topology-query-assembly")
        .expect("query workspace should build");
    let assembly =
        WorthTopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");
    let receipt = assembly
        .import_read_view(&mut workspace, &read_view, &verified.read_basis)
        .expect("read view should import into query truth");
    let snapshot = assembly
        .snapshot(&mut workspace)
        .expect("query snapshot should decode");
    let persistent_name_rows = workspace.read(assembly.persistent_names());
    let staged = reader
        .stage(&verified.read_basis)
        .expect("reader should stage");

    assert_eq!(
        receipt.affected_live_view_ids(),
        &[
            "worth.naming.persistent_names".to_string(),
            "worth.topology.entities".to_string(),
            "worth.topology.relations".to_string(),
        ]
    );
    assert!(receipt.write_count() > 1);
    assert_eq!(
        snapshot.naming_attachments,
        certify_milestone_one_read_view_traced(&read_view, verified.read_basis.clone())
            .expect("milestone one certification should succeed")
            .into_primary_result()
            .naming_attachment_report
    );
    let expected_evidence = serde_json::to_value(
        WorthTopologyQueryMutationEvidence::from_read_basis(&verified.read_basis),
    )
    .expect("query mutation evidence should serialize");
    assert!(receipt.write_receipts().iter().all(|write| {
        write
            .mutation_metadata()
            .get(WorthTopologyQueryMutationEvidence::metadata_key())
            == Some(&expected_evidence)
    }));
    assert!(persistent_name_rows.iter().all(|row| {
        row.payload
            .get("lineage")
            .and_then(|value| value.get("provenance"))
            .is_some()
    }));
    assert_eq!(receipt.write_count(), receipt.write_receipts().len());
    assert_eq!(
        snapshot.materialized.topology(),
        staged.materialized().topology()
    );
    assert_eq!(
        snapshot.materialized.report().breadth.topology_entity_count,
        staged.materialized().report().breadth.topology_entity_count
    );
    assert_eq!(
        snapshot
            .materialized
            .report()
            .breadth
            .topology_relation_count,
        staged
            .materialized()
            .report()
            .breadth
            .topology_relation_count
    );
    assert_eq!(
        snapshot.materialized.report().whole_view_materialization,
        staged.materialized().report().whole_view_materialization
    );
    assert_eq!(
        snapshot.materialized.report().fallback_class,
        staged.materialized().report().fallback_class
    );
    assert_eq!(
        snapshot.interpreted.interpretations(),
        staged.interpreted().interpretations()
    );
    assert_eq!(
        snapshot.interpreted.boundary_summaries(),
        staged.interpreted().boundary_summaries()
    );
    assert_eq!(
        snapshot.interpreted.radial_summaries(),
        staged.interpreted().radial_summaries()
    );
    assert_eq!(snapshot.interpreted.report(), staged.interpreted().report());
    assert_eq!(snapshot.validation, staged.validation().clone());
    assert_eq!(
        snapshot.diagnostics.invalidation_report,
        crate::diagnostics::build_derived_invalidation_report(&verified.read_basis)
    );
    assert_eq!(
        snapshot.diagnostics.rebuild_report,
        crate::diagnostics::build_derived_rebuild_report(
            staged.materialized(),
            &snapshot.interpreted,
            staged.validation(),
        )
    );
    assert_eq!(
        snapshot.diagnostics.fallback_report,
        crate::diagnostics::build_derived_fallback_report(
            &verified.read_basis,
            staged.materialized(),
        )
    );
    assert_eq!(
        snapshot.equivalence_contract.authority_snapshot_id,
        verified.read_basis.snapshot().snapshot_id.0
    );
    assert_eq!(
        snapshot.equivalence_contract.authority_branch_id,
        verified.read_basis.branch_id().0.as_str()
    );
    assert_eq!(
        snapshot.equivalence_contract.authoritative_mutation_origin,
        verified.read_basis.authoritative_mutation_origin()
    );
    assert_eq!(
        snapshot.equivalence_contract.derivation_origin,
        verified.read_basis.derivation_origin()
    );
    assert_eq!(
        snapshot.equivalence_contract.truth_basis_digest_hex,
        verified
            .read_basis
            .authority
            .truth_basis_identity
            .mutation_batch_digest_hex
    );
    assert_eq!(
        snapshot.equivalence_contract.touched_aspect_count,
        verified.read_basis.touched_aspects().len()
    );
    assert_eq!(
        snapshot.equivalence_contract.triggered_invalidation_targets,
        snapshot
            .diagnostics
            .invalidation_report
            .rows
            .iter()
            .filter(|row| row.triggered)
            .map(|row| row.target)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        snapshot.equivalence_contract.precision_fallback_count,
        verified.read_basis.precision_fallbacks.len()
    );
    assert_eq!(
        snapshot
            .equivalence_contract
            .precision_budget_fallback_count,
        verified.read_basis.precision_budget_fallbacks.len()
    );
}

#[test]
fn query_native_assembly_rejects_relation_import_when_endpoint_mapping_is_missing() {
    let mut runtime = worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native-assembly-missing-endpoint",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let reader = WorthTopologyReader::new(&runtime);
    let read_view = reader
        .read_view(&verified.read_basis)
        .expect("read view should open");
    let evidence = WorthTopologyQueryMutationEvidence::from_read_basis(&verified.read_basis);
    let mut workspace = worth_topology_query_workspace("worth-topology-query-import-denial")
        .expect("query workspace should build");

    let error = import_topology_relation_records(
        &mut workspace,
        &[read_view.relations()[0].clone()],
        &std::collections::BTreeMap::new(),
        &evidence,
    )
    .expect_err("relation import should fail closed when entity identities were not imported");

    assert!(error
        .to_string()
        .contains("missing imported query identity mapping"));
}

#[test]
fn query_native_assembly_applies_created_entity_refs_through_ordered_query_receipts() {
    let mut runtime = worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native-assembly-apply-created-refs",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let reader = WorthTopologyReader::new(&runtime);
    let read_view = reader
        .read_view(&verified.read_basis)
        .expect("read view should open");

    let mut workspace = worth_topology_query_workspace("worth-topology-query-apply-created-refs")
        .expect("query workspace should build");
    let assembly =
        WorthTopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");
    assembly
        .import_read_view(&mut workspace, &read_view, &verified.read_basis)
        .expect("read view should import into query truth");
    let entity_count_before = workspace.read(assembly.entities()).len();
    let relation_count_before = workspace.read(assembly.relations()).len();

    let applied = assembly
        .apply_raw_intent(
            &mut workspace,
            RawWorthTopologyIntent::new(
                vec![
                    WorthTopologyMutation::CreateEntity {
                        create_key: WorthCreateKey::new("query.apply.half-edge-a"),
                        kind: WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
                    },
                    WorthTopologyMutation::CreateEntity {
                        create_key: WorthCreateKey::new("query.apply.vertex-b"),
                        kind: WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex),
                    },
                    WorthTopologyMutation::CreateRelation {
                        create_key: WorthCreateKey::new("query.apply.link"),
                        kind: worth_schema::facade::WorthRelationKind::Topology(
                            WorthTopologyRelationKind::HalfEdgeStartsAtVertex,
                        ),
                        source: WorthEntityReference::Created(WorthCreateKey::new(
                            "query.apply.half-edge-a",
                        )),
                        target: WorthEntityReference::Created(WorthCreateKey::new(
                            "query.apply.vertex-b",
                        )),
                    },
                ],
                WorthMutationOrigin::LocalEdit,
            ),
            &verified.read_basis,
        )
        .expect("ordered create refs should lower through query receipts");
    let entity_rows = workspace.read(assembly.entities());
    let relation_rows = workspace.read(assembly.relations());
    let created_entity_identities = applied.receipt.write_receipts()[0..2]
        .iter()
        .map(|receipt| receipt.deltas()[0].entity_identity.clone())
        .collect::<Vec<_>>();
    let created_relation_identity = applied.receipt.write_receipts()[2].deltas()[0]
        .entity_identity
        .clone();
    let created_relation_row = relation_rows
        .iter()
        .find(|row| row.identity == created_relation_identity)
        .expect("created relation row should exist");

    assert_eq!(applied.receipt.write_count(), 3);
    assert!(applied
        .mutation_evidence
        .touched_aspect_paths
        .contains(&"topology.boundary".to_string()));
    assert_eq!(entity_rows.len(), entity_count_before + 2);
    assert_eq!(relation_rows.len(), relation_count_before + 1);
    assert_eq!(
        created_relation_row
            .payload
            .get("topology")
            .and_then(|value| value.get("source_identity"))
            .and_then(Value::as_str),
        Some(created_entity_identities[0].as_str())
    );
    assert_eq!(
        created_relation_row
            .payload
            .get("topology")
            .and_then(|value| value.get("target_identity"))
            .and_then(Value::as_str),
        Some(created_entity_identities[1].as_str())
    );
}

#[test]
fn query_native_assembly_applies_topology_relation_delete_through_existing_binding() {
    let mut runtime = worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native-assembly-apply-delete",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");
    let reader = WorthTopologyReader::new(&runtime);
    let read_view = reader
        .read_view(&verified.read_basis)
        .expect("read view should open");
    let relation_id = read_view.relations()[0].relation_id;

    let mut workspace = worth_topology_query_workspace("worth-topology-query-apply-delete")
        .expect("query workspace should build");
    let assembly =
        WorthTopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");
    assembly
        .import_read_view(&mut workspace, &read_view, &verified.read_basis)
        .expect("read view should import into query truth");
    let relation_count_before = workspace.read(assembly.relations()).len();

    let applied = assembly
        .apply_raw_intent(
            &mut workspace,
            RawWorthTopologyIntent::new(
                vec![WorthTopologyMutation::RemoveRelation { relation_id }],
                WorthMutationOrigin::LocalEdit,
            ),
            &verified.read_basis,
        )
        .expect("topology relation delete should lower through existing binding");
    let relation_rows = workspace.read(assembly.relations());

    assert_eq!(applied.receipt.write_count(), 1);
    assert_eq!(
        applied.receipt.write_receipts()[0]
            .existing_truth_binding_evidence()
            .expect("delete should retain existing-binding evidence")
            .target_collection(),
        Some("WorthTopologyRelation")
    );
    assert_eq!(relation_rows.len(), relation_count_before - 1);
    assert!(!relation_rows.iter().any(|row| {
        row.payload
            .get("lineage")
            .and_then(|value| value.get("provenance"))
            .cloned()
            .and_then(|value| serde_json::from_value(value).ok())
            == Some(relation_id)
    }));
}
