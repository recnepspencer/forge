use std::collections::{BTreeMap, BTreeSet, VecDeque};

use forge_relational::facade::identity::{EntityId, KindId};
use forge_relational::facade::runtime::CustomInvariantScopePlanner;
use forge_relational::facade::transactions::{CreatedEntityRef, EntityReference};
use worth_schema::facade::{
    WorthEntityKind, WorthNamingEntityKind, WorthNamingRelationKind, WorthRelationKind,
    WorthTopologyEntityKind, WorthTopologyRelationKind,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RuntimeEntityRef {
    Existing(EntityId),
    Planned(CreatedEntityRef),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeRelationRecord {
    pub kind_id: KindId,
    pub source: RuntimeEntityRef,
    pub target: RuntimeEntityRef,
}

#[derive(Debug, Clone, Default)]
pub struct RuntimeTopologyGraph {
    pub topology_entities: BTreeMap<RuntimeEntityRef, KindId>,
    pub outgoing_by_entity: BTreeMap<RuntimeEntityRef, Vec<RuntimeRelationRecord>>,
    pub incoming_by_entity: BTreeMap<RuntimeEntityRef, Vec<RuntimeRelationRecord>>,
}

impl RuntimeTopologyGraph {
    pub fn from_planner(planner: &CustomInvariantScopePlanner<'_>) -> Self {
        let touched = planner.touched();
        let visible_entity_ids = touched.visible_entity_ids();
        let visible_relation_ids = touched.visible_relation_ids();
        let planned_entity_creates = touched.planned_entity_creates();
        let planned_relation_creates = touched.planned_relation_creates();
        let relations = planner.relations();
        let topology_kind_ids: BTreeSet<KindId> = WorthTopologyEntityKind::WRAPPED_ALL
            .into_iter()
            .map(WorthEntityKind::kind_id)
            .collect();
        let mut topology_entities = visible_entity_ids
            .iter()
            .filter_map(|entity_id| {
                let kind_id = relations.entity_kind(*entity_id)?;
                topology_kind_ids
                    .contains(&kind_id)
                    .then_some((RuntimeEntityRef::Existing(*entity_id), kind_id))
            })
            .collect::<BTreeMap<_, _>>();
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

        for relation_id in visible_relation_ids {
            if let Some(record) = relations.relation(*relation_id) {
                let runtime_record = RuntimeRelationRecord {
                    kind_id: record.kind_id,
                    source: RuntimeEntityRef::Existing(record.source),
                    target: RuntimeEntityRef::Existing(record.target),
                };
                outgoing_by_entity
                    .entry(runtime_record.source.clone())
                    .or_default()
                    .push(runtime_record.clone());
                incoming_by_entity
                    .entry(runtime_record.target.clone())
                    .or_default()
                    .push(runtime_record);
            }
        }

        for relation in planned_relation_creates {
            let runtime_record = RuntimeRelationRecord {
                kind_id: relation.kind_id,
                source: runtime_entity_reference(&relation.source),
                target: runtime_entity_reference(&relation.target),
            };
            outgoing_by_entity
                .entry(runtime_record.source.clone())
                .or_default()
                .push(runtime_record.clone());
            incoming_by_entity
                .entry(runtime_record.target.clone())
                .or_default()
                .push(runtime_record);
        }

        Self {
            topology_entities,
            outgoing_by_entity,
            incoming_by_entity,
        }
    }

    pub fn outgoing_kind(
        &self,
        entity_id: &RuntimeEntityRef,
        kind: WorthRelationKind,
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
        kind: WorthRelationKind,
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
}

fn runtime_entity_reference(reference: &EntityReference) -> RuntimeEntityRef {
    match reference {
        EntityReference::Existing(entity_id) => RuntimeEntityRef::Existing(*entity_id),
        EntityReference::Created(created) => RuntimeEntityRef::Planned(created.clone()),
    }
}

pub fn kind_name(kind_id: KindId) -> &'static str {
    if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::Model).kind_id() {
        "Topology::Model"
    } else if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::Body).kind_id() {
        "Topology::Body"
    } else if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::Lump).kind_id() {
        "Topology::Lump"
    } else if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::Region).kind_id() {
        "Topology::Region"
    } else if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::Shell).kind_id() {
        "Topology::Shell"
    } else if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::Face).kind_id() {
        "Topology::Face"
    } else if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::Loop).kind_id() {
        "Topology::Loop"
    } else if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::Wire).kind_id() {
        "Topology::Wire"
    } else if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge).kind_id() {
        "Topology::HalfEdge"
    } else if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::Edge).kind_id() {
        "Topology::Edge"
    } else if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex).kind_id() {
        "Topology::Vertex"
    } else if kind_id == WorthEntityKind::Naming(WorthNamingEntityKind::PersistentName).kind_id() {
        "Naming::PersistentName"
    } else {
        "UnknownKind"
    }
}

pub fn connected_components(
    vertices: &BTreeSet<RuntimeEntityRef>,
    adjacency: &BTreeMap<RuntimeEntityRef, BTreeSet<RuntimeEntityRef>>,
) -> usize {
    let mut remaining = vertices.clone();
    let mut components = 0usize;

    while let Some(seed) = remaining.iter().next().cloned() {
        components += 1;
        remaining.remove(&seed);
        let mut frontier = VecDeque::from([seed]);
        while let Some(current) = frontier.pop_front() {
            for next in adjacency.get(&current).into_iter().flatten().cloned() {
                if remaining.remove(&next) {
                    frontier.push_back(next);
                }
            }
        }
    }

    components
}

pub fn naming_relation_kind() -> WorthRelationKind {
    WorthRelationKind::Naming(WorthNamingRelationKind::PersistentNameTargetsEntity)
}

pub fn owner_relation_for_kind(kind_id: KindId) -> Option<WorthRelationKind> {
    if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::Body).kind_id() {
        Some(WorthRelationKind::Topology(WorthTopologyRelationKind::ModelOwnsBody))
    } else if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::Lump).kind_id() {
        Some(WorthRelationKind::Topology(WorthTopologyRelationKind::BodyOwnsLump))
    } else if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::Region).kind_id() {
        Some(WorthRelationKind::Topology(WorthTopologyRelationKind::LumpOwnsRegion))
    } else if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::Shell).kind_id() {
        Some(WorthRelationKind::Topology(WorthTopologyRelationKind::RegionOwnsShell))
    } else if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::Face).kind_id() {
        Some(WorthRelationKind::Topology(WorthTopologyRelationKind::ShellOwnsFace))
    } else if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::Loop).kind_id() {
        None
    } else if kind_id == WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge).kind_id() {
        None
    } else {
        None
    }
}
