mod capture;
mod validation;

use std::collections::BTreeMap;

use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::diagnostics::policy::OrdinaryAccessLane;
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::state::{
    SignalBranchHandle, SignalBranchId, SignalSnapshotV1, SnapshotArtifactRestoreMode,
    SnapshotDependencyRestoreMode, SnapshotRestoreIntent,
};

use super::super::runtime_state::SignalRuntime;
use super::branches::SnapshotBranchState;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn restore_snapshot(&mut self, snapshot: &SignalSnapshotV1) -> Result<(), SignalError> {
        self.restore_snapshot_with_intent(snapshot, SnapshotRestoreIntent::restore_runtime_truth())
    }

    pub fn restore_snapshot_with_intent(
        &mut self,
        snapshot: &SignalSnapshotV1,
        intent: SnapshotRestoreIntent,
    ) -> Result<(), SignalError> {
        let reconstructability_proof = snapshot.reconstructability_proof()?;
        let restore_plan = self.graph.plan_snapshot_restore(snapshot, intent)?;
        self.graph.validate_snapshot_compatibility(snapshot)?;
        if matches!(
            intent.dependency_state,
            SnapshotDependencyRestoreMode::SeedRecomputationFromSnapshot
        ) {
            return Err(SignalError::invalid_input(
                "snapshot restore intent `SeedRecomputationFromSnapshot` is not implemented yet",
            ));
        }
        let snapshot_state = self
            .branches
            .snapshot_state(snapshot.meta.branch_id, snapshot.meta.snapshot_id)
            .cloned();
        Self::ensure_managed_queue_branch_transfer_allowed(&self.resource)?;
        if let Some(snapshot_state) = snapshot_state.as_ref() {
            Self::ensure_managed_queue_branch_transfer_allowed(snapshot_state.resource())?;
        }
        if let Some(snapshot_state) = snapshot_state {
            let current_diagnostics = self.graph.diagnostics_state().clone();
            let current_policy = current_diagnostics.policy();
            return self.restore_stored_snapshot_branch(
                snapshot,
                intent,
                &reconstructability_proof,
                &restore_plan,
                snapshot_state,
                &current_diagnostics,
                current_policy,
            );
        }

        self.graph.restore_snapshot_with_intent(snapshot, intent)?;
        if let Some(telemetry) = &snapshot.runtime_telemetry {
            self.telemetry = *telemetry;
        }
        let branch_catalog = self.graph.diagnostics_state().branch_catalog().clone();
        self.synchronize_branch_catalogs(branch_catalog);
        Ok(())
    }

    pub fn restore_branch_snapshot(
        &mut self,
        branch: SignalBranchHandle,
        snapshot: &SignalSnapshotV1,
    ) -> Result<(), SignalError> {
        self.restore_branch_snapshot_with_intent(
            branch,
            snapshot,
            SnapshotRestoreIntent::restore_runtime_truth(),
        )
    }

    pub fn restore_branch_snapshot_with_intent(
        &mut self,
        branch: SignalBranchHandle,
        snapshot: &SignalSnapshotV1,
        intent: SnapshotRestoreIntent,
    ) -> Result<(), SignalError> {
        let reconstructability_proof = snapshot.reconstructability_proof()?;
        let restore_plan = self.graph.plan_snapshot_restore(snapshot, intent)?;
        self.graph.validate_snapshot_compatibility(snapshot)?;
        if snapshot.meta.branch_id != branch.id {
            return Err(SignalError::incompatible_snapshot(format!(
                "snapshot `{}` from branch `{}` cannot be restored into branch `{}`",
                snapshot.meta.snapshot_id.0, snapshot.meta.branch_name, branch.name
            )));
        }
        if branch.id == self.graph.current_branch().id {
            return self.restore_snapshot_with_intent(snapshot, intent);
        }
        if matches!(
            intent.dependency_state,
            SnapshotDependencyRestoreMode::SeedRecomputationFromSnapshot
        ) {
            return Err(SignalError::invalid_input(
                "snapshot restore intent `SeedRecomputationFromSnapshot` is not implemented yet",
            ));
        }
        let (snapshot_state, current_diagnostics) =
            self.load_noncurrent_restore_inputs(&branch, snapshot)?;
        let current_policy = current_diagnostics.policy();
        let mut graph = self.restore_snapshot_graph(
            snapshot,
            &reconstructability_proof,
            &restore_plan,
            &current_diagnostics,
            current_policy,
            intent,
        )?;
        graph.diagnostics_state_mut().set_active_branch(branch.id);
        graph
            .diagnostics_state_mut()
            .set_branch_head_snapshot(branch.id, snapshot.meta.snapshot_id);
        let (state, branch_catalog) =
            Self::finalize_restored_branch_state(snapshot_state, graph, snapshot);
        self.record_snapshot_restore_telemetry(intent, &restore_plan);
        self.branches.store_branch_state(state);
        self.synchronize_branch_catalogs(branch_catalog);
        Ok(())
    }

    fn load_noncurrent_restore_inputs(
        &self,
        branch: &SignalBranchHandle,
        snapshot: &SignalSnapshotV1,
    ) -> Result<
        (
            SnapshotBranchState<D, I, T>,
            crate::diagnostics::state::DiagnosticsState,
        ),
        SignalError,
    > {
        let snapshot_state = self
            .branches
            .snapshot_state(snapshot.meta.branch_id, snapshot.meta.snapshot_id)
            .cloned()
            .ok_or_else(|| {
                SignalError::internal(format!(
                    "snapshot `{}:{}` is missing runtime-local branch semantic state",
                    snapshot.meta.branch_id.0, snapshot.meta.snapshot_id.0
                ))
            })?;
        let target_state = self
            .branches
            .branch_state(branch.id)
            .ok_or_else(|| SignalError::unknown_branch(Some(branch.id), branch.name.clone()))?;
        Self::ensure_managed_queue_branch_transfer_allowed(target_state.resource())?;
        Self::ensure_managed_queue_branch_transfer_allowed(snapshot_state.resource())?;
        Ok((
            snapshot_state,
            target_state.graph().diagnostics_state().clone(),
        ))
    }

    fn restore_stored_snapshot_branch(
        &mut self,
        snapshot: &SignalSnapshotV1,
        intent: SnapshotRestoreIntent,
        reconstructability_proof: &crate::logic::transaction::ReconstructabilityProof,
        restore_plan: &crate::state::SnapshotRestorePlan,
        snapshot_state: SnapshotBranchState<D, I, T>,
        current_diagnostics: &crate::diagnostics::state::DiagnosticsState,
        current_policy: crate::diagnostics::policy::SignalRuntimePolicy,
    ) -> Result<(), SignalError> {
        let mut graph = self.restore_snapshot_graph(
            snapshot,
            reconstructability_proof,
            restore_plan,
            current_diagnostics,
            current_policy,
            intent,
        )?;
        graph
            .diagnostics_state_mut()
            .set_active_branch(snapshot.meta.branch_id);
        graph
            .diagnostics_state_mut()
            .set_branch_head_snapshot(snapshot.meta.branch_id, snapshot.meta.snapshot_id);
        let (state, branch_catalog) =
            Self::finalize_restored_branch_state(snapshot_state, graph, snapshot);
        let preserved_transaction = self.telemetry.transaction;
        self.apply_branch_lifecycle_transfer(
            crate::logic::transaction::runtime::state::runtime_state::BranchLifecycleTransfer::Restore(
                crate::logic::transaction::runtime::state::runtime_state::RestoreTransferPacket::new(
                    snapshot.meta.branch_id,
                    state,
                ),
            ),
        )?;
        Self::merge_global_transaction_telemetry(
            preserved_transaction,
            &mut self.telemetry.transaction,
        );
        self.record_snapshot_restore_telemetry(intent, restore_plan);
        self.synchronize_branch_catalogs(branch_catalog);
        Ok(())
    }

    fn record_snapshot_restore_telemetry(
        &mut self,
        intent: SnapshotRestoreIntent,
        restore_plan: &crate::state::SnapshotRestorePlan,
    ) {
        self.telemetry.checkpoint.snapshot_restore_count += 1;
        if matches!(
            intent.artifacts,
            SnapshotArtifactRestoreMode::ApplyActiveRuntimePolicy
        ) {
            self.telemetry
                .checkpoint
                .snapshot_restore_apply_active_policy_count += 1;
        }
        self.telemetry
            .checkpoint
            .snapshot_restore_shared_delta_node_count +=
            restore_plan.dependency_snapshot_delta_node_count();
        self.telemetry
            .checkpoint
            .snapshot_restore_coarse_reason_count += restore_plan.coarse_reasons().len() as u64;
    }

    fn restore_snapshot_graph(
        &self,
        snapshot: &SignalSnapshotV1,
        reconstructability_proof: &crate::logic::transaction::ReconstructabilityProof,
        restore_plan: &crate::state::SnapshotRestorePlan,
        current_diagnostics: &crate::diagnostics::state::DiagnosticsState,
        current_policy: crate::diagnostics::policy::SignalRuntimePolicy,
        intent: SnapshotRestoreIntent,
    ) -> Result<SignalGraph, SignalError> {
        let mut graph =
            self.restore_runtime_authority_from_snapshot_proof(snapshot, reconstructability_proof)?;
        *graph.telemetry_mut() = snapshot.checkpoint_image.graph_telemetry;
        Self::rebuild_runtime_required_derived_from_proof(
            &mut graph,
            snapshot,
            reconstructability_proof,
            restore_plan,
        )?;
        Self::apply_runtime_diagnostic_policy_richness(
            &mut graph,
            snapshot,
            current_diagnostics,
            current_policy,
            intent,
        );
        graph.telemetry_mut().checkpoint.snapshot_restore_count += 1;
        if matches!(
            intent.artifacts,
            SnapshotArtifactRestoreMode::ApplyActiveRuntimePolicy
        ) {
            graph
                .telemetry_mut()
                .checkpoint
                .snapshot_restore_apply_active_policy_count += 1;
        }
        Ok(graph)
    }

    fn finalize_restored_branch_state(
        snapshot_state: SnapshotBranchState<D, I, T>,
        graph: SignalGraph,
        snapshot: &SignalSnapshotV1,
    ) -> (
        super::branches::BranchState<D, I, T>,
        BTreeMap<SignalBranchId, SignalBranchHandle>,
    ) {
        let mut state = snapshot_state.into_branch_state(graph, snapshot.runtime_telemetry);
        crate::diagnostics::recorder::record_snapshot_restore_lineage(
            state.graph_mut(),
            snapshot.meta.snapshot_id,
        );
        let retention_budget = state.graph().runtime_policy().retention_budget;
        let profile = state.graph().diagnostics_profile();
        let history = ExecutionHistorySummary::from_graph(
            state.graph(),
            profile,
            retention_budget.detail_limit,
            retention_budget.retain_history_details,
            OrdinaryAccessLane,
        );
        let graph_summary = GraphSummary::from_graph(
            state.graph(),
            profile,
            retention_budget.detail_limit,
            OrdinaryAccessLane,
        );
        state
            .graph_mut()
            .diagnostics_state_mut()
            .refresh_retained_views(history, graph_summary);
        let branch_catalog = state.graph().diagnostics_state().branch_catalog().clone();
        (state, branch_catalog)
    }
}
