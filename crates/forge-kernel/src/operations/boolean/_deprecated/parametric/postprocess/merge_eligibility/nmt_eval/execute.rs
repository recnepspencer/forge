use forge_core::tracing::{DecisionContext, DecisionId, DecisionKind, DecisionTier, TraceAdjunctSet, TracedDecision};
use forge_core::{KernelError, OperationResult};
use forge_topo::boundary_editing::join_faces::JoinFaces;
use forge_topo::boundary_editing::join_faces_nmt::JoinFacesNmt;
use forge_topo::handles::FaceId;
use forge_topo::operator::apply_op;
use forge_topo::traverse::radial_valence;

use crate::core::kernel_draft::KernelDraft;
use crate::core::ModelingContext;
use crate::core::{KernelState, OperationFinalizer, TopologyHashBoundary};

use super::super::eval::certify_merge_boundary;
use super::super::schema::{
    MergeRegionSelection, MergeRegionSelectionPersistent, MergeResult, SheetRegionMergeOutput,
};

use super::plan::{build_merge_plan, rederive_halfedges_for_step};
use super::resolve::resolve_merge_region_selection_persistent;
use super::validate::{apply_boundary_cert_gate_policy, validate_connectivity, validate_protected_faces};

/// Persistent-name variant of `execute_sheet_region_merge`.
pub fn execute_sheet_region_merge_persistent(
    state: KernelState,
    selection: &MergeRegionSelectionPersistent,
    ctx: &mut ModelingContext,
) -> Result<OperationResult<SheetRegionMergeOutput>, KernelError> {
    let snapshot_selection = resolve_merge_region_selection_persistent(&state, selection, ctx)?;
    execute_sheet_region_merge(state, &snapshot_selection, ctx)
}

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
    let finalization_adjuncts = TraceAdjunctSet::new();

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
        let (he_survive, he_kill, killed_face) =
            rederive_halfedges_for_step(draft.arena(), step, step_idx)?;

        let current_valence = radial_valence(draft.arena(), he_survive)?;

        if current_valence == 2 {
            apply_op(draft.draft_mut(), JoinFaces { edge: he_survive })?.into_value();
        } else {
            apply_op(
                draft.draft_mut(),
                JoinFacesNmt {
                    he_survive,
                    he_kill,
                },
            )?
            .into_value();
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
                    step_idx + 1,
                    plan.step_count(),
                    step.edge_index,
                    step.kill_face_index,
                    current_valence,
                ),
            },
        );
        ctx.get_decision_log_mut().record(decision);
    }

    let new_state = draft.commit_with_mode(
        forge_topo::validate::ValidationLevel::Intermediate,
        forge_topo::validate::TopologyMode::NmtIntermediate,
    )?;

    let merge_result = MergeResult::new(selection.get_surviving_face(), killed_faces, plan);

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
            None,
        )
        .map_err(|e| KernelError::InternalError {
            message: format!("region merge finalization failed: {:?}", e),
            context: None,
        })?;

    Ok(op_result)
}
