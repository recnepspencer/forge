use worth_relational::facade::identity::EntityId;

use super::{
    WorthQueryApplicationCommitDenial, WorthQueryApplicationCommitOutcome,
    WorthQueryApplicationCommitReceipt, WorthQueryApplicationStaleAttempt,
    WorthQueryRequestedElevation,
};
use crate::domain_computation::authorization::{
    WorthQueryAuthorizationDecisionFact, WorthQueryElevationApprovalBinding,
    WorthQueryElevationRequestBinding, WorthQueryRetainedCapabilityRequest,
};

/// Move-only authority proving that Query committed the exact approval.
///
/// ```compile_fail
/// use worth_query_execution::facade::primary_graph::WorthQueryApprovedElevation;
///
/// fn approved_elevation_cannot_be_copied(receipt: WorthQueryApprovedElevation) {
///     let _copied = receipt.clone();
/// }
/// ```
#[derive(Debug)]
pub struct WorthQueryApprovedElevation {
    requested: WorthQueryElevationRequestBinding,
    request_commit: WorthQueryApplicationCommitReceipt,
    approval_commit: WorthQueryApplicationCommitReceipt,
    elevation: EntityId,
    review: EntityId,
    approver: EntityId,
}

impl WorthQueryApprovedElevation {
    pub const fn request_commit_receipt(&self) -> &WorthQueryApplicationCommitReceipt {
        &self.request_commit
    }

    pub const fn approval_commit_receipt(&self) -> &WorthQueryApplicationCommitReceipt {
        &self.approval_commit
    }

    pub const fn requester(&self) -> EntityId {
        self.requested.requester()
    }

    pub const fn approver(&self) -> EntityId {
        self.approver
    }

    pub const fn resource(&self) -> EntityId {
        self.requested.resource()
    }

    pub const fn grant(&self) -> EntityId {
        self.requested.grant()
    }

    pub const fn elevation(&self) -> EntityId {
        self.elevation
    }

    pub const fn review(&self) -> EntityId {
        self.review
    }

    pub const fn action(&self) -> &worth_foundational::facade::AspectValue {
        self.requested.upper_bound.action()
    }

    pub const fn purpose(&self) -> &worth_foundational::facade::AspectValue {
        self.requested.upper_bound.purpose()
    }

    pub const fn field(&self) -> Option<&worth_foundational::facade::AspectValue> {
        self.requested.upper_bound.field()
    }

    pub const fn magnitude(&self) -> Option<&worth_foundational::facade::AspectValue> {
        self.requested.upper_bound.magnitude()
    }

    pub const fn cardinality(&self) -> u32 {
        self.requested.upper_bound.cardinality()
    }

    pub const fn reason(&self) -> &worth_foundational::facade::AspectValue {
        &self.requested.reason
    }

    pub const fn issued_at(&self) -> &worth_foundational::facade::AspectValue {
        &self.requested.issued_at
    }

    pub const fn expires_at(&self) -> &worth_foundational::facade::AspectValue {
        &self.requested.expires_at
    }

    pub(in crate::domain_computation) fn belongs_to_lifecycle(
        &self,
        runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
        branch: &worth_relational::facade::history::BranchId,
        capability_identity: [u8; 32],
        capability_authority_identity: &str,
    ) -> bool {
        self.requested.runtime_authority == runtime_authority
            && &self.requested.branch == branch
            && self.requested.capability_identity == capability_identity
            && self.requested.capability_authority_identity.as_ref()
                == capability_authority_identity
            && self.request_commit.terminal().branch() == branch
            && self.approval_commit.terminal().branch() == branch
            && self.request_commit.provider_runtime_instance_id()
                == self.approval_commit.provider_runtime_instance_id()
    }

    pub(in crate::domain_computation) const fn request_binding(
        &self,
    ) -> &WorthQueryElevationRequestBinding {
        &self.requested
    }

    pub(in crate::domain_computation) fn support_remains_current_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
        bridge: &worth_runtime_bridge::facade::BridgeAuthorizationRuntime,
    ) -> bool {
        self.requested
            .supporting
            .decision()
            .remains_current_in(runtime, snapshot, bridge)
    }

    pub(in crate::domain_computation) fn support_decision(
        &self,
    ) -> &WorthQueryAuthorizationDecisionFact {
        self.requested.supporting.decision()
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::domain_computation) fn admits_active_use(
        &self,
        runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
        branch: &worth_relational::facade::history::BranchId,
        capability_identity: [u8; 32],
        capability_authority_identity: &str,
        request: &WorthQueryRetainedCapabilityRequest,
        elevation: EntityId,
        grant: EntityId,
    ) -> bool {
        self.belongs_to_lifecycle(
            runtime_authority,
            branch,
            capability_identity,
            capability_authority_identity,
        ) && self.requested.upper_bound.capability_identity() == capability_identity
            && self.elevation == elevation
            && self
                .requested
                .upper_bound
                .matches_active_request(request, elevation, grant)
    }
}

#[derive(Debug)]
pub enum WorthQueryElevationApprovalOutcome {
    Approved(WorthQueryApprovedElevation),
    AlreadyApproved(WorthQueryApprovedElevation),
    Stale(
        WorthQueryApplicationStaleAttempt,
        WorthQueryRequestedElevation,
    ),
    Cancelled(WorthQueryRequestedElevation),
    Denied(
        WorthQueryApplicationCommitDenial,
        WorthQueryRequestedElevation,
    ),
    Aborted(WorthQueryRequestedElevation),
    PartialEffect,
    Indeterminate,
}

pub(in crate::domain_computation::primary_graph) fn approved_outcome(
    outcome: WorthQueryApplicationCommitOutcome,
    binding: WorthQueryElevationApprovalBinding,
) -> WorthQueryElevationApprovalOutcome {
    match outcome {
        WorthQueryApplicationCommitOutcome::Committed(commit) => {
            WorthQueryElevationApprovalOutcome::Approved(approved(binding, commit))
        }
        WorthQueryApplicationCommitOutcome::AlreadyCommitted(commit) => {
            WorthQueryElevationApprovalOutcome::AlreadyApproved(approved(binding, commit))
        }
        WorthQueryApplicationCommitOutcome::Stale(stale) => {
            WorthQueryElevationApprovalOutcome::Stale(stale, binding.into_requested())
        }
        WorthQueryApplicationCommitOutcome::Cancelled => {
            WorthQueryElevationApprovalOutcome::Cancelled(binding.into_requested())
        }
        WorthQueryApplicationCommitOutcome::Denied(denial) => {
            WorthQueryElevationApprovalOutcome::Denied(denial, binding.into_requested())
        }
        WorthQueryApplicationCommitOutcome::Aborted => {
            WorthQueryElevationApprovalOutcome::Aborted(binding.into_requested())
        }
        WorthQueryApplicationCommitOutcome::PartialEffect(_) => {
            WorthQueryElevationApprovalOutcome::PartialEffect
        }
        WorthQueryApplicationCommitOutcome::Indeterminate(_) => {
            WorthQueryElevationApprovalOutcome::Indeterminate
        }
    }
}

fn approved(
    binding: WorthQueryElevationApprovalBinding,
    approval_commit: WorthQueryApplicationCommitReceipt,
) -> WorthQueryApprovedElevation {
    WorthQueryApprovedElevation {
        requested: binding.requested,
        request_commit: binding.request_commit,
        approval_commit,
        elevation: binding.elevation,
        review: binding.review,
        approver: binding.approver,
    }
}
