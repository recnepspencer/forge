use std::collections::{BTreeMap, BTreeSet, VecDeque};

use forge_relational::facade::identity::EntityId;
use schema::facade::platform::authority::WireInterpretationClass;

use crate::brep::topology_graph::TopologyHalfEdge;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireBranchInterpretation {
    pub class: WireInterpretationClass,
    pub connected_component_count: usize,
    pub terminal_vertex_ids: Vec<EntityId>,
    pub branch_vertex_ids: Vec<EntityId>,
}

pub fn interpret_wire_branching(
    half_edge_ids: BTreeSet<EntityId>,
    half_edge_map: &BTreeMap<EntityId, &TopologyHalfEdge>,
) -> WireBranchInterpretation {
    let components = connected_components(half_edge_ids.clone(), half_edge_map);
    let degree_map = vertex_degree_map(half_edge_ids, half_edge_map);
    let mut terminal_vertex_ids: Vec<_> = degree_map
        .iter()
        .filter_map(|(vertex_id, degree)| (*degree == 1).then_some(*vertex_id))
        .collect();
    let mut branch_vertex_ids: Vec<_> = degree_map
        .iter()
        .filter_map(|(vertex_id, degree)| (*degree >= 3).then_some(*vertex_id))
        .collect();
    terminal_vertex_ids.sort();
    branch_vertex_ids.sort();

    let closed_cycle =
        components == 1 && !degree_map.is_empty() && degree_map.values().all(|degree| *degree == 2);
    let class = if components > 1 {
        WireInterpretationClass::Disconnected
    } else if closed_cycle {
        WireInterpretationClass::ClosedCycle
    } else if !branch_vertex_ids.is_empty() {
        WireInterpretationClass::ConnectedBranch
    } else {
        WireInterpretationClass::OpenChain
    };

    WireBranchInterpretation {
        class,
        connected_component_count: components,
        terminal_vertex_ids,
        branch_vertex_ids,
    }
}

fn connected_components(
    half_edge_ids: BTreeSet<EntityId>,
    half_edge_map: &BTreeMap<EntityId, &TopologyHalfEdge>,
) -> usize {
    if half_edge_ids.is_empty() {
        return 0;
    }

    let mut unvisited = half_edge_ids;
    let mut components = 0;

    while let Some(seed_id) = unvisited.iter().next().copied() {
        components += 1;
        let mut queue = VecDeque::from([seed_id]);
        unvisited.remove(&seed_id);

        while let Some(current_id) = queue.pop_front() {
            let Some(current) = half_edge_map.get(&current_id).copied() else {
                continue;
            };
            let current_vertices = incident_vertices(current, half_edge_map);

            let neighbors: Vec<_> = unvisited
                .iter()
                .copied()
                .filter(|candidate_id| {
                    half_edge_map
                        .get(candidate_id)
                        .copied()
                        .map(|candidate| {
                            let candidate_vertices = incident_vertices(candidate, half_edge_map);
                            current_vertices
                                .iter()
                                .any(|vertex_id| candidate_vertices.contains(vertex_id))
                        })
                        .unwrap_or(false)
                })
                .collect();

            for neighbor_id in neighbors {
                unvisited.remove(&neighbor_id);
                queue.push_back(neighbor_id);
            }
        }
    }

    components
}

fn vertex_degree_map(
    half_edge_ids: BTreeSet<EntityId>,
    half_edge_map: &BTreeMap<EntityId, &TopologyHalfEdge>,
) -> BTreeMap<EntityId, usize> {
    let mut degrees = BTreeMap::new();
    for half_edge_id in half_edge_ids {
        let Some(half_edge) = half_edge_map.get(&half_edge_id).copied() else {
            continue;
        };
        for vertex_id in incident_vertices(half_edge, half_edge_map) {
            *degrees.entry(vertex_id).or_insert(0) += 1;
        }
    }
    degrees
}

fn incident_vertices(
    half_edge: &TopologyHalfEdge,
    half_edge_map: &BTreeMap<EntityId, &TopologyHalfEdge>,
) -> BTreeSet<EntityId> {
    let mut vertices = BTreeSet::new();
    if let Some(origin) = half_edge.origin_vertex_id {
        vertices.insert(origin);
    }
    if let Some(target) = half_edge.target_vertex_id {
        vertices.insert(target);
        return vertices;
    }
    if let Some(next_id) = half_edge.next_half_edge_id {
        if let Some(next) = half_edge_map.get(&next_id).copied() {
            if let Some(target) = next.origin_vertex_id {
                vertices.insert(target);
            }
        }
    }
    vertices
}




