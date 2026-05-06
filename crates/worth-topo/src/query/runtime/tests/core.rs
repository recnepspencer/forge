use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyQueryEditFamilySupportStatus,
    WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;
use forge_query::facade::ForgeQueryBridgeBackedVerificationSupportStatus;
use worth_schema::facade::topology_authoring::seed_minimal_topology;

#[test]
fn current_head_runtime_support_reports_authoritative_current_head_posture() {
    let runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let support = adapters.support();

    assert!(support.current_head_live_reads_supported());
    assert!(!support.current_head_materialization_supported());
    assert!(support.post_write_materialization_supported());
    assert!(!support.historical_basis_supported());
    assert!(support.authoritative_writes_supported());
    assert!(support.query_edit_execution_supported());
    assert!(support.query_edit_family_supported(
        crate::edit::WorthTopologyEditFamily::AttachBoundaryMembership
    ));
    assert_eq!(
        support.query_edit_family_support_status(
            crate::edit::WorthTopologyEditFamily::AttachBoundaryMembership
        ),
        WorthTopologyQueryEditFamilySupportStatus::PartiallyAdmittedByLane
    );
    assert!(support.query_edit_family_supported(
        crate::edit::WorthTopologyEditFamily::AttachShellOrWireMembership
    ));
    assert_eq!(
        support.query_edit_family_support_status(
            crate::edit::WorthTopologyEditFamily::AttachShellOrWireMembership
        ),
        WorthTopologyQueryEditFamilySupportStatus::PartiallyAdmittedByLane
    );
    assert!(support
        .query_edit_family_supported(crate::edit::WorthTopologyEditFamily::RewireLoopEndpoint));
    assert!(support
        .query_edit_family_supported(crate::edit::WorthTopologyEditFamily::SpliceRadialAdjacency));
    assert!(support
        .query_edit_family_supported(crate::edit::WorthTopologyEditFamily::RewireLoopSuccessor));
    assert_eq!(
        support.query_edit_family_support_status(
            crate::edit::WorthTopologyEditFamily::RewireLoopSuccessor
        ),
        WorthTopologyQueryEditFamilySupportStatus::PartiallyAdmittedByLane
    );
    assert!(support.query_edit_lane_supported("RelocateHalfEdgeBeforeSuccessor"));
    assert!(support.query_edit_lane_supported("RelocateHalfEdgeSpanBeforeSuccessor"));
    assert!(support.query_edit_lane_supported("CreateInnerLoopOnExistingFace"));
    assert!(support.query_edit_lane_supported("RehomeAllOwnedHalfEdgesToNewWire"));
    assert!(support.query_edit_lane_supported("SplitConnectedHalfEdgeSetIntoNewWire"));
    assert!(support.query_edit_lane_supported("SplitSingleFaceFromTwoFaceShellToNewShell"));
    assert!(support.query_edit_lane_supported("RehomeAllOwnedFacesToNewShell"));
    assert!(support.query_edit_lane_supported("RewireLoopEndpoint"));
    assert!(support.query_edit_lane_supported("SpliceRadialAdjacency"));
    assert!(support
        .query_edit_family_supported(crate::edit::WorthTopologyEditFamily::CreateTopologyEntity));
    assert!(support.query_edit_family_supported(
        crate::edit::WorthTopologyEditFamily::DetachBoundaryMembership
    ));
    assert!(support
        .query_edit_family_supported(crate::edit::WorthTopologyEditFamily::DetachRadialAdjacency));
    assert!(support.query_edit_family_supported(
        crate::edit::WorthTopologyEditFamily::DetachShellOrWireMembership
    ));
    assert!(support
        .query_edit_family_supported(crate::edit::WorthTopologyEditFamily::RetireTopologyEntity));
}

#[test]
fn current_head_runtime_reads_seeded_topology_without_query_import() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_minimal_topology(&mut runtime, "worth-query-runtime").expect("seed topology");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.runtime").expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");

    let entity_rows = workspace.read(assembly.entities());
    let relation_rows = workspace.read(assembly.relations());
    let persistent_name_rows = workspace.read(assembly.persistent_names());

    assert!(!entity_rows.is_empty());
    assert!(!relation_rows.is_empty());
    assert!(!persistent_name_rows.is_empty());
    assert!(entity_rows.iter().any(|row| {
        row.payload
            .get("naming")
            .and_then(|value| value.get("persistent_name"))
            .and_then(|value| value.as_str())
            .is_some()
    }));
}

#[test]
fn current_head_runtime_writes_topology_through_real_runtime() {
    let runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.authoritative").expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");

    let receipt = workspace
        .insert("WorthTopologyEntity", |builder| {
            builder
                .aspect("topology.kind", "worth.vertex")
                .aspect(
                    "topology.structure",
                    "worth-query-runtime-write.added_vertex",
                )
                .aspect(
                    "naming.persistent_name",
                    "worth-query-runtime-write.added_vertex",
                )
        })
        .expect("entity create should commit");

    let entity_rows = workspace.read(assembly.entities());
    let materialized_rows = workspace.materialize(assembly.materialized());
    let materialized: crate::facade::MaterializedTopologyView =
        serde_json::from_value(materialized_rows[0].clone()).expect("materialized topology row");
    assert!(entity_rows.iter().any(|row| {
        row.payload
            .get("naming")
            .and_then(|value| value.get("persistent_name"))
            .and_then(|value| value.as_str())
            .is_some_and(|name| name == "worth-query-runtime-write.added_vertex")
    }));
    assert!(receipt
        .affected_derived_view_ids()
        .contains(&assembly.materialized().name().to_string()));
    assert!(!materialized.topology().vertices.is_empty());
}

#[test]
fn current_head_runtime_denies_unsupported_insert_collections() {
    let runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.denials").expect("workspace");

    let error = workspace
        .insert("WorthPersistentName", |builder| {
            builder
                .aspect("topology.kind", "worth.naming.persistent_name")
                .aspect("naming.persistent_name", "worth.current-head.denials.name")
        })
        .expect_err("unsupported insert collections must fail closed");

    assert!(error
        .to_string()
        .contains("does not admit insert collection `WorthPersistentName`"));
}

#[test]
fn snapshot_read_only_runtime_support_reports_historical_read_only_posture() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "worth-query-runtime-snapshot").expect("seed topology");
    let read_view = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect("seeded snapshot");
    let adapters = WorthTopologyRuntimeAdapters::snapshot_read_only(read_view, seeded.snapshot);
    let support = adapters.support();

    assert!(!support.current_head_live_reads_supported());
    assert!(!support.current_head_materialization_supported());
    assert!(!support.post_write_materialization_supported());
    assert!(support.historical_basis_supported());
    assert!(!support.authoritative_writes_supported());
    assert!(!support.query_edit_execution_supported());
}

#[test]
fn snapshot_read_only_runtime_reads_seeded_topology_and_denies_writes() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "worth-query-runtime-snapshot").expect("seed topology");
    let read_view = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect("seeded snapshot");
    let adapters = WorthTopologyRuntimeAdapters::snapshot_read_only(read_view, seeded.snapshot);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.snapshot-read-only.runtime").expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");

    assert!(!workspace.read(assembly.entities()).is_empty());

    let error = workspace
        .insert("WorthTopologyEntity", |builder| {
            builder
                .aspect("topology.kind", "worth.vertex")
                .aspect(
                    "topology.structure",
                    "worth.snapshot-read-only.runtime.vertex",
                )
                .aspect(
                    "naming.persistent_name",
                    "worth.snapshot-read-only.runtime.vertex",
                )
        })
        .expect_err("snapshot runtime must deny authoritative writes");

    assert!(error
        .to_string()
        .contains("snapshot certification runtime is read-only"));
}

#[test]
fn current_head_runtime_admits_bridge_backed_entity_verification_families() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "worth-query-runtime-verify").expect("seed topology");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.verify-existing").expect("workspace");
    let support = workspace.public_authoritative_mutation_evidence_support();
    let binding = workspace
        .bind_existing_entity(
            forge_query::facade::ForgeQueryExistingEntityTarget::new(
                format!("{:?}", seeded.vertex),
                entity_identity(seeded.vertex),
            )
            .expect("existing entity target should build")
            .in_target_collection("WorthTopologyEntity")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    for operation_family in [
        "verify_existing",
        "probe_existing",
        "delete_existing_verified",
    ] {
        let row = support
            .bridge_backed_verification_support_rows()
            .iter()
            .find(|row| {
                row.operation_family() == operation_family
                    && row.target_binding_family() == "direct_entity_identity"
            })
            .expect("entity verification support row should exist");
        assert_eq!(
            row.current_posture_status(),
            ForgeQueryBridgeBackedVerificationSupportStatus::Admitted
        );
        assert!(row.primary_bridge_backed_runtime_supported());
    }
    let update_row = support
        .bridge_backed_verification_support_rows()
        .iter()
        .find(|row| {
            row.operation_family() == "update_existing_verified"
                && row.target_binding_family() == "direct_entity_identity"
        })
        .expect("entity verified update support row should exist");
    assert_eq!(
        update_row.current_posture_status(),
        ForgeQueryBridgeBackedVerificationSupportStatus::Denied
    );

    workspace
        .verify_existing(binding.clone(), |entity| {
            entity.aspect("topology.kind", "worth.vertex")
        })
        .expect("entity verify should execute");
    let probe = workspace
        .probe_existing(binding.clone(), ["topology.kind", "naming.persistent_name"])
        .expect("entity probe should execute");
    assert_eq!(
        probe
            .field("topology.kind")
            .expect("topology.kind should be present")
            .value_json(),
        "\"worth.vertex\""
    );
    workspace
        .delete_existing_verified(
            binding,
            |entity| entity.aspect("topology.kind", "worth.vertex"),
            |delete| delete.touch("topology.kind"),
        )
        .expect("entity verified delete should execute");
}

#[test]
fn current_head_runtime_admits_bridge_backed_relation_verification_families() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "worth-query-runtime-probe").expect("seed topology");
    let read_view = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect("seeded snapshot");
    let relation_id = read_view
        .relations()
        .iter()
        .find(|record| {
            worth_schema::facade::WorthRelationKind::from_kind_id(record.kind.kind_id)
                == Some(worth_schema::facade::WorthRelationKind::Topology(
                    worth_schema::facade::WorthTopologyRelationKind::LoopOwnsHalfEdge,
                ))
        })
        .map(|record| record.relation_id)
        .expect("seeded topology should contain loop->half-edge relation");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.probe-existing").expect("workspace");
    let support = workspace.public_authoritative_mutation_evidence_support();
    let binding = workspace
        .bind_existing_relation(
            forge_query::facade::ForgeQueryExistingRelationTarget::new(
                format!("{relation_id:?}"),
                relation_identity(relation_id),
            )
            .expect("existing relation target should build")
            .in_target_collection("WorthTopologyRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("binding should build");

    let row = support
        .bridge_backed_verification_support_rows()
        .iter()
        .find(|row| {
            row.operation_family() == "probe_existing"
                && row.target_binding_family() == "direct_relation_identity"
        })
        .expect("relation probe support row should exist");
    assert_eq!(
        row.current_posture_status(),
        ForgeQueryBridgeBackedVerificationSupportStatus::Admitted
    );
    let update_row = support
        .bridge_backed_verification_support_rows()
        .iter()
        .find(|row| {
            row.operation_family() == "update_existing_verified"
                && row.target_binding_family() == "direct_relation_identity"
        })
        .expect("relation verified update support row should exist");
    assert_eq!(
        update_row.current_posture_status(),
        ForgeQueryBridgeBackedVerificationSupportStatus::Admitted
    );

    let probe = workspace
        .probe_existing(
            binding.clone(),
            [
                "topology.kind",
                "topology.source_identity",
                "topology.target_identity",
            ],
        )
        .expect("relation probe should execute");
    assert_eq!(
        probe
            .field("topology.kind")
            .expect("topology.kind should be present")
            .value_json(),
        "\"worth.loop_owns_half_edge\""
    );
    workspace
        .verify_existing(binding, |relation| {
            relation.aspect("topology.kind", "worth.loop_owns_half_edge")
        })
        .expect("relation verify should execute");
    let binding = workspace
        .bind_existing_relation(
            forge_query::facade::ForgeQueryExistingRelationTarget::new(
                format!("{relation_id:?}"),
                relation_identity(relation_id),
            )
            .expect("existing relation target should build")
            .in_target_collection("WorthTopologyRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("binding should build");
    workspace
        .delete_existing_verified(
            binding,
            |relation| relation.aspect("topology.kind", "worth.loop_owns_half_edge"),
            |delete| delete.touch("topology.kind"),
        )
        .expect("relation verified delete should execute");
}

fn entity_identity(entity: forge_relational::facade::identity::EntityId) -> String {
    format!(
        "entity:{}:{}:{}",
        entity.partition_id.0, entity.local_slot.0, entity.generation.0
    )
}

fn relation_identity(relation: forge_relational::facade::identity::RelationId) -> String {
    format!(
        "relation:{}:{}:{}",
        relation.partition_id.0, relation.local_slot.0, relation.generation.0
    )
}
