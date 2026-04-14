use std::collections::{BTreeMap, BTreeSet, VecDeque};

use forge_relational::facade::identity::{EntityId, KindId};
use forge_relational::facade::runtime::{
    CustomInvariantExecutionContext, CustomInvariantScopePlanner, StructuralRelationRecord,
    StructuralRelationView,
};
use worth_schema::facade::{
    WorthEntityKind, WorthNamingEntityKind, WorthNamingRelationKind, WorthRelationKind,
    WorthTopologyEntityKind, WorthTopologyRelationKind,
};

#[derive(Debug, Clone, Default)]
pub struct RuntimeTopologyGraph {
    pub topology_entities: BTreeMap<EntityId, KindId>,
    pub naming_entities: BTreeSet<EntityId>,
    pub outgoing_by_entity: BTreeMap<EntityId, Vec<StructuralRelationRecord>>,
    pub incoming_by_entity: BTreeMap<EntityId, Vec<StructuralRelationRecord>>,
}

impl RuntimeTopologyGraph {
    pub fn from_context(context: &CustomInvariantExecutionContext<'_>) -> Self {
        Self::build(
            context.touched().visible_entity_ids(),
            context.touched().visible_relation_ids(),
            context.relations(),
        )
    }

    pub fn from_planner(planner: &CustomInvariantScopePlanner<'_>) -> Self {
        Self::build(
            planner.touched().visible_entity_ids(),
            planner.touched().visible_relation_ids(),
            planner.relations(),
        )
    }

    fn build(
        visible_entity_ids: &[EntityId],
        visible_relation_ids: &[forge_relational::facade::identity::RelationId],
        relations: StructuralRelationView<'_>,
    ) -> Self {
        let topology_kind_ids: BTreeSet<KindId> = WorthTopologyEntityKind::WRAPPED_ALL
            .into_iter()
            .map(WorthEntityKind::kind_id)
            .collect();
        let naming_kind_id = WorthEntityKind::Naming(WorthNamingEntityKind::PersistentName).kind_id();

        let topology_entities = visible_entity_ids
            .iter()
            .filter_map(|entity_id| {
                let kind_id = relations.entity_kind(*entity_id)?;
                topology_kind_ids
                    .contains(&kind_id)
                    .then_some((*entity_id, kind_id))
            })
            .collect::<BTreeMap<_, _>>();
        let naming_entities = visible_entity_ids
            .iter()
            .copied()
            .filter(|entity_id| relations.entity_kind(*entity_id) == Some(naming_kind_id))
            .collect::<BTreeSet<_>>();

        let mut outgoing_by_entity: BTreeMap<EntityId, Vec<StructuralRelationRecord>> = BTreeMap::new();
        let mut incoming_by_entity: BTreeMap<EntityId, Vec<StructuralRelationRecord>> = BTreeMap::new();
        for relation_id in visible_relation_ids {
            if let Some(record) = relations.relation(*relation_id) {
                outgoing_by_entity
                    .entry(record.source)
                    .or_default()
                    .push(record);
                incoming_by_entity
                    .entry(record.target)
                    .or_default()
                    .push(record);
            }
        }

        Self {
            topology_entities,
            naming_entities,
            outgoing_by_entity,
            incoming_by_entity,
        }
    }

    pub fn outgoing_kind(
        &self,
        entity_id: EntityId,
        kind: WorthRelationKind,
    ) -> Vec<StructuralRelationRecord> {
        let kind_id = kind.kind_id();
        self.outgoing_by_entity
            .get(&entity_id)
            .into_iter()
            .flatten()
            .copied()
            .filter(|record| record.kind_id == kind_id)
            .collect()
    }

    pub fn incoming_kind(
        &self,
        entity_id: EntityId,
        kind: WorthRelationKind,
    ) -> Vec<StructuralRelationRecord> {
        let kind_id = kind.kind_id();
        self.incoming_by_entity
            .get(&entity_id)
            .into_iter()
            .flatten()
            .copied()
            .filter(|record| record.kind_id == kind_id)
            .collect()
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

pub fn connected_components(vertices: &BTreeSet<EntityId>, adjacency: &BTreeMap<EntityId, BTreeSet<EntityId>>) -> usize {
    let mut remaining = vertices.clone();
    let mut components = 0usize;

    while let Some(seed) = remaining.iter().next().copied() {
        components += 1;
        remaining.remove(&seed);
        let mut frontier = VecDeque::from([seed]);
        while let Some(current) = frontier.pop_front() {
            for next in adjacency.get(&current).into_iter().flatten().copied() {
                if remaining.remove(&next) {
                    frontier.push_back(next);
                }
            }
        }
    }

    components
}

pub fn relation_source_target<'runtime>(
    relations: StructuralRelationView<'runtime>,
    relation_id: forge_relational::facade::identity::RelationId,
) -> Option<(EntityId, EntityId)> {
    relations.relation(relation_id).map(|record| (record.source, record.target))
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
