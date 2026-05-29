use std::collections::{BTreeMap, BTreeSet, VecDeque};

use forge_relational::facade::identity::{EntityId, KindId, RelationId};
use forge_relational::facade::runtime::CustomInvariantScopePlanner;
use forge_relational::facade::transactions::{CreatedEntityRef, EntityReference};
use schema::facade::{EntityKind, RelationKind, TopologyEntityKind, TopologyRelationKind};

use super::shared_queries::naming_relation_kind;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RuntimeEntityRef {
    Existing(EntityId),
    Planned(CreatedEntityRef),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeRelationRecord {
    pub relation_id: Option<RelationId>,
    pub kind_id: KindId,
    pub source: RuntimeEntityRef,
    pub target: RuntimeEntityRef,
}

#[derive(Debug, Clone, Default)]
pub struct RuntimeTopologyGraph {
    pub topology_entities: BTreeMap<RuntimeEntityRef, KindId>,
    pub outgoing_by_entity: BTreeMap<RuntimeEntityRef, Vec<RuntimeRelationRecord>>,
    pub incoming_by_entity: BTreeMap<RuntimeEntityRef, Vec<RuntimeRelationRecord>>,
    pub planned_relation_endpoint_updates: BTreeMap<RelationId, RuntimeRelationRecord>,
}

impl RuntimeTopologyGraph {
    pub fn from_planner(planner: &CustomInvariantScopePlanner<'_>) -> Self {
        let touched = planner.touched();
        let visible_entity_ids = touched.visible_entity_ids();
        let visible_relation_ids = touched.visible_relation_ids();
        let planned_entity_deletes = touched
            .planned_entity_deletes()
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        let planned_entity_creates = touched.planned_entity_creates();
        let planned_relation_creates = touched.planned_relation_creates();
        let planned_relation_deletes = touched
            .planned_relation_deletes()
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        let planned_relation_endpoint_updates = touched.planned_relation_endpoint_updates();
        let relations = planner.relations();
        let topology_kind_ids: BTreeSet<KindId> = TopologyEntityKind::WRAPPED_ALL
            .into_iter()
            .map(EntityKind::kind_id)
            .collect();
        let visible_existing_topology_entity_ids = visible_entity_ids
            .iter()
            .copied()
            .filter(|entity_id| {
                !planned_entity_deletes.contains(entity_id)
                    && relations
                        .entity_kind(*entity_id)
                        .is_some_and(|kind_id| topology_kind_ids.contains(&kind_id))
            })
            .collect::<BTreeSet<_>>();
        let mut topology_entities = visible_entity_ids
            .iter()
            .filter_map(|entity_id| {
                if planned_entity_deletes.contains(entity_id) {
                    return None;
                }
                relations.entity_kind(*entity_id).and_then(|kind_id| {
                    topology_kind_ids
                        .contains(&kind_id)
                        .then_some((RuntimeEntityRef::Existing(*entity_id), kind_id))
                })
            })
            .collect::<BTreeMap<_, _>>();
        let supporting_existing_relation_ids = collect_supporting_existing_relation_ids(
            &relations,
            &topology_kind_ids,
            &visible_existing_topology_entity_ids,
            &planned_relation_deletes,
            &planned_entity_deletes,
        );
        for create in planned_entity_creates {
            if topology_kind_ids.contains(&create.kind_id) {
                topology_entities.insert(
                    RuntimeEntityRef::Planned(CreatedEntityRef {
                        partition_id: create.partition_id,
                        kind_id: create.kind_id,
                        client_key: create.client_key.clone(),
                    }),
                    create.kind_id,
                );
            }
        }

        let mut outgoing_by_entity: BTreeMap<RuntimeEntityRef, Vec<RuntimeRelationRecord>> =
            BTreeMap::new();
        let mut incoming_by_entity: BTreeMap<RuntimeEntityRef, Vec<RuntimeRelationRecord>> =
            BTreeMap::new();
        let mut planned_relation_endpoint_update_map = planned_relation_endpoint_updates
            .iter()
            .map(|update| {
                (
                    update.relation_id,
                    RuntimeRelationRecord {
                        relation_id: Some(update.relation_id),
                        kind_id: update.kind_id,
                        source: runtime_entity_reference(&update.source),
                        target: runtime_entity_reference(&update.target),
                    },
                )
            })
            .collect::<BTreeMap<_, _>>();

        let existing_relation_ids = visible_relation_ids
            .iter()
            .copied()
            .chain(supporting_existing_relation_ids)
            .filter(|relation_id| {
                if planned_relation_deletes.contains(relation_id) {
                    return false;
                }
                relations.relation(*relation_id).is_none_or(|record| {
                    let updated = planned_relation_endpoint_update_map.get(relation_id);
                    let source = updated.map_or_else(
                        || RuntimeEntityRef::Existing(record.source),
                        |updated_record| updated_record.source.clone(),
                    );
                    let target = updated.map_or_else(
                        || RuntimeEntityRef::Existing(record.target),
                        |updated_record| updated_record.target.clone(),
                    );
                    runtime_entity_ref_survives(&source, &planned_entity_deletes)
                        && runtime_entity_ref_survives(&target, &planned_entity_deletes)
                })
            })
            .collect::<BTreeSet<_>>();
        for relation_id in existing_relation_ids {
            if let Some(record) = relations.relation(relation_id) {
                let updated = planned_relation_endpoint_updates
                    .iter()
                    .find(|update| update.relation_id == relation_id);
                push_runtime_relation(
                    &mut outgoing_by_entity,
                    &mut incoming_by_entity,
                    RuntimeRelationRecord {
                        relation_id: Some(relation_id),
                        kind_id: updated.map_or(record.kind_id, |update| update.kind_id),
                        source: updated.map_or_else(
                            || RuntimeEntityRef::Existing(record.source),
                            |update| runtime_entity_reference(&update.source),
                        ),
                        target: updated.map_or_else(
                            || RuntimeEntityRef::Existing(record.target),
                            |update| runtime_entity_reference(&update.target),
                        ),
                    },
                );
            }
        }

        for relation in planned_relation_creates {
            push_runtime_relation(
                &mut outgoing_by_entity,
                &mut incoming_by_entity,
                RuntimeRelationRecord {
                    relation_id: None,
                    kind_id: relation.kind_id,
                    source: runtime_entity_reference(&relation.source),
                    target: runtime_entity_reference(&relation.target),
                },
            );
        }

        Self {
            topology_entities,
            outgoing_by_entity,
            incoming_by_entity,
            planned_relation_endpoint_updates: std::mem::take(
                &mut planned_relation_endpoint_update_map,
            ),
        }
    }

    pub fn outgoing_kind(
        &self,
        entity_id: &RuntimeEntityRef,
        kind: RelationKind,
    ) -> Vec<RuntimeRelationRecord> {
        let kind_id = kind.kind_id();
        self.outgoing_by_entity
            .get(entity_id)
            .into_iter()
            .flatten()
            .filter(|record| record.kind_id == kind_id)
            .cloned()
            .collect()
    }

    pub fn incoming_kind(
        &self,
        entity_id: &RuntimeEntityRef,
        kind: RelationKind,
    ) -> Vec<RuntimeRelationRecord> {
        let kind_id = kind.kind_id();
        self.incoming_by_entity
            .get(entity_id)
            .into_iter()
            .flatten()
            .filter(|record| record.kind_id == kind_id)
            .cloned()
            .collect()
    }

    pub fn planned_outgoing_kind(
        &self,
        entity_id: &RuntimeEntityRef,
        kind: RelationKind,
    ) -> Vec<RuntimeRelationRecord> {
        let kind_id = kind.kind_id();
        self.planned_relation_endpoint_updates
            .values()
            .filter(|record| record.kind_id == kind_id && &record.source == entity_id)
            .cloned()
            .collect()
    }

    pub fn planned_kind(&self, kind: RelationKind) -> Vec<RuntimeRelationRecord> {
        let kind_id = kind.kind_id();
        self.planned_relation_endpoint_updates
            .values()
            .filter(|record| record.kind_id == kind_id)
            .cloned()
            .collect()
    }
}

fn collect_supporting_existing_relation_ids(
    relations: &forge_relational::facade::runtime::StructuralRelationView<'_>,
    topology_kind_ids: &BTreeSet<KindId>,
    seed_entity_ids: &BTreeSet<EntityId>,
    planned_relation_deletes: &BTreeSet<RelationId>,
    planned_entity_deletes: &BTreeSet<EntityId>,
) -> BTreeSet<RelationId> {
    let supported_relation_kind_ids = invariant_support_relation_kind_ids();
    let mut visited_topology_entities = seed_entity_ids.clone();
    let mut frontier = VecDeque::from_iter(seed_entity_ids.iter().copied());
    let mut relation_ids = BTreeSet::new();

    while let Some(entity_id) = frontier.pop_front() {
        for relation_id in relations.all_relations_for_entity(entity_id) {
            let Some(record) = relations.relation(relation_id) else {
                continue;
            };
            if planned_relation_deletes.contains(&relation_id) {
                continue;
            }
            if planned_entity_deletes.contains(&record.source)
                || planned_entity_deletes.contains(&record.target)
            {
                continue;
            }
            if !supported_relation_kind_ids.contains(&record.kind_id) {
                continue;
            }
            relation_ids.insert(relation_id);
            for adjacent_entity_id in [record.source, record.target] {
                let Some(kind_id) = relations.entity_kind(adjacent_entity_id) else {
                    continue;
                };
                if topology_kind_ids.contains(&kind_id)
                    && visited_topology_entities.insert(adjacent_entity_id)
                {
                    frontier.push_back(adjacent_entity_id);
                }
            }
        }
    }

    relation_ids
}

fn invariant_support_relation_kind_ids() -> BTreeSet<KindId> {
    TopologyRelationKind::WRAPPED_ALL
        .into_iter()
        .chain(std::iter::once(naming_relation_kind()))
        .map(RelationKind::kind_id)
        .collect()
}

fn push_runtime_relation(
    outgoing_by_entity: &mut BTreeMap<RuntimeEntityRef, Vec<RuntimeRelationRecord>>,
    incoming_by_entity: &mut BTreeMap<RuntimeEntityRef, Vec<RuntimeRelationRecord>>,
    runtime_record: RuntimeRelationRecord,
) {
    outgoing_by_entity
        .entry(runtime_record.source.clone())
        .or_default()
        .push(runtime_record.clone());
    incoming_by_entity
        .entry(runtime_record.target.clone())
        .or_default()
        .push(runtime_record);
}

fn runtime_entity_reference(reference: &EntityReference) -> RuntimeEntityRef {
    match reference {
        EntityReference::Existing(entity_id) => RuntimeEntityRef::Existing(*entity_id),
        EntityReference::Created(created) => RuntimeEntityRef::Planned(created.clone()),
    }
}

fn runtime_entity_ref_survives(
    entity_ref: &RuntimeEntityRef,
    planned_entity_deletes: &BTreeSet<EntityId>,
) -> bool {
    match entity_ref {
        RuntimeEntityRef::Existing(entity_id) => !planned_entity_deletes.contains(entity_id),
        RuntimeEntityRef::Planned(_) => true,
    }
}
