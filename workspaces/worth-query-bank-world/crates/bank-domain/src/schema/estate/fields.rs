use worth_query_decl::facade::{worth_query_aspect, worth_query_field};

use crate::estate::{
    BranchId, CapabilityGrantId, CapabilityGrantStatus, DeathNoticeId, DeathNoticeStatus,
    DelegationLimit, EmergencyAccessId, EmergencyAccessReason, EmergencyAccessStatus,
    EstateCapabilityOperation, EstateCapabilityPurpose, EstateCaseId, EstateCaseStatus,
    EstateMoment, EstateWorkflowStage, LegalAuthorityId, LegalAuthorityKind, MandatoryReviewId,
    MandatoryReviewKind, MandatoryReviewStatus, RestrictedBankField,
};
use crate::model::{Money, USD};
use crate::schema::BankSchema;

use super::entities::{
    Branch, CapabilityGrant, DeathNotice, EmergencyAccess, EstateCase, LegalAuthority,
    MandatoryReview,
};

worth_query_aspect!(pub BranchIdentity in BankSchema, Branch);
worth_query_aspect!(pub DeathNoticeRecord in BankSchema, DeathNotice);
worth_query_aspect!(pub EstateCaseRecord in BankSchema, EstateCase);
worth_query_aspect!(pub LegalAuthorityRecord in BankSchema, LegalAuthority);
worth_query_aspect!(pub CapabilityGrantRecord in BankSchema, CapabilityGrant);
worth_query_aspect!(pub EmergencyAccessRecord in BankSchema, EmergencyAccess);
worth_query_aspect!(pub MandatoryReviewRecord in BankSchema, MandatoryReview);

worth_query_field!(
    pub BranchIdentityField in BankSchema, Branch, BranchIdentity:
    BranchId, read_only, equality
);
worth_query_field!(
    pub DeathNoticeIdentityField in BankSchema, DeathNotice, DeathNoticeRecord:
    DeathNoticeId, read_only, equality
);
worth_query_field!(
    pub DeathNoticeStatusField in BankSchema, DeathNotice, DeathNoticeRecord:
    DeathNoticeStatus, read_write, equality
);
worth_query_field!(
    pub EstateCaseIdentityField in BankSchema, EstateCase, EstateCaseRecord:
    EstateCaseId, read_only, equality
);
worth_query_field!(
    pub EstateWorkflowStageField in BankSchema, EstateCase, EstateCaseRecord:
    EstateWorkflowStage, read_write, equality
);
worth_query_field!(
    pub EstateCaseStatusField in BankSchema, EstateCase, EstateCaseRecord:
    EstateCaseStatus, read_write, equality
);
worth_query_field!(
    pub LegalAuthorityIdentityField in BankSchema, LegalAuthority, LegalAuthorityRecord:
    LegalAuthorityId, read_only, equality
);
worth_query_field!(
    pub LegalAuthorityKindField in BankSchema, LegalAuthority, LegalAuthorityRecord:
    LegalAuthorityKind, read_write, equality
);
worth_query_field!(
    pub LegalAuthorityRecognizedField in BankSchema, LegalAuthority, LegalAuthorityRecord:
    bool, read_write, equality
);
worth_query_field!(
    pub CapabilityGrantIdentityField in BankSchema, CapabilityGrant, CapabilityGrantRecord:
    CapabilityGrantId, read_only, equality
);
worth_query_field!(
    pub CapabilityOperationField in BankSchema, CapabilityGrant, CapabilityGrantRecord:
    EstateCapabilityOperation, read_write, equality
);
worth_query_field!(
    pub CapabilityPurposeField in BankSchema, CapabilityGrant, CapabilityGrantRecord:
    EstateCapabilityPurpose, read_write, equality
);
worth_query_field!(
    pub CapabilityDisclosureField in BankSchema, CapabilityGrant, CapabilityGrantRecord:
    optional RestrictedBankField, read_write, equality
);
worth_query_field!(
    pub CapabilityAmountCeilingField in BankSchema, CapabilityGrant, CapabilityGrantRecord:
    optional Money<USD>, unit crate::schema::UsdCurrency, read_write, no_equality
);
worth_query_field!(
    pub CapabilityValidFromField in BankSchema, CapabilityGrant, CapabilityGrantRecord:
    EstateMoment, read_write, equality
);
worth_query_field!(
    pub CapabilityValidThroughField in BankSchema, CapabilityGrant, CapabilityGrantRecord:
    EstateMoment, read_write, equality
);
worth_query_field!(
    pub CapabilityDelegationLimitField in BankSchema, CapabilityGrant, CapabilityGrantRecord:
    DelegationLimit, read_write, equality
);
worth_query_field!(
    pub CapabilityWorkflowStageField in BankSchema, CapabilityGrant, CapabilityGrantRecord:
    EstateWorkflowStage, read_write, equality
);
worth_query_field!(
    pub CapabilityGrantStatusField in BankSchema, CapabilityGrant, CapabilityGrantRecord:
    CapabilityGrantStatus, read_write, equality
);
worth_query_field!(
    pub EmergencyAccessIdentityField in BankSchema, EmergencyAccess, EmergencyAccessRecord:
    EmergencyAccessId, read_only, equality
);
worth_query_field!(
    pub EmergencyAccessReasonField in BankSchema, EmergencyAccess, EmergencyAccessRecord:
    EmergencyAccessReason, read_write, equality
);
worth_query_field!(
    pub EmergencyAccessStatusField in BankSchema, EmergencyAccess, EmergencyAccessRecord:
    EmergencyAccessStatus, read_write, equality
);
worth_query_field!(
    pub EmergencyAccessIssuedAtField in BankSchema, EmergencyAccess, EmergencyAccessRecord:
    EstateMoment, read_write, equality
);
worth_query_field!(
    pub EmergencyAccessExpiresAtField in BankSchema, EmergencyAccess, EmergencyAccessRecord:
    EstateMoment, read_write, equality
);
worth_query_field!(
    pub MandatoryReviewIdentityField in BankSchema, MandatoryReview, MandatoryReviewRecord:
    MandatoryReviewId, read_only, equality
);
worth_query_field!(
    pub MandatoryReviewStatusField in BankSchema, MandatoryReview, MandatoryReviewRecord:
    MandatoryReviewStatus, read_write, equality
);
worth_query_field!(
    pub MandatoryReviewKindField in BankSchema, MandatoryReview, MandatoryReviewRecord:
    MandatoryReviewKind, read_only, equality
);
