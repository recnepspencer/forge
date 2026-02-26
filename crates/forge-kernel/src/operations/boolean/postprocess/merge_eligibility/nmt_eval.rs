//! Sheet region merge execution engine.
//!
//! DOMAIN: Compound algorithm that orchestrates face merges using
//! `JoinFaces` (manifold) and `JoinFacesNmt` (NMT) based on radial valence.
//! Operates on `KernelDraft` for atomic topo+geom transactionality.
//!
//! STATUS: Staged for integration (test-exercised only). Not yet called from
//! production boolean postprocess flow.
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
    KernelError, OperationResult, PolicyKind, PolicyQuery,
};
use forge_core::errors::MergeError;
use forge_core::tracing::{
    CandidateValueSummary, DecisionId, DecisionKind, DecisionTier, DecisionContext, TracedDecision, TraceAdjunctSet,
};
use forge_topo::handles::{FaceId, HalfEdgeId, EdgeId};
use forge_topo::operator::apply_op;
use forge_topo::euler::join_faces::JoinFaces;
use forge_topo::euler::join_faces_nmt::JoinFacesNmt;
use forge_topo::traverse::radial_valence;

use crate::core::{KernelState, OperationFinalizer, TopologyHashBoundary};
use crate::core::kernel_draft::KernelDraft;
use crate::core::ModelingContext;

use super::eval::{certify_merge_boundary, compute_group_hash};
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
/// 1. Certify merge boundary (Epic A gate) before touching topology
/// 2. Create `KernelDraft` from `KernelState`
/// 3. Validate protected-face / selected-face disjointness
/// 4. Validate connectivity of selected faces (BFS)
/// 5. Build `MergePlan` (deterministic step ordering by edge_index)
/// 6. Execute steps one-at-a-time with handle re-derivation
/// 7. Propagate decisions to both `OperationResult` and `ModelingContext`
/// 8. `commit_with_mode(Intermediate, NmtIntermediate)`
pub fn execute_sheet_region_merge(
    state: KernelState,
    selection: &MergeRegionSelection,
    ctx: &mut ModelingContext,
) -> Result<OperationResult<SheetRegionMergeOutput>, KernelError> {
    let topo_hash_before = state.topology().topology_hash();
    let mut finalization_adjuncts = TraceAdjunctSet::new();

    // Mandatory Epic A gate: certify before creating a draft so a rejected
    // boundary cannot produce any topo/geom mutations.
    let mut cert_result = certify_merge_boundary(
        state.topology().arena(),
        selection.get_selected_faces(),
        state.geometry(),
    )?;
    ctx.absorb_sub_result(&mut cert_result);
    apply_boundary_cert_gate_policy(cert_result.get_value(), selection, ctx)?;

    let mut draft = KernelDraft::new(state);

    validate_protected_faces(selection)?;
    validate_connectivity(draft.arena(), selection)?;

    let plan = build_merge_plan(draft.arena(), selection)?;
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
        ctx.get_decision_log_mut().record(decision);
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
    let topo_hash_after = output.get_state().topology().topology_hash();

    let mut op_result = OperationResult::new(output);
    let mut finalizer = OperationFinalizer::new(ctx);
    let _collected = finalizer
        .collect_success(
            &mut op_result,
            finalization_adjuncts,
            TopologyHashBoundary {
                before: Some(topo_hash_before),
                after: Some(topo_hash_after),
            },
        )
        .map_err(|e| KernelError::InternalError {
            message: format!("region merge finalization failed: {:?}", e),
            context: None,
        })?;

    Ok(op_result)
}

fn apply_boundary_cert_gate_policy(
    cert: &forge_geom::algorithms::boundary_cert::schema::WeakSimpleCertificate,
    selection: &MergeRegionSelection,
    ctx: &mut ModelingContext,
) -> Result<(), KernelError> {
    match cert {
        forge_geom::algorithms::boundary_cert::schema::WeakSimpleCertificate::Rejected { reason, witness } => {
            Err(KernelError::MergeFailure(MergeError::BoundaryCertificationFailed {
                reason: format!("{:?}", reason),
                witness: Some(*witness),
            }))
        }
        forge_geom::algorithms::boundary_cert::schema::WeakSimpleCertificate::WeaklySimple { touch_count } => {
            let group_hash = compute_group_hash(selection.get_selected_faces())?;
            let policy_decision_id = DecisionId(group_hash ^ 0x9e37_79b9_7f4a_7c15);
            let prev_scope = ctx.get_active_operation_policy_scope().map(str::to_string);
            ctx.set_active_operation_policy_scope(Some("sheet_region_merge".to_string()));
            let policy_query = PolicyQuery {
                kind: PolicyKind::CoincidentGeometry,
                location: [0.0, 0.0, 0.0],
                margin: *touch_count as f64,
                overridable: true,
            };
            let resolved_result = ctx.resolve_policy_query(
                policy_decision_id,
                &policy_query,
                Some(0.0),
                CandidateValueSummary::EnumTag {
                    type_name: "WeakSimpleCertificate".to_string(),
                    variant: "WeaklySimple".to_string(),
                },
            );
            ctx.set_active_operation_policy_scope(prev_scope);
            let resolved = resolved_result?;
            if !resolved.accept_potential_value {
                return Err(KernelError::MergeFailure(MergeError::BoundaryCertificationFailed {
                    reason: "CoincidentGeometry policy rejected WeaklySimple boundary".to_string(),
                    witness: None,
                }));
            }
            Ok(())
        }
        forge_geom::algorithms::boundary_cert::schema::WeakSimpleCertificate::Simple => Ok(()),
    }
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

#[cfg(test)]
pub(super) fn test_build_merge_plan(
    arena: &forge_topo::arena::TopologyArena,
    selection: &MergeRegionSelection,
) -> Result<MergePlan, KernelError> {
    build_merge_plan(arena, selection)
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

#[cfg(test)]
pub(super) fn test_validate_connectivity(
    arena: &forge_topo::arena::TopologyArena,
    selection: &MergeRegionSelection,
) -> Result<(), KernelError> {
    validate_connectivity(arena, selection)
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

#[cfg(test)]
mod gate_policy_tests {
    use super::*;
    use forge_geom::algorithms::boundary_cert::schema::WeakSimpleCertificate;
    use forge_topo::bitset::EntityBitset;

    #[test]
    fn weakly_simple_gate_uses_registry_backed_policy_resolution() {
        let mut selected = EntityBitset::with_capacity(4);
        selected.insert(0).expect("bitset capacity");
        let protected = EntityBitset::with_capacity(4);
        let selection = MergeRegionSelection::new(
            selected,
            protected,
            FaceId::from_raw_parts(0, 0),
        );

        let mut ctx = ModelingContext::new();
        ctx.set_session_policy_override(PolicyKind::CoincidentGeometry, false, Some("qa".into()));

        let err = apply_boundary_cert_gate_policy(
            &WeakSimpleCertificate::WeaklySimple { touch_count: 2 },
            &selection,
            &mut ctx,
        ).expect_err("session override rejecting CoincidentGeometry must fail merge gate");

        assert!(matches!(err, KernelError::MergeFailure(MergeError::BoundaryCertificationFailed { .. })));
        assert_eq!(ctx.get_trace_adjuncts().records().len(), 1);

        let payload = ctx.get_trace_adjuncts().records()[0]
            .as_policy_payload()
            .expect("policy adjunct kind")
            .expect("decode policy payload");
        assert_eq!(payload.source, forge_core::PolicyResolutionSource::SessionUserOverride);
        assert_eq!(
            payload.source_scope,
            Some(forge_core::PolicyResolutionScopeRef::SessionUser {
                scope_id: Some("qa".to_string()),
            })
        );
        assert_eq!(payload.operation_scope_id.as_deref(), Some("sheet_region_merge"));
        assert_eq!(payload.outcome, forge_core::PolicyResolutionOutcome::RejectedPotentialValue);
        assert_eq!(ctx.get_decision_count(), 1, "policy resolution must emit one traced decision");
    }
}
