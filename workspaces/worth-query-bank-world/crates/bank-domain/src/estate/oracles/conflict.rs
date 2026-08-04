use crate::estate::{
    BankEstateWorld, EstateAction, EstateActorContext, EstateCapabilityUse, EstateCase,
    EstateDenial,
};

pub(super) fn validate(
    world: &BankEstateWorld,
    actor: EstateActorContext,
    action: EstateAction,
    _capability_use: EstateCapabilityUse,
    estate: &EstateCase,
) -> Result<(), EstateDenial> {
    if let EstateAction::ApproveEmergencyAccess { access, .. } = action {
        let access = world
            .emergency_access(access)
            .ok_or(EstateDenial::EmergencyAccessMissing)?;
        if access.requester == actor.principal || access.approver != Some(actor.principal) {
            return Err(EstateDenial::EmergencySelfApproval);
        }
    }
    let beneficiary_conflict = world.is_beneficiary(estate.id, actor.principal);
    if beneficiary_conflict
        && matches!(
            action,
            EstateAction::ViewRestrictedEstate { .. }
                | EstateAction::ApproveEmergencyAccess { .. }
                | EstateAction::CompleteMandatoryReview { .. }
                | EstateAction::ReleaseEstate { .. }
                | EstateAction::DisburseEstate(_)
        )
    {
        return Err(EstateDenial::BeneficiaryConflict);
    }
    let actor_is_executor = world.is_executor(estate.id, actor.principal);
    let actor_recognizes_self = matches!(
        action,
        EstateAction::RecognizeExecutor { executor, .. } if executor == actor.principal
    );
    if actor_recognizes_self
        || (actor_is_executor
            && matches!(
                action,
                EstateAction::CompleteMandatoryReview { .. }
                    | EstateAction::ReleaseEstate { .. }
                    | EstateAction::DisburseEstate(_)
            ))
    {
        return Err(EstateDenial::SeparationOfDutyConflict);
    }
    Ok(())
}
