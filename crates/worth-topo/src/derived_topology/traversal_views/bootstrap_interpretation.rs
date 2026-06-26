use std::collections::{BTreeMap, BTreeSet, VecDeque};

use forge_relational::facade::identity::EntityId;
use schema::facade::platform::authority::{ShellInterpretationClass, WireInterpretationClass};
use schema::facade::topology_authoring::{
    CertifiedTopologyInterpretation, DerivedTopologyReadBasis, TopologyReadArtifact,
};

use super::boundary_summaries::summarize_boundary_interpretations;
use crate::brep::topology_graph::{TopologyHalfEdge, TopologyView};
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::types::{
    InterpretationReport, InterpretedTopologyView, RadialInterpretationSummary,
    ShellInterpretation, TopologyInterpretationSet, WireInterpretation,
};

pub(crate) fn bootstrap_topology_interpretation(
    view: &MaterializedTopologyView,
) -> InterpretedTopologyView {
    let wires = bootstrap_wire_views(view);
    let (shells, radial_summaries) = bootstrap_shell_views(view);
    let interpretations = TopologyInterpretationSet { wires, shells };
    let boundary_summaries = summarize_boundary_interpretations(&interpretations);
    let report = InterpretationReport {
        interpreted_wire_count: interpretations.wires.len(),
        interpreted_shell_count: interpretations.shells.len(),
        boundary_interpretation_count: boundary_summaries.len(),
        radial_interpretation_count: radial_summaries.len(),
    };

    InterpretedTopologyView::new(
        view.clone(),
        interpretations,
        boundary_summaries,
        radial_summaries,
        report,
    )
}

pub fn build_topology_read_artifact(
    read_basis: &DerivedTopologyReadBasis,
    view: &InterpretedTopologyView,
) -> TopologyReadArtifact {
    TopologyReadArtifact::from_read_basis_and_interpretation(
        read_basis,
        view.interpretations().clone(),
    )
}

pub fn certify_topology_view(
    read_basis: DerivedTopologyReadBasis,
    view: &InterpretedTopologyView,
) -> CertifiedTopologyInterpretation {
    CertifiedTopologyInterpretation::from_read_basis_and_interpretation(
        read_basis,
        view.interpretations().clone(),
    )
}

fn bootstrap_wire_views(view: &MaterializedTopologyView) -> Vec<WireInterpretation> {
    let topology = view.topology();
    let half_edge_map: BTreeMap<EntityId, &TopologyHalfEdge> = topology
        .half_edges
        .iter()
        .map(|record| (record.entity_id, record))
        .collect();

    topology
        .wires
        .iter()
        .map(|wire| {
            let branching = bootstrap_wire_branching(
                wire.half_edge_ids.iter().copied().collect(),
                &half_edge_map,
            );

            WireInterpretation {
                wire_id: wire.entity_id,
                class: branching.class,
                connected_component_count: branching.connected_component_count,
                terminal_vertex_ids: branching.terminal_vertex_ids,
                branch_vertex_ids: branching.branch_vertex_ids,
            }
        })
        .collect()
}

fn bootstrap_shell_views(
    view: &MaterializedTopologyView,
) -> (Vec<ShellInterpretation>, Vec<RadialInterpretationSummary>) {
    let topology = view.topology();
    let half_edge_map: BTreeMap<EntityId, &TopologyHalfEdge> = topology
        .half_edges
        .iter()
        .map(|record| (record.entity_id, record))
        .collect();

    let mut radial_summaries = Vec::new();
    let shells = topology
        .shells
        .iter()
        .map(|shell| {
            let shell_face_ids: BTreeSet<_> = shell.face_ids.iter().copied().collect();
            let shell_half_edges = shell_boundary_half_edges(topology, &shell_face_ids);
            let boundary_component_count =
                count_boundary_components(&shell_half_edges, &half_edge_map);
            let radial = summarize_shell_radial(shell.entity_id, &shell_half_edges, &half_edge_map);
            radial_summaries.push(radial.clone());

            ShellInterpretation {
                shell_id: shell.entity_id,
                class: shell_interpretation_class(&radial),
                face_count: shell.face_ids.len(),
                boundary_component_count,
                boundary_half_edge_count: radial.boundary_half_edge_count,
                non_manifold_edge_ids: radial.non_manifold_edge_ids,
            }
        })
        .collect();

    (shells, radial_summaries)
}

fn shell_interpretation_class(radial: &RadialInterpretationSummary) -> ShellInterpretationClass {
    match (
        radial.boundary_half_edge_count == 0,
        radial.non_manifold_edge_ids.is_empty(),
    ) {
        (true, true) => ShellInterpretationClass::ClosedSolid,
        (true, false) => ShellInterpretationClass::ClosedNonManifold,
        (false, true) => ShellInterpretationClass::OpenSheet,
        (false, false) => ShellInterpretationClass::OpenNonManifold,
    }
}

fn shell_boundary_half_edges(
    view: &TopologyView,
    shell_face_ids: &BTreeSet<EntityId>,
) -> BTreeSet<EntityId> {
    let mut half_edge_ids = BTreeSet::new();
    for face in &view.faces {
        if shell_face_ids.contains(&face.entity_id) {
            half_edge_ids.extend(face.boundary_half_edge_ids.iter().copied());
        }
    }
    half_edge_ids
}

fn count_boundary_components(
    shell_half_edges: &BTreeSet<EntityId>,
    half_edge_map: &BTreeMap<EntityId, &TopologyHalfEdge>,
) -> usize {
    let boundary_half_edges = shell_half_edges
        .iter()
        .filter_map(|half_edge_id| half_edge_map.get(half_edge_id).copied())
        .filter(|half_edge| half_edge.radial_next_half_edge_id == Some(half_edge.entity_id))
        .collect::<Vec<_>>();

    let boundary_ids: BTreeSet<EntityId> = boundary_half_edges
        .iter()
        .map(|half_edge| half_edge.entity_id)
        .collect();
    let mut seen = BTreeSet::new();
    let mut components = 0usize;

    for half_edge in boundary_half_edges {
        if !seen.insert(half_edge.entity_id) {
            continue;
        }
        components += 1;
        walk_boundary_component(half_edge, &boundary_ids, half_edge_map, &mut seen);
    }

    components
}

fn walk_boundary_component(
    half_edge: &TopologyHalfEdge,
    boundary_ids: &BTreeSet<EntityId>,
    half_edge_map: &BTreeMap<EntityId, &TopologyHalfEdge>,
    seen: &mut BTreeSet<EntityId>,
) {
    let mut cursor = half_edge.next_half_edge_id;
    while let Some(cursor_id) = cursor {
        let Some(record) = half_edge_map.get(&cursor_id).copied() else {
            break;
        };
        if boundary_ids.contains(&cursor_id) && !seen.insert(cursor_id) {
            break;
        }
        cursor = record.next_half_edge_id;
    }
}

fn summarize_shell_radial(
    shell_id: EntityId,
    shell_half_edges: &BTreeSet<EntityId>,
    half_edge_map: &BTreeMap<EntityId, &TopologyHalfEdge>,
) -> RadialInterpretationSummary {
    let radial = bootstrap_shell_radial(shell_half_edges, half_edge_map);
    RadialInterpretationSummary {
        shell_id,
        boundary_half_edge_count: radial.boundary_half_edge_count,
        non_manifold_edge_ids: radial.non_manifold_edge_ids,
    }
}

struct ShellRadialBootstrap {
    boundary_half_edge_count: usize,
    non_manifold_edge_ids: Vec<EntityId>,
}

fn bootstrap_shell_radial(
    shell_half_edges: &BTreeSet<EntityId>,
    half_edge_map: &BTreeMap<EntityId, &TopologyHalfEdge>,
) -> ShellRadialBootstrap {
    let mut boundary_half_edge_count = 0usize;
    let mut non_manifold_edge_ids = BTreeSet::new();

    for half_edge_id in shell_half_edges {
        let Some(half_edge) = half_edge_map.get(half_edge_id).copied() else {
            continue;
        };
        if half_edge.radial_next_half_edge_id == Some(half_edge.entity_id) {
            boundary_half_edge_count += 1;
            continue;
        }
        let ring_len = walk_shell_radial_ring_len(half_edge.entity_id, half_edge_map);
        if ring_len > 2 {
            if let Some(edge_id) = half_edge.edge_id {
                non_manifold_edge_ids.insert(edge_id);
            }
        }
    }

    ShellRadialBootstrap {
        boundary_half_edge_count,
        non_manifold_edge_ids: non_manifold_edge_ids.into_iter().collect(),
    }
}

fn walk_shell_radial_ring_len(
    start_id: EntityId,
    half_edge_map: &BTreeMap<EntityId, &TopologyHalfEdge>,
) -> usize {
    let mut seen = BTreeSet::new();
    let mut current_id = start_id;

    loop {
        if !seen.insert(current_id) {
            break;
        }
        let Some(current) = half_edge_map.get(&current_id).copied() else {
            break;
        };
        let Some(next_id) = current.radial_next_half_edge_id else {
            break;
        };
        current_id = next_id;
    }

    seen.len()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WireBranchBootstrap {
    class: WireInterpretationClass,
    connected_component_count: usize,
    terminal_vertex_ids: Vec<EntityId>,
    branch_vertex_ids: Vec<EntityId>,
}

fn bootstrap_wire_branching(
    half_edge_ids: BTreeSet<EntityId>,
    half_edge_map: &BTreeMap<EntityId, &TopologyHalfEdge>,
) -> WireBranchBootstrap {
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

    WireBranchBootstrap {
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
