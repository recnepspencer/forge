use bank_domain::{
    estate::{
        CapabilityGrantStatus, EstateCapabilityOperation, EstateCapabilityPurpose,
        EstateWorkflowStage, RestrictedBankField,
    },
    model::{AccountId, Money, USD},
};

use super::ACCOUNT;

#[derive(Clone, Copy)]
pub(crate) struct GrantSpec {
    pub(crate) operation: EstateCapabilityOperation,
    pub(crate) purpose: EstateCapabilityPurpose,
    pub(crate) account: Option<AccountId>,
    pub(crate) field: Option<RestrictedBankField>,
    pub(crate) amount_ceiling: Option<Money<USD>>,
    pub(crate) status: CapabilityGrantStatus,
    pub(crate) not_before: u64,
    pub(crate) not_after: u64,
    pub(crate) workflow: EstateWorkflowStage,
}

impl GrantSpec {
    pub(crate) fn view() -> Self {
        Self {
            operation: EstateCapabilityOperation::ViewRestrictedEstate,
            purpose: EstateCapabilityPurpose::EstateAdministration,
            account: None,
            field: Some(RestrictedBankField::CustomerIdentity),
            amount_ceiling: None,
            status: CapabilityGrantStatus::Active,
            not_before: 0,
            not_after: u64::MAX,
            workflow: EstateWorkflowStage::Administration,
        }
    }

    pub(crate) fn freeze() -> Self {
        Self {
            operation: EstateCapabilityOperation::FreezeAccount,
            purpose: EstateCapabilityPurpose::EstateAdministration,
            account: Some(ACCOUNT),
            field: None,
            ..Self::view()
        }
    }

    pub(crate) fn identity_verification() -> Self {
        Self {
            purpose: EstateCapabilityPurpose::IdentityVerification,
            ..Self::view()
        }
    }

    pub(crate) fn emergency_view() -> Self {
        Self {
            purpose: EstateCapabilityPurpose::EmergencyProtection,
            field: Some(RestrictedBankField::AccountDetails),
            ..Self::view()
        }
    }

    pub(crate) fn emergency_request() -> Self {
        Self {
            operation: EstateCapabilityOperation::RequestEmergencyAccess,
            purpose: EstateCapabilityPurpose::EmergencyProtection,
            field: None,
            ..Self::view()
        }
    }

    pub(crate) fn emergency_approval() -> Self {
        Self {
            operation: EstateCapabilityOperation::ApproveEmergencyAccess,
            purpose: EstateCapabilityPurpose::EmergencyProtection,
            field: None,
            ..Self::view()
        }
    }

    pub(crate) fn emergency_close() -> Self {
        Self {
            operation: EstateCapabilityOperation::RevokeEmergencyAccess,
            purpose: EstateCapabilityPurpose::EmergencyProtection,
            field: None,
            ..Self::view()
        }
    }

    pub(crate) fn mandatory_review() -> Self {
        Self {
            operation: EstateCapabilityOperation::CompleteMandatoryReview,
            purpose: EstateCapabilityPurpose::MandatoryReview,
            field: None,
            ..Self::view()
        }
    }

    pub(crate) fn disburse(maximum_minor_units: i64) -> Self {
        Self {
            operation: EstateCapabilityOperation::DisburseEstate,
            purpose: EstateCapabilityPurpose::EstateDisbursement,
            account: Some(ACCOUNT),
            field: None,
            amount_ceiling: Some(Money::from_minor(maximum_minor_units).unwrap()),
            ..Self::view()
        }
    }

    pub(crate) fn recognize() -> Self {
        Self {
            operation: EstateCapabilityOperation::RecognizeExecutor,
            purpose: EstateCapabilityPurpose::LegalCompliance,
            account: None,
            field: None,
            ..Self::view()
        }
    }
}
