use std::collections::BTreeSet;

use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::logic::transaction::runtime::{BranchMergeRequest, DependencyRemapRecord, MergeNodeMap};

use super::super::super::runtime_state::SignalRuntime;
use super::super::branches::BranchState;

pub(super) struct PreparedMergeExecution<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) source_state: BranchState<D, I, T>,
    pub(super) target_state: BranchState<D, I, T>,
    pub(super) node_map: MergeNodeMap,
    pub(super) dependency_remaps: Vec<DependencyRemapRecord>,
    pub(super) touched: BTreeSet<NodeId>,
    pub(super) repaired_sources: BTreeSet<NodeId>,
}

pub(super) fn prepare<D, I, E, Ctx, T>(
    runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    request: &BranchMergeRequest,
    plan: &crate::logic::transaction::runtime::BranchMergePlan,
) -> Result<PreparedMergeExecution<D, I, T>, SignalError>
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
    let target_state = if request.target_branch.id == runtime.graph.current_branch().id {
        runtime.capture_heavy_branch_state()?
    } else {
        let state = runtime
            .branches
            .branch_state(request.target_branch.id)
            .ok_or_else(|| {
                SignalError::unknown_branch(
                    Some(request.target_branch.id),
                    request.target_branch.name.clone(),
                )
            })?;
        SignalRuntime::<D, I, E, Ctx, T>::ensure_managed_queue_branch_transfer_allowed(
            state.resource(),
        )?;
        state.clone()
    };
    Ok(PreparedMergeExecution {
        source_state,
        target_state,
        node_map: plan.node_map().clone(),
        dependency_remaps: Vec::new(),
        touched: BTreeSet::new(),
        repaired_sources: BTreeSet::new(),
    })
}
