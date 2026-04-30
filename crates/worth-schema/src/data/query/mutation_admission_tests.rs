use crate::facade::{
    admit_worth_query_mutation_batch, RawWorthTopologyIntent, WorthEntityKind, WorthMutationOrigin,
    WorthNamingEntityKind, WorthNamingRelationKind, WorthQueryMutationAdmissionBlocker,
    WorthRelationKind, WorthTopologyEntityKind, WorthTopologyMutation, WorthTopologyRelationKind,
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
fn query_mutation_admission_marks_topology_upserts_as_ready() {
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
        "topology upserts should now lower through imported existing-truth bindings"
    );
    assert!(
        !admission.blockers().iter().any(|row| {
            row.blocker == WorthQueryMutationAdmissionBlocker::ExistingIdentityBindingRequired
        }),
        "topology upserts should no longer claim the old explicit-binding blocker"
    );
}
