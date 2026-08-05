use bank_domain::estate::{
    BankEstateWorld, DelegationLimit, EmergencyAccessReason, EmergencyAccessStatus,
    EstateEmergencyAccess, EstateMoment, MandatoryEstateReview, MandatoryReviewKind,
    MandatoryReviewStatus,
};

use super::super::*;

pub(super) fn install_truth(estate: BankEstateWorld) -> BankEstateWorld {
    let mut parent = grant(GRANT, SPECIALIST, GrantSpec::governance_view());
    parent.scope.delegation = DelegationLimit::generations(1);
    let mut child = grant(DELEGATED_GRANT, APPROVER, GrantSpec::governance_view());
    child.parent = Some(GRANT);
    estate
        .with_grant(parent)
        .with_grant(child)
        .with_grant(grant(
            DISBURSEMENT_GRANT,
            REVIEWER,
            GrantSpec::disburse(50_000),
        ))
        .with_grant(grant(
            EMERGENCY_BOUND_GRANT,
            SPECIALIST,
            GrantSpec::emergency_view(),
        ))
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
            kind: MandatoryReviewKind::EmergencyAccess,
            reviewer: Some(REVIEWER),
            status: MandatoryReviewStatus::Completed,
        })
        .with_emergency_access(EstateEmergencyAccess {
            id: REQUESTED_ACCESS,
            requester: SPECIALIST,
            approver: None,
            reviewer: None,
            grant: EMERGENCY_BOUND_GRANT,
            review: REQUESTED_REVIEW,
            reason: EmergencyAccessReason::PreventImmediateLoss,
            status: EmergencyAccessStatus::Requested,
            issued_at: EstateMoment::from_epoch_seconds(100),
            expires_at: EstateMoment::from_epoch_seconds(200),
        })
        .with_emergency_access(EstateEmergencyAccess {
            id: CLOSED_ACCESS,
            requester: SPECIALIST,
            approver: Some(APPROVER),
            reviewer: Some(REVIEWER),
            grant: EMERGENCY_BOUND_GRANT,
            review: COMPLETED_REVIEW,
            reason: EmergencyAccessReason::MeetLegalDeadline,
            status: EmergencyAccessStatus::Revoked,
            issued_at: EstateMoment::from_epoch_seconds(300),
            expires_at: EstateMoment::from_epoch_seconds(400),
        })
}
