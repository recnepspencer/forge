mod accounting;
mod capability;
mod conflict;
mod disclosure;
mod integrity;
mod role;

use crate::model::{BankPrincipalId, EmployeeAssignmentId};

use super::{
    CapabilityGrantId, EmergencyAccessId, EstateAction, EstateMoment, EstateWorkflowStage,
};
use crate::estate::BankEstateWorld;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateActorContext {
    pub principal: BankPrincipalId,
    pub assignment: EmployeeAssignmentId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateCapabilityUse {
    pub grant: CapabilityGrantId,
    pub workflow_stage: EstateWorkflowStage,
    pub now: EstateMoment,
    pub emergency_access: Option<EmergencyAccessId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstateDenial {
    UnknownEstate,
    EstateClosed,
    UnknownEmployeeAssignment,
    EmployeeAssignmentMismatch,
    EmployeeRoleMismatch,
    UnknownGrant,
    GrantRevoked,
    GrantPrincipalMismatch,
    GrantScopeMismatch,
    GrantExpired,
    DelegationParentMissing,
    DelegationGrantorMismatch,
    DelegationWidensAuthority,
    EmergencyAccessMissing,
    EmergencyAccessInactive,
    EmergencyGrantMismatch,
    EmergencySelfApproval,
    EmergencyReviewerConflict,
    EmergencyReviewRequired,
    DisclosurePurposeMismatch,
    BeneficiaryConflict,
    SeparationOfDutyConflict,
    LegalAuthorityMissing,
    LegalAuthorityMismatch,
    MandatoryReviewIncomplete,
    AccountingShapeInvalid,
    InsufficientEstateFunds,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstateDecision {
    Allowed,
    Denied(EstateDenial),
}

pub struct BankEstateOracles;

impl BankEstateOracles {
    pub fn evaluate(
        world: &BankEstateWorld,
        actor: EstateActorContext,
        action: EstateAction,
        capability_use: EstateCapabilityUse,
    ) -> EstateDecision {
        let evaluation = || {
            let estate = integrity::resolve_estate(world, action, capability_use)?;
            let assignment = integrity::validate_actor(world, actor, estate)?;
            integrity::validate_action(world, action, actor, estate)?;
            conflict::validate(world, actor, action, capability_use, estate)?;
            role::validate(assignment.role, action)?;
            capability::validate(world, actor, action, capability_use, estate)?;
            disclosure::validate(action)?;
            accounting::validate(world, action)?;
            Ok(())
        };
        match evaluation() {
            Ok(()) => EstateDecision::Allowed,
            Err(denial) => EstateDecision::Denied(denial),
        }
    }
}
