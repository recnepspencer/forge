use crate::model::{AccountId, BankPrincipalId, Money, SignedMoney, USD};

use super::{
    CapabilityGrantId, DeathNoticeId, EmergencyAccessId, EmergencyAccessReason,
    EstateCapabilityOperation, EstateCapabilityPurpose, EstateCaseId, LegalAuthorityId,
    MandatoryReviewId, RestrictedBankField,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstatePosting {
    pub account: AccountId,
    pub amount: SignedMoney<USD>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateDisbursement {
    pub estate: EstateCaseId,
    pub source_account: AccountId,
    pub destination_account: AccountId,
    pub beneficiary: BankPrincipalId,
    pub amount: Money<USD>,
    pub postings: [EstatePosting; 2],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstateAction {
    NotifyDeath {
        estate: EstateCaseId,
        notice: DeathNoticeId,
        subject: BankPrincipalId,
    },
    FreezeAccount {
        estate: EstateCaseId,
        account: AccountId,
    },
    OpenEstateCase {
        estate: EstateCaseId,
        notice: DeathNoticeId,
    },
    RecognizeExecutor {
        estate: EstateCaseId,
        executor: BankPrincipalId,
        authority: LegalAuthorityId,
    },
    DelegateCapability {
        estate: EstateCaseId,
        parent: CapabilityGrantId,
        child: CapabilityGrantId,
    },
    RevokeCapability {
        estate: EstateCaseId,
        grant: CapabilityGrantId,
    },
    RequestEmergencyAccess {
        estate: EstateCaseId,
        access: EmergencyAccessId,
        review: MandatoryReviewId,
        grant: CapabilityGrantId,
        reason: EmergencyAccessReason,
        field: RestrictedBankField,
        duration: std::time::Duration,
    },
    ApproveEmergencyAccess {
        estate: EstateCaseId,
        access: EmergencyAccessId,
    },
    RevokeEmergencyAccess {
        estate: EstateCaseId,
        access: EmergencyAccessId,
    },
    CompleteMandatoryReview {
        estate: EstateCaseId,
        access: EmergencyAccessId,
        review: MandatoryReviewId,
    },
    ReleaseEstate {
        estate: EstateCaseId,
    },
    DisburseEstate(EstateDisbursement),
    ViewRestrictedEstate {
        estate: EstateCaseId,
        field: RestrictedBankField,
        purpose: EstateCapabilityPurpose,
    },
}

impl EstateAction {
    pub const fn operation(self) -> EstateCapabilityOperation {
        match self {
            Self::NotifyDeath { .. } => EstateCapabilityOperation::NotifyDeath,
            Self::FreezeAccount { .. } => EstateCapabilityOperation::FreezeAccount,
            Self::OpenEstateCase { .. } => EstateCapabilityOperation::OpenEstateCase,
            Self::RecognizeExecutor { .. } => EstateCapabilityOperation::RecognizeExecutor,
            Self::DelegateCapability { .. } => EstateCapabilityOperation::DelegateCapability,
            Self::RevokeCapability { .. } => EstateCapabilityOperation::RevokeCapability,
            Self::RequestEmergencyAccess { .. } => {
                EstateCapabilityOperation::RequestEmergencyAccess
            }
            Self::ApproveEmergencyAccess { .. } => {
                EstateCapabilityOperation::ApproveEmergencyAccess
            }
            Self::RevokeEmergencyAccess { .. } => EstateCapabilityOperation::RevokeEmergencyAccess,
            Self::CompleteMandatoryReview { .. } => {
                EstateCapabilityOperation::CompleteMandatoryReview
            }
            Self::ReleaseEstate { .. } => EstateCapabilityOperation::ReleaseEstate,
            Self::DisburseEstate(_) => EstateCapabilityOperation::DisburseEstate,
            Self::ViewRestrictedEstate { .. } => EstateCapabilityOperation::ViewRestrictedEstate,
        }
    }

    pub const fn estate(self) -> Option<EstateCaseId> {
        match self {
            Self::NotifyDeath { estate, .. }
            | Self::FreezeAccount { estate, .. }
            | Self::OpenEstateCase { estate, .. }
            | Self::RecognizeExecutor { estate, .. }
            | Self::DelegateCapability { estate, .. }
            | Self::RevokeCapability { estate, .. }
            | Self::RequestEmergencyAccess { estate, .. }
            | Self::ApproveEmergencyAccess { estate, .. }
            | Self::RevokeEmergencyAccess { estate, .. }
            | Self::CompleteMandatoryReview { estate, .. }
            | Self::ReleaseEstate { estate }
            | Self::ViewRestrictedEstate { estate, .. } => Some(estate),
            Self::DisburseEstate(disbursement) => Some(disbursement.estate),
        }
    }

    pub const fn account(self) -> Option<AccountId> {
        match self {
            Self::FreezeAccount { account, .. } => Some(account),
            Self::DisburseEstate(disbursement) => Some(disbursement.source_account),
            _ => None,
        }
    }

    pub const fn field(self) -> Option<RestrictedBankField> {
        match self {
            Self::ViewRestrictedEstate { field, .. } => Some(field),
            _ => None,
        }
    }

    pub const fn purpose(self) -> EstateCapabilityPurpose {
        match self {
            Self::DisburseEstate(_) => EstateCapabilityPurpose::EstateDisbursement,
            Self::ViewRestrictedEstate { purpose, .. } => purpose,
            Self::CompleteMandatoryReview { .. } => EstateCapabilityPurpose::MandatoryReview,
            Self::RequestEmergencyAccess { .. }
            | Self::ApproveEmergencyAccess { .. }
            | Self::RevokeEmergencyAccess { .. } => EstateCapabilityPurpose::EmergencyProtection,
            Self::RecognizeExecutor { .. } => EstateCapabilityPurpose::LegalCompliance,
            _ => EstateCapabilityPurpose::EstateAdministration,
        }
    }

    pub const fn amount(self) -> Option<Money<USD>> {
        match self {
            Self::DisburseEstate(disbursement) => Some(disbursement.amount),
            _ => None,
        }
    }
}
