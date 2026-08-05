use crate::{
    estate::{
        BranchId, CapabilityGrantId, CapabilityGrantStatus, CapabilityValidity, DelegationLimit,
        EmergencyAccessId, EmergencyAccessReason, EmergencyAccessStatus, EstateCapabilityGrant,
        EstateCapabilityOperation, EstateCapabilityPurpose, EstateCaseId, EstateEmergencyAccess,
        EstateMoment, EstateWorkflowStage, MandatoryEstateReview, MandatoryReviewId,
        RestrictedBankField,
    },
    model::{AccountId, BankPrincipalId, InstitutionId, Money, USD},
};

use super::EstateAssignmentView;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateEmergencyContext {
    access: EstateEmergencyAccess,
    review: MandatoryEstateReview,
}

impl EstateEmergencyContext {
    pub(crate) const fn from_projection(
        access: EstateEmergencyAccess,
        review: MandatoryEstateReview,
    ) -> Self {
        Self { access, review }
    }

    pub const fn access(&self) -> EstateEmergencyAccess {
        self.access
    }

    pub const fn mandatory_review(&self) -> MandatoryEstateReview {
        self.review
    }

    pub const fn id(&self) -> EmergencyAccessId {
        self.access.id
    }

    pub const fn reason(&self) -> EmergencyAccessReason {
        self.access.reason
    }

    pub const fn status(&self) -> EmergencyAccessStatus {
        self.access.status
    }

    pub const fn requester(&self) -> BankPrincipalId {
        self.access.requester
    }

    pub const fn approver(&self) -> Option<BankPrincipalId> {
        self.access.approver
    }

    pub const fn reviewer(&self) -> Option<BankPrincipalId> {
        self.access.reviewer
    }

    pub const fn grant(&self) -> CapabilityGrantId {
        self.access.grant
    }

    pub const fn review(&self) -> MandatoryReviewId {
        self.access.review
    }

    pub const fn issued_at(&self) -> EstateMoment {
        self.access.issued_at
    }

    pub const fn expires_at(&self) -> EstateMoment {
        self.access.expires_at
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstateCapabilityContext {
    grant: EstateCapabilityGrant,
    emergencies: Vec<EstateEmergencyContext>,
}

impl EstateCapabilityContext {
    pub(crate) fn from_projection(
        grant: EstateCapabilityGrant,
        emergencies: Vec<EstateEmergencyContext>,
    ) -> Self {
        Self { grant, emergencies }
    }

    pub const fn grant(&self) -> EstateCapabilityGrant {
        self.grant
    }

    pub const fn id(&self) -> CapabilityGrantId {
        self.grant.id
    }

    pub const fn account(&self) -> Option<AccountId> {
        self.grant.scope.account
    }

    pub const fn estate(&self) -> EstateCaseId {
        self.grant.scope.estate
    }

    pub const fn institution(&self) -> InstitutionId {
        self.grant.scope.institution
    }

    pub const fn branch(&self) -> BranchId {
        self.grant.scope.branch
    }

    pub const fn operation(&self) -> EstateCapabilityOperation {
        self.grant.scope.operation
    }

    pub const fn purpose(&self) -> EstateCapabilityPurpose {
        self.grant.scope.purpose
    }

    pub const fn field(&self) -> Option<RestrictedBankField> {
        self.grant.scope.field
    }

    pub const fn amount_ceiling(&self) -> Option<Money<USD>> {
        self.grant.scope.amount_ceiling
    }

    pub const fn validity(&self) -> CapabilityValidity {
        self.grant.scope.validity
    }

    pub const fn valid_from(&self) -> EstateMoment {
        self.grant.scope.validity.not_before()
    }

    pub const fn valid_through(&self) -> EstateMoment {
        self.grant.scope.validity.not_after()
    }

    pub const fn delegation(&self) -> DelegationLimit {
        self.grant.scope.delegation
    }

    pub const fn workflow_stage(&self) -> EstateWorkflowStage {
        self.grant.scope.workflow_stage
    }

    pub const fn status(&self) -> CapabilityGrantStatus {
        self.grant.status
    }

    pub const fn grantee(&self) -> BankPrincipalId {
        self.grant.grantee
    }

    pub const fn grantor(&self) -> BankPrincipalId {
        self.grant.grantor
    }

    pub const fn parent(&self) -> Option<CapabilityGrantId> {
        self.grant.parent
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
