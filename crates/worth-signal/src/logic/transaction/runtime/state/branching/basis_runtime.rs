use worth_proof::TransitionOutcome;

use crate::state::{SignalBranchHandle, SignalSnapshotV1, SnapshotRestoreIntent};

use super::super::runtime_state::SignalRuntime;
use super::basis::{
    materialize_branch_basis, SignalBranchBasisArtifact, SignalBranchBasisDenial,
    SignalBranchBasisIdentity, SignalBranchBasisValidationOutcome,
};

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn current_branch_basis_artifact(&mut self) -> SignalBranchBasisArtifact {
        self.telemetry.transaction.branch_basis_production_count += 1;
        let branch = self.graph.current_branch();
        materialize_branch_basis(
            branch.name.clone(),
            SignalBranchBasisIdentity::from_branch_handle(&branch),
        )
    }

    pub fn branch_basis_artifact(
        &mut self,
        branch: SignalBranchHandle,
    ) -> TransitionOutcome<SignalBranchBasisArtifact, SignalBranchBasisDenial> {
        self.telemetry.transaction.branch_basis_production_count += 1;
        let live_branch = self
            .graph
            .branch_handle(branch.id)
            .or_else(|| self.branches.branch_handle(branch.id))
            .ok_or(SignalBranchBasisDenial::UnknownBranch {
                branch_id: branch.id,
                branch_name: branch.name,
            });
        match live_branch {
            Ok(branch) => TransitionOutcome::success(materialize_branch_basis(
                branch.name.clone(),
                SignalBranchBasisIdentity::from_branch_handle(&branch),
            )),
            Err(denial) => {
                self.telemetry.transaction.branch_basis_denial_count += 1;
                TransitionOutcome::denied(denial)
            }
        }
    }

    pub fn snapshot_restore_branch_basis_artifact(
        &mut self,
        snapshot: &SignalSnapshotV1,
        intent: SnapshotRestoreIntent,
    ) -> TransitionOutcome<SignalBranchBasisArtifact, SignalBranchBasisDenial> {
        self.telemetry.transaction.branch_basis_production_count += 1;
        let branch_id = snapshot.meta.branch_id;
        let snapshot_id = snapshot.meta.snapshot_id;
        let Some(live_branch) = self
            .graph
            .branch_handle(branch_id)
            .or_else(|| self.branches.branch_handle(branch_id))
        else {
            self.telemetry.transaction.branch_basis_denial_count += 1;
            return TransitionOutcome::denied(SignalBranchBasisDenial::UnknownBranch {
                branch_id,
                branch_name: snapshot.meta.branch_name.clone(),
            });
        };

        if self
            .branches
            .snapshot_state(branch_id, snapshot_id)
            .is_none()
        {
            self.telemetry.transaction.branch_basis_denial_count += 1;
            return TransitionOutcome::denied(SignalBranchBasisDenial::UntrackedSnapshot {
                branch_id,
                snapshot_id,
            });
        }

        TransitionOutcome::success(materialize_branch_basis(
            live_branch.name.clone(),
            SignalBranchBasisIdentity::from_snapshot_restore(snapshot, intent),
        ))
    }

    pub fn snapshot_branch_basis_artifact(
        &mut self,
        branch: SignalBranchHandle,
        snapshot: &SignalSnapshotV1,
    ) -> TransitionOutcome<SignalBranchBasisArtifact, SignalBranchBasisDenial> {
        self.telemetry.transaction.branch_basis_production_count += 1;
        let Some(live_branch) = self
            .graph
            .branch_handle(branch.id)
            .or_else(|| self.branches.branch_handle(branch.id))
        else {
            self.telemetry.transaction.branch_basis_denial_count += 1;
            return TransitionOutcome::denied(SignalBranchBasisDenial::UnknownBranch {
                branch_id: branch.id,
                branch_name: branch.name,
            });
        };

        if snapshot.meta.branch_id != live_branch.id {
            self.telemetry.transaction.branch_basis_denial_count += 1;
            return TransitionOutcome::denied(SignalBranchBasisDenial::CrossBranchMismatch {
                basis_branch_id: snapshot.meta.branch_id,
                expected_branch_id: live_branch.id,
            });
        }

        if self
            .branches
            .snapshot_state(live_branch.id, snapshot.meta.snapshot_id)
            .is_none()
        {
            self.telemetry.transaction.branch_basis_denial_count += 1;
            return TransitionOutcome::denied(SignalBranchBasisDenial::UntrackedSnapshot {
                branch_id: live_branch.id,
                snapshot_id: snapshot.meta.snapshot_id,
            });
        }

        TransitionOutcome::success(materialize_branch_basis(
            live_branch.name.clone(),
            SignalBranchBasisIdentity::from_branch_snapshot(
                &live_branch,
                snapshot.meta.snapshot_id,
            ),
        ))
    }

    pub fn validate_branch_basis_artifact(
        &mut self,
        basis: SignalBranchBasisArtifact,
        branch: SignalBranchHandle,
    ) -> SignalBranchBasisValidationOutcome {
        self.telemetry.transaction.branch_basis_validation_count += 1;
        if basis.payload().branch_id() != branch.id {
            self.telemetry.transaction.branch_basis_denial_count += 1;
            return TransitionOutcome::denied(SignalBranchBasisDenial::CrossBranchMismatch {
                basis_branch_id: basis.payload().branch_id(),
                expected_branch_id: branch.id,
            });
        }

        let Some(live_branch) = self
            .graph
            .branch_handle(branch.id)
            .or_else(|| self.branches.branch_handle(branch.id))
        else {
            self.telemetry.transaction.branch_basis_denial_count += 1;
            return TransitionOutcome::denied(SignalBranchBasisDenial::UnknownBranch {
                branch_id: branch.id,
                branch_name: branch.name,
            });
        };

        let live_identity = SignalBranchBasisIdentity::from_branch_handle(&live_branch);
        if basis.strong_basis().value() != &live_identity {
            self.telemetry.transaction.branch_basis_stale_count += 1;
            return TransitionOutcome::stale(basis.downgrade_to_stale_readable());
        }

        TransitionOutcome::success(basis)
    }
}
