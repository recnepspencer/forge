use worth_signal::facade::adapters::BranchStateProofReport;
use worth_signal::facade::history::{RuntimeBranch, RuntimeSnapshot};

use crate::boundary::errors::WORTHSignalJsError;
use crate::recipe::model::TransactionOp;
use crate::runtime::summaries::{ReplaySummary, RuntimeSnapshotEnvelope};

use super::{WorkerBranchTruthEnvelope, WorkerCommittedTransactionEnvelope, WorkerRuntimeShell};

impl WorkerRuntimeShell {
    pub fn apply_committed_transaction(
        &mut self,
        ops: Vec<TransactionOp>,
    ) -> Result<WorkerCommittedTransactionEnvelope, WORTHSignalJsError> {
        let run_summary = self.core.apply_transaction(ops)?;
        let branch = self.core.current_branch();
        let committed_truth_digest = super::committed_truth_digest_for_runtime(&self.core)?;
        let envelope = WorkerCommittedTransactionEnvelope::from_committed_worker_transaction(
            branch.id.0,
            committed_truth_digest,
            run_summary,
        );
        self.clear_worker_boundary_certification_evidence();
        Ok(envelope)
    }

    pub fn create_branch(&mut self, name: String) -> Result<RuntimeBranch, WORTHSignalJsError> {
        let branch = self.core.create_branch(name)?;
        self.clear_worker_boundary_certification_evidence();
        Ok(branch)
    }

    pub fn current_branch(&self) -> RuntimeBranch {
        self.core.current_branch()
    }

    pub fn branches(&self) -> Vec<RuntimeBranch> {
        self.core.branches()
    }

    pub fn switch_branch(
        &mut self,
        branch_id: u64,
    ) -> Result<WorkerBranchTruthEnvelope, WORTHSignalJsError> {
        self.core.switch_branch(branch_id)?;
        self.clear_worker_boundary_certification_evidence();
        self.branch_truth_envelope()
    }

    pub fn branch_snapshot(
        &mut self,
        branch_id: u64,
    ) -> Result<RuntimeSnapshot, WORTHSignalJsError> {
        let snapshot = self.core.branch_snapshot(branch_id)?;
        self.clear_worker_boundary_certification_evidence();
        Ok(snapshot)
    }

    pub fn replay_for_branch(
        &mut self,
        branch_id: u64,
    ) -> Result<ReplaySummary, WORTHSignalJsError> {
        self.core.replay_for_branch(branch_id)
    }

    pub fn branch_snapshot_id(&mut self, branch_id: u64) -> Result<u64, WORTHSignalJsError> {
        self.core.branch_snapshot_id(branch_id)
    }

    pub fn branch_snapshot_envelope(
        &mut self,
        branch_id: u64,
    ) -> Result<RuntimeSnapshotEnvelope, WORTHSignalJsError> {
        self.core.branch_snapshot_envelope(branch_id)
    }

    pub fn branch_state_proof(
        &self,
        branch_id: u64,
    ) -> Result<BranchStateProofReport, WORTHSignalJsError> {
        self.core.branch_state_proof(branch_id)
    }

    pub fn export_worker_snapshot_envelope(
        &mut self,
    ) -> Result<RuntimeSnapshotEnvelope, WORTHSignalJsError> {
        let snapshot = self.core.snapshot()?;
        self.clear_worker_boundary_certification_evidence();
        Ok(snapshot)
    }

    pub fn restore_snapshot(
        &mut self,
        snapshot: RuntimeSnapshotEnvelope,
    ) -> Result<WorkerBranchTruthEnvelope, WORTHSignalJsError> {
        self.core.restore_snapshot(snapshot)?;
        self.clear_worker_boundary_certification_evidence();
        self.branch_truth_envelope()
    }

    pub fn restore_branch_snapshot(
        &mut self,
        branch_id: u64,
        snapshot: RuntimeSnapshot,
    ) -> Result<WorkerBranchTruthEnvelope, WORTHSignalJsError> {
        self.core.restore_branch_snapshot(branch_id, snapshot)?;
        self.clear_worker_boundary_certification_evidence();
        self.branch_truth_envelope_for_branch(branch_id)
    }

    pub fn restore_branch_snapshot_by_id(
        &mut self,
        branch_id: u64,
        snapshot_id: u64,
    ) -> Result<WorkerBranchTruthEnvelope, WORTHSignalJsError> {
        self.core
            .restore_branch_snapshot_by_id(branch_id, snapshot_id)?;
        self.clear_worker_boundary_certification_evidence();
        self.branch_truth_envelope_for_branch(branch_id)
    }

    #[cfg(test)]
    pub fn read_value(
        &mut self,
        id: &str,
    ) -> Result<crate::expression::model::SignalValue, WORTHSignalJsError> {
        self.core.read_value(id)
    }

    #[cfg(test)]
    pub fn peek_value(
        &self,
        id: &str,
    ) -> Result<crate::expression::model::SignalValue, WORTHSignalJsError> {
        self.core.peek_value(id)
    }

    pub fn branch_truth_envelope(&self) -> Result<WorkerBranchTruthEnvelope, WORTHSignalJsError> {
        let branch = self.core.current_branch();
        self.branch_truth_envelope_for_branch(branch.id.0)
    }

    pub(in crate::runtime::worker_host) fn branch_truth_envelope_for_branch(
        &self,
        branch_id: u64,
    ) -> Result<WorkerBranchTruthEnvelope, WORTHSignalJsError> {
        let proof = self.core.branch_state_proof(branch_id)?;
        Ok(WorkerBranchTruthEnvelope::from_worker_branch(
            proof.branch_id,
            proof.branch_name,
            proof.snapshot_id,
            proof.state_digest,
        ))
    }
}
