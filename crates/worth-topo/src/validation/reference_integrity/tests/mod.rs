pub(super) use forge_relational::facade::transactions::TransactionCommitError;
pub(super) use schema::facade::topology_authoring::{
    seed_minimal_topology, verify_topology_intent,
};
pub(super) use schema::facade::{
    bootstrap_runtime_invariant_plan, CreateKey, EntityKind, EntityReference, MutationOrigin,
    NamingEntityKind, NamingRelationKind, RawTopologyIntent, RelationKind, TopologyAuthorityError,
    TopologyEntityKind, TopologyMutation, TopologyRelationKind,
};

pub(super) use crate::facade::{milestone_one_runtime_builder, milestone_one_runtime_invariants};

mod bootstrap_boundary;
mod disconnected_wire_creation;
mod missing_name_creation;
mod non_distinct_wire_branch;

fn entity(create_key: &str, kind: TopologyEntityKind) -> TopologyMutation {
    TopologyMutation::CreateEntity {
        create_key: CreateKey::new(create_key),
        kind: EntityKind::Topology(kind),
    }
}

fn relation(
    create_key: &str,
    kind: TopologyRelationKind,
    source: &str,
    target: &str,
) -> TopologyMutation {
    TopologyMutation::CreateRelation {
        create_key: CreateKey::new(create_key),
        kind: RelationKind::Topology(kind),
        source: EntityReference::Created(CreateKey::new(source)),
        target: EntityReference::Created(CreateKey::new(target)),
    }
}

fn naming_bundle<'a>(topology_keys: &'a [&'a str]) -> impl Iterator<Item = TopologyMutation> + 'a {
    topology_keys.iter().flat_map(|topology_key| {
        let name_key = format!("{topology_key}.persistent_name");
        [
            TopologyMutation::CreateEntity {
                create_key: CreateKey::new(name_key.clone()),
                kind: EntityKind::Naming(NamingEntityKind::PersistentName),
            },
            TopologyMutation::CreateRelation {
                create_key: CreateKey::new(format!("{name_key}.targets")),
                kind: RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity),
                source: EntityReference::Created(CreateKey::new(name_key)),
                target: EntityReference::Created(CreateKey::new(*topology_key)),
            },
        ]
        .into_iter()
    })
}
