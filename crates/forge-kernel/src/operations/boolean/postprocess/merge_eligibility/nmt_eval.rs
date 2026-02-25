//! Sheet region merge execution engine.
//!
//! DOMAIN: Compound algorithm that orchestrates face merges using
//! `JoinFaces` (manifold) and `JoinFacesNmt` (NMT) based on radial valence.
//! Operates on `KernelDraft` for atomic topo+geom transactionality.
//!
//! STATUS: Staged for integration (test-exercised only). Not yet called from
//! production boolean postprocess flow. Will be wired via coplanar.rs after
//! Epic A boundary certification gate is in the production path.
//!
//! DEPENDENCIES: `KernelDraft`, `GeometryPatch`, `JoinFaces`, `JoinFacesNmt`,
//! `radial_valence`, `ModelingContext`, `TracedDecision`.
//!
//! INVARIANTS:
//!   - Drop KernelDraft = atomic rollback of topology AND geometry (D6)
//!   - Handles re-derived per step from draft arena (no stale handles)
//!   - Steps sorted by edge_index for determinism
//!   - TracedDecision emitted per step
//!   - Decisions propagated to both OperationResult and ModelingContext

use std::collections::BTreeSet;

use forge_core::{
    KernelError, OperationResult,
};
use forge_core::errors::MergeError;
use forge_core::tracing::{
    DecisionId, DecisionKind, DecisionTier, DecisionContext, TracedDecision, DecisionLog,
};
use forge_topo::handles::{FaceId, HalfEdgeId, EdgeId};
use forge_topo::operator::apply_op;
use forge_topo::euler::join_faces::JoinFaces;
use forge_topo::euler::join_faces_nmt::JoinFacesNmt;
use forge_topo::traverse::radial_valence;

use crate::core::KernelState;
use crate::core::kernel_draft::KernelDraft;
use crate::core::ModelingContext;

use super::schema::{
    MergeRegionSelection, MergePlan, MergeStepPlan, MergeResult, SheetRegionMergeOutput,
};

/// Execute a sheet region merge: validate, plan, execute, commit.
///
/// Takes `KernelState` by value. On success, returns the committed state
/// bundled with merge metadata in `SheetRegionMergeOutput`.
/// On failure, the draft is dropped (atomic rollback of topo + geometry).
///
/// Internal flow (spec §5.9):
/// 1. Create `KernelDraft` from `KernelState`
/// 2. Validate protected-face / selected-face disjointness
/// 3. Validate connectivity of selected faces (BFS)
/// 4. Build `MergePlan` (deterministic step ordering by edge_index)
/// 5. Execute steps one-at-a-time with handle re-derivation
/// 6. Propagate decisions to both `OperationResult` and `ModelingContext`
/// 7. `commit_with_mode(Intermediate, NmtIntermediate)`
pub fn execute_sheet_region_merge(
    state: KernelState,
    selection: &MergeRegionSelection,
    ctx: &mut ModelingContext,
) -> Result<OperationResult<SheetRegionMergeOutput>, KernelError> {
    let mut draft = KernelDraft::new(state);

    validate_protected_faces(selection)?;
    validate_connectivity(draft.arena(), selection)?;

    let plan = build_merge_plan(draft.arena(), selection)?;

    let mut decision_log = DecisionLog::new();
    let mut killed_faces: Vec<FaceId> = Vec::with_capacity(plan.step_count());

    for (step_idx, step) in plan.get_steps().iter().enumerate() {
        let (he_survive, he_kill, killed_face) = rederive_halfedges_for_step(
            draft.arena(),
            step,
            step_idx,
        )?;

        let current_valence = radial_valence(draft.arena(), he_survive)?;

        if current_valence == 2 {
            apply_op(draft.draft_mut(), JoinFaces { edge: he_survive })?
                .into_value();
        } else {
            apply_op(draft.draft_mut(), JoinFacesNmt {
                he_survive,
                he_kill,
            })?.into_value();
        }

        draft.geometry_mut().remove_face_plane(killed_face);

        killed_faces.push(killed_face);

        let decision = TracedDecision::new(
            DecisionId(step.edge_index as u64),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Degeneracy {
                description: format!(
                    "MergeStep {}/{}: edge_idx={} kill_face_idx={} valence={}",
                    step_idx + 1, plan.step_count(),
                    step.edge_index, step.kill_face_index, current_valence,
                ),
            },
        );
        decision_log.record(decision);
    }

    let new_state = draft.commit_with_mode(
        forge_topo::validate::ValidationLevel::Intermediate,
        forge_topo::validate::TopologyMode::NmtIntermediate,
    )?;

    let merge_result = MergeResult::new(
        selection.get_surviving_face(),
        killed_faces,
        plan,
    );

    let output = SheetRegionMergeOutput::new(new_state, merge_result);

    ctx.get_decision_log_mut().merge(decision_log.clone());

    let mut op_result = OperationResult::new(output);
    op_result.set_decision_log(decision_log);

    Ok(op_result)
}

/// Reject if any face appears in both `selected_faces` and `protected_faces`.
///
/// This is a deterministic input validation: the two sets must be disjoint.
/// If overlap exists, no merge can proceed without violating protection semantics.
fn validate_protected_faces(
    selection: &MergeRegionSelection,
) -> Result<(), KernelError> {
    let selected = selection.get_selected_faces();
    let protected = selection.get_protected_faces();

    for idx in selected.iter_ones() {
        if protected.contains(idx)? {
            return Err(KernelError::MergeFailure(MergeError::ProtectedUseConflict {
                face_index: idx,
                edge_index: None,
            }));
        }
    }

    Ok(())
}

/// Validate that all selected faces form a connected subgraph.
///
/// Uses BFS via shared edges: two selected faces are connected if they
/// share at least one edge. All selected faces must be reachable from
/// the surviving face.
fn validate_connectivity(
    arena: &forge_topo::arena::TopologyArena,
    selection: &MergeRegionSelection,
) -> Result<(), KernelError> {
    let selected = selection.get_selected_faces();

    let mut selected_indices: Vec<u32> = Vec::new();
    for (face_id, _) in arena.iter_faces() {
        let idx = face_id.index();
        if selected.contains(idx)? {
            selected_indices.push(idx);
        }
    }

    if selected_indices.is_empty() {
        return Err(KernelError::MergeFailure(MergeError::WouldDisconnectSheet {
            face_index: 0,
        }));
    }

    let start_idx = selection.get_surviving_face().index();
    if !selected_indices.contains(&start_idx) {
        return Err(KernelError::InvalidInput {
            message: "Surviving face is not in selected_faces set".into(),
            context: None,
        });
    }

    let mut visited: BTreeSet<u32> = BTreeSet::new();
    let mut queue: Vec<u32> = vec![start_idx];
    visited.insert(start_idx);

    while let Some(current_face_idx) = queue.pop() {
        let current_face_id = find_face_by_index(arena, current_face_idx)?;

        let outer_loop = arena.get_face(current_face_id)?.outer_loop();
        let loop_he = arena.get_loop(outer_loop)?.half_edge();

        let mut he = loop_he;
        loop {
            let twin = arena.get_half_edge(he)?.radial_next();
            if twin != he {
                let neighbor_face = arena.get_half_edge(twin)?.face();
                let neighbor_idx = neighbor_face.index();

                if selected.contains(neighbor_idx)? && !visited.contains(&neighbor_idx) {
                    visited.insert(neighbor_idx);
                    queue.push(neighbor_idx);
                }

                let mut radial_cur = arena.get_half_edge(twin)?.radial_next();
                while radial_cur != he {
                    let rf = arena.get_half_edge(radial_cur)?.face();
                    let ri = rf.index();
                    if selected.contains(ri)? && !visited.contains(&ri) {
                        visited.insert(ri);
                        queue.push(ri);
                    }
                    radial_cur = arena.get_half_edge(radial_cur)?.radial_next();
                }
            }

            he = arena.get_half_edge(he)?.next();
            if he == loop_he { break; }
        }
    }

    if visited.len() != selected_indices.len() {
        let disconnected = selected_indices.iter()
            .find(|idx| !visited.contains(idx))
            .copied()
            .unwrap_or(0);
        return Err(KernelError::MergeFailure(MergeError::WouldDisconnectSheet {
            face_index: disconnected,
        }));
    }

    Ok(())
}

/// Build a deterministic merge plan from a selection and topology snapshot.
///
/// Iterates all edges in the arena. For each edge where BOTH faces on a
/// halfedge pair are in the selected set, creates a `MergeStepPlan`.
/// Steps are sorted by `edge_index` for determinism.
fn build_merge_plan(
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
        let entry_face_idx = entry_face.index();

        let mut radial_ring: Vec<(HalfEdgeId, u32)> = Vec::new();
        let mut cur = entry_he;
        loop {
            let face = arena.get_half_edge(cur)?.face();
            radial_ring.push((cur, face.index()));
            cur = arena.get_half_edge(cur)?.radial_next();
            if cur == entry_he { break; }
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

        if let Some(selector) = selectors.iter().find(|s| s.get_edge_index() == edge_id.index()) {
            let kill_fi = selector.get_kill_face_index();
            if protected.contains(kill_fi)? {
                return Err(KernelError::MergeFailure(MergeError::ProtectedUseConflict {
                    face_index: kill_fi,
                    edge_index: Some(edge_id.index()),
                }));
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
                return Err(KernelError::MergeFailure(MergeError::ProtectedUseConflict {
                    face_index: kill_idx,
                    edge_index: Some(edge_id.index()),
                }));
            }

            steps.push(MergeStepPlan {
                edge_index: edge_id.index(),
                survive_face_index: survive_idx,
                kill_face_index: kill_idx,
            });
        } else {
            return Err(KernelError::MergeFailure(MergeError::AmbiguousRadialSelection {
                edge_index: edge_id.index(),
                valence: selected_uses.len() as u32,
            }));
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
fn rederive_halfedges_for_step(
    arena: &forge_topo::arena::TopologyArena,
    step: &MergeStepPlan,
    step_idx: usize,
) -> Result<(HalfEdgeId, HalfEdgeId, FaceId), KernelError> {
    let edge_id = find_edge_by_index(arena, step.edge_index)
        .ok_or_else(|| KernelError::MergeFailure(MergeError::PartialMergePlanRejected {
            step_index: Some(step_idx as u32),
            reason: format!("Edge with index {} no longer exists in arena", step.edge_index),
        }))?;

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
        if cur == entry_he { break; }
    }

    match (he_survive, he_kill, kill_face) {
        (Some(hs), Some(hk), Some(kf)) => Ok((hs, hk, kf)),
        _ => Err(KernelError::MergeFailure(MergeError::PartialMergePlanRejected {
            step_index: Some(step_idx as u32),
            reason: format!(
                "Edge {} radial ring does not contain faces {} and {}",
                step.edge_index, step.survive_face_index, step.kill_face_index,
            ),
        })),
    }
}

/// Find a FaceId by its arena index.
fn find_face_by_index(
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
fn find_edge_by_index(
    arena: &forge_topo::arena::TopologyArena,
    index: u32,
) -> Option<EdgeId> {
    for (edge_id, _) in arena.iter_edges() {
        if edge_id.index() == index {
            return Some(edge_id);
        }
    }
    None
}
