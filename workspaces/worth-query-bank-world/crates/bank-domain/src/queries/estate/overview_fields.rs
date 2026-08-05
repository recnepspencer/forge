use worth_query_decl::facade::application_query::ApplicationQueryResultFieldRef;
use worth_query_decl::facade::application_schema::{
    EqualityPredicate, NoApplicationCurrency, ReadOnly, ReadWrite,
};

use crate::estate::{
    BranchId, DeathNoticeId, DeathNoticeStatus, EstateCaseId, EstateCaseStatus,
    EstateWorkflowStage, LegalAuthorityId, LegalAuthorityKind, MandatoryReviewId,
    MandatoryReviewKind, MandatoryReviewStatus,
};
use crate::model::{AccountId, AccountName, BankPrincipalId, EmployeeAssignmentId, EmployeeRole};
use crate::schema::{
    Account, AccountDisplayName, AccountIdentity, AccountProfile, AccountState, AccountStatus,
    AssignmentRole, BankSchema, Branch, BranchIdentity, BranchIdentityField, DeathNotice,
    DeathNoticeIdentityField, DeathNoticeRecord, DeathNoticeStatusField, EmployeeAssignment,
    EmployeeAssignmentIdentityField, EmployeeScope, EstateCase, EstateCaseIdentityField,
    EstateCaseRecord, EstateCaseStatusField, EstateWorkflowStageField, Identity, LegalAuthority,
    LegalAuthorityIdentityField, LegalAuthorityKindField, LegalAuthorityRecognizedField,
    LegalAuthorityRecord, MandatoryReview, MandatoryReviewIdentityField, MandatoryReviewKindField,
    MandatoryReviewRecord, MandatoryReviewStatusField, Principal, PrincipalIdentity,
    PrincipalIdentityField, Status,
};

use super::overview::EstateCaseOverviewQuery;

pub(super) struct EstateIdentitySlot;
pub(super) struct EstateStageSlot;
pub(super) struct EstateStatusSlot;
pub(super) struct AccountIdentitySlot;
pub(super) struct AccountNameSlot;
pub(super) struct AccountStatusSlot;
pub(super) struct BranchIdentitySlot;
pub(super) struct NoticeIdentitySlot;
pub(super) struct NoticeStatusSlot;
pub(super) struct DeceasedIdentitySlot;
pub(super) struct ExecutorIdentitySlot;
pub(super) struct BeneficiaryIdentitySlot;
pub(super) struct AssignmentIdentitySlot;
pub(super) struct AssignmentRoleSlot;
pub(super) struct AssignmentPrincipalIdentitySlot;
pub(super) struct AuthorityIdentitySlot;
pub(super) struct AuthorityKindSlot;
pub(super) struct AuthorityRecognizedSlot;
pub(super) struct AuthorityHolderIdentitySlot;
pub(super) struct ReviewIdentitySlot;
pub(super) struct ReviewKindSlot;
pub(super) struct ReviewStatusSlot;
pub(super) struct ReviewPrincipalIdentitySlot;

macro_rules! selector {
    (
        $name:ident,
        $slot:ty,
        $entity:ty,
        $aspect:ty,
        $field:ty,
        $value:ty,
        $write:ty,
        $alias:literal
    ) => {
        pub(super) fn $name() -> ApplicationQueryResultFieldRef<
            EstateCaseOverviewQuery,
            $slot,
            BankSchema,
            $entity,
            $aspect,
            $field,
            $value,
            $write,
            EqualityPredicate,
            NoApplicationCurrency,
        > {
            ApplicationQueryResultFieldRef::new($alias, <$field>::reference())
        }
    };
}

selector!(
    estate_identity,
    EstateIdentitySlot,
    EstateCase,
    EstateCaseRecord,
    EstateCaseIdentityField,
    EstateCaseId,
    ReadOnly,
    "estate"
);
selector!(
    estate_stage,
    EstateStageSlot,
    EstateCase,
    EstateCaseRecord,
    EstateWorkflowStageField,
    EstateWorkflowStage,
    ReadWrite,
    "stage"
);
selector!(
    estate_status,
    EstateStatusSlot,
    EstateCase,
    EstateCaseRecord,
    EstateCaseStatusField,
    EstateCaseStatus,
    ReadWrite,
    "status"
);
selector!(
    account_identity,
    AccountIdentitySlot,
    Account,
    Identity,
    AccountIdentity,
    AccountId,
    ReadOnly,
    "account"
);
selector!(
    account_name,
    AccountNameSlot,
    Account,
    AccountProfile,
    AccountDisplayName,
    AccountName,
    ReadWrite,
    "display_name"
);
selector!(
    account_status,
    AccountStatusSlot,
    Account,
    AccountState,
    Status,
    AccountStatus,
    ReadWrite,
    "status"
);
selector!(
    branch_identity,
    BranchIdentitySlot,
    Branch,
    BranchIdentity,
    BranchIdentityField,
    BranchId,
    ReadOnly,
    "branch"
);
selector!(
    notice_identity,
    NoticeIdentitySlot,
    DeathNotice,
    DeathNoticeRecord,
    DeathNoticeIdentityField,
    DeathNoticeId,
    ReadOnly,
    "notice"
);
selector!(
    notice_status,
    NoticeStatusSlot,
    DeathNotice,
    DeathNoticeRecord,
    DeathNoticeStatusField,
    DeathNoticeStatus,
    ReadWrite,
    "status"
);

macro_rules! principal_selector {
    ($name:ident, $slot:ty, $alias:literal) => {
        selector!(
            $name,
            $slot,
            Principal,
            PrincipalIdentity,
            PrincipalIdentityField,
            BankPrincipalId,
            ReadOnly,
            $alias
        );
    };
}

principal_selector!(
    deceased_identity,
    DeceasedIdentitySlot,
    "deceased_principal"
);
principal_selector!(executor_identity, ExecutorIdentitySlot, "executor");
principal_selector!(beneficiary_identity, BeneficiaryIdentitySlot, "beneficiary");
principal_selector!(
    assignment_principal_identity,
    AssignmentPrincipalIdentitySlot,
    "principal"
);
principal_selector!(
    authority_holder_identity,
    AuthorityHolderIdentitySlot,
    "holder"
);
principal_selector!(
    review_principal_identity,
    ReviewPrincipalIdentitySlot,
    "reviewer"
);

selector!(
    assignment_identity,
    AssignmentIdentitySlot,
    EmployeeAssignment,
    EmployeeScope,
    EmployeeAssignmentIdentityField,
    EmployeeAssignmentId,
    ReadOnly,
    "assignment"
);
selector!(
    assignment_role,
    AssignmentRoleSlot,
    EmployeeAssignment,
    EmployeeScope,
    AssignmentRole,
    EmployeeRole,
    ReadWrite,
    "role"
);
selector!(
    authority_identity,
    AuthorityIdentitySlot,
    LegalAuthority,
    LegalAuthorityRecord,
    LegalAuthorityIdentityField,
    LegalAuthorityId,
    ReadOnly,
    "authority"
);
selector!(
    authority_kind,
    AuthorityKindSlot,
    LegalAuthority,
    LegalAuthorityRecord,
    LegalAuthorityKindField,
    LegalAuthorityKind,
    ReadWrite,
    "kind"
);
selector!(
    authority_recognized,
    AuthorityRecognizedSlot,
    LegalAuthority,
    LegalAuthorityRecord,
    LegalAuthorityRecognizedField,
    bool,
    ReadWrite,
    "recognized"
);
selector!(
    review_identity,
    ReviewIdentitySlot,
    MandatoryReview,
    MandatoryReviewRecord,
    MandatoryReviewIdentityField,
    MandatoryReviewId,
    ReadOnly,
    "review"
);
selector!(
    review_kind,
    ReviewKindSlot,
    MandatoryReview,
    MandatoryReviewRecord,
    MandatoryReviewKindField,
    MandatoryReviewKind,
    ReadOnly,
    "kind"
);
selector!(
    review_status,
    ReviewStatusSlot,
    MandatoryReview,
    MandatoryReviewRecord,
    MandatoryReviewStatusField,
    MandatoryReviewStatus,
    ReadWrite,
    "status"
);
