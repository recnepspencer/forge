use std::collections::{BTreeMap, BTreeSet};

pub(super) use crate::validation::reference_integrity::{
    milestone_one_invariant_registrations, milestone_one_runtime_builder,
};
use forge_relational::facade::identity::PartitionId;
pub(super) use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::symbols::ClientKey;
pub(super) use forge_relational::facade::transactions::TransactionCommitError;
use forge_relational::facade::transactions::{
    AspectFieldPatch, CreateIntent, CreatedEntityRef, EntityReference as RelationalEntityReference,
    EntitySpec, MutationIntent, RelationSpec,
};
pub(super) use schema::facade::platform::authority::{
    CreateKey, EntityReference, MutationOrigin, RawTopologyIntent, TopologyMutation,
};
pub(super) use schema::facade::platform::entities::{
    EntityKind, NamingEntityKind, TopologyEntityKind,
};
pub(super) use schema::facade::platform::relations::{
    NamingRelationKind, RelationKind, TopologyRelationKind,
};
use schema::facade::topology_authoring::commit_topology_mutation_set;
pub(super) use schema::facade::topology_authoring::seed_minimal_topology;

use crate::relational_aspect_boundary::topology_entity_create_fields;

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

fn commit_raw_intent(
    runtime: &mut RelationalRuntime,
    intent: RawTopologyIntent,
) -> Result<(), TransactionCommitError> {
    let mut seen_create_keys = BTreeSet::new();
    let mut created_entities = BTreeMap::new();

    for mutation in &intent.mutations {
        match mutation {
            TopologyMutation::CreateEntity { create_key, kind } => {
                if !seen_create_keys.insert(create_key.clone()) {
                    continue;
                }
                created_entities.insert(
                    create_key.clone(),
                    CreatedEntityRef {
                        partition_id: PartitionId::main(),
                        kind_id: kind.kind_id(),
                        client_key: ClientKey::raw(create_key.as_str()),
                    },
                );
            }
            TopologyMutation::CreateRelation { create_key, .. } => {
                seen_create_keys.insert(create_key.clone());
            }
            other => panic!(
                "reference-integrity tests only support create-only raw intents, got {other:?}"
            ),
        }
    }

    let lowered_mutations = intent
        .mutations
        .into_iter()
        .map(|mutation| match mutation {
            TopologyMutation::CreateEntity { create_key, kind } => {
                MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: kind.kind_id(),
                    client_key: ClientKey::raw(create_key.as_str()),
                    fields: entity_create_fields(kind, create_key.as_str()),
                }))
            }
            TopologyMutation::CreateRelation {
                create_key,
                kind,
                source,
                target,
            } => MutationIntent::Create(CreateIntent::Relation(RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: kind.kind_id(),
                client_key: ClientKey::raw(create_key.as_str()),
                source: lower_entity_reference(&source, &created_entities),
                target: lower_entity_reference(&target, &created_entities),
                fields: AspectFieldPatch::default(),
            })),
            other => panic!(
                "reference-integrity tests only support create-only raw intents, got {other:?}"
            ),
        })
        .collect::<Vec<_>>();

    commit_create_only_mutation_set(
        runtime,
        "reference-integrity-create-only-mutation-set",
        lowered_mutations,
    )
}

fn commit_create_only_mutation_set(
    runtime: &mut RelationalRuntime,
    transaction_label: &'static str,
    mutations: impl IntoIterator<Item = MutationIntent>,
) -> Result<(), TransactionCommitError> {
    commit_topology_mutation_set(runtime, transaction_label, mutations)
        .map(|_| ())
        .map_err(|error| match error {
            schema::facade::topology_authoring::TopologyMutationSetCommitError::Commit(error) => {
                error
            }
        })
}

fn lower_entity_reference(
    reference: &EntityReference,
    created_entities: &BTreeMap<CreateKey, CreatedEntityRef>,
) -> RelationalEntityReference {
    match reference {
        EntityReference::Existing(entity_id) => RelationalEntityReference::Existing(*entity_id),
        EntityReference::Created(create_key) => RelationalEntityReference::Created(
            created_entities
                .get(create_key)
                .cloned()
                .unwrap_or_else(|| {
                    panic!("missing created entity reference `{}`", create_key.as_str())
                }),
        ),
    }
}

fn entity_create_fields(kind: EntityKind, create_key: &str) -> AspectFieldPatch {
    match kind {
        EntityKind::Topology(topology_kind) => {
            topology_entity_create_fields(EntityKind::Topology(topology_kind), create_key)
        }
        EntityKind::Naming(NamingEntityKind::PersistentName) => {
            crate::relational_aspect_boundary::persistent_name_create_fields(create_key)
        }
        other => {
            panic!("reference-integrity test helper does not support `{other:?}` entity fields")
        }
    }
}
