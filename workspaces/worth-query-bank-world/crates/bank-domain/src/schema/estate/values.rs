use worth_foundational::facade::{AspectValue, InternedString, ScalarAspectType};
use worth_query_decl::facade::application_schema::{
    TypedApplicationReadableValue, TypedApplicationValue,
};

use crate::estate::*;

macro_rules! uint_application_value {
    ($($type:ty => $constructor:expr),+ $(,)?) => {
        $(
            impl TypedApplicationValue for $type {
                const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::UInt64;

                fn into_foundational_value(self) -> AspectValue {
                    AspectValue::UInt64(self.get())
                }
            }

            impl TypedApplicationReadableValue for $type {
                fn from_foundational_value(value: &AspectValue) -> Option<Self> {
                    let AspectValue::UInt64(value) = value else {
                        return None;
                    };
                    $constructor(*value)
                }
            }
        )+
    };
}

uint_application_value!(
    BranchId => BranchId::new,
    CapabilityGrantId => CapabilityGrantId::new,
    DeathNoticeId => DeathNoticeId::new,
    EmergencyAccessId => EmergencyAccessId::new,
    EstateCaseId => EstateCaseId::new,
    LegalAuthorityId => LegalAuthorityId::new,
    MandatoryReviewId => MandatoryReviewId::new,
);

impl TypedApplicationValue for EstateMoment {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::UInt64;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::UInt64(self.epoch_seconds())
    }
}

impl TypedApplicationReadableValue for EstateMoment {
    fn from_foundational_value(value: &AspectValue) -> Option<Self> {
        match value {
            AspectValue::UInt64(value) => Some(Self::from_epoch_seconds(*value)),
            _ => None,
        }
    }
}

impl TypedApplicationValue for DelegationLimit {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::UInt64;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::UInt64(u64::from(self.remaining()))
    }
}

impl TypedApplicationReadableValue for DelegationLimit {
    fn from_foundational_value(value: &AspectValue) -> Option<Self> {
        match value {
            AspectValue::UInt64(value) => u8::try_from(*value).ok().map(Self::generations),
            _ => None,
        }
    }
}

macro_rules! string_application_value {
    ($type:ty, {$($variant:path => $value:literal),+ $(,)?}) => {
        impl TypedApplicationValue for $type {
            const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::String;

            fn into_foundational_value(self) -> AspectValue {
                let value = match self {
                    $($variant => $value),+
                };
                AspectValue::String(InternedString::from(value))
            }
        }

        impl TypedApplicationReadableValue for $type {
            fn from_foundational_value(value: &AspectValue) -> Option<Self> {
                let AspectValue::String(InternedString::Raw(value)) = value else {
                    return None;
                };
                match value.as_str() {
                    $($value => Some($variant),)+
                    _ => None,
                }
            }
        }
    };
}

string_application_value!(DeathNoticeStatus, {
    DeathNoticeStatus::Reported => "reported",
    DeathNoticeStatus::NotificationRequested => "notification-requested",
    DeathNoticeStatus::Verified => "verified",
    DeathNoticeStatus::Rejected => "rejected",
});
string_application_value!(EstateCaseStatus, {
    EstateCaseStatus::PendingOpening => "pending-opening",
    EstateCaseStatus::Open => "open",
    EstateCaseStatus::Released => "released",
    EstateCaseStatus::Closed => "closed",
});
string_application_value!(EstateWorkflowStage, {
    EstateWorkflowStage::DeathReported => "death-reported",
    EstateWorkflowStage::AccountsFrozen => "accounts-frozen",
    EstateWorkflowStage::AuthorityReview => "authority-review",
    EstateWorkflowStage::Administration => "administration",
    EstateWorkflowStage::ReleaseReview => "release-review",
    EstateWorkflowStage::Released => "released",
});
string_application_value!(LegalAuthorityKind, {
    LegalAuthorityKind::CourtAppointment => "court-appointment",
    LegalAuthorityKind::SmallEstateAffidavit => "small-estate-affidavit",
    LegalAuthorityKind::InstitutionalRecognition => "institutional-recognition",
});
string_application_value!(CapabilityGrantStatus, {
    CapabilityGrantStatus::Active => "active",
    CapabilityGrantStatus::Revoked => "revoked",
});
string_application_value!(EmergencyAccessReason, {
    EmergencyAccessReason::PreventImmediateLoss => "prevent-immediate-loss",
    EmergencyAccessReason::ProtectVulnerableCustomer => "protect-vulnerable-customer",
    EmergencyAccessReason::MeetLegalDeadline => "meet-legal-deadline",
});
string_application_value!(EmergencyAccessStatus, {
    EmergencyAccessStatus::Requested => "requested",
    EmergencyAccessStatus::Approved => "approved",
    EmergencyAccessStatus::Expired => "expired",
    EmergencyAccessStatus::Revoked => "revoked",
});
string_application_value!(MandatoryReviewStatus, {
    MandatoryReviewStatus::Required => "required",
    MandatoryReviewStatus::Completed => "completed",
});
string_application_value!(MandatoryReviewKind, {
    MandatoryReviewKind::EstateRelease => "estate-release",
    MandatoryReviewKind::EmergencyAccess => "emergency-access",
});
string_application_value!(RestrictedBankField, {
    RestrictedBankField::CustomerIdentity => "customer-identity",
    RestrictedBankField::BeneficiaryIdentity => "beneficiary-identity",
    RestrictedBankField::LegalDocument => "legal-document",
    RestrictedBankField::AccountDetails => "account-details",
    RestrictedBankField::PostingHistory => "posting-history",
    RestrictedBankField::AuditTrail => "audit-trail",
    RestrictedBankField::GovernanceMetadata => "governance-metadata",
    RestrictedBankField::EmergencyAccessActivity => "emergency-access-activity",
});
string_application_value!(EstateCapabilityPurpose, {
    EstateCapabilityPurpose::EstateAdministration => "estate-administration",
    EstateCapabilityPurpose::IdentityVerification => "identity-verification",
    EstateCapabilityPurpose::LegalCompliance => "legal-compliance",
    EstateCapabilityPurpose::EmergencyProtection => "emergency-protection",
    EstateCapabilityPurpose::EstateDisbursement => "estate-disbursement",
    EstateCapabilityPurpose::MandatoryReview => "mandatory-review",
});
string_application_value!(EstateCapabilityOperation, {
    EstateCapabilityOperation::NotifyDeath => "notify-death",
    EstateCapabilityOperation::RetransmitDeathNotice => "retransmit-death-notice",
    EstateCapabilityOperation::FreezeAccount => "freeze-account",
    EstateCapabilityOperation::OpenEstateCase => "open-estate-case",
    EstateCapabilityOperation::RecognizeExecutor => "recognize-executor",
    EstateCapabilityOperation::DelegateCapability => "delegate-capability",
    EstateCapabilityOperation::RevokeCapability => "revoke-capability",
    EstateCapabilityOperation::RequestEmergencyAccess => "request-emergency-access",
    EstateCapabilityOperation::ApproveEmergencyAccess => "approve-emergency-access",
    EstateCapabilityOperation::RevokeEmergencyAccess => "revoke-emergency-access",
    EstateCapabilityOperation::CompleteMandatoryReview => "complete-mandatory-review",
    EstateCapabilityOperation::ReleaseEstate => "release-estate",
    EstateCapabilityOperation::DisburseEstate => "disburse-estate",
    EstateCapabilityOperation::ViewRestrictedEstate => "view-restricted-estate",
});
