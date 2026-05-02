use crate::facade::{
    admit_worth_query_mutation_batch, worth_query_mutation_support_contract,
    RawWorthTopologyIntent, WorthEntityKind, WorthMutationOrigin, WorthNamingEntityKind,
    WorthNamingRelationKind, WorthQueryMutationAdmissionBlocker, WorthRelationKind,
    WorthTopologyEntityKind, WorthTopologyMutation, WorthTopologyRelationKind,
};

#[test]
fn query_mutation_admission_surfaces_naming_writeback_blockers_literally() {
    let admission = admit_worth_query_mutation_batch(&RawWorthTopologyIntent::new(
        vec![
            WorthTopologyMutation::CreateEntity {
                create_key: crate::facade::WorthCreateKey::new("query-gap.name"),
                kind: WorthEntityKind::Naming(WorthNamingEntityKind::PersistentName),
            },
            WorthTopologyMutation::CreateRelation {
                create_key: crate::facade::WorthCreateKey::new("query-gap.name.targets"),
                kind: WorthRelationKind::Naming(
                    WorthNamingRelationKind::PersistentNameTargetsEntity,
                ),
                source: crate::facade::created_ref("query-gap.name"),
                target: crate::facade::created_ref("query-gap.target"),
            },
        ],
        WorthMutationOrigin::LocalEdit,
    ));

    let blockers = admission.blockers();
    assert!(blockers.iter().any(|row| {
        row.blocker == WorthQueryMutationAdmissionBlocker::ProjectedNamingWritebackRequired
    }));
    assert!(!blockers.iter().any(|row| {
        row.blocker == WorthQueryMutationAdmissionBlocker::SymbolicCreateReferenceRequired
    }));
}

#[test]
fn query_mutation_admission_surfaces_existing_identity_and_kind_gaps_for_removals() {
    let admission = admit_worth_query_mutation_batch(&RawWorthTopologyIntent::new(
        vec![
            WorthTopologyMutation::RemoveEntity {
                entity_id: forge_relational::facade::identity::EntityId::new(
                    forge_relational::facade::identity::PartitionId::main(),
                    7,
                    1,
                ),
            },
            WorthTopologyMutation::RemoveRelation {
                relation_id: forge_relational::facade::identity::RelationId::new(
                    forge_relational::facade::identity::PartitionId::main(),
                    8,
                    1,
                ),
            },
        ],
        WorthMutationOrigin::LocalEdit,
    ));

    assert!(
        admission.is_admitted(),
        "topology removals should now lower through imported query binding evidence"
    );
}

#[test]
fn query_mutation_admission_marks_topology_existing_truth_verification_as_ready() {
    let admission = admit_worth_query_mutation_batch(&RawWorthTopologyIntent::new(
        vec![
            WorthTopologyMutation::UpsertEntity {
                entity_id: forge_relational::facade::identity::EntityId::new(
                    forge_relational::facade::identity::PartitionId::main(),
                    10,
                    1,
                ),
                kind: WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex),
            },
            WorthTopologyMutation::UpsertRelation {
                relation_id: forge_relational::facade::identity::RelationId::new(
                    forge_relational::facade::identity::PartitionId::main(),
                    11,
                    1,
                ),
                kind: WorthRelationKind::Topology(
                    WorthTopologyRelationKind::HalfEdgeStartsAtVertex,
                ),
                source: forge_relational::facade::identity::EntityId::new(
                    forge_relational::facade::identity::PartitionId::main(),
                    12,
                    1,
                ),
                target: forge_relational::facade::identity::EntityId::new(
                    forge_relational::facade::identity::PartitionId::main(),
                    13,
                    1,
                ),
            },
        ],
        WorthMutationOrigin::LocalEdit,
    ));

    assert!(
        admission.is_admitted(),
        "topology existing-truth checks should now lower through direct existing bindings"
    );
}

#[test]
fn query_mutation_support_contract_distinguishes_substrate_from_workflow_widening() {
    let contract = worth_query_mutation_support_contract()
        .expect("worth query mutation contract should derive");

    assert!(contract
        .admitted_query_substrate_families
        .iter()
        .any(|family| family == "verify_existing_topology_relation_shape"));
    assert!(contract
        .admitted_query_substrate_families
        .iter()
        .any(|family| family == "update_existing_topology_relation_shape_identity_preserving"));
    assert!(contract
        .blocked_until_invariant_complete_workflow
        .iter()
        .any(|family| {
            family
                == "topology_relation_create_workflows_beyond_face_inner_loop_require_invariant_complete_subgraphs"
        }));
    assert!(contract
        .blocked_until_invariant_complete_workflow
        .iter()
        .any(|family| {
            family
                == "topology_shell_or_wire_membership_workflows_beyond_admitted_full_wire_rehome_connected_wire_split_single_face_two_face_shell_split_and_full_shell_face_set_rehome_require_invariant_complete_owner_rehome_or_shell_subgraphs"
        }));
    assert!(contract
        .blocked_until_invariant_complete_workflow
        .iter()
        .any(|family| {
            family
                == "topology_relation_loop_successor_workflows_beyond_admitted_half_edge_relocation_lanes_require_invariant_complete_topology_update_workflows"
        }));
}
