use bank_domain::estate::*;

#[path = "estate_capability_courtroom/support.rs"]
mod support;

use support::{courtroom, grant, Courtroom};

const NOW: EstateMoment = EstateMoment::from_epoch_seconds(150);

#[test]
fn elevation_grant_and_requester_mismatch_share_one_fail_closed_boundary() {
    let courtroom = courtroom();
    let grant = view_grant(&courtroom, 180);
    let alternate = view_grant(&courtroom, 181);
    let grant_mismatch = EmergencyAccessId::new(182).unwrap();
    let world = courtroom
        .world
        .clone()
        .with_grant(grant)
        .with_grant(alternate)
        .with_emergency_access(access(
            &courtroom,
            grant_mismatch,
            alternate.id,
            courtroom.specialist,
            EmergencyAccessStatus::Approved,
        ));
    assert_denied(
        &world,
        &courtroom,
        grant.id,
        grant_mismatch,
        EstateDenial::EmergencyGrantMismatch,
    );

    let requester_mismatch = EmergencyAccessId::new(183).unwrap();
    let world = world.with_emergency_access(access(
        &courtroom,
        requester_mismatch,
        grant.id,
        courtroom.manager,
        EmergencyAccessStatus::Approved,
    ));
    assert_denied(
        &world,
        &courtroom,
        grant.id,
        requester_mismatch,
        EstateDenial::EmergencyGrantMismatch,
    );
}

#[test]
fn inactive_and_review_required_elevations_remain_distinct() {
    let courtroom = courtroom();
    let grant = view_grant(&courtroom, 190);
    let inactive = EmergencyAccessId::new(191).unwrap();
    let review_required = EmergencyAccessId::new(192).unwrap();
    let world = courtroom
        .world
        .clone()
        .with_grant(grant)
        .with_emergency_access(access(
            &courtroom,
            inactive,
            grant.id,
            courtroom.specialist,
            EmergencyAccessStatus::Requested,
        ))
        .with_emergency_access(access(
            &courtroom,
            review_required,
            grant.id,
            courtroom.specialist,
            EmergencyAccessStatus::Revoked,
        ));
    assert_denied(
        &world,
        &courtroom,
        grant.id,
        inactive,
        EstateDenial::EmergencyAccessInactive,
    );
    assert_denied(
        &world,
        &courtroom,
        grant.id,
        review_required,
        EstateDenial::EmergencyReviewRequired,
    );
}

#[test]
fn approved_elevation_is_active_only_inside_its_own_time_window() {
    let courtroom = courtroom();
    let grant = view_grant(&courtroom, 193);
    let active = EmergencyAccessId::new(194).unwrap();
    let expired = EmergencyAccessId::new(195).unwrap();
    let expired_access = EstateEmergencyAccess {
        expires_at: NOW,
        ..access(
            &courtroom,
            expired,
            grant.id,
            courtroom.specialist,
            EmergencyAccessStatus::Approved,
        )
    };
    let world = courtroom
        .world
        .clone()
        .with_grant(grant)
        .with_emergency_access(access(
            &courtroom,
            active,
            grant.id,
            courtroom.specialist,
            EmergencyAccessStatus::Approved,
        ))
        .with_emergency_access(expired_access);

    assert_eq!(
        decision(&world, &courtroom, grant.id, active),
        EstateDecision::Allowed
    );
    assert_eq!(
        decision(&world, &courtroom, grant.id, expired),
        EstateDecision::Denied(EstateDenial::EmergencyAccessInactive)
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

fn access(
    courtroom: &Courtroom,
    id: EmergencyAccessId,
    grant: CapabilityGrantId,
    requester: bank_domain::model::BankPrincipalId,
    status: EmergencyAccessStatus,
) -> EstateEmergencyAccess {
    EstateEmergencyAccess {
        id,
        requester,
        approver: Some(courtroom.manager),
        reviewer: None,
        grant,
        review: courtroom.emergency_review,
        reason: EmergencyAccessReason::PreventImmediateLoss,
        status,
        issued_at: EstateMoment::from_epoch_seconds(100),
        expires_at: EstateMoment::from_epoch_seconds(200),
    }
}

fn assert_denied(
    world: &BankEstateWorld,
    courtroom: &Courtroom,
    grant: CapabilityGrantId,
    emergency_access: EmergencyAccessId,
    expected: EstateDenial,
) {
    assert_eq!(
        decision(world, courtroom, grant, emergency_access),
        EstateDecision::Denied(expected)
    );
}

fn decision(
    world: &BankEstateWorld,
    courtroom: &Courtroom,
    grant: CapabilityGrantId,
    emergency_access: EmergencyAccessId,
) -> EstateDecision {
    let actor = EstateActorContext {
        principal: courtroom.specialist,
        assignment: courtroom.specialist_assignment,
    };
    let action = EstateAction::ViewRestrictedEstate {
        estate: courtroom.estate,
        field: RestrictedBankField::AccountDetails,
        purpose: EstateCapabilityPurpose::EstateAdministration,
    };
    let capability_use = EstateCapabilityUse {
        grant,
        workflow_stage: EstateWorkflowStage::Administration,
        now: NOW,
        emergency_access: Some(emergency_access),
    };
    BankEstateOracles::evaluate(world, actor, action, capability_use)
}
