use forge_core::errors::MergeError;
use forge_core::KernelError;
use forge_topo::handles::{EdgeId, FaceId, HalfEdgeId};

use super::super::schema::{MergePlan, MergeRegionSelection, MergeStepPlan};

/// Build a deterministic merge plan from a selection and topology snapshot.
///
/// Iterates all edges in the arena. For each edge where BOTH faces on a
/// halfedge pair are in the selected set, creates a `MergeStepPlan`.
/// Steps are sorted by `edge_index` for determinism.
pub(crate) fn build_merge_plan(
    arena: &forge_topo::arena::TopologyArena,
    selection: &MergeRegionSelection,
) -> Result<MergePlan, KernelError> {
    let selected = selection.get_selected_faces();
    let protected = selection.get_protected_faces();
    let surviving_idx = selection.get_surviving_face().index();
    let selectors = selection.get_radial_selectors();
    let mut steps: Vec<MergeStepPlan> = Vec::new();

    for (edge_id, edge_data) in arena.iter_edges() {
        let entry_he = edge_data.half_edge();
        let entry_face = arena.get_half_edge(entry_he)?.face();
        let _entry_face_idx = entry_face.index();

        let mut radial_ring: Vec<(HalfEdgeId, u32)> = Vec::new();
        let mut cur = entry_he;
        loop {
            let face = arena.get_half_edge(cur)?.face();
            radial_ring.push((cur, face.index()));
            cur = arena.get_half_edge(cur)?.radial_next();
            if cur == entry_he {
                break;
            }
        }

        let mut selected_uses: Vec<(HalfEdgeId, u32)> = Vec::new();
        for &(he, fi) in &radial_ring {
            if selected.contains(fi)? {
                selected_uses.push((he, fi));
            }
        }

        if selected_uses.len() < 2 {
            continue;
        }

        if let Some(selector) = selectors
            .iter()
            .find(|s| s.get_edge_index() == edge_id.index())
        {
            let kill_fi = selector.get_kill_face_index();
            if protected.contains(kill_fi)? {
                return Err(KernelError::MergeFailure(
                    MergeError::ProtectedUseConflict {
                        face_index: kill_fi,
                        edge_index: Some(edge_id.index()),
                    },
                ));
            }
            steps.push(MergeStepPlan {
                edge_index: edge_id.index(),
                survive_face_index: selector.get_survive_face_index(),
                kill_face_index: kill_fi,
            });
        } else if selected_uses.len() == 2 {
            let (_, fi_a) = selected_uses[0];
            let (_, fi_b) = selected_uses[1];

            let (survive_idx, kill_idx) = if fi_a == surviving_idx {
                (fi_a, fi_b)
            } else if fi_b == surviving_idx {
                (fi_b, fi_a)
            } else {
                (fi_a.min(fi_b), fi_a.max(fi_b))
            };

            if protected.contains(kill_idx)? {
                return Err(KernelError::MergeFailure(
                    MergeError::ProtectedUseConflict {
                        face_index: kill_idx,
                        edge_index: Some(edge_id.index()),
                    },
                ));
            }

            steps.push(MergeStepPlan {
                edge_index: edge_id.index(),
                survive_face_index: survive_idx,
                kill_face_index: kill_idx,
            });
        } else {
            return Err(KernelError::MergeFailure(
                MergeError::AmbiguousRadialSelection {
                    edge_index: edge_id.index(),
                    valence: selected_uses.len() as u32,
                },
            ));
        }
    }

    steps.sort_by_key(|s| s.edge_index);

    Ok(MergePlan::new(steps))
}

/// Re-derive halfedge handles for a merge step from the current draft arena.
///
/// Looks up the edge by index, walks its radial ring, and finds halfedges
/// on the survive and kill faces. Returns `PartialMergePlanRejected` if
/// the edge no longer exists or faces don't match.
pub(crate) fn rederive_halfedges_for_step(
    arena: &forge_topo::arena::TopologyArena,
    step: &MergeStepPlan,
    step_idx: usize,
) -> Result<(HalfEdgeId, HalfEdgeId, FaceId), KernelError> {
    let edge_id = find_edge_by_index(arena, step.edge_index).ok_or_else(|| {
        KernelError::MergeFailure(MergeError::PartialMergePlanRejected {
            step_index: Some(step_idx as u32),
            reason: format!(
                "Edge with index {} no longer exists in arena",
                step.edge_index
            ),
        })
    })?;

    let entry_he = arena.get_edge(edge_id)?.half_edge();
    let mut he_survive: Option<HalfEdgeId> = None;
    let mut he_kill: Option<HalfEdgeId> = None;
    let mut kill_face: Option<FaceId> = None;

    let mut cur = entry_he;
    loop {
        let face = arena.get_half_edge(cur)?.face();
        let fi = face.index();

        if fi == step.survive_face_index && he_survive.is_none() {
            he_survive = Some(cur);
        } else if fi == step.kill_face_index && he_kill.is_none() {
            he_kill = Some(cur);
            kill_face = Some(face);
        }

        cur = arena.get_half_edge(cur)?.radial_next();
        if cur == entry_he {
            break;
        }
    }

    match (he_survive, he_kill, kill_face) {
        (Some(hs), Some(hk), Some(kf)) => Ok((hs, hk, kf)),
        _ => Err(KernelError::MergeFailure(
            MergeError::PartialMergePlanRejected {
                step_index: Some(step_idx as u32),
                reason: format!(
                    "Edge {} radial ring does not contain faces {} and {}",
                    step.edge_index, step.survive_face_index, step.kill_face_index,
                ),
            },
        )),
    }
}

/// Find a FaceId by its arena index.
pub(crate) fn find_face_by_index(
    arena: &forge_topo::arena::TopologyArena,
    index: u32,
) -> Result<FaceId, KernelError> {
    for (face_id, _) in arena.iter_faces() {
        if face_id.index() == index {
            return Ok(face_id);
        }
    }
    Err(KernelError::InvalidInput {
        message: format!("No face with index {} in arena", index),
        context: None,
    })
}

/// Find an EdgeId by its arena index.
pub(crate) fn find_edge_by_index(arena: &forge_topo::arena::TopologyArena, index: u32) -> Option<EdgeId> {
    for (edge_id, _) in arena.iter_edges() {
        if edge_id.index() == index {
            return Some(edge_id);
        }
    }
    None
}
