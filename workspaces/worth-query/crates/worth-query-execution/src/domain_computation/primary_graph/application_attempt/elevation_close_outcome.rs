use worth_foundational::facade::AspectValue;
use worth_relational::facade::identity::EntityId;

use super::{
    WorthQueryApplicationCommitDenial, WorthQueryApplicationCommitOutcome,
    WorthQueryApplicationCommitReceipt, WorthQueryApplicationStaleAttempt,
    WorthQueryApprovedElevation,
};
use crate::domain_computation::authorization::WorthQueryElevationCloseBinding;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryElevationClosureKind {
    Revoked,
    Expired,
}

/// Move-only obligation produced only after the approved elevation is closed.
///
/// ```compile_fail
/// use worth_query_execution::facade::primary_graph::WorthQueryMandatoryReview;
///
/// fn mandatory_review_cannot_be_copied(review: WorthQueryMandatoryReview) {
///     let _copied = review.clone();
/// }
/// ```
#[derive(Debug)]
pub struct WorthQueryMandatoryReview {
    approved: WorthQueryApprovedElevation,
    close_commit: WorthQueryApplicationCommitReceipt,
    closure_kind: WorthQueryElevationClosureKind,
    closed_at: AspectValue,
    closer: EntityId,
}

impl WorthQueryMandatoryReview {
    pub const fn closure_kind(&self) -> WorthQueryElevationClosureKind {
        self.closure_kind
    }

    pub const fn closed_at(&self) -> &AspectValue {
        &self.closed_at
    }

    pub const fn closer(&self) -> EntityId {
        self.closer
    }

    pub const fn close_commit_receipt(&self) -> &WorthQueryApplicationCommitReceipt {
        &self.close_commit
    }

    pub const fn requester(&self) -> EntityId {
        self.approved.requester()
    }

    pub const fn approver(&self) -> EntityId {
        self.approved.approver()
    }

    pub const fn resource(&self) -> EntityId {
        self.approved.resource()
    }

    pub const fn grant(&self) -> EntityId {
        self.approved.grant()
    }

    pub const fn elevation(&self) -> EntityId {
        self.approved.elevation()
    }

    pub const fn review(&self) -> EntityId {
        self.approved.review()
    }

    pub const fn action(&self) -> &AspectValue {
        self.approved.action()
    }

    pub const fn purpose(&self) -> &AspectValue {
        self.approved.purpose()
    }

    pub const fn field(&self) -> Option<&AspectValue> {
        self.approved.field()
    }

    pub const fn amount(&self) -> Option<&AspectValue> {
        self.approved.amount()
    }

    pub const fn cardinality(&self) -> u32 {
        self.approved.cardinality()
    }

    pub const fn reason(&self) -> &AspectValue {
        self.approved.reason()
    }

    pub const fn issued_at(&self) -> &AspectValue {
        self.approved.issued_at()
    }

    pub const fn expires_at(&self) -> &AspectValue {
        self.approved.expires_at()
    }

    pub(in crate::domain_computation) const fn approved(&self) -> &WorthQueryApprovedElevation {
        &self.approved
    }

    pub(in crate::domain_computation) fn belongs_to_lifecycle(
        &self,
        runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
        branch: &worth_relational::facade::history::BranchId,
        capability_identity: [u8; 32],
        capability_authority_identity: &str,
    ) -> bool {
        self.approved.belongs_to_lifecycle(
            runtime_authority,
            branch,
            capability_identity,
            capability_authority_identity,
        ) && self.close_commit.terminal().branch() == branch
            && self.close_commit.provider_runtime_instance_id()
                == self
                    .approved
                    .approval_commit_receipt()
                    .provider_runtime_instance_id()
    }
}

#[derive(Debug)]
pub enum WorthQueryElevationCloseOutcome {
    Closed(WorthQueryMandatoryReview),
    AlreadyClosed(WorthQueryMandatoryReview),
    Stale(
        WorthQueryApplicationStaleAttempt,
        WorthQueryApprovedElevation,
    ),
    Cancelled(WorthQueryApprovedElevation),
    Denied(
        WorthQueryApplicationCommitDenial,
        WorthQueryApprovedElevation,
    ),
    Aborted(WorthQueryApprovedElevation),
    PartialEffect,
    Indeterminate,
}

pub(in crate::domain_computation::primary_graph) fn closed_outcome(
    outcome: WorthQueryApplicationCommitOutcome,
    binding: WorthQueryElevationCloseBinding,
) -> WorthQueryElevationCloseOutcome {
    match outcome {
        WorthQueryApplicationCommitOutcome::Committed(commit) => {
            WorthQueryElevationCloseOutcome::Closed(closed(binding, commit))
        }
        WorthQueryApplicationCommitOutcome::AlreadyCommitted(commit) => {
            WorthQueryElevationCloseOutcome::AlreadyClosed(closed(binding, commit))
        }
        WorthQueryApplicationCommitOutcome::Stale(stale) => {
            WorthQueryElevationCloseOutcome::Stale(stale, binding.into_approved())
        }
        WorthQueryApplicationCommitOutcome::Cancelled => {
            WorthQueryElevationCloseOutcome::Cancelled(binding.into_approved())
        }
        WorthQueryApplicationCommitOutcome::Denied(denial) => {
            WorthQueryElevationCloseOutcome::Denied(denial, binding.into_approved())
        }
        WorthQueryApplicationCommitOutcome::Aborted => {
            WorthQueryElevationCloseOutcome::Aborted(binding.into_approved())
        }
        WorthQueryApplicationCommitOutcome::PartialEffect => {
            WorthQueryElevationCloseOutcome::PartialEffect
        }
        WorthQueryApplicationCommitOutcome::Indeterminate => {
            WorthQueryElevationCloseOutcome::Indeterminate
        }
    }
}

fn closed(
    binding: WorthQueryElevationCloseBinding,
    close_commit: WorthQueryApplicationCommitReceipt,
) -> WorthQueryMandatoryReview {
    WorthQueryMandatoryReview {
        approved: binding.approved,
        close_commit,
        closure_kind: binding.draft.closure_kind,
        closed_at: binding.draft.closed_at,
        closer: binding.draft.closer,
    }
}
