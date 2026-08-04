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

    pub const fn id(&self) -> EmergencyAccessId {
        self.id
    }

    pub const fn reason(&self) -> EmergencyAccessReason {
        self.reason
    }

    pub const fn status(&self) -> EmergencyAccessStatus {
        self.status
    }

    pub const fn requester(&self) -> BankPrincipalId {
        self.requester
    }

    pub const fn approver(&self) -> Option<BankPrincipalId> {
        self.approver
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

    pub const fn id(&self) -> CapabilityGrantId {
        self.id
    }

    pub const fn operation(&self) -> EstateCapabilityOperation {
        self.operation
    }

    pub const fn purpose(&self) -> EstateCapabilityPurpose {
        self.purpose
    }

    pub const fn valid_from(&self) -> EstateMoment {
        self.valid_from
    }

    pub const fn valid_through(&self) -> EstateMoment {
        self.valid_through
    }

    pub const fn delegation(&self) -> DelegationLimit {
        self.delegation
    }

    pub const fn workflow_stage(&self) -> EstateWorkflowStage {
        self.workflow_stage
    }

    pub const fn status(&self) -> CapabilityGrantStatus {
        self.status
    }

    pub const fn grantee(&self) -> BankPrincipalId {
        self.grantee
    }

    pub const fn grantor(&self) -> BankPrincipalId {
        self.grantor
    }

    pub fn emergencies(&self) -> &[EstateEmergencyContext] {
        &self.emergencies
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

    pub const fn estate(&self) -> EstateCaseId {
        self.estate
    }

    pub const fn stage(&self) -> EstateWorkflowStage {
        self.stage
    }

    pub fn beneficiaries(&self) -> &[BankPrincipalId] {
        &self.beneficiaries
    }

    pub fn assignments(&self) -> &[EstateAssignmentView] {
        &self.assignments
    }

    pub fn capabilities(&self) -> &[EstateCapabilityContext] {
        &self.capabilities
    }
}
