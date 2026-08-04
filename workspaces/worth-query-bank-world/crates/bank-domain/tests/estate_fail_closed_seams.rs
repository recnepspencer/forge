use bank_domain::estate::*;
use bank_domain::model::{Money, SignedMoney};

#[path = "estate_capability_courtroom/support.rs"]
mod support;

use support::{courtroom, grant, Courtroom};

const NOW: EstateMoment = EstateMoment::from_epoch_seconds(150);

#[test]
fn current_principal_time_and_stage_each_remain_required() {
    let courtroom = courtroom();
    let action = view_account_details(&courtroom);

    let mut expired = view_grant(&courtroom, 130);
    expired.scope.validity = CapabilityValidity::new(
        EstateMoment::from_epoch_seconds(100),
        EstateMoment::from_epoch_seconds(149),
    )
    .unwrap();
    assert_denied(
        &courtroom.world.clone().with_grant(expired),
        &courtroom,
        action,
        expired.id,
        EstateWorkflowStage::Administration,
        None,
        EstateDenial::GrantExpired,
    );

    let mut wrong_principal = view_grant(&courtroom, 131);
    wrong_principal.grantee = courtroom.manager;
    assert_denied(
        &courtroom.world.clone().with_grant(wrong_principal),
        &courtroom,
        action,
        wrong_principal.id,
        EstateWorkflowStage::Administration,
        None,
        EstateDenial::GrantPrincipalMismatch,
    );

    let mut wrong_stage = view_grant(&courtroom, 132);
    wrong_stage.scope.workflow_stage = EstateWorkflowStage::ReleaseReview;
    assert_denied(
        &courtroom.world.clone().with_grant(wrong_stage),
        &courtroom,
        action,
        wrong_stage.id,
        EstateWorkflowStage::ReleaseReview,
        None,
        EstateDenial::GrantScopeMismatch,
    );
}

#[test]
fn delegation_requires_a_current_parent_and_exact_grantor() {
    let courtroom = courtroom();
    let action = view_account_details(&courtroom);
    let mut missing_parent = view_grant(&courtroom, 140);
    missing_parent.parent = Some(CapabilityGrantId::new(999).unwrap());
    missing_parent.scope.delegation = DelegationLimit::none();
    assert_denied(
        &courtroom.world.clone().with_grant(missing_parent),
        &courtroom,
        action,
        missing_parent.id,
        EstateWorkflowStage::Administration,
        None,
        EstateDenial::DelegationParentMissing,
    );

    let mut parent = view_grant(&courtroom, 141);
    parent.scope.delegation = DelegationLimit::generations(2);
    let mut wrong_grantor = view_grant(&courtroom, 142);
    wrong_grantor.parent = Some(parent.id);
    wrong_grantor.grantor = courtroom.manager;
    wrong_grantor.scope.delegation = DelegationLimit::generations(1);
    let world = courtroom
        .world
        .clone()
        .with_grant(parent)
        .with_grant(wrong_grantor);
    assert_denied(
        &world,
        &courtroom,
        action,
        wrong_grantor.id,
        EstateWorkflowStage::Administration,
        None,
        EstateDenial::DelegationGrantorMismatch,
    );
}

#[test]
fn release_requires_both_executor_relation_and_release_review_kind() {
    let courtroom = courtroom();
    let grant = grant(
        150,
        courtroom.specialist,
        EstateCapabilityOperation::ReleaseEstate,
        None,
        None,
        None,
        DelegationLimit::none(),
    );
    let action = EstateAction::ReleaseEstate {
        estate: courtroom.estate,
    };
    let world_without_executor = courtroom
        .world
        .clone()
        .with_legal_authority(EstateLegalAuthority {
            id: LegalAuthorityId::new(30).unwrap(),
            estate: courtroom.estate,
            holder: courtroom.manager,
            kind: LegalAuthorityKind::CourtAppointment,
            recognized: true,
        })
        .with_grant(grant);
    assert_denied(
        &world_without_executor,
        &courtroom,
        action,
        grant.id,
        EstateWorkflowStage::Administration,
        None,
        EstateDenial::MandatoryReviewIncomplete,
    );

    let world_with_wrong_review = courtroom
        .world
        .clone()
        .with_review(MandatoryEstateReview {
            id: MandatoryReviewId::new(20).unwrap(),
            estate: courtroom.estate,
            kind: MandatoryReviewKind::EmergencyAccess,
            reviewer: Some(courtroom.specialist),
            status: MandatoryReviewStatus::Completed,
        })
        .with_grant(grant);
    assert_denied(
        &world_with_wrong_review,
        &courtroom,
        action,
        grant.id,
        EstateWorkflowStage::Administration,
        None,
        EstateDenial::MandatoryReviewIncomplete,
    );
}

#[test]
fn emergency_reviewer_cannot_be_the_approver() {
    let courtroom = courtroom();
    let grant = view_grant(&courtroom, 160);
    let access = EmergencyAccessId::new(161).unwrap();
    let world = courtroom
        .world
        .clone()
        .with_review(MandatoryEstateReview {
            id: courtroom.emergency_review,
            estate: courtroom.estate,
            kind: MandatoryReviewKind::EmergencyAccess,
            reviewer: Some(courtroom.manager),
            status: MandatoryReviewStatus::Completed,
        })
        .with_grant(grant)
        .with_emergency_access(EstateEmergencyAccess {
            id: access,
            requester: courtroom.specialist,
            approver: Some(courtroom.manager),
            reviewer: Some(courtroom.manager),
            grant: grant.id,
            review: courtroom.emergency_review,
            reason: EmergencyAccessReason::PreventImmediateLoss,
            status: EmergencyAccessStatus::Revoked,
            issued_at: EstateMoment::from_epoch_seconds(100),
            expires_at: EstateMoment::from_epoch_seconds(200),
        });
    assert_denied(
        &world,
        &courtroom,
        view_account_details(&courtroom),
        grant.id,
        EstateWorkflowStage::Administration,
        Some(access),
        EstateDenial::EmergencyReviewerConflict,
    );
}

#[test]
fn employee_executor_cannot_complete_their_own_release_review() {
    let courtroom = courtroom();
    let grant = grant(
        165,
        courtroom.specialist,
        EstateCapabilityOperation::CompleteMandatoryReview,
        None,
        None,
        None,
        DelegationLimit::none(),
    );
    let world = courtroom
        .world
        .clone()
        .with_executor(courtroom.estate, courtroom.specialist)
        .with_grant(grant);
    assert_denied(
        &world,
        &courtroom,
        EstateAction::CompleteMandatoryReview {
            estate: courtroom.estate,
            access: EmergencyAccessId::new(166).unwrap(),
            review: MandatoryReviewId::new(20).unwrap(),
        },
        grant.id,
        EstateWorkflowStage::Administration,
        None,
        EstateDenial::SeparationOfDutyConflict,
    );
}

#[test]
fn balanced_shape_still_denies_when_estate_funds_are_insufficient() {
    let courtroom = courtroom();
    let grant = grant(
        170,
        courtroom.specialist,
        EstateCapabilityOperation::DisburseEstate,
        Some(courtroom.source),
        None,
        Some(200_000),
        DelegationLimit::none(),
    );
    let amount = 150_000;
    let action = EstateAction::DisburseEstate(EstateDisbursement {
        estate: courtroom.estate,
        source_account: courtroom.source,
        destination_account: courtroom.destination,
        beneficiary: courtroom.beneficiary,
        amount: Money::from_minor(amount).unwrap(),
        postings: [
            EstatePosting {
                account: courtroom.source,
                amount: SignedMoney::from_minor(-amount),
            },
            EstatePosting {
                account: courtroom.destination,
                amount: SignedMoney::from_minor(amount),
            },
        ],
    });
    assert_denied(
        &courtroom.world.clone().with_grant(grant),
        &courtroom,
        action,
        grant.id,
        EstateWorkflowStage::Administration,
        None,
        EstateDenial::InsufficientEstateFunds,
    );
}

fn view_grant(courtroom: &Courtroom, id: u64) -> EstateCapabilityGrant {
    grant(
        id,
        courtroom.specialist,
        EstateCapabilityOperation::ViewRestrictedEstate,
        None,
        Some(RestrictedBankField::AccountDetails),
        None,
        DelegationLimit::none(),
    )
}

fn view_account_details(courtroom: &Courtroom) -> EstateAction {
    EstateAction::ViewRestrictedEstate {
        estate: courtroom.estate,
        field: RestrictedBankField::AccountDetails,
        purpose: EstateCapabilityPurpose::EstateAdministration,
    }
}

fn assert_denied(
    world: &BankEstateWorld,
    courtroom: &Courtroom,
    action: EstateAction,
    grant: CapabilityGrantId,
    workflow_stage: EstateWorkflowStage,
    emergency_access: Option<EmergencyAccessId>,
    expected: EstateDenial,
) {
    let actor = EstateActorContext {
        principal: courtroom.specialist,
        assignment: courtroom.specialist_assignment,
    };
    let capability_use = EstateCapabilityUse {
        grant,
        workflow_stage,
        now: NOW,
        emergency_access,
    };
    assert_eq!(
        BankEstateOracles::evaluate(world, actor, action, capability_use),
        EstateDecision::Denied(expected)
    );
}
