use crate::estate::{
    BankEstateWorld, CapabilityGrantStatus, EmergencyAccessStatus, EstateAction,
    EstateActorContext, EstateCapabilityUse, EstateCase, EstateDenial,
};

pub(super) fn validate(
    world: &BankEstateWorld,
    actor: EstateActorContext,
    action: EstateAction,
    capability_use: EstateCapabilityUse,
    estate: &EstateCase,
) -> Result<(), EstateDenial> {
    let grant = world
        .grant(capability_use.grant)
        .ok_or(EstateDenial::UnknownGrant)?;
    if grant.status != CapabilityGrantStatus::Active {
        return Err(EstateDenial::GrantRevoked);
    }
    if grant.grantee != actor.principal {
        return Err(EstateDenial::GrantPrincipalMismatch);
    }
    if !grant.scope.validity.contains(capability_use.now) {
        return Err(EstateDenial::GrantExpired);
    }
    if grant.scope.estate != estate.id
        || grant.scope.institution != estate.institution
        || grant.scope.branch != estate.branch
        || grant.scope.operation != action.operation()
        || grant.scope.purpose != action.purpose()
        || grant.scope.workflow_stage != capability_use.workflow_stage
        || grant.scope.workflow_stage != estate.stage
        || grant.scope.account != action.account()
        || grant.scope.field != action.field()
        || !amount_matches(action, grant.scope.amount_ceiling)
    {
        return Err(EstateDenial::GrantScopeMismatch);
    }
    validate_delegation(world, grant)?;
    validate_emergency(world, actor, capability_use, estate)?;
    Ok(())
}

fn validate_delegation(
    world: &BankEstateWorld,
    grant: &crate::estate::EstateCapabilityGrant,
) -> Result<(), EstateDenial> {
    let Some(parent_id) = grant.parent else {
        return Ok(());
    };
    let parent = world
        .grant(parent_id)
        .ok_or(EstateDenial::DelegationParentMissing)?;
    if parent.status != CapabilityGrantStatus::Active {
        return Err(EstateDenial::GrantRevoked);
    }
    if parent.grantee != grant.grantor {
        return Err(EstateDenial::DelegationGrantorMismatch);
    }
    if !grant.scope.is_within(parent.scope) {
        return Err(EstateDenial::DelegationWidensAuthority);
    }
    Ok(())
}

fn validate_emergency(
    world: &BankEstateWorld,
    actor: EstateActorContext,
    capability_use: EstateCapabilityUse,
    estate: &EstateCase,
) -> Result<(), EstateDenial> {
    let Some(access_id) = capability_use.emergency_access else {
        return Ok(());
    };
    let access = super::integrity::validate_access(world, access_id, estate)?;
    if access.grant != capability_use.grant || access.requester != actor.principal {
        return Err(EstateDenial::EmergencyGrantMismatch);
    }
    let approver = access
        .approver
        .ok_or(EstateDenial::EmergencyAccessInactive)?;
    if approver == actor.principal || approver == access.requester {
        return Err(EstateDenial::EmergencySelfApproval);
    }
    if access
        .reviewer
        .is_some_and(|reviewer| reviewer == access.requester || reviewer == approver)
    {
        return Err(EstateDenial::EmergencyReviewerConflict);
    }
    match access.status {
        EmergencyAccessStatus::Approved
            if access.issued_at.epoch_seconds() <= capability_use.now.epoch_seconds()
                && capability_use.now.epoch_seconds() < access.expires_at.epoch_seconds() =>
        {
            Ok(())
        }
        EmergencyAccessStatus::Expired | EmergencyAccessStatus::Revoked
            if world.review(access.review).is_some_and(|review| {
                review.status == super::super::MandatoryReviewStatus::Required
            }) =>
        {
            Err(EstateDenial::EmergencyReviewRequired)
        }
        EmergencyAccessStatus::Requested
        | EmergencyAccessStatus::Approved
        | EmergencyAccessStatus::Expired
        | EmergencyAccessStatus::Revoked => Err(EstateDenial::EmergencyAccessInactive),
    }
}

fn amount_matches(
    action: EstateAction,
    ceiling: Option<crate::model::Money<crate::model::USD>>,
) -> bool {
    match (action.amount(), ceiling) {
        (None, None) => true,
        (Some(amount), Some(ceiling)) => amount.minor_units() <= ceiling.minor_units(),
        (None, Some(_)) | (Some(_), None) => false,
    }
}
