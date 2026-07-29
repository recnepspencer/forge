use bank_domain::estate::*;
use bank_domain::model::{Money, SignedMoney};

#[path = "estate_capability_courtroom/support.rs"]
mod support;

use support::{courtroom, grant, Courtroom};

const NOW: EstateMoment = EstateMoment::from_epoch_seconds(150);

#[test]
fn lawful_specialist_can_recognize_release_and_disburse_without_superuser_authority() {
    let courtroom = courtroom();
    let authority = LegalAuthorityId::new(30).unwrap();
    let recognition = EstateAction::RecognizeExecutor {
        estate: courtroom.estate,
        executor: courtroom.beneficiary,
        authority,
    };
    let recognition_grant = grant(
        40,
        courtroom.specialist,
        EstateCapabilityOperation::RecognizeExecutor,
        None,
        None,
        None,
        DelegationLimit::none(),
    );
    let world = courtroom.world.clone().with_grant(recognition_grant);
    assert_allowed(&world, specialist(&courtroom), recognition, 40);

    let release_grant = grant(
        41,
        courtroom.specialist,
        EstateCapabilityOperation::ReleaseEstate,
        None,
        None,
        None,
        DelegationLimit::none(),
    );
    let world = world.with_grant(release_grant);
    assert_allowed(
        &world,
        specialist(&courtroom),
        EstateAction::ReleaseEstate {
            estate: courtroom.estate,
        },
        41,
    );

    let disbursement_grant = grant(
        42,
        courtroom.specialist,
        EstateCapabilityOperation::DisburseEstate,
        Some(courtroom.source),
        None,
        Some(50_000),
        DelegationLimit::none(),
    );
    let world = world.with_grant(disbursement_grant);
    assert_allowed(
        &world,
        specialist(&courtroom),
        balanced_disbursement(&courtroom, 25_000),
        42,
    );
}

#[test]
fn beneficiary_branch_manager_cannot_read_approve_or_disburse_for_self() {
    let courtroom = courtroom();
    let view_grant = grant(
        50,
        courtroom.manager,
        EstateCapabilityOperation::ViewRestrictedEstate,
        None,
        Some(RestrictedBankField::LegalDocument),
        None,
        DelegationLimit::none(),
    );
    let world = courtroom.world.clone().with_grant(view_grant);
    assert_denied(
        &world,
        manager(&courtroom),
        EstateAction::ViewRestrictedEstate {
            estate: courtroom.estate,
            field: RestrictedBankField::LegalDocument,
            purpose: EstateCapabilityPurpose::EstateAdministration,
        },
        50,
        None,
        EstateDenial::BeneficiaryConflict,
    );

    let approval_grant = grant(
        51,
        courtroom.manager,
        EstateCapabilityOperation::ApproveEmergencyAccess,
        None,
        None,
        None,
        DelegationLimit::none(),
    );
    let access_id = EmergencyAccessId::new(60).unwrap();
    let world = world
        .with_grant(approval_grant)
        .with_emergency_access(EstateEmergencyAccess {
            id: access_id,
            requester: courtroom.manager,
            approver: Some(courtroom.manager),
            reviewer: None,
            grant: CapabilityGrantId::new(51).unwrap(),
            review: courtroom.emergency_review,
            reason: EmergencyAccessReason::PreventImmediateLoss,
            status: EmergencyAccessStatus::Requested,
        });
    assert_denied(
        &world,
        manager(&courtroom),
        EstateAction::ApproveEmergencyAccess { access: access_id },
        51,
        None,
        EstateDenial::EmergencySelfApproval,
    );

    let disbursement_grant = grant(
        52,
        courtroom.manager,
        EstateCapabilityOperation::DisburseEstate,
        Some(courtroom.source),
        None,
        Some(50_000),
        DelegationLimit::none(),
    );
    let world = world.with_grant(disbursement_grant);
    assert_denied(
        &world,
        manager(&courtroom),
        EstateAction::DisburseEstate(EstateDisbursement {
            beneficiary: courtroom.manager,
            ..disbursement(&courtroom, 25_000)
        }),
        52,
        None,
        EstateDenial::BeneficiaryConflict,
    );
}

#[test]
fn delegation_and_emergency_access_cannot_widen_or_bypass_conflict() {
    let courtroom = courtroom();
    let parent = grant(
        70,
        courtroom.specialist,
        EstateCapabilityOperation::DisburseEstate,
        Some(courtroom.source),
        None,
        Some(10_000),
        DelegationLimit::generations(2),
    );
    let mut child = grant(
        71,
        courtroom.specialist,
        EstateCapabilityOperation::DisburseEstate,
        Some(courtroom.source),
        None,
        Some(50_000),
        DelegationLimit::generations(1),
    );
    child.grantor = courtroom.specialist;
    child.parent = Some(parent.id);
    let world = courtroom.world.clone().with_grant(parent).with_grant(child);
    assert_denied(
        &world,
        specialist(&courtroom),
        balanced_disbursement(&courtroom, 25_000),
        71,
        None,
        EstateDenial::DelegationWidensAuthority,
    );

    let emergency_grant = grant(
        72,
        courtroom.manager,
        EstateCapabilityOperation::ViewRestrictedEstate,
        None,
        Some(RestrictedBankField::AuditTrail),
        None,
        DelegationLimit::none(),
    );
    let access_id = EmergencyAccessId::new(73).unwrap();
    let world = world
        .with_grant(emergency_grant)
        .with_emergency_access(EstateEmergencyAccess {
            id: access_id,
            requester: courtroom.manager,
            approver: Some(courtroom.specialist),
            reviewer: None,
            grant: emergency_grant.id,
            review: courtroom.emergency_review,
            reason: EmergencyAccessReason::ProtectVulnerableCustomer,
            status: EmergencyAccessStatus::Active,
        });
    assert_denied(
        &world,
        manager(&courtroom),
        EstateAction::ViewRestrictedEstate {
            estate: courtroom.estate,
            field: RestrictedBankField::AuditTrail,
            purpose: EstateCapabilityPurpose::EstateAdministration,
        },
        72,
        Some(access_id),
        EstateDenial::BeneficiaryConflict,
    );
}

#[test]
fn scope_currentness_and_accounting_fail_closed_independently() {
    let courtroom = courtroom();
    let mut revoked = grant(
        80,
        courtroom.specialist,
        EstateCapabilityOperation::DisburseEstate,
        Some(courtroom.source),
        None,
        Some(50_000),
        DelegationLimit::none(),
    );
    revoked.status = CapabilityGrantStatus::Revoked;
    let world = courtroom.world.clone().with_grant(revoked);
    assert_denied(
        &world,
        specialist(&courtroom),
        balanced_disbursement(&courtroom, 25_000),
        80,
        None,
        EstateDenial::GrantRevoked,
    );

    let limited_grant = grant(
        81,
        courtroom.specialist,
        EstateCapabilityOperation::DisburseEstate,
        Some(courtroom.source),
        None,
        Some(20_000),
        DelegationLimit::none(),
    );
    let world = world.with_grant(limited_grant);
    assert_denied(
        &world,
        specialist(&courtroom),
        balanced_disbursement(&courtroom, 25_000),
        81,
        None,
        EstateDenial::GrantScopeMismatch,
    );

    let accounting_grant = grant(
        82,
        courtroom.specialist,
        EstateCapabilityOperation::DisburseEstate,
        Some(courtroom.source),
        None,
        Some(50_000),
        DelegationLimit::none(),
    );
    let world = world.with_grant(accounting_grant);
    let mut malformed = disbursement(&courtroom, 25_000);
    malformed.postings[1].amount = SignedMoney::from_minor(24_999);
    assert_denied(
        &world,
        specialist(&courtroom),
        EstateAction::DisburseEstate(malformed),
        82,
        None,
        EstateDenial::AccountingShapeInvalid,
    );
}

fn specialist(courtroom: &Courtroom) -> EstateActorContext {
    EstateActorContext {
        principal: courtroom.specialist,
        assignment: courtroom.specialist_assignment,
    }
}

fn manager(courtroom: &Courtroom) -> EstateActorContext {
    EstateActorContext {
        principal: courtroom.manager,
        assignment: courtroom.manager_assignment,
    }
}

fn balanced_disbursement(courtroom: &Courtroom, amount: i64) -> EstateAction {
    EstateAction::DisburseEstate(disbursement(courtroom, amount))
}

fn disbursement(courtroom: &Courtroom, amount: i64) -> EstateDisbursement {
    EstateDisbursement {
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
    }
}

fn assert_allowed(
    world: &BankEstateWorld,
    actor: EstateActorContext,
    action: EstateAction,
    grant: u64,
) {
    assert_eq!(
        BankEstateOracles::evaluate(world, actor, action, capability_use(grant, None)),
        EstateDecision::Allowed
    );
}

fn assert_denied(
    world: &BankEstateWorld,
    actor: EstateActorContext,
    action: EstateAction,
    grant: u64,
    emergency_access: Option<EmergencyAccessId>,
    expected: EstateDenial,
) {
    assert_eq!(
        BankEstateOracles::evaluate(
            world,
            actor,
            action,
            capability_use(grant, emergency_access)
        ),
        EstateDecision::Denied(expected)
    );
}

fn capability_use(grant: u64, emergency_access: Option<EmergencyAccessId>) -> EstateCapabilityUse {
    EstateCapabilityUse {
        grant: CapabilityGrantId::new(grant).unwrap(),
        workflow_stage: EstateWorkflowStage::Administration,
        now: NOW,
        emergency_access,
    }
}
