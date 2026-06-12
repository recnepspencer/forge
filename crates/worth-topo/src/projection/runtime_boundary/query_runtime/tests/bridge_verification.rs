use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::test_support::schema_topology_authoring_boundary::seed_minimal_topology_through_schema_execution;
use crate::validation::reference_integrity::build_milestone_one_runtime;
use forge_query::facade::{
    ForgeQueryBridgeBackedVerificationSupportStatus, ForgeQueryEntityIdentity,
};
use forge_runtime_bridge::facade::RelationalBridgeRecordIdentityParts;

#[test]
fn current_head_runtime_admits_bridge_backed_entity_verification_families() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded =
        seed_minimal_topology_through_schema_execution(&mut runtime, "query-runtime-verify")
            .expect("seed topology");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.verify-existing").expect("workspace");
    let support = workspace.public_authoritative_mutation_evidence_support();
    let binding = workspace
        .bind_existing_entity(
            forge_query::facade::ForgeQueryExistingEntityTarget::new(
                format!("{:?}", seeded.vertex),
                entity_identity(seeded.vertex),
            )
            .expect("existing entity target should build")
            .in_target_collection("TopologyEntity")
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
            entity.aspect("topology.kind", ".vertex")
        })
        .expect("entity verify should execute");
    let probe = workspace
        .probe_existing(binding.clone(), ["topology.kind", "naming.persistent_name"])
        .expect("entity probe should execute");
    assert_eq!(
        probe
            .field("topology.kind")
            .expect("topology.kind should be present")
            .external_value_json(),
        "\".vertex\""
    );
    workspace
        .delete_existing_verified(
            binding,
            |entity| entity.aspect("topology.kind", ".vertex"),
            |delete| delete.touch("topology.kind"),
        )
        .expect("entity verified delete should execute");
}

#[test]
fn current_head_runtime_admits_bridge_backed_relation_verification_families() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded =
        seed_minimal_topology_through_schema_execution(&mut runtime, "query-runtime-probe")
            .expect("seed topology");
    let read_view = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect("seeded snapshot");
    let relation_id = read_view
        .relations()
        .iter()
        .find(|record| {
            schema::facade::platform::relations::RelationKind::from_kind_id(record.kind.kind_id)
                == Some(schema::facade::platform::relations::RelationKind::Topology(
                    schema::facade::platform::relations::TopologyRelationKind::LoopOwnsHalfEdge,
                ))
        })
        .map(|record| record.relation_id)
        .expect("seeded topology should contain loop->half-edge relation");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.probe-existing").expect("workspace");
    let support = workspace.public_authoritative_mutation_evidence_support();
    let binding = workspace
        .bind_existing_relation(
            forge_query::facade::ForgeQueryExistingRelationTarget::new(
                format!("{relation_id:?}"),
                relation_identity(relation_id),
            )
            .expect("existing relation target should build")
            .in_target_collection("TopologyRelation")
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
            .external_value_json(),
        "\".loop_owns_half_edge\""
    );
    workspace
        .verify_existing(binding, |relation| {
            relation.aspect("topology.kind", ".loop_owns_half_edge")
        })
        .expect("relation verify should execute");
    let binding = workspace
        .bind_existing_relation(
            forge_query::facade::ForgeQueryExistingRelationTarget::new(
                format!("{relation_id:?}"),
                relation_identity(relation_id),
            )
            .expect("existing relation target should build")
            .in_target_collection("TopologyRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("binding should build");
    workspace
        .delete_existing_verified(
            binding,
            |relation| relation.aspect("topology.kind", ".loop_owns_half_edge"),
            |delete| delete.touch("topology.kind"),
        )
        .expect("relation verified delete should execute");
}

fn entity_identity(
    entity: forge_relational::facade::identity::EntityId,
) -> ForgeQueryEntityIdentity {
    ForgeQueryEntityIdentity::from_relational_record(RelationalBridgeRecordIdentityParts::entity(
        entity.partition_id.0,
        entity.local_slot.0,
        entity.generation.0,
    ))
}

fn relation_identity(
    relation: forge_relational::facade::identity::RelationId,
) -> ForgeQueryEntityIdentity {
    ForgeQueryEntityIdentity::from_relational_record(RelationalBridgeRecordIdentityParts::relation(
        relation.partition_id.0,
        relation.local_slot.0,
        relation.generation.0,
    ))
}
