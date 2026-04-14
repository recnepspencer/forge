use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::identity::EntityId;
use worth_schema::facade::WorthShellInterpretationClass;

use crate::data::topology_view::{WorthTopologyHalfEdge, WorthTopologyView};
use crate::interpretation::types::WorthShellInterpretation;

pub fn interpret_shells(view: &WorthTopologyView) -> Vec<WorthShellInterpretation> {
    let half_edge_map: BTreeMap<EntityId, &WorthTopologyHalfEdge> = view
        .half_edges
        .iter()
        .map(|record| (record.entity_id, record))
        .collect();

    view.shells
        .iter()
        .map(|shell| {
            let shell_face_ids: BTreeSet<_> = shell.face_ids.iter().copied().collect();
            let shell_half_edges = shell_boundary_half_edges(view, &shell_face_ids);
            let boundary_component_count =
                count_boundary_components(&shell_half_edges, &half_edge_map);

            let mut boundary_half_edge_count = 0;
            let mut non_manifold_edge_ids = BTreeSet::new();

            for half_edge_id in &shell_half_edges {
                let Some(half_edge) = half_edge_map.get(half_edge_id).copied() else {
                    continue;
                };
                if half_edge.radial_next_half_edge_id == Some(half_edge.entity_id) {
                    boundary_half_edge_count += 1;
                    continue;
                }
                if let Some(edge_id) = half_edge.edge_id {
                    let ring_len = walk_radial_ring_len(half_edge.entity_id, &half_edge_map);
                    if ring_len > 2 {
                        non_manifold_edge_ids.insert(edge_id);
                    }
                }
            }

            WorthShellInterpretation {
                shell_id: shell.entity_id,
                class: if boundary_half_edge_count == 0 {
                    if non_manifold_edge_ids.is_empty() {
                        WorthShellInterpretationClass::ClosedSolid
                    } else {
                        WorthShellInterpretationClass::ClosedNonManifold
                    }
                } else {
                    if non_manifold_edge_ids.is_empty() {
                        WorthShellInterpretationClass::OpenSheet
                    } else {
                        WorthShellInterpretationClass::OpenNonManifold
                    }
                },
                face_count: shell.face_ids.len(),
                boundary_component_count,
                boundary_half_edge_count,
                non_manifold_edge_ids: non_manifold_edge_ids.into_iter().collect(),
            }
        })
        .collect()
}

fn shell_boundary_half_edges(
    view: &WorthTopologyView,
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

fn walk_radial_ring_len(
    start_id: EntityId,
    half_edge_map: &BTreeMap<EntityId, &WorthTopologyHalfEdge>,
) -> usize {
    let mut count = 0;
    let mut seen = BTreeSet::new();
    let mut current_id = start_id;

    loop {
        if !seen.insert(current_id) {
            break;
        }
        count += 1;
        let Some(current) = half_edge_map.get(&current_id).copied() else {
            break;
        };
        let Some(next_id) = current.radial_next_half_edge_id else {
            break;
        };
        current_id = next_id;
    }

    count
}

fn count_boundary_components(
    shell_half_edges: &BTreeSet<EntityId>,
    half_edge_map: &BTreeMap<EntityId, &WorthTopologyHalfEdge>,
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
