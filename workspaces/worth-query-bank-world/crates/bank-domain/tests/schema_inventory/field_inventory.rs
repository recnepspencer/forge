use worth_query_decl::facade::application_schema::ApplicationSchemaMember;

use super::support::{
    aspect_name, currency_name, effect_name, expected, field_name, names, policy_name,
};

pub(super) fn assert_field_and_governance_inventory(members: &[ApplicationSchemaMember]) {
    assert_eq!(
        names(members, aspect_name),
        expected(&[
            "AccountProfile",
            "AccountState",
            "AuthorizationIdentity",
            "AuthorizationScope",
            "BranchIdentity",
            "BusinessIdentity",
            "CapabilityGrantRecord",
            "DeathNoticeRecord",
            "EmployeeScope",
            "EmergencyAccessRecord",
            "EstateCaseRecord",
            "ExternalPrincipalIdentity",
            "Identity",
            "InstitutionIdentity",
            "JournalIdentity",
            "JournalState",
            "LegalAuthorityRecord",
            "MandatoryReviewRecord",
            "PaymentIdentity",
            "PaymentState",
            "PaymentValue",
            "PostingIdentity",
            "PostingValue",
            "PrincipalIdentity",
        ])
    );
    assert_eq!(
        names(members, field_name),
        expected(&[
            "AccountAuthorizationIdentity",
            "AccountDisplayName",
            "AccountIdentity",
            "AccountingRevision",
            "AssignmentRole",
            "AuthorizationRole",
            "BranchIdentityField",
            "BusinessIdentityField",
            "CapabilityAmountCeilingField",
            "CapabilityDelegationLimitField",
            "CapabilityDisclosureField",
            "CapabilityGrantIdentityField",
            "CapabilityGrantStatusField",
            "CapabilityOperationField",
            "CapabilityPurposeField",
            "CapabilityValidFromField",
            "CapabilityValidThroughField",
            "CapabilityWorkflowStageField",
            "DeathNoticeIdentityField",
            "DeathNoticeStatusField",
            "EmergencyAccessIdentityField",
            "EmergencyAccessReasonField",
            "EmergencyAccessStatusField",
            "EmployeeAssignmentIdentityField",
            "EstateCaseIdentityField",
            "EstateCaseStatusField",
            "EstateWorkflowStageField",
            "ExternalIdentityKey",
            "ExternalMappingStatus",
            "InstitutionIdentityField",
            "JournalIdentityField",
            "JournalPurpose",
            "Kind",
            "LegalAuthorityIdentityField",
            "LegalAuthorityKindField",
            "LegalAuthorityRecognizedField",
            "MandatoryReviewIdentityField",
            "MandatoryReviewKindField",
            "MandatoryReviewStatusField",
            "PaymentAmount",
            "PaymentIdentityField",
            "PaymentStatusField",
            "PostingAccountSequence",
            "PostingAmount",
            "PostingIdentityField",
            "PrincipalIdentityField",
            "Purpose",
            "Status",
        ])
    );
    assert_eq!(
        names(members, policy_name),
        expected(&[
            "AccountMutationScopePolicy",
            "AccountVisibilityPolicy",
            "DistinctApproverPolicy",
            "EmployeeScopePolicy",
            "EstateCapabilityScopePolicy",
        ])
    );
    assert_eq!(names(members, currency_name), expected(&["UsdCurrency"]));
    assert_eq!(
        names(members, effect_name),
        expected(&["AccountActivityEffect"])
    );
}
