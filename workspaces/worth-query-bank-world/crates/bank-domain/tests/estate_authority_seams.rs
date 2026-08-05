use bank_domain::estate::*;
use bank_domain::model::AccountId;

#[path = "estate_capability_courtroom/support.rs"]
mod support;

use support::{courtroom, grant, Courtroom};

const NOW: EstateMoment = EstateMoment::from_epoch_seconds(150);

#[test]
fn case_opening_requires_a_verified_notice_and_distinct_pre_open_posture() {
    let courtroom = courtroom();
    let opening_grant = grant(
        80,
        courtroom.specialist,
        EstateCapabilityOperation::OpenEstateCase,
        None,
        None,
        None,
        DelegationLimit::none(),
    );
    let action = EstateAction::OpenEstateCase {
        estate: courtroom.estate,
        notice: courtroom
            .world
            .case(courtroom.estate)
            .expect("the canonical courtroom owns its estate")
            .death_notice,
    };
    let mut pending_case = *courtroom
        .world
        .case(courtroom.estate)
        .expect("the canonical courtroom owns its estate");
    pending_case.status = EstateCaseStatus::PendingOpening;
    let pending_world = courtroom
        .world
        .clone()
        .with_case(pending_case)
        .with_grant(opening_grant);
    assert_eq!(
        BankEstateOracles::evaluate(
            &pending_world,
            specialist(&courtroom),
            action,
            capability_use(opening_grant.id, None),
        ),
        EstateDecision::Allowed
    );

    let already_open = courtroom.world.clone().with_grant(opening_grant);
    assert_denied(
        &already_open,
        specialist(&courtroom),
        action,
        opening_grant.id,
        None,
        EstateDenial::LegalAuthorityMismatch,
    );
}

#[test]
fn administrative_targets_cannot_escape_the_capability_estate() {
    let courtroom = courtroom();
    let revoke_authority = grant(
        90,
        courtroom.specialist,
        EstateCapabilityOperation::RevokeCapability,
        None,
        None,
        None,
        DelegationLimit::none(),
    );
    let mut foreign_target = grant(
        91,
        courtroom.manager,
        EstateCapabilityOperation::ViewRestrictedEstate,
        None,
        Some(RestrictedBankField::AccountDetails),
        None,
        DelegationLimit::none(),
    );
    foreign_target.scope.estate = EstateCaseId::new(999).unwrap();
    let world = courtroom
        .world
        .clone()
        .with_grant(revoke_authority)
        .with_grant(foreign_target);
    assert_denied(
        &world,
        specialist(&courtroom),
        EstateAction::RevokeCapability {
            estate: courtroom.estate,
            grant: foreign_target.id,
        },
        revoke_authority.id,
        None,
        EstateDenial::GrantScopeMismatch,
    );

    let freeze_authority = grant(
        92,
        courtroom.specialist,
        EstateCapabilityOperation::FreezeAccount,
        Some(AccountId::new(998).unwrap()),
        None,
        None,
        DelegationLimit::none(),
    );
    let world = world.with_grant(freeze_authority);
    assert_denied(
        &world,
        specialist(&courtroom),
        EstateAction::FreezeAccount {
            estate: courtroom.estate,
            account: AccountId::new(998).unwrap(),
        },
        freeze_authority.id,
        None,
        EstateDenial::LegalAuthorityMismatch,
    );
}

#[test]
fn role_and_capability_are_independent_required_contributions() {
    let courtroom = courtroom();
    let teller_grant = grant(
        100,
        courtroom.teller,
        EstateCapabilityOperation::ViewRestrictedEstate,
        None,
        Some(RestrictedBankField::AccountDetails),
        None,
        DelegationLimit::none(),
    );
    let world = courtroom.world.clone().with_grant(teller_grant);
    assert_denied(
        &world,
        EstateActorContext {
            principal: courtroom.teller,
            assignment: courtroom.teller_assignment,
        },
        EstateAction::ViewRestrictedEstate {
            estate: courtroom.estate,
            field: RestrictedBankField::AccountDetails,
            purpose: EstateCapabilityPurpose::EstateAdministration,
        },
        teller_grant.id,
        None,
        EstateDenial::EmployeeRoleMismatch,
    );
}

#[test]
fn disclosure_purpose_is_an_independent_oracle() {
    let courtroom = courtroom();
    let grant = grant(
        110,
        courtroom.specialist,
        EstateCapabilityOperation::ViewRestrictedEstate,
        None,
        Some(RestrictedBankField::AuditTrail),
        None,
        DelegationLimit::none(),
    );
    let world = courtroom.world.clone().with_grant(grant);
    assert_denied(
        &world,
        specialist(&courtroom),
        EstateAction::ViewRestrictedEstate {
            estate: courtroom.estate,
            field: RestrictedBankField::AuditTrail,
            purpose: EstateCapabilityPurpose::EstateAdministration,
        },
        grant.id,
        None,
        EstateDenial::DisclosurePurposeMismatch,
    );
}

#[test]
fn active_elevation_requires_distinct_actors_and_an_exact_review() {
    let courtroom = courtroom();
    let grant = grant(
        120,
        courtroom.specialist,
        EstateCapabilityOperation::ViewRestrictedEstate,
        None,
        Some(RestrictedBankField::AccountDetails),
        None,
        DelegationLimit::none(),
    );
    let missing_review = EmergencyAccessId::new(121).unwrap();
    let world = courtroom
        .world
        .clone()
        .with_grant(grant)
        .with_emergency_access(EstateEmergencyAccess {
            id: missing_review,
            requester: courtroom.specialist,
            approver: Some(courtroom.manager),
            reviewer: None,
            grant: grant.id,
            review: MandatoryReviewId::new(999).unwrap(),
            reason: EmergencyAccessReason::PreventImmediateLoss,
            status: EmergencyAccessStatus::Approved,
            issued_at: EstateMoment::from_epoch_seconds(100),
            expires_at: EstateMoment::from_epoch_seconds(200),
        });
    assert_denied(
        &world,
        specialist(&courtroom),
        view_account_details(&courtroom),
        grant.id,
        Some(missing_review),
        EstateDenial::MandatoryReviewIncomplete,
    );

    let missing_approver = EmergencyAccessId::new(122).unwrap();
    let world = world.with_emergency_access(EstateEmergencyAccess {
        id: missing_approver,
        requester: courtroom.specialist,
        approver: None,
        reviewer: None,
        grant: grant.id,
        review: courtroom.emergency_review,
        reason: EmergencyAccessReason::PreventImmediateLoss,
        status: EmergencyAccessStatus::Approved,
        issued_at: EstateMoment::from_epoch_seconds(100),
        expires_at: EstateMoment::from_epoch_seconds(200),
    });
    assert_denied(
        &world,
        specialist(&courtroom),
        view_account_details(&courtroom),
        grant.id,
        Some(missing_approver),
        EstateDenial::EmergencyAccessInactive,
    );

    let reviewer_mismatch = EmergencyAccessId::new(124).unwrap();
    let world = world.with_emergency_access(EstateEmergencyAccess {
        id: reviewer_mismatch,
        requester: courtroom.specialist,
        approver: Some(courtroom.manager),
        reviewer: Some(courtroom.beneficiary),
        grant: grant.id,
        review: courtroom.emergency_review,
        reason: EmergencyAccessReason::PreventImmediateLoss,
        status: EmergencyAccessStatus::Approved,
        issued_at: EstateMoment::from_epoch_seconds(100),
        expires_at: EstateMoment::from_epoch_seconds(200),
    });
    assert_denied(
        &world,
        specialist(&courtroom),
        view_account_details(&courtroom),
        grant.id,
        Some(reviewer_mismatch),
        EstateDenial::MandatoryReviewIncomplete,
    );

    let lawful = EmergencyAccessId::new(123).unwrap();
    let world = world.with_emergency_access(EstateEmergencyAccess {
        id: lawful,
        requester: courtroom.specialist,
        approver: Some(courtroom.manager),
        reviewer: None,
        grant: grant.id,
        review: courtroom.emergency_review,
        reason: EmergencyAccessReason::PreventImmediateLoss,
        status: EmergencyAccessStatus::Approved,
        issued_at: EstateMoment::from_epoch_seconds(100),
        expires_at: EstateMoment::from_epoch_seconds(200),
    });
    assert_eq!(
        BankEstateOracles::evaluate(
            &world,
            specialist(&courtroom),
            view_account_details(&courtroom),
            capability_use(grant.id, Some(lawful)),
        ),
        EstateDecision::Allowed
    );
}

fn view_account_details(courtroom: &Courtroom) -> EstateAction {
    EstateAction::ViewRestrictedEstate {
        estate: courtroom.estate,
        field: RestrictedBankField::AccountDetails,
        purpose: EstateCapabilityPurpose::EstateAdministration,
    }
}

fn specialist(courtroom: &Courtroom) -> EstateActorContext {
    EstateActorContext {
        principal: courtroom.specialist,
        assignment: courtroom.specialist_assignment,
    }
}

fn assert_denied(
    world: &BankEstateWorld,
    actor: EstateActorContext,
    action: EstateAction,
    grant: CapabilityGrantId,
    emergency_access: Option<EmergencyAccessId>,
    expected: EstateDenial,
) {
    assert_eq!(
        BankEstateOracles::evaluate(
            world,
            actor,
            action,
            capability_use(grant, emergency_access),
        ),
        EstateDecision::Denied(expected)
    );
}

fn capability_use(
    grant: CapabilityGrantId,
    emergency_access: Option<EmergencyAccessId>,
) -> EstateCapabilityUse {
    EstateCapabilityUse {
        grant,
        workflow_stage: EstateWorkflowStage::Administration,
        now: NOW,
        emergency_access,
    }
}
