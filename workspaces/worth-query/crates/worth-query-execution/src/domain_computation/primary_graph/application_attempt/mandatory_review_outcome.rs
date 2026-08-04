use worth_foundational::facade::AspectValue;
use worth_relational::facade::identity::EntityId;

use super::{
    WorthQueryApplicationCommitDenial, WorthQueryApplicationCommitOutcome,
    WorthQueryApplicationCommitReceipt, WorthQueryApplicationStaleAttempt,
    WorthQueryElevationClosureKind, WorthQueryMandatoryReview,
};
use crate::domain_computation::authorization::WorthQueryMandatoryReviewBinding;

pub struct WorthQueryReviewedElevation {
    mandatory: WorthQueryMandatoryReview,
    review_commit: WorthQueryApplicationCommitReceipt,
    reviewer: EntityId,
    reviewed_at: AspectValue,
}

impl WorthQueryReviewedElevation {
    pub const fn review_commit_receipt(&self) -> &WorthQueryApplicationCommitReceipt {
        &self.review_commit
    }

    pub const fn reviewer(&self) -> EntityId {
        self.reviewer
    }

    pub const fn reviewed_at(&self) -> &AspectValue {
        &self.reviewed_at
    }

    pub const fn closure_kind(&self) -> WorthQueryElevationClosureKind {
        self.mandatory.closure_kind()
    }

    pub const fn requester(&self) -> EntityId {
        self.mandatory.requester()
    }

    pub const fn approver(&self) -> EntityId {
        self.mandatory.approver()
    }

    pub const fn closer(&self) -> EntityId {
        self.mandatory.closer()
    }

    pub const fn resource(&self) -> EntityId {
        self.mandatory.resource()
    }

    pub const fn grant(&self) -> EntityId {
        self.mandatory.grant()
    }

    pub const fn elevation(&self) -> EntityId {
        self.mandatory.elevation()
    }

    pub const fn review(&self) -> EntityId {
        self.mandatory.review()
    }

    pub const fn action(&self) -> &AspectValue {
        self.mandatory.action()
    }

    pub const fn purpose(&self) -> &AspectValue {
        self.mandatory.purpose()
    }

    pub const fn field(&self) -> Option<&AspectValue> {
        self.mandatory.field()
    }

    pub const fn amount(&self) -> Option<&AspectValue> {
        self.mandatory.amount()
    }

    pub const fn cardinality(&self) -> u32 {
        self.mandatory.cardinality()
    }

    pub const fn reason(&self) -> &AspectValue {
        self.mandatory.reason()
    }

    pub const fn issued_at(&self) -> &AspectValue {
        self.mandatory.issued_at()
    }

    pub const fn expires_at(&self) -> &AspectValue {
        self.mandatory.expires_at()
    }
}

pub enum WorthQueryMandatoryReviewOutcome {
    Reviewed(WorthQueryReviewedElevation),
    AlreadyReviewed(WorthQueryReviewedElevation),
    Stale(WorthQueryApplicationStaleAttempt, WorthQueryMandatoryReview),
    Cancelled(WorthQueryMandatoryReview),
    Denied(WorthQueryApplicationCommitDenial, WorthQueryMandatoryReview),
    Aborted(WorthQueryMandatoryReview),
    PartialEffect,
    Indeterminate,
}

pub(in crate::domain_computation::primary_graph) fn reviewed_outcome(
    outcome: WorthQueryApplicationCommitOutcome,
    binding: WorthQueryMandatoryReviewBinding,
) -> WorthQueryMandatoryReviewOutcome {
    match outcome {
        WorthQueryApplicationCommitOutcome::Committed(commit) => {
            WorthQueryMandatoryReviewOutcome::Reviewed(reviewed(binding, commit))
        }
        WorthQueryApplicationCommitOutcome::AlreadyCommitted(commit) => {
            WorthQueryMandatoryReviewOutcome::AlreadyReviewed(reviewed(binding, commit))
        }
        WorthQueryApplicationCommitOutcome::Stale(stale) => {
            WorthQueryMandatoryReviewOutcome::Stale(stale, binding.into_mandatory())
        }
        WorthQueryApplicationCommitOutcome::Cancelled => {
            WorthQueryMandatoryReviewOutcome::Cancelled(binding.into_mandatory())
        }
        WorthQueryApplicationCommitOutcome::Denied(denial) => {
            WorthQueryMandatoryReviewOutcome::Denied(denial, binding.into_mandatory())
        }
        WorthQueryApplicationCommitOutcome::Aborted => {
            WorthQueryMandatoryReviewOutcome::Aborted(binding.into_mandatory())
        }
        WorthQueryApplicationCommitOutcome::PartialEffect => {
            WorthQueryMandatoryReviewOutcome::PartialEffect
        }
        WorthQueryApplicationCommitOutcome::Indeterminate => {
            WorthQueryMandatoryReviewOutcome::Indeterminate
        }
    }
}

fn reviewed(
    binding: WorthQueryMandatoryReviewBinding,
    review_commit: WorthQueryApplicationCommitReceipt,
) -> WorthQueryReviewedElevation {
    WorthQueryReviewedElevation {
        mandatory: binding.mandatory,
        review_commit,
        reviewer: binding.draft.reviewer,
        reviewed_at: binding.draft.reviewed_at,
    }
}
