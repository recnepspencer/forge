use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::identity::EntityId;
use schema::facade::platform::authority::ShellInterpretationClass;

use crate::brep::topology_graph::{TopologyHalfEdge, TopologyView};
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::radial_rings::summarize_shell_radial;
use crate::derived_topology::traversal_views::types::{
    RadialInterpretationSummary, ShellInterpretation,
};

pub fn interpret_shells(
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
                class: if radial.boundary_half_edge_count == 0 {
                    if radial.non_manifold_edge_ids.is_empty() {
                        ShellInterpretationClass::ClosedSolid
                    } else {
                        ShellInterpretationClass::ClosedNonManifold
                    }
                } else {
                    if radial.non_manifold_edge_ids.is_empty() {
                        ShellInterpretationClass::OpenSheet
                    } else {
                        ShellInterpretationClass::OpenNonManifold
                    }
                },
                face_count: shell.face_ids.len(),
                boundary_component_count,
                boundary_half_edge_count: radial.boundary_half_edge_count,
                non_manifold_edge_ids: radial.non_manifold_edge_ids,
            }
        })
        .collect();

    (shells, radial_summaries)
}

fn shell_boundary_half_edges(
    view: &TopologyView,
    shell_face_ids: &BTreeSet<EntityId>,
) -> BTreeSet<EntityId> {
    let mut half_edge_ids = BTreeSet::new();
    for face in &view.faces {
        if shell_face_ids.contains(&face.entity_id) {
            for half_edge_id in &face.boundary_half_edge_ids {
                half_edge_ids.insert(*half_edge_id);
            }
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

        let mut cursor = half_edge.next_half_edge_id;
        while let Some(cursor_id) = cursor {
            if !boundary_ids.contains(&cursor_id) {
                let Some(record) = half_edge_map.get(&cursor_id).copied() else {
                    break;
                };
                cursor = record.next_half_edge_id;
                continue;
            }
            if !seen.insert(cursor_id) {
                break;
            }
            let Some(record) = half_edge_map.get(&cursor_id).copied() else {
                break;
            };
            cursor = record.next_half_edge_id;
        }
    }

    components
}
