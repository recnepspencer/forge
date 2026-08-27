use crate::data::error::SignalError;
use crate::logic::transaction::runtime::{
    BranchMergeFailureKind, BranchMergePlan, SignalMergeCompatibilityWitness,
};

use super::super::super::runtime_state::{
    AuthorityTransferPacket, BranchLifecycleTransfer, SignalRuntime,
};
use super::execution_artifacts::ArtifactFinalization;
use super::execution_preparation::PreparedMergeExecution;

pub(super) struct BranchFinalization {
    pub(super) compatibility_witness: SignalMergeCompatibilityWitness,
    pub(super) target_snapshot_after: Option<crate::state::SignalSnapshotId>,
    pub(super) records: Vec<crate::logic::transaction::runtime::MergedArtifactRecord>,
    pub(super) touched_set: crate::logic::transaction::runtime::MergeTouchedNodeSet,
    pub(super) node_map: crate::logic::transaction::runtime::MergeNodeMap,
    pub(super) dependency_remaps: Vec<crate::logic::transaction::runtime::DependencyRemapRecord>,
    pub(super) subscriber_repair_breadth: u64,
}

pub(super) fn finalize_branch_state<D, I, E, Ctx, T>(
    runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    request: &crate::logic::transaction::runtime::BranchMergeRequest,
    plan: &BranchMergePlan,
    mut prepared: PreparedMergeExecution<D, I, T>,
    artifacts: ArtifactFinalization<D, I, T>,
) -> Result<BranchFinalization, SignalError>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    let super::execution_artifacts::ArtifactFinalization {
        target_snapshot_after,
        records,
        touched_set,
        merged_source_nodes,
        target_snapshot_packet,
        node_map,
        dependency_remaps,
        subscriber_repair_breadth,
    } = artifacts;
    if let Some(snapshot_id) = target_snapshot_after {
        runtime
            .branches
            .set_branch_head_snapshot(request.target_branch.id, snapshot_id);
        runtime
            .branches
            .project_catalog(request.target_branch.id, prepared.target_state.graph_mut());
    }
    let target_branch_is_current = request.target_branch.id == runtime.graph.current_branch().id;
    if target_branch_is_current {
        runtime.apply_branch_lifecycle_transfer(BranchLifecycleTransfer::Move(
            AuthorityTransferPacket::new(request.target_branch.id, prepared.target_state),
        ))?;
    } else {
        runtime.branches.store_branch_state(prepared.target_state);
    }

    if request.source_branch.id == runtime.graph.current_branch().id {
        let mut updated_source_state = runtime.capture_heavy_branch_state()?;
        updated_source_state
            .mutation_ledger_mut()
            .clear_merged_nodes(
                merged_source_nodes.iter().copied(),
                plan.source_snapshot_id(),
            );
        updated_source_state.clear_branch_mutation_nodes();
        runtime.apply_branch_lifecycle_transfer(BranchLifecycleTransfer::Move(
            AuthorityTransferPacket::new(request.source_branch.id, updated_source_state),
        ))?;
    } else if let Some(()) =
        runtime
            .branches
            .with_stored_branch_state_mut(request.source_branch.id, |source_state| {
                source_state.mutation_ledger_mut().clear_merged_nodes(
                    merged_source_nodes.iter().copied(),
                    plan.source_snapshot_id(),
                );
                source_state.clear_branch_mutation_nodes();
            })
    {
    }
    runtime.branches.insert_snapshot(target_snapshot_packet);

    runtime.project_branch_catalog();

    let target_branch_handle = request.target_branch.clone();
    let branch_basis = match runtime.branch_basis_artifact(target_branch_handle.clone()) {
        worth_proof::TransitionOutcome::Success(artifact) => artifact,
        outcome => {
            return Err(SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "post-merge compatibility witness requires a valid target branch basis artifact, got {outcome:?}"
                ),
            ))
        }
    };
    let compatibility_witness = match runtime.planned_merge_compatibility_artifact(
        branch_basis,
        target_branch_handle,
        plan,
    ) {
        worth_proof::TransitionOutcome::Success(artifact) => artifact.payload().clone(),
        outcome => {
            return Err(SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "post-merge compatibility witness could not be retained from admitted merge proof, got {outcome:?}"
                ),
            ))
        }
    };
    Ok(BranchFinalization {
        compatibility_witness,
        target_snapshot_after,
        records,
        touched_set,
        node_map,
        dependency_remaps,
        subscriber_repair_breadth,
    })
}
