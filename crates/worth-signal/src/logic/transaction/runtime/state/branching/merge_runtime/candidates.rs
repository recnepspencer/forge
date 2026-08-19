use std::collections::BTreeSet;

use crate::data::error::SignalError;
use crate::logic::transaction::runtime::{
    BranchMergeFailureKind, BranchMergeRequestScopeFamily, LoweredFoundationalMergeRequest,
    MergeBoundaryWitness, MergeBoundaryWitnessKind, MergeNodeMap, PlannedMergeCandidateSet,
    ScopedMergeProofPacket, StructuralMergeJournalSlice,
};
use crate::state::{SignalBranchId, SignalSnapshotId};

use super::super::super::merge::{
    classify_initial_scoped_merge_admission, scoped_admission_outcome_to_signal_error,
    BranchMutationJournalSlice, LoweredScopedMergeCandidateSet,
};
use super::super::super::runtime_state::SignalRuntime;
use super::super::branches::BranchState;
use super::merge_base_strategy::MergeBaseResolution;

pub(super) struct BranchStateDiscovery<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) source_state: BranchState<D, I, T>,
    pub(super) target_state_owned: Option<BranchState<D, I, T>>,
    pub(super) source_snapshot_id: Option<SignalSnapshotId>,
    pub(super) target_snapshot_id_before: Option<SignalSnapshotId>,
    pub(super) source_branch_id: SignalBranchId,
    pub(super) target_branch_id: SignalBranchId,
}

pub(super) struct CandidateDiscovery<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) branch_states: BranchStateDiscovery<D, I, T>,
    pub(super) boundary_witness: MergeBoundaryWitness,
    pub(super) source_journal: StructuralMergeJournalSlice,
    pub(super) target_identity_journal: BranchMutationJournalSlice,
    pub(super) scoped_candidates: LoweredScopedMergeCandidateSet,
    pub(super) scoped_merge_proof: ScopedMergeProofPacket,
    pub(super) source_nodes: Vec<crate::data::handle::NodeId>,
    pub(super) planned_candidates: PlannedMergeCandidateSet,
    pub(super) node_map: MergeNodeMap,
    pub(super) conservative_overlap_nodes: BTreeSet<crate::data::handle::NodeId>,
    pub(super) conservative_support_nodes: BTreeSet<crate::data::handle::NodeId>,
}

pub(super) fn discover_branch_states<D, I, E, Ctx, T>(
    runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    request: &crate::logic::transaction::runtime::BranchMergeRequest,
) -> Result<BranchStateDiscovery<D, I, T>, SignalError>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    let source_state = if request.source_branch.id == runtime.graph.current_branch().id {
        runtime.capture_heavy_branch_state()?
    } else {
        let state = runtime
            .branches
            .branch_state(request.source_branch.id)
            .ok_or_else(|| {
                SignalError::unknown_branch(
                    Some(request.source_branch.id),
                    request.source_branch.name.clone(),
                )
            })?;
        SignalRuntime::<D, I, E, Ctx, T>::ensure_managed_queue_branch_transfer_allowed(
            state.resource(),
        )?;
        state.clone()
    };
    let target_state_owned = if request.target_branch.id == runtime.graph.current_branch().id {
        Some(runtime.capture_heavy_branch_state()?)
    } else {
        None
    };
    let target_state = target_state_owned
        .as_ref()
        .or_else(|| runtime.branches.branch_state(request.target_branch.id))
        .ok_or_else(|| {
            SignalError::unknown_branch(
                Some(request.target_branch.id),
                request.target_branch.name.clone(),
            )
        })?;
    SignalRuntime::<D, I, E, Ctx, T>::ensure_managed_queue_branch_transfer_allowed(
        target_state.resource(),
    )?;

    Ok(BranchStateDiscovery {
        source_snapshot_id: source_state
            .graph()
            .branch_head_snapshot_id(request.source_branch.id),
        target_snapshot_id_before: target_state
            .graph()
            .branch_head_snapshot_id(request.target_branch.id),
        source_state,
        target_state_owned,
        source_branch_id: request.source_branch.id,
        target_branch_id: request.target_branch.id,
    })
}

pub(super) fn lower_scoped_candidates<D, I, E, Ctx, T>(
    runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    request: &LoweredFoundationalMergeRequest,
    states: BranchStateDiscovery<D, I, T>,
    merge_base: &MergeBaseResolution,
) -> Result<CandidateDiscovery<D, I, T>, SignalError>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime.with_telemetry(|telemetry| telemetry.transaction.scoped_candidate_lowering_count += 1);
    let source_graph = states.source_state.graph();
    let target_state = states
        .target_state_owned
        .as_ref()
        .or_else(|| runtime.branches.branch_state(states.target_branch_id))
        .ok_or_else(|| SignalError::invalid_input("merge target branch state disappeared"))?;
    let target_graph = target_state.graph();
    if !states.source_state.mutation_ledger().boundary_established {
        runtime.with_telemetry(|telemetry| {
            telemetry
                .transaction
                .scoped_candidate_broad_scan_denial_count += 1
        });
        return Err(SignalError::branch_merge_failed(
            BranchMergeFailureKind::UnsupportedMergeStrategy,
            "branch merge requires an established mutation-journal boundary; whole-live branch scans are no longer admitted",
        ));
    }

    let boundary_witness = MergeBoundaryWitness {
        source_branch_id: states.source_branch_id,
        target_branch_id: states.target_branch_id,
        kind: MergeBoundaryWitnessKind::MutationJournalBoundary,
        forked_from_snapshot_id: merge_base.base.forked_from_snapshot_id,
        source_snapshot_id: states.source_snapshot_id,
        target_snapshot_id_before: states.target_snapshot_id_before,
    };
    let source_journal = StructuralMergeJournalSlice::from_branch_journal(
        boundary_witness.clone(),
        states
            .source_state
            .mutation_ledger()
            .structural_merge_journal(),
    );
    let target_identity_journal = target_state.mutation_ledger().structural_merge_journal();

    let mut scoped_candidates =
        LoweredScopedMergeCandidateSet::lower(request, &source_journal, source_graph)?;
    let candidate_scope_family = scoped_candidates.scope_family();
    let breadth = scoped_candidates.breadth_summary().clone();

    let source_nodes = scoped_candidates.admitted_candidate_nodes().to_vec();
    let planned_candidates = scoped_candidates.planned_candidates().clone();
    let mut conservative_overlap_nodes: BTreeSet<crate::data::handle::NodeId> =
        planned_candidates.nodes.iter().copied().collect();
    let mut conservative_support_nodes = BTreeSet::new();
    let mut node_map = MergeNodeMap::default();
    for source_node in &source_nodes {
        for dependency in source_graph.dependencies_of(*source_node)? {
            if target_graph.is_alive(dependency.source()) {
                node_map.insert(dependency.source(), dependency.source());
                if !planned_candidates.nodes.contains(&dependency.source()) {
                    conservative_support_nodes.insert(dependency.source());
                }
                conservative_overlap_nodes.insert(dependency.source());
            }
        }
        for snapshot_entry in source_graph.get_dep_snapshot(*source_node)?.entries() {
            if target_graph.is_alive(snapshot_entry.source) {
                node_map.insert(snapshot_entry.source, snapshot_entry.source);
                if !planned_candidates.nodes.contains(&snapshot_entry.source) {
                    conservative_support_nodes.insert(snapshot_entry.source);
                }
                conservative_overlap_nodes.insert(snapshot_entry.source);
            }
        }
    }
    scoped_candidates =
        scoped_candidates.with_support_closure_nodes(conservative_support_nodes.iter().copied());
    let support_closure_width = scoped_candidates.breadth_summary().support_closure_width;
    let admission = classify_initial_scoped_merge_admission(
        request,
        &scoped_candidates,
        source_graph,
        request.normalized_request().request().strategy_hint,
    );
    runtime.with_telemetry(|telemetry| {
        match candidate_scope_family {
            BranchMergeRequestScopeFamily::FullBranch => {
                telemetry.transaction.scoped_candidate_full_branch_count += 1
            }
            BranchMergeRequestScopeFamily::SelectedNodes => {
                telemetry.transaction.scoped_candidate_selected_node_count += 1
            }
            BranchMergeRequestScopeFamily::SelectedAspects => {
                telemetry.transaction.scoped_candidate_selected_aspect_count += 1
            }
        }
        telemetry
            .transaction
            .scoped_candidate_requested_scope_breadth += breadth.requested_scope_width;
        telemetry.transaction.scoped_candidate_admitted_breadth += breadth.admitted_candidate_width;
        telemetry.transaction.scoped_candidate_skipped_breadth += breadth.skipped_scope_width;
        telemetry.transaction.scoped_candidate_no_op_breadth += breadth.no_op_scope_width;
        telemetry
            .transaction
            .scoped_candidate_support_closure_breadth += support_closure_width;
        telemetry.transaction.scoped_merge_admission_count += 1;
    });
    let scoped_candidates = match admission {
        worth_proof::TransitionOutcome::Success(ready) => ready.scoped_candidates().clone(),
        outcome => {
            runtime.with_telemetry(|telemetry| {
                telemetry.transaction.scoped_merge_denial_count +=
                    u64::from(matches!(outcome, worth_proof::TransitionOutcome::Denied(_)));
                telemetry.transaction.scoped_merge_unavailable_count += u64::from(!matches!(
                    outcome,
                    worth_proof::TransitionOutcome::Success(_)
                        | worth_proof::TransitionOutcome::Denied(_)
                ));
            });
            return Err(scoped_admission_outcome_to_signal_error(outcome));
        }
    };
    let scoped_merge_proof =
        ScopedMergeProofPacket::from_request_and_candidates(request, &scoped_candidates);

    Ok(CandidateDiscovery {
        branch_states: states,
        boundary_witness,
        source_journal,
        target_identity_journal,
        scoped_candidates,
        scoped_merge_proof,
        source_nodes,
        planned_candidates,
        node_map,
        conservative_overlap_nodes,
        conservative_support_nodes,
    })
}
