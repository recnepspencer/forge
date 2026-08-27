use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use worth_proof::TransitionOutcome;

use crate::logic::transaction::canonical_digest;
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

use super::super::runtime_state::SignalRuntime;
use super::{transaction_head::SignalBranchTransactionHead, SignalBranchBasisArtifact};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalBranchRetirementReason {
    Rejected,
    Merged,
    Superseded,
    DependencyCancellation,
    ProjectionRebuild,
}

#[derive(Debug, Clone)]
pub struct SignalBranchRetirementRequest {
    branch: SignalBranchHandle,
    expected_head: SignalBranchTransactionHead,
    reason: SignalBranchRetirementReason,
}

impl SignalBranchRetirementRequest {
    pub(crate) fn new(
        branch: SignalBranchHandle,
        expected_head: SignalBranchTransactionHead,
        reason: SignalBranchRetirementReason,
    ) -> Self {
        Self {
            branch,
            expected_head,
            reason,
        }
    }

    pub(crate) fn branch(&self) -> &SignalBranchHandle {
        &self.branch
    }

    pub(crate) fn expected_head(&self) -> &SignalBranchTransactionHead {
        &self.expected_head
    }

    pub(crate) fn reason(&self) -> SignalBranchRetirementReason {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalBranchRetirementDenial {
    UnknownBranch {
        branch_id: SignalBranchId,
    },
    CurrentBranch {
        branch_id: SignalBranchId,
    },
    CanonicalBranch {
        branch_id: SignalBranchId,
    },
    StaleBranchHead {
        expected: SignalBranchTransactionHead,
        observed: SignalBranchTransactionHead,
    },
    CanonicalBasisMismatch,
    LiveChildren {
        branch_id: SignalBranchId,
        child_branch_ids: Vec<SignalBranchId>,
    },
    MergeParticipant {
        branch_id: SignalBranchId,
    },
}

#[derive(Debug, Clone)]
pub struct PlannedSignalBranchRetirement {
    pub(super) request: SignalBranchRetirementRequest,
    pub(super) validated_basis: SignalBranchBasisArtifact,
    pub(super) planned_child_membership_count: u32,
}

impl PlannedSignalBranchRetirement {
    pub(crate) fn request(&self) -> &SignalBranchRetirementRequest {
        &self.request
    }

    pub(crate) fn validated_basis(&self) -> &SignalBranchBasisArtifact {
        &self.validated_basis
    }

    pub fn planned_child_membership_count(&self) -> u32 {
        self.planned_child_membership_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalBranchRetirementReceipt {
    retired_branch: SignalBranchHandle,
    parent_branch_id: SignalBranchId,
    forked_from_snapshot_id: Option<SignalSnapshotId>,
    terminal_head_snapshot_id: Option<SignalSnapshotId>,
    reason: SignalBranchRetirementReason,
    terminal_basis_digest: String,
    closeout_digest: String,
    reclaimed_branch_state_count: u32,
    reclaimed_snapshot_state_count: u32,
    reclaimed_runtime_meta_count: u32,
    retained_proof_record_count: u32,
}

impl SignalBranchRetirementReceipt {
    pub fn retired_branch(&self) -> &SignalBranchHandle {
        &self.retired_branch
    }

    pub fn parent_branch_id(&self) -> SignalBranchId {
        self.parent_branch_id
    }

    pub fn forked_from_snapshot_id(&self) -> Option<SignalSnapshotId> {
        self.forked_from_snapshot_id
    }

    pub fn terminal_head_snapshot_id(&self) -> Option<SignalSnapshotId> {
        self.terminal_head_snapshot_id
    }

    pub fn reason(&self) -> SignalBranchRetirementReason {
        self.reason
    }

    pub fn terminal_basis_digest(&self) -> &str {
        &self.terminal_basis_digest
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub fn reclaimed_branch_state_count(&self) -> u32 {
        self.reclaimed_branch_state_count
    }

    pub fn reclaimed_snapshot_state_count(&self) -> u32 {
        self.reclaimed_snapshot_state_count
    }

    pub fn reclaimed_runtime_meta_count(&self) -> u32 {
        self.reclaimed_runtime_meta_count
    }

    pub fn retained_proof_record_count(&self) -> u32 {
        self.retained_proof_record_count
    }
}

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn plan_branch_retirement(
        &mut self,
        request: SignalBranchRetirementRequest,
    ) -> TransitionOutcome<PlannedSignalBranchRetirement, SignalBranchRetirementDenial> {
        self.with_telemetry(|telemetry| telemetry.transaction.branch_retirement_plan_count += 1);
        if let Err(denial) = self.validate_retirement_request(&request) {
            self.with_telemetry(|telemetry| {
                telemetry.transaction.branch_retirement_denial_count += 1
            });
            return TransitionOutcome::denied(denial);
        }

        let child_count = self.branches.branch_children(request.branch.id).len() as u32;
        let validated_basis = match self.branch_basis_artifact(request.branch.clone()) {
            TransitionOutcome::Success(basis) => basis,
            other => panic!("validated retirement branch basis must succeed: {other:?}"),
        };
        TransitionOutcome::success(PlannedSignalBranchRetirement {
            validated_basis,
            request,
            planned_child_membership_count: child_count,
        })
    }

    pub(crate) fn retire_branch(
        &mut self,
        plan: PlannedSignalBranchRetirement,
    ) -> TransitionOutcome<SignalBranchRetirementReceipt, SignalBranchRetirementDenial> {
        if let Err(denial) = self.validate_retirement_request(&plan.request) {
            self.with_telemetry(|telemetry| {
                telemetry.transaction.branch_retirement_denial_count += 1
            });
            return TransitionOutcome::denied(denial);
        }

        let branch = plan.request.branch.clone();
        let ancestry = self
            .branches
            .branch_ancestry_state(branch.id)
            .expect("validated retirement must retain ancestry")
            .clone();
        let parent_branch_id = ancestry
            .parent_branch_id()
            .expect("canonical branch retirement is denied during planning");
        let terminal_head_snapshot_id = self.branch_head_snapshot_id(branch.id);
        let terminal_basis_digest = plan.validated_basis.payload().basis_digest().to_owned();
        let reclaimed = self
            .branches
            .retire_stored_branch(branch.id)
            .expect("validated retirement must reclaim a stored branch");
        let closeout_digest = canonical_digest(&(
            branch.id,
            parent_branch_id,
            ancestry.forked_from_snapshot_id(),
            terminal_head_snapshot_id,
            plan.request.reason,
            terminal_basis_digest.as_str(),
        ));
        let receipt = SignalBranchRetirementReceipt {
            retired_branch: branch.clone(),
            parent_branch_id,
            forked_from_snapshot_id: ancestry.forked_from_snapshot_id(),
            terminal_head_snapshot_id,
            reason: plan.request.reason,
            terminal_basis_digest,
            closeout_digest,
            reclaimed_branch_state_count: reclaimed.branch_state_count,
            reclaimed_snapshot_state_count: reclaimed.snapshot_state_count,
            reclaimed_runtime_meta_count: reclaimed.runtime_meta_count,
            retained_proof_record_count: 1,
        };
        self.branches.retain_retirement_receipt(receipt.clone());
        self.graph
            .diagnostics_state_mut()
            .retire_branch_from_catalog(branch.id);
        let branch_catalog = self.graph.diagnostics_state().branch_catalog().clone();
        self.synchronize_branch_catalogs(branch_catalog);
        self.with_telemetry(|telemetry| {
            telemetry.transaction.branch_retirement_execution_count += 1;
            telemetry
                .transaction
                .branch_retirement_reclaimed_branch_state_count +=
                u64::from(receipt.reclaimed_branch_state_count);
            telemetry
                .transaction
                .branch_retirement_reclaimed_snapshot_state_count +=
                u64::from(receipt.reclaimed_snapshot_state_count);
            telemetry
                .transaction
                .branch_retirement_reclaimed_runtime_meta_count +=
                u64::from(receipt.reclaimed_runtime_meta_count);
            telemetry.transaction.branch_retirement_retained_proof_count += 1;
        });
        crate::diagnostics::recorder::record_snapshot_event(
            &mut self.graph,
            crate::diagnostics::replay::ReplayEventKind::BranchRetired,
            None,
            format!(
                "retired branch `{}` with closeout `{}`",
                branch.name,
                receipt.closeout_digest()
            ),
        );
        TransitionOutcome::success(receipt)
    }

    fn validate_retirement_request(
        &self,
        request: &SignalBranchRetirementRequest,
    ) -> Result<(), SignalBranchRetirementDenial> {
        self.validate_retirement_request_after(request, &BTreeSet::new())
    }

    pub(super) fn validate_retirement_request_after(
        &self,
        request: &SignalBranchRetirementRequest,
        retired_before: &BTreeSet<SignalBranchId>,
    ) -> Result<(), SignalBranchRetirementDenial> {
        let branch_id = request.branch.id;
        if branch_id == self.graph.current_branch().id {
            return Err(SignalBranchRetirementDenial::CurrentBranch { branch_id });
        }
        let Some(live_branch) = self.branches.branch_handle(branch_id) else {
            return Err(SignalBranchRetirementDenial::UnknownBranch { branch_id });
        };
        if live_branch.parent_branch_id.is_none() {
            return Err(SignalBranchRetirementDenial::CanonicalBranch { branch_id });
        }
        let observed = self
            .observe_branch_transaction_head(&live_branch)
            .expect("validated stored branch must expose a transaction head");
        if request.expected_head != observed {
            return Err(SignalBranchRetirementDenial::StaleBranchHead {
                expected: request.expected_head.clone(),
                observed,
            });
        }
        let children = self
            .branches
            .branch_children(branch_id)
            .into_iter()
            .filter(|child_id| !retired_before.contains(child_id))
            .collect::<Vec<_>>();
        if !children.is_empty() {
            return Err(SignalBranchRetirementDenial::LiveChildren {
                branch_id,
                child_branch_ids: children,
            });
        }
        if self.branches.is_merge_participant(branch_id) {
            return Err(SignalBranchRetirementDenial::MergeParticipant { branch_id });
        }
        Ok(())
    }
}
