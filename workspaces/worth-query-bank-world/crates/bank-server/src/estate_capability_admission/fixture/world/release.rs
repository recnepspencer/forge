use bank_domain::estate::{
    BankEstateWorld, EstateLegalAuthority, LegalAuthorityKind, MandatoryEstateReview,
    MandatoryReviewKind, MandatoryReviewStatus,
};

use super::super::{AUTHORITY, COMPLETED_REVIEW, ESTATE, EXECUTOR, REVIEWER};

pub(super) fn install_truth(estate: BankEstateWorld) -> BankEstateWorld {
    estate
        .with_legal_authority(EstateLegalAuthority {
            id: AUTHORITY,
            estate: ESTATE,
            holder: EXECUTOR,
            kind: LegalAuthorityKind::CourtAppointment,
            recognized: true,
        })
        .with_executor(ESTATE, EXECUTOR)
        .with_review(MandatoryEstateReview {
            id: COMPLETED_REVIEW,
            estate: ESTATE,
            kind: MandatoryReviewKind::EstateRelease,
            reviewer: Some(REVIEWER),
            status: MandatoryReviewStatus::Completed,
        })
}
