use crate::estate::{
    CapabilityGrantId, CapabilityGrantStatus, DelegationLimit, EmergencyAccessId,
    EmergencyAccessReason, EmergencyAccessStatus, EstateCapabilityOperation,
    EstateCapabilityPurpose, EstateCaseId, EstateMoment, EstateWorkflowStage,
};
use crate::model::BankPrincipalId;

use super::EstateAssignmentView;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateEmergencyContext {
    id: EmergencyAccessId,
    reason: EmergencyAccessReason,
    status: EmergencyAccessStatus,
    requester: BankPrincipalId,
    approver: Option<BankPrincipalId>,
}

impl EstateEmergencyContext {
    pub(crate) const fn from_projection(
        id: EmergencyAccessId,
        reason: EmergencyAccessReason,
        status: EmergencyAccessStatus,
        requester: BankPrincipalId,
        approver: Option<BankPrincipalId>,
    ) -> Self {
        Self {
            id,
            reason,
            status,
            requester,
            approver,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstateCapabilityContext {
    id: CapabilityGrantId,
    operation: EstateCapabilityOperation,
    purpose: EstateCapabilityPurpose,
    valid_from: EstateMoment,
    valid_through: EstateMoment,
    delegation: DelegationLimit,
    workflow_stage: EstateWorkflowStage,
    status: CapabilityGrantStatus,
    grantee: BankPrincipalId,
    grantor: BankPrincipalId,
    emergencies: Vec<EstateEmergencyContext>,
}

pub(crate) struct EstateCapabilityProjection {
    pub(crate) id: CapabilityGrantId,
    pub(crate) operation: EstateCapabilityOperation,
    pub(crate) purpose: EstateCapabilityPurpose,
    pub(crate) valid_from: EstateMoment,
    pub(crate) valid_through: EstateMoment,
    pub(crate) delegation: DelegationLimit,
    pub(crate) workflow_stage: EstateWorkflowStage,
    pub(crate) status: CapabilityGrantStatus,
    pub(crate) grantee: BankPrincipalId,
    pub(crate) grantor: BankPrincipalId,
    pub(crate) emergencies: Vec<EstateEmergencyContext>,
}

impl EstateCapabilityContext {
    pub(crate) fn from_projection(value: EstateCapabilityProjection) -> Self {
        Self {
            id: value.id,
            operation: value.operation,
            purpose: value.purpose,
            valid_from: value.valid_from,
            valid_through: value.valid_through,
            delegation: value.delegation,
            workflow_stage: value.workflow_stage,
            status: value.status,
            grantee: value.grantee,
            grantor: value.grantor,
            emergencies: value.emergencies,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstateGovernanceContext {
    estate: EstateCaseId,
    stage: EstateWorkflowStage,
    beneficiaries: Vec<BankPrincipalId>,
    assignments: Vec<EstateAssignmentView>,
    capabilities: Vec<EstateCapabilityContext>,
}

impl EstateGovernanceContext {
    pub(crate) fn from_projection(
        estate: EstateCaseId,
        stage: EstateWorkflowStage,
        beneficiaries: Vec<BankPrincipalId>,
        assignments: Vec<EstateAssignmentView>,
        capabilities: Vec<EstateCapabilityContext>,
    ) -> Self {
        Self {
            estate,
            stage,
            beneficiaries,
            assignments,
            capabilities,
        }
    }
}
