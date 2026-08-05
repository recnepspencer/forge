use bank_domain::estate::{
    BankEstateWorld, EstateCase, EstateCaseId, EstateLegalAuthority, LegalAuthorityId,
    LegalAuthorityKind, MandatoryEstateReview, MandatoryReviewId, MandatoryReviewKind,
    MandatoryReviewStatus,
};

use super::{super::*, WarmLocalityAxis};

pub(super) fn install_growth(
    mut estate: BankEstateWorld,
    axis: WarmLocalityAxis,
    count: usize,
) -> BankEstateWorld {
    for ordinal in 0..count {
        estate = match axis {
            WarmLocalityAxis::Grants => estate,
            WarmLocalityAxis::Relationships => {
                estate.with_beneficiary(ESTATE, extra_principal(ordinal))
            }
            WarmLocalityAxis::Fields => estate.with_review(MandatoryEstateReview {
                id: MandatoryReviewId::new(10_000 + ordinal as u64).unwrap(),
                estate: ESTATE,
                kind: MandatoryReviewKind::EstateRelease,
                reviewer: None,
                status: MandatoryReviewStatus::Required,
            }),
            WarmLocalityAxis::Cases => estate.with_case(EstateCase {
                id: EstateCaseId::new(20_000 + ordinal as u64).unwrap(),
                institution: INSTITUTION,
                branch: BRANCH,
                deceased: DECEASED,
                account: ACCOUNT,
                death_notice: NOTICE,
                stage: bank_domain::estate::EstateWorkflowStage::Administration,
                status: bank_domain::estate::EstateCaseStatus::Open,
            }),
            WarmLocalityAxis::ResultRows => estate.with_legal_authority(EstateLegalAuthority {
                id: LegalAuthorityId::new(30_000 + ordinal as u64).unwrap(),
                estate: ESTATE,
                holder: EXECUTOR,
                kind: LegalAuthorityKind::InstitutionalRecognition,
                recognized: false,
            }),
        };
    }
    estate
}
