use bank_domain::estate::{
    BankEstateWorld, MandatoryEstateReview, MandatoryReviewKind, MandatoryReviewStatus,
};

use super::super::*;

pub(super) fn install_truth(estate: BankEstateWorld, spec: GrantSpec) -> BankEstateWorld {
    estate
        .with_grant(grant(GRANT, REVIEWER, spec))
        .with_review(MandatoryEstateReview {
            id: REQUESTED_REVIEW,
            estate: ESTATE,
            kind: MandatoryReviewKind::EmergencyAccess,
            reviewer: None,
            status: MandatoryReviewStatus::Required,
        })
        .with_review(MandatoryEstateReview {
            id: COMPLETED_REVIEW,
            estate: ESTATE,
            kind: MandatoryReviewKind::EstateRelease,
            reviewer: Some(REVIEWER),
            status: MandatoryReviewStatus::Completed,
        })
}
