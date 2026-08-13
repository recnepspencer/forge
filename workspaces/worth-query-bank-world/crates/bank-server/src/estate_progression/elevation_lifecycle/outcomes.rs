//! Bank-owned classifications for every estate elevation transition.

use worth_query_host::facade::primary_graph::{
    WorthQueryElevationApprovalOutcome, WorthQueryElevationCloseOutcome,
    WorthQueryElevationRequestOutcome, WorthQueryMandatoryReviewOutcome,
};

use super::{
    BankApprovedEstateElevation, BankEstateMandatoryReview, BankRequestedEstateElevation,
    BankReviewedEstateElevation,
};
use crate::operation_commit::{denial_kind, denial_stage};
use crate::{BankCommitDenialKind, BankCommitDenialStage};

#[derive(Debug)]
pub enum BankEstateElevationRequestOutcome {
    Requested(BankRequestedEstateElevation),
    AlreadyRequested(BankRequestedEstateElevation),
    Stale {
        stale_fact_count: usize,
    },
    Cancelled,
    Denied {
        kind: BankCommitDenialKind,
        stage: BankCommitDenialStage,
    },
    Aborted,
    PartialEffect,
    Indeterminate,
}

#[derive(Debug)]
pub enum BankEstateElevationApprovalOutcome {
    Approved(BankApprovedEstateElevation),
    AlreadyApproved(BankApprovedEstateElevation),
    Stale {
        stale_fact_count: usize,
        requested: BankRequestedEstateElevation,
    },
    Cancelled(BankRequestedEstateElevation),
    Denied {
        kind: BankCommitDenialKind,
        stage: BankCommitDenialStage,
        requested: BankRequestedEstateElevation,
    },
    Aborted(BankRequestedEstateElevation),
    PartialEffect,
    Indeterminate,
}

#[derive(Debug)]
pub enum BankEstateElevationCloseOutcome {
    Closed(BankEstateMandatoryReview),
    AlreadyClosed(BankEstateMandatoryReview),
    Stale {
        stale_fact_count: usize,
        approved: BankApprovedEstateElevation,
    },
    Cancelled(BankApprovedEstateElevation),
    Denied {
        kind: BankCommitDenialKind,
        stage: BankCommitDenialStage,
        approved: BankApprovedEstateElevation,
    },
    Aborted(BankApprovedEstateElevation),
    PartialEffect,
    Indeterminate,
}

#[derive(Debug)]
pub enum BankEstateMandatoryReviewOutcome {
    Reviewed(BankReviewedEstateElevation),
    AlreadyReviewed(BankReviewedEstateElevation),
    Stale {
        stale_fact_count: usize,
        mandatory: BankEstateMandatoryReview,
    },
    Cancelled(BankEstateMandatoryReview),
    Denied {
        kind: BankCommitDenialKind,
        stage: BankCommitDenialStage,
        mandatory: BankEstateMandatoryReview,
    },
    Aborted(BankEstateMandatoryReview),
    PartialEffect,
    Indeterminate,
}

impl BankEstateElevationRequestOutcome {
    pub(crate) fn from_query(outcome: WorthQueryElevationRequestOutcome) -> Self {
        match outcome {
            WorthQueryElevationRequestOutcome::Requested(value) => {
                Self::Requested(BankRequestedEstateElevation::from_query(value))
            }
            WorthQueryElevationRequestOutcome::AlreadyRequested(value) => {
                Self::AlreadyRequested(BankRequestedEstateElevation::from_query(value))
            }
            WorthQueryElevationRequestOutcome::Stale(stale) => Self::Stale {
                stale_fact_count: stale.stale_fact_count(),
            },
            WorthQueryElevationRequestOutcome::Cancelled => Self::Cancelled,
            WorthQueryElevationRequestOutcome::Denied(denial) => Self::Denied {
                kind: denial_kind(denial.kind()),
                stage: denial_stage(denial.stage()),
            },
            WorthQueryElevationRequestOutcome::Aborted => Self::Aborted,
            WorthQueryElevationRequestOutcome::PartialEffect => Self::PartialEffect,
            WorthQueryElevationRequestOutcome::Indeterminate => Self::Indeterminate,
        }
    }
}

impl BankEstateElevationApprovalOutcome {
    pub(crate) fn from_query(outcome: WorthQueryElevationApprovalOutcome) -> Self {
        match outcome {
            WorthQueryElevationApprovalOutcome::Approved(value) => {
                Self::Approved(BankApprovedEstateElevation::from_query(value))
            }
            WorthQueryElevationApprovalOutcome::AlreadyApproved(value) => {
                Self::AlreadyApproved(BankApprovedEstateElevation::from_query(value))
            }
            WorthQueryElevationApprovalOutcome::Stale(stale, requested) => Self::Stale {
                stale_fact_count: stale.stale_fact_count(),
                requested: BankRequestedEstateElevation::from_query(requested),
            },
            WorthQueryElevationApprovalOutcome::Cancelled(requested) => {
                Self::Cancelled(BankRequestedEstateElevation::from_query(requested))
            }
            WorthQueryElevationApprovalOutcome::Denied(denial, requested) => Self::Denied {
                kind: denial_kind(denial.kind()),
                stage: denial_stage(denial.stage()),
                requested: BankRequestedEstateElevation::from_query(requested),
            },
            WorthQueryElevationApprovalOutcome::Aborted(requested) => {
                Self::Aborted(BankRequestedEstateElevation::from_query(requested))
            }
            WorthQueryElevationApprovalOutcome::PartialEffect => Self::PartialEffect,
            WorthQueryElevationApprovalOutcome::Indeterminate => Self::Indeterminate,
        }
    }
}

impl BankEstateElevationCloseOutcome {
    pub(crate) fn from_query(outcome: WorthQueryElevationCloseOutcome) -> Self {
        match outcome {
            WorthQueryElevationCloseOutcome::Closed(value) => {
                Self::Closed(BankEstateMandatoryReview::from_query(value))
            }
            WorthQueryElevationCloseOutcome::AlreadyClosed(value) => {
                Self::AlreadyClosed(BankEstateMandatoryReview::from_query(value))
            }
            WorthQueryElevationCloseOutcome::Stale(stale, approved) => Self::Stale {
                stale_fact_count: stale.stale_fact_count(),
                approved: BankApprovedEstateElevation::from_query(approved),
            },
            WorthQueryElevationCloseOutcome::Cancelled(approved) => {
                Self::Cancelled(BankApprovedEstateElevation::from_query(approved))
            }
            WorthQueryElevationCloseOutcome::Denied(denial, approved) => Self::Denied {
                kind: denial_kind(denial.kind()),
                stage: denial_stage(denial.stage()),
                approved: BankApprovedEstateElevation::from_query(approved),
            },
            WorthQueryElevationCloseOutcome::Aborted(approved) => {
                Self::Aborted(BankApprovedEstateElevation::from_query(approved))
            }
            WorthQueryElevationCloseOutcome::PartialEffect => Self::PartialEffect,
            WorthQueryElevationCloseOutcome::Indeterminate => Self::Indeterminate,
        }
    }
}

impl BankEstateMandatoryReviewOutcome {
    pub(crate) fn from_query(outcome: WorthQueryMandatoryReviewOutcome) -> Self {
        match outcome {
            WorthQueryMandatoryReviewOutcome::Reviewed(value) => {
                Self::Reviewed(BankReviewedEstateElevation::from_query(value))
            }
            WorthQueryMandatoryReviewOutcome::AlreadyReviewed(value) => {
                Self::AlreadyReviewed(BankReviewedEstateElevation::from_query(value))
            }
            WorthQueryMandatoryReviewOutcome::Stale(stale, mandatory) => Self::Stale {
                stale_fact_count: stale.stale_fact_count(),
                mandatory: BankEstateMandatoryReview::from_query(mandatory),
            },
            WorthQueryMandatoryReviewOutcome::Cancelled(mandatory) => {
                Self::Cancelled(BankEstateMandatoryReview::from_query(mandatory))
            }
            WorthQueryMandatoryReviewOutcome::Denied(denial, mandatory) => Self::Denied {
                kind: denial_kind(denial.kind()),
                stage: denial_stage(denial.stage()),
                mandatory: BankEstateMandatoryReview::from_query(mandatory),
            },
            WorthQueryMandatoryReviewOutcome::Aborted(mandatory) => {
                Self::Aborted(BankEstateMandatoryReview::from_query(mandatory))
            }
            WorthQueryMandatoryReviewOutcome::PartialEffect => Self::PartialEffect,
            WorthQueryMandatoryReviewOutcome::Indeterminate => Self::Indeterminate,
        }
    }
}
