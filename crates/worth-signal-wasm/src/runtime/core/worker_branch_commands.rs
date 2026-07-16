use std::sync::{Arc, Mutex};

use crate::boundary::errors::WORTHSignalJsError;
use crate::recipe::model::TransactionOp;
use crate::runtime::summaries::RunSummary;
use worth_proof::TransitionOutcome;
use worth_signal::facade::history::{
    RuntimeBranch, RuntimeBranchId, SignalBranchForkRequest, SignalBranchRetirementReason,
    SignalBranchRetirementRequest,
};
use worth_signal::facade::{
    BranchTargetedTransactionRequest, SignalBranchTransactionHead, SignalError,
};

use super::certification_digest::canonical_certification_digest;
use super::state::{BranchRuntimeState, PendingCallbackDependencyPatch};
use super::transactions::apply::{
    apply_pending_dependency_patches_in_transaction, apply_set_changes,
};
use super::worker_branch_command_model::{
    WorkerApplyTransactionToBranchReceipt, WorkerApplyTransactionToBranchRequest,
    WorkerBranchBasisReceipt, WorkerBranchRetirementReason, WorkerForkBranchReceipt,
    WorkerForkBranchRequest, WorkerRetireBranchReceipt, WorkerRetireBranchRequest,
};
use super::RuntimeCore;

impl RuntimeCore {
    pub fn worker_branch_basis(
        &self,
        branch_id: u64,
    ) -> Result<WorkerBranchBasisReceipt, WORTHSignalJsError> {
        let branch = self
            .runtime
            .branch_handle(RuntimeBranchId(branch_id))
            .ok_or_else(|| unknown_branch(branch_id))?;
        let native_head = expect_success(
            self.runtime.branch_transaction_head(branch.clone()),
            "read worker branch head",
        )?;
        let state = self.state_for_branch(branch_id);
        let native_state_digest = self.branch_state_proof(branch_id)?.state_digest;
        let authored_state_digest = canonical_certification_digest(&(
            native_state_digest,
            &state.store,
            state.authored_graph_generation,
        ))?;
        Ok(worker_basis(
            branch,
            native_head,
            state,
            authored_state_digest,
        ))
    }

    pub fn fork_worker_branch(
        &mut self,
        request: WorkerForkBranchRequest,
    ) -> Result<WorkerForkBranchReceipt, WORTHSignalJsError> {
        let parent_basis = self.worker_branch_basis(request.parent_branch_id)?;
        require_basis(&request.expected_parent_basis, &parent_basis, "forkBranch")?;
        let parent_state = self.state_for_branch(request.parent_branch_id);
        let receipt = expect_success(
            self.runtime
                .fork_branch(SignalBranchForkRequest::from_parent_branch_head(
                    request.name,
                    RuntimeBranchId(request.parent_branch_id),
                )),
            "fork worker branch",
        )?;
        let branch = receipt.created_branch().clone();
        self.branch_states.insert(branch.id.0, parent_state);
        let created_basis = self.worker_branch_basis(branch.id.0)?;
        Ok(WorkerForkBranchReceipt {
            branch,
            parent_basis,
            created_basis,
        })
    }

    pub fn apply_transaction_to_worker_branch(
        &mut self,
        request: WorkerApplyTransactionToBranchRequest,
    ) -> Result<WorkerApplyTransactionToBranchReceipt, WORTHSignalJsError> {
        let before_basis = self.worker_branch_basis(request.branch_id)?;
        require_basis(
            &request.expected_basis,
            &before_basis,
            "applyTransactionToBranch",
        )?;
        let branch = self
            .runtime
            .branch_handle(RuntimeBranchId(request.branch_id))
            .ok_or_else(|| unknown_branch(request.branch_id))?;
        let native_head = expect_success(
            self.runtime.branch_transaction_head(branch.clone()),
            "read targeted worker branch head",
        )?;
        let plan =
            expect_success(
                self.runtime.plan_branch_targeted_transaction(
                    BranchTargetedTransactionRequest::new(branch, native_head),
                ),
                "plan targeted worker transaction",
            )?;

        let active_branch_id_before = self.runtime.current_branch().id.0;
        let active_state = self.snapshot_branch_state();
        let target_state = self
            .branch_states
            .get(&request.branch_id)
            .cloned()
            .ok_or_else(|| unknown_branch(request.branch_id))?;
        self.install_companion_state(&target_state)?;
        if let Err(error) = self.validate_targeted_transaction_shape(&request.transaction_ops) {
            self.install_companion_state(&active_state)?;
            return Err(error);
        }
        let changes = match self.collect_changes(&request.transaction_ops) {
            Ok(changes) => changes,
            Err(error) => {
                self.install_companion_state(&active_state)?;
                return Err(error);
            }
        };
        let store = self.store.clone();
        let dense_grids = self.dense_grids.clone();
        let evaluator = self.evaluator();
        let dependency_patches = Arc::new(Mutex::new(
            None::<(Vec<PendingCallbackDependencyPatch>, u64)>,
        ));
        let dependency_patches_for_tx = dependency_patches.clone();

        let outcome =
            self.runtime
                .execute_branch_targeted_transaction(&mut self.store, plan, move |tx| {
                    apply_set_changes(tx, &store, &dense_grids, &changes)?;
                    tx.evaluate_dirty(&evaluator)?;
                    let patches = apply_pending_dependency_patches_in_transaction(tx, &store)?;
                    *dependency_patches_for_tx.lock().map_err(|_| {
                        SignalError::internal("dependency patch receipt mutex poisoned")
                    })? = Some(patches);
                    Ok(())
                });

        let executed = match outcome {
            TransitionOutcome::Success(receipt) => receipt,
            other => {
                self.install_companion_state(&active_state)?;
                return Err(outcome_error("execute targeted worker transaction", other));
            }
        };
        let (pending, runtime_read_breadth) = dependency_patches
            .lock()
            .map_err(|_| WORTHSignalJsError::internal("dependency patch receipt mutex poisoned"))?
            .take()
            .unwrap_or_default();
        self.record_committed_callback_dependency_patches(pending, runtime_read_breadth)?;
        let committed_target_state = BranchRuntimeState {
            metadata: self.snapshot_branch_metadata(),
            store: self.lock_store()?.snapshot(&self.catalog),
            authored_graph_generation: target_state.authored_graph_generation.saturating_add(1),
        };
        self.branch_states
            .insert(request.branch_id, committed_target_state);
        self.install_companion_state(&active_state)?;
        let active_branch_id_after = self.runtime.current_branch().id.0;
        let after_basis = self.worker_branch_basis(request.branch_id)?;
        Ok(WorkerApplyTransactionToBranchReceipt {
            before_basis,
            after_basis,
            active_branch_id_before,
            active_branch_id_after,
            run_summary: run_summary(executed.transaction()),
        })
    }

    pub fn retire_worker_branch(
        &mut self,
        request: WorkerRetireBranchRequest,
    ) -> Result<WorkerRetireBranchReceipt, WORTHSignalJsError> {
        let terminal_basis = self.worker_branch_basis(request.branch_id)?;
        require_basis(&request.expected_basis, &terminal_basis, "retireBranch")?;
        let branch = self
            .runtime
            .branch_handle(RuntimeBranchId(request.branch_id))
            .ok_or_else(|| unknown_branch(request.branch_id))?;
        let native_head = expect_success(
            self.runtime.branch_transaction_head(branch.clone()),
            "read retiring worker branch head",
        )?;
        let plan = expect_success(
            self.runtime
                .plan_branch_retirement(SignalBranchRetirementRequest::new(
                    branch,
                    native_head,
                    request.reason.into(),
                )),
            "plan worker branch retirement",
        )?;
        let receipt = expect_success(self.runtime.retire_branch(plan), "retire worker branch")?;
        self.branch_states.remove(&request.branch_id);
        self.snapshot_states
            .retain(|(branch_id, _), _| *branch_id != request.branch_id);
        self.runtime_snapshots
            .retain(|(branch_id, _), _| *branch_id != request.branch_id);
        Ok(WorkerRetireBranchReceipt {
            retired_branch_id: request.branch_id,
            parent_branch_id: receipt.parent_branch_id().0,
            terminal_basis,
            closeout_digest: receipt.closeout_digest().to_owned(),
            reclaimed_branch_state_count: receipt.reclaimed_branch_state_count(),
            reclaimed_snapshot_state_count: receipt.reclaimed_snapshot_state_count(),
            reclaimed_runtime_meta_count: receipt.reclaimed_runtime_meta_count(),
            retained_proof_record_count: receipt.retained_proof_record_count(),
        })
    }

    fn install_companion_state(
        &mut self,
        state: &BranchRuntimeState,
    ) -> Result<(), WORTHSignalJsError> {
        self.ensure_callback_snapshot_availability(&state.store)?;
        self.restore_branch_metadata(state.metadata.clone());
        self.lock_store()?.restore_snapshot(state.store.clone());
        self.sync_callback_diagnostics_from_store()
    }

    fn validate_targeted_transaction_shape(
        &self,
        ops: &[TransactionOp],
    ) -> Result<(), WORTHSignalJsError> {
        for op in ops {
            match op {
                TransactionOp::Set { id, .. } | TransactionOp::SetWithRegions { id, .. } => {
                    if !self.catalog.contains_key(id) {
                        return Err(unknown_target_signal(id));
                    }
                }
                TransactionOp::SetMany { values } => {
                    for value in values {
                        if !self.catalog.contains_key(&value.id) {
                            return Err(unknown_target_signal(&value.id));
                        }
                    }
                }
                TransactionOp::SetManyWithRegions { values } => {
                    for value in values {
                        if !self.catalog.contains_key(&value.id) {
                            return Err(unknown_target_signal(&value.id));
                        }
                    }
                }
                TransactionOp::SetManyKeyed { family_id, .. } => {
                    return Err(WORTHSignalJsError::invalid_input(format!(
                        "branch-targeted transaction cannot materialize keyed family `{family_id}`; publish authored keys before forking"
                    )));
                }
                TransactionOp::SetPackedGridRgba {
                    family_id,
                    width,
                    height,
                    ..
                } => {
                    let Some(family) = self.dense_grids.get(family_id) else {
                        return Err(WORTHSignalJsError::invalid_input(format!(
                            "branch-targeted transaction references unknown dense family `{family_id}`"
                        )));
                    };
                    if family.width != *width || family.height != *height {
                        return Err(WORTHSignalJsError::invalid_input(format!(
                            "branch-targeted dense family `{family_id}` shape does not match its authored graph"
                        )));
                    }
                }
            }
        }
        Ok(())
    }
}

impl From<WorkerBranchRetirementReason> for SignalBranchRetirementReason {
    fn from(reason: WorkerBranchRetirementReason) -> Self {
        match reason {
            WorkerBranchRetirementReason::Rejected => Self::Rejected,
            WorkerBranchRetirementReason::Merged => Self::Merged,
            WorkerBranchRetirementReason::Superseded => Self::Superseded,
            WorkerBranchRetirementReason::DependencyCancellation => Self::DependencyCancellation,
            WorkerBranchRetirementReason::ProjectionRebuild => Self::ProjectionRebuild,
        }
    }
}

fn worker_basis(
    branch: RuntimeBranch,
    native_head: SignalBranchTransactionHead,
    state: BranchRuntimeState,
    authored_state_digest: String,
) -> WorkerBranchBasisReceipt {
    WorkerBranchBasisReceipt {
        branch_id: branch.id.0,
        branch_name: branch.name,
        snapshot_id: native_head.snapshot_id().map(|id| id.0),
        native_head_generation: native_head.generation(),
        native_head_digest: native_head.head_digest().to_owned(),
        authored_graph_generation: state.authored_graph_generation,
        authored_state_digest,
    }
}

pub(super) fn require_basis(
    expected: &WorkerBranchBasisReceipt,
    observed: &WorkerBranchBasisReceipt,
    operation: &str,
) -> Result<(), WORTHSignalJsError> {
    if expected == observed {
        return Ok(());
    }
    Err(WORTHSignalJsError::invalid_input(format!(
        "{operation} denied a stale worker branch basis: expected generation {}/{}, observed {}/{}",
        expected.native_head_generation,
        expected.authored_graph_generation,
        observed.native_head_generation,
        observed.authored_graph_generation,
    )))
}

pub(super) fn expect_success<T: std::fmt::Debug, D: std::fmt::Debug>(
    outcome: TransitionOutcome<T, D>,
    operation: &str,
) -> Result<T, WORTHSignalJsError> {
    match outcome {
        TransitionOutcome::Success(value) => Ok(value),
        other => Err(outcome_error(operation, other)),
    }
}

fn outcome_error<
    T: std::fmt::Debug,
    D: std::fmt::Debug,
    De: std::fmt::Debug,
    S: std::fmt::Debug,
    R: std::fmt::Debug,
    F: std::fmt::Debug,
>(
    operation: &str,
    outcome: TransitionOutcome<T, D, De, S, R, F>,
) -> WORTHSignalJsError {
    WORTHSignalJsError::invalid_input(format!("{operation} denied: {outcome:?}"))
}

fn unknown_branch(branch_id: u64) -> WORTHSignalJsError {
    WORTHSignalJsError::invalid_input(format!("unknown worker branch `{branch_id}`"))
}

fn unknown_target_signal(id: &str) -> WORTHSignalJsError {
    WORTHSignalJsError::invalid_input(format!(
        "branch-targeted transaction references unknown authored signal `{id}`"
    ))
}

fn run_summary(result: &worth_signal::facade::TransactionResult) -> RunSummary {
    RunSummary {
        touched_nodes: result.touched_nodes,
        nodes_evaluated: result.evaluation_summary.nodes_evaluated,
        nodes_recomputed: result.evaluation_summary.nodes_recomputed,
        nodes_suppressed: result.evaluation_summary.nodes_suppressed,
        plans_built: result.evaluation_summary.plans_built,
        stages_executed: result.evaluation_summary.stages_executed,
        total_nanos: result.timing.total_nanos.to_string(),
        evaluation_nanos: result.timing.evaluation_nanos.to_string(),
        commit_nanos: result.timing.commit_nanos.to_string(),
    }
}
