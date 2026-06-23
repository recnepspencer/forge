use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::test_support::schema_topology_authoring_boundary::seed_minimal_topology_through_schema_execution;
use crate::topology_operators::authority_identity::{
    existing_entity_authority, existing_relation_authority,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;
use forge_query::facade::{
    ForgeQueryBridgeBackedVerificationSupportStatus, ForgeQueryEntityIdentity,
    ForgeQueryExistingEntityTarget, ForgeQueryExistingRelationTarget,
    ForgeQueryExistingTruthProbeMode, ForgeQueryExistingTruthProbeRequest,
    ForgeQueryExistingTruthTargetBinding,
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
    let binding = ForgeQueryExistingTruthTargetBinding::from_entity_target(
        ForgeQueryExistingEntityTarget::new(
            existing_entity_authority(seeded.vertex).expect("entity authority"),
            entity_identity(seeded.vertex),
        )
        .expect("existing entity target should build")
        .in_target_collection("TopologyEntity")
        .expect("existing entity target collection should build"),
    )
    .expect("binding should build");

    let probe = workspace
        .probe_existing_intent(
            ForgeQueryExistingTruthProbeRequest::new(
                binding.clone(),
                ["topology.kind", "naming.persistent_name"],
            )
            .expect("entity probe request should build"),
        )
        .execute()
        .expect("entity probe should execute")
        .probe()
        .clone();
    assert_eq!(
        probe.mode(),
        ForgeQueryExistingTruthProbeMode::BackendVerifiedProbe
    );
    assert_eq!(
        probe
            .field("topology.kind")
            .expect("topology.kind should be present")
            .external_value_json(),
        "\".vertex\""
    );

    workspace
        .compose_graph(|graph| {
            graph.delete_existing_verified(
                binding,
                |entity| entity.aspect("topology.kind", ".vertex"),
                |delete| delete.touch("topology.kind"),
            )?;
            Ok(())
        })
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
    let binding = ForgeQueryExistingTruthTargetBinding::from_relation_target(
        ForgeQueryExistingRelationTarget::new(
            existing_relation_authority(relation_id).expect("relation authority"),
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
        .probe_existing_intent(
            ForgeQueryExistingTruthProbeRequest::new(
                binding.clone(),
                [
                    "topology.kind",
                    "topology.source_identity",
                    "topology.target_identity",
                ],
            )
            .expect("relation probe request should build"),
        )
        .execute()
        .expect("relation probe should execute")
        .probe()
        .clone();
    assert_eq!(
        probe.mode(),
        ForgeQueryExistingTruthProbeMode::BackendVerifiedProbe
    );
    assert_eq!(
        probe
            .field("topology.kind")
            .expect("topology.kind should be present")
            .external_value_json(),
        "\".loop_owns_half_edge\""
    );
    let source_identity: String = serde_json::from_str(
        probe
            .field("topology.source_identity")
            .expect("source identity should be present")
            .external_value_json(),
    )
    .expect("source identity probe value should decode");
    let target_identity: String = serde_json::from_str(
        probe
            .field("topology.target_identity")
            .expect("target identity should be present")
            .external_value_json(),
    )
    .expect("target identity probe value should decode");
    workspace
        .compose_graph(|graph| {
            graph.update_existing_verified(
                binding.clone(),
                |relation| relation.aspect("topology.kind", ".loop_owns_half_edge"),
                |update| {
                    update
                        .aspect("topology.kind", ".loop_owns_half_edge")
                        .aspect("topology.source_identity", &source_identity)
                        .aspect("topology.target_identity", &target_identity)
                },
            )?;
            Ok(())
        })
        .expect("relation verified graph update should execute");
    workspace
        .compose_graph(|graph| {
            graph.delete_existing_verified(
                binding,
                |relation| relation.aspect("topology.kind", ".loop_owns_half_edge"),
                |delete| delete.touch("topology.kind"),
            )?;
            Ok(())
        })
        .expect("relation verified graph delete should execute");
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
