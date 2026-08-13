use std::time::Instant;

use bank_domain::estate::{EmergencyAccessId, EstateAction, EstateCaseId, MandatoryReviewId};
use bank_domain::proposals::BankIdempotencyKey;
use bank_server::{
    BankApprovedEstateElevation, BankEstateMandatoryReview, BankRequestedEstateElevation,
};

use super::super::authenticated_owner::BankHttpAuthenticatedOwner;
use super::{BankHttpElevationFacts, BankHttpElevationReviewFacts};

#[derive(Clone, Copy)]
pub(in crate::http::server) struct BankHttpElevationContext {
    estate: EstateCaseId,
    access: EmergencyAccessId,
    review: MandatoryReviewId,
}

impl BankHttpElevationContext {
    pub(in crate::http::server) const fn new(
        estate: EstateCaseId,
        access: EmergencyAccessId,
        review: MandatoryReviewId,
    ) -> Self {
        Self {
            estate,
            access,
            review,
        }
    }

    pub(in crate::http::server) const fn approval_action(self) -> EstateAction {
        EstateAction::ApproveEmergencyAccess {
            estate: self.estate,
            access: self.access,
        }
    }

    pub(in crate::http::server) const fn revocation_action(self) -> EstateAction {
        EstateAction::RevokeEmergencyAccess {
            estate: self.estate,
            access: self.access,
        }
    }

    pub(in crate::http::server) const fn review_action(self) -> EstateAction {
        EstateAction::CompleteMandatoryReview {
            estate: self.estate,
            access: self.access,
            review: self.review,
        }
    }
}

pub(super) enum BankHttpElevationState {
    Requested(BankRequestedEstateElevation),
    Approved(BankApprovedEstateElevation),
    MandatoryReview(BankEstateMandatoryReview),
    Terminal,
}

pub(super) struct BankHttpElevationTransitionReplay<Facts> {
    pub(super) actor: BankHttpAuthenticatedOwner,
    pub(super) key: BankIdempotencyKey,
    pub(super) facts: Facts,
}

pub(super) struct BankHttpElevationRecord {
    pub(super) request_action: EstateAction,
    pub(super) request_facts: BankHttpElevationFacts,
    pub(super) context: BankHttpElevationContext,
    pub(super) state: BankHttpElevationState,
    pub(super) approval: Option<BankHttpElevationTransitionReplay<BankHttpElevationFacts>>,
    pub(super) close: Option<BankHttpElevationTransitionReplay<BankHttpElevationReviewFacts>>,
    pub(super) review: Option<BankHttpElevationTransitionReplay<BankHttpElevationReviewFacts>>,
    pub(super) expires_at: Instant,
}
