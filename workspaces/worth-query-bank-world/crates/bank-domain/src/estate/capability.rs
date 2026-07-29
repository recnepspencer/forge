use crate::model::{AccountId, BankPrincipalId, InstitutionId, Money, USD};

use super::{
    BranchId, CapabilityGrantId, DelegationLimit, EstateCaseId, EstateMoment, EstateWorkflowStage,
    RestrictedBankField,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstateCapabilityOperation {
    NotifyDeath,
    FreezeAccount,
    OpenEstateCase,
    RecognizeExecutor,
    DelegateCapability,
    RevokeCapability,
    RequestEmergencyAccess,
    ApproveEmergencyAccess,
    RevokeEmergencyAccess,
    CompleteMandatoryReview,
    ReleaseEstate,
    DisburseEstate,
    ViewRestrictedEstate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstateCapabilityPurpose {
    EstateAdministration,
    IdentityVerification,
    LegalCompliance,
    EmergencyProtection,
    EstateDisbursement,
    MandatoryReview,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityGrantStatus {
    Active,
    Revoked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapabilityValidity {
    not_before: EstateMoment,
    not_after: EstateMoment,
}

impl CapabilityValidity {
    pub const fn new(not_before: EstateMoment, not_after: EstateMoment) -> Option<Self> {
        if not_before.epoch_seconds() > not_after.epoch_seconds() {
            None
        } else {
            Some(Self {
                not_before,
                not_after,
            })
        }
    }

    pub const fn contains(self, moment: EstateMoment) -> bool {
        self.not_before.epoch_seconds() <= moment.epoch_seconds()
            && moment.epoch_seconds() <= self.not_after.epoch_seconds()
    }

    pub const fn is_within(self, parent: Self) -> bool {
        parent.not_before.epoch_seconds() <= self.not_before.epoch_seconds()
            && self.not_after.epoch_seconds() <= parent.not_after.epoch_seconds()
    }

    pub const fn not_before(self) -> EstateMoment {
        self.not_before
    }

    pub const fn not_after(self) -> EstateMoment {
        self.not_after
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateCapabilityScope {
    pub account: Option<AccountId>,
    pub estate: EstateCaseId,
    pub institution: InstitutionId,
    pub branch: BranchId,
    pub operation: EstateCapabilityOperation,
    pub purpose: EstateCapabilityPurpose,
    pub field: Option<RestrictedBankField>,
    pub amount_ceiling: Option<Money<USD>>,
    pub validity: CapabilityValidity,
    pub delegation: DelegationLimit,
    pub workflow_stage: EstateWorkflowStage,
}

impl EstateCapabilityScope {
    pub fn is_within(self, parent: Self) -> bool {
        self.account == parent.account
            && self.estate == parent.estate
            && self.institution == parent.institution
            && self.branch == parent.branch
            && self.operation == parent.operation
            && self.purpose == parent.purpose
            && self.field == parent.field
            && amount_is_within(self.amount_ceiling, parent.amount_ceiling)
            && self.validity.is_within(parent.validity)
            && self.delegation.remaining() < parent.delegation.remaining()
            && self.workflow_stage == parent.workflow_stage
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateCapabilityGrant {
    pub id: CapabilityGrantId,
    pub grantor: BankPrincipalId,
    pub grantee: BankPrincipalId,
    pub scope: EstateCapabilityScope,
    pub parent: Option<CapabilityGrantId>,
    pub status: CapabilityGrantStatus,
}

const fn amount_is_within(child: Option<Money<USD>>, parent: Option<Money<USD>>) -> bool {
    match (child, parent) {
        (None, None) => true,
        (Some(child), Some(parent)) => child.minor_units() <= parent.minor_units(),
        (Some(_), None) | (None, Some(_)) => false,
    }
}
