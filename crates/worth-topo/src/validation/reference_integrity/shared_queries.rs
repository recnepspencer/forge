use std::collections::{BTreeMap, BTreeSet, VecDeque};

use forge_relational::facade::identity::KindId;
use schema::facade::platform::entities::{EntityKind, NamingEntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{NamingRelationKind, RelationKind, TopologyRelationKind};

use super::shared::RuntimeEntityRef;

pub fn kind_name(kind_id: KindId) -> &'static str {
    if kind_id == EntityKind::Topology(TopologyEntityKind::Model).kind_id() {
        "Topology::Model"
    } else if kind_id == EntityKind::Topology(TopologyEntityKind::Body).kind_id() {
        "Topology::Body"
    } else if kind_id == EntityKind::Topology(TopologyEntityKind::Lump).kind_id() {
        "Topology::Lump"
    } else if kind_id == EntityKind::Topology(TopologyEntityKind::Region).kind_id() {
        "Topology::Region"
    } else if kind_id == EntityKind::Topology(TopologyEntityKind::Shell).kind_id() {
        "Topology::Shell"
    } else if kind_id == EntityKind::Topology(TopologyEntityKind::Face).kind_id() {
        "Topology::Face"
    } else if kind_id == EntityKind::Topology(TopologyEntityKind::Loop).kind_id() {
        "Topology::Loop"
    } else if kind_id == EntityKind::Topology(TopologyEntityKind::Wire).kind_id() {
        "Topology::Wire"
    } else if kind_id == EntityKind::Topology(TopologyEntityKind::HalfEdge).kind_id() {
        "Topology::HalfEdge"
    } else if kind_id == EntityKind::Topology(TopologyEntityKind::Edge).kind_id() {
        "Topology::Edge"
    } else if kind_id == EntityKind::Topology(TopologyEntityKind::Vertex).kind_id() {
        "Topology::Vertex"
    } else if kind_id == EntityKind::Naming(NamingEntityKind::PersistentName).kind_id() {
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

pub fn naming_relation_kind() -> RelationKind {
    RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity)
}

pub fn owner_relation_for_kind(kind_id: KindId) -> Option<RelationKind> {
    if kind_id == EntityKind::Topology(TopologyEntityKind::Body).kind_id() {
        Some(RelationKind::Topology(TopologyRelationKind::ModelOwnsBody))
    } else if kind_id == EntityKind::Topology(TopologyEntityKind::Lump).kind_id() {
        Some(RelationKind::Topology(TopologyRelationKind::BodyOwnsLump))
    } else if kind_id == EntityKind::Topology(TopologyEntityKind::Region).kind_id() {
        Some(RelationKind::Topology(TopologyRelationKind::LumpOwnsRegion))
    } else if kind_id == EntityKind::Topology(TopologyEntityKind::Shell).kind_id() {
        Some(RelationKind::Topology(
            TopologyRelationKind::RegionOwnsShell,
        ))
    } else if kind_id == EntityKind::Topology(TopologyEntityKind::Face).kind_id() {
        Some(RelationKind::Topology(TopologyRelationKind::ShellOwnsFace))
    } else if kind_id == EntityKind::Topology(TopologyEntityKind::Loop).kind_id() {
        None
    } else if kind_id == EntityKind::Topology(TopologyEntityKind::HalfEdge).kind_id() {
        None
    } else {
        None
    }
}
