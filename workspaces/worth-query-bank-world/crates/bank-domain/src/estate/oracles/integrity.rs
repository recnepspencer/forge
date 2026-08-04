use crate::estate::{
    BankEstateWorld, CapabilityGrantStatus, EmergencyAccessStatus, EstateAction,
    EstateActorContext, EstateCapabilityUse, EstateCase, EstateCaseStatus, EstateDenial,
    EstateEmployeeAssignment, MandatoryReviewKind, MandatoryReviewStatus,
};

pub(super) fn resolve_estate(
    world: &BankEstateWorld,
    action: EstateAction,
    capability_use: EstateCapabilityUse,
) -> Result<&EstateCase, EstateDenial> {
    let estate = action
        .estate()
        .or_else(|| {
            world
                .grant(capability_use.grant)
                .map(|grant| grant.scope.estate)
        })
        .ok_or(EstateDenial::UnknownEstate)?;
    let case = world.case(estate).ok_or(EstateDenial::UnknownEstate)?;
    if case.status == EstateCaseStatus::Closed {
        return Err(EstateDenial::EstateClosed);
    }
    Ok(case)
}

pub(super) fn validate_actor<'a>(
    world: &'a BankEstateWorld,
    actor: EstateActorContext,
    estate: &EstateCase,
) -> Result<&'a EstateEmployeeAssignment, EstateDenial> {
    let branch = world
        .branch(estate.branch)
        .ok_or(EstateDenial::EmployeeAssignmentMismatch)?;
    let notice = world
        .death_notice(estate.death_notice)
        .ok_or(EstateDenial::LegalAuthorityMissing)?;
    let assignment = world
        .assignment(actor.assignment)
        .ok_or(EstateDenial::UnknownEmployeeAssignment)?;
    if branch.institution != estate.institution
        || notice.subject != estate.deceased
        || assignment.principal != actor.principal
        || assignment.institution != estate.institution
        || assignment.branch != estate.branch
    {
        return Err(EstateDenial::EmployeeAssignmentMismatch);
    }
    Ok(assignment)
}

pub(super) fn validate_action(
    world: &BankEstateWorld,
    action: EstateAction,
    actor: EstateActorContext,
    estate: &EstateCase,
) -> Result<(), EstateDenial> {
    validate_estate_record_action(world, action, estate)?;
    validate_legal_action(world, action, actor, estate)?;
    validate_capability_action(world, action, estate)?;
    validate_emergency_action(world, action, actor, estate)?;
    validate_disbursement(world, action, estate)
}

fn validate_estate_record_action(
    world: &BankEstateWorld,
    action: EstateAction,
    estate: &EstateCase,
) -> Result<(), EstateDenial> {
    match action {
        EstateAction::NotifyDeath {
            notice, subject, ..
        } => {
            let notice = world
                .death_notice(notice)
                .ok_or(EstateDenial::LegalAuthorityMissing)?;
            if notice.id != estate.death_notice
                || notice.subject != subject
                || subject != estate.deceased
            {
                return Err(EstateDenial::LegalAuthorityMismatch);
            }
        }
        EstateAction::FreezeAccount {
            estate: action_estate,
            account,
        } => {
            if action_estate != estate.id || account != estate.account {
                return Err(EstateDenial::LegalAuthorityMismatch);
            }
        }
        EstateAction::OpenEstateCase {
            estate: action_estate,
            notice,
        } => {
            let notice = world
                .death_notice(notice)
                .ok_or(EstateDenial::LegalAuthorityMissing)?;
            if action_estate != estate.id
                || notice.id != estate.death_notice
                || notice.status != crate::estate::DeathNoticeStatus::Verified
            {
                return Err(EstateDenial::LegalAuthorityMismatch);
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_legal_action(
    world: &BankEstateWorld,
    action: EstateAction,
    actor: EstateActorContext,
    estate: &EstateCase,
) -> Result<(), EstateDenial> {
    match action {
        EstateAction::RecognizeExecutor {
            estate: action_estate,
            executor,
            authority,
        } => validate_executor_recognition(world, action_estate, executor, authority, estate)?,
        EstateAction::ReleaseEstate {
            estate: action_estate,
        } => {
            if action_estate != estate.id
                || !world.has_recognized_executor(estate.id)
                || !world.has_completed_review(estate.id, MandatoryReviewKind::EstateRelease)
            {
                return Err(EstateDenial::MandatoryReviewIncomplete);
            }
        }
        EstateAction::CompleteMandatoryReview { review, .. } => {
            let review = world
                .review(review)
                .ok_or(EstateDenial::MandatoryReviewIncomplete)?;
            if review.estate != estate.id
                || review.status != MandatoryReviewStatus::Completed
                || review.reviewer != Some(actor.principal)
            {
                return Err(EstateDenial::MandatoryReviewIncomplete);
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_executor_recognition(
    world: &BankEstateWorld,
    action_estate: crate::estate::EstateCaseId,
    executor: crate::model::BankPrincipalId,
    authority: crate::estate::LegalAuthorityId,
    estate: &EstateCase,
) -> Result<(), EstateDenial> {
    let authority = world
        .legal_authority(authority)
        .ok_or(EstateDenial::LegalAuthorityMissing)?;
    if action_estate != estate.id
        || authority.estate != estate.id
        || authority.holder != executor
        || !authority.recognized
    {
        return Err(EstateDenial::LegalAuthorityMismatch);
    }
    Ok(())
}

fn validate_capability_action(
    world: &BankEstateWorld,
    action: EstateAction,
    estate: &EstateCase,
) -> Result<(), EstateDenial> {
    match action {
        EstateAction::DelegateCapability { parent, child, .. } => {
            let parent = world
                .grant(parent)
                .ok_or(EstateDenial::DelegationParentMissing)?;
            let child = world.grant(child).ok_or(EstateDenial::UnknownGrant)?;
            if parent.scope.estate != estate.id
                || child.scope.estate != estate.id
                || child.parent != Some(parent.id)
                || parent.status != CapabilityGrantStatus::Active
                || child.status != CapabilityGrantStatus::Active
            {
                return Err(EstateDenial::GrantScopeMismatch);
            }
            if parent.grantee != child.grantor {
                return Err(EstateDenial::DelegationGrantorMismatch);
            }
            if !child.scope.is_within(parent.scope) {
                return Err(EstateDenial::DelegationWidensAuthority);
            }
        }
        EstateAction::RevokeCapability { grant, .. } => {
            validate_target_grant(world, grant, estate)?;
        }
        _ => {}
    }
    Ok(())
}

fn validate_emergency_action(
    world: &BankEstateWorld,
    action: EstateAction,
    actor: EstateActorContext,
    estate: &EstateCase,
) -> Result<(), EstateDenial> {
    match action {
        EstateAction::RequestEmergencyAccess { access, .. }
        | EstateAction::ApproveEmergencyAccess { access, .. }
        | EstateAction::RevokeEmergencyAccess { access, .. } => {
            let access = validate_access(world, access, estate)?;
            if matches!(action, EstateAction::RequestEmergencyAccess { .. })
                && (access.requester != actor.principal
                    || access.status != EmergencyAccessStatus::Requested)
            {
                return Err(EstateDenial::EmergencyGrantMismatch);
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_disbursement(
    world: &BankEstateWorld,
    action: EstateAction,
    estate: &EstateCase,
) -> Result<(), EstateDenial> {
    if let EstateAction::DisburseEstate(disbursement) = action {
        if disbursement.estate != estate.id
            || disbursement.source_account != estate.account
            || !world.has_recognized_executor(estate.id)
        {
            return Err(EstateDenial::LegalAuthorityMismatch);
        }
        if !world.is_beneficiary(estate.id, disbursement.beneficiary)
            || !world.is_joint_owner(disbursement.destination_account, disbursement.beneficiary)
        {
            return Err(EstateDenial::LegalAuthorityMismatch);
        }
    }
    Ok(())
}

fn validate_target_grant(
    world: &BankEstateWorld,
    grant: crate::estate::CapabilityGrantId,
    estate: &EstateCase,
) -> Result<(), EstateDenial> {
    let grant = world.grant(grant).ok_or(EstateDenial::UnknownGrant)?;
    if grant.scope.estate != estate.id
        || grant.scope.institution != estate.institution
        || grant.scope.branch != estate.branch
    {
        return Err(EstateDenial::GrantScopeMismatch);
    }
    Ok(())
}

pub(super) fn validate_access<'a>(
    world: &'a BankEstateWorld,
    access: crate::estate::EmergencyAccessId,
    estate: &EstateCase,
) -> Result<&'a crate::estate::EstateEmergencyAccess, EstateDenial> {
    let access = world
        .emergency_access(access)
        .ok_or(EstateDenial::EmergencyAccessMissing)?;
    validate_target_grant(world, access.grant, estate)?;
    let review = world
        .review(access.review)
        .ok_or(EstateDenial::MandatoryReviewIncomplete)?;
    let review_state_matches = match access.status {
        EmergencyAccessStatus::Requested | EmergencyAccessStatus::Approved => {
            review.status == MandatoryReviewStatus::Required && review.reviewer.is_none()
        }
        EmergencyAccessStatus::Expired | EmergencyAccessStatus::Revoked => match review.status {
            MandatoryReviewStatus::Required => review.reviewer.is_none(),
            MandatoryReviewStatus::Completed => review.reviewer.is_some(),
        },
    };
    if review.estate != estate.id
        || review.kind != MandatoryReviewKind::EmergencyAccess
        || access.reviewer != review.reviewer
        || !review_state_matches
    {
        return Err(EstateDenial::MandatoryReviewIncomplete);
    }
    Ok(access)
}
