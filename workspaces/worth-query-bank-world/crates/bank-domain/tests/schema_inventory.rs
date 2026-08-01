use bank_domain::schema::{BankPrincipalBinding, BankSchema};
use worth_query_decl::facade::application_schema::ApplicationSchemaMember;

#[path = "schema_inventory/field_inventory.rs"]
mod field_inventory;
#[path = "schema_inventory/support.rs"]
mod support;

use support::*;

#[test]
fn bank_manifest_matches_the_frozen_schema_world() {
    let declaration = BankSchema::declaration().unwrap();
    let members = declaration.erased().members();
    assert_entity_and_relation_inventory(members);
    assert_operation_inventory(members);
    assert_capability_inventory(members);
    assert_application_query_inventory(members);
    field_inventory::assert_field_and_governance_inventory(members);
    assert_account_creation_programs(members);
    assert_money_programs(members);
    assert_payment_and_authorization_programs(members);
    assert_eq!(
        declaration.erased().owner(),
        "WORTH.bank",
        "host package and application schema must share one canonical owner"
    );
    let binding = BankPrincipalBinding::reference();
    assert_eq!(binding.mapping_entity(), "ExternalPrincipalMapping");
    assert_eq!(binding.identity_field(), "ExternalIdentityKey");
    assert_eq!(binding.status_field(), "ExternalMappingStatus");
    assert_eq!(binding.target_relation(), "ExternalPrincipal");
    assert_eq!(binding.principal_entity(), "Principal");
    assert_eq!(binding.principal_identity_aspect(), "PrincipalIdentity");
    assert_eq!(binding.principal_identity_field(), "PrincipalIdentityField");
}

#[test]
fn send_projection_capability_does_not_widen_mandatory_commit_dependencies() {
    let declaration = BankSchema::declaration().unwrap();
    let members = declaration.erased().members();
    assert_decision_reads(
        members,
        "SendMoneyOperation",
        &[
            "field:Account/AccountState/AccountingRevision",
            "field:Account/AccountState/Status",
            "field:Account/Identity/AccountIdentity",
            "field:Principal/PrincipalIdentity/PrincipalIdentityField",
            "relation:PersonalOwner:Principal->Account",
        ],
    );
}

#[test]
fn revoke_projection_capability_does_not_widen_mandatory_commit_dependencies() {
    let declaration = BankSchema::declaration().unwrap();
    let members = declaration.erased().members();
    assert_decision_reads(
        members,
        "RevokeAccountAuthorizationOperation",
        &[
            "entity:AccountAuthorization",
            "field:Account/Identity/AccountIdentity",
            "field:AccountAuthorization/AuthorizationIdentity/AccountAuthorizationIdentity",
            "field:AccountAuthorization/AuthorizationScope/AuthorizationRole",
            "field:Principal/PrincipalIdentity/PrincipalIdentityField",
            "relation:AccountAuthorizedUser:Principal->AccountAuthorization",
            "relation:AuthorizationAccount:AccountAuthorization->Account",
        ],
    );
}

fn assert_entity_and_relation_inventory(members: &[ApplicationSchemaMember]) {
    assert_eq!(
        names(members, entity_name),
        expected(&[
            "Account",
            "AccountAuthorization",
            "Approval",
            "Branch",
            "Business",
            "CapabilityGrant",
            "Customer",
            "DeathNotice",
            "EmployeeAssignment",
            "EmergencyAccess",
            "EstateCase",
            "ExternalPrincipalMapping",
            "Institution",
            "JournalEntry",
            "LegalAuthority",
            "MandatoryReview",
            "PaymentIntent",
            "Posting",
            "Principal",
        ])
    );
    assert_eq!(
        names(members, relation_name),
        expected(&[
            "AccountAuthorizedUser",
            "ApprovalPrincipal",
            "AssignmentPrincipal",
            "AuthorizationAccount",
            "BusinessAccount",
            "BusinessOwner",
            "BranchInstitution",
            "CapabilityAccount",
            "CapabilityBranch",
            "CapabilityEstate",
            "CapabilityGrantee",
            "CapabilityGrantor",
            "CapabilityInstitution",
            "CapabilityParent",
            "DeathNoticeSubject",
            "EmergencyApprover",
            "EmergencyGrant",
            "EmergencyRequester",
            "EmergencyReview",
            "EstateAccount",
            "EstateAssignment",
            "EstateAuthorizedSigner",
            "EstateBeneficiary",
            "EstateBranch",
            "EstateDeathNotice",
            "EstateDeceased",
            "EstateExecutor",
            "EstateJointOwner",
            "ExternalPrincipal",
            "InstitutionAccount",
            "InstitutionCashAccount",
            "InstitutionEmployee",
            "JournalPosting",
            "JournalReversal",
            "LegalAuthorityEstate",
            "LegalAuthorityHolder",
            "PaymentApproval",
            "PaymentBusiness",
            "PaymentDestination",
            "PaymentInitiator",
            "PaymentSource",
            "PersonalOwner",
            "PostingAccount",
            "PrincipalCustomer",
            "ReviewEstate",
            "ReviewPrincipal",
        ])
    );
}

fn assert_operation_inventory(members: &[ApplicationSchemaMember]) {
    assert_eq!(
        names(members, operation_name),
        expected(&[
            "ApplyOpeningFundingOperation",
            "ApproveEstateEmergencyAccessOperation",
            "ApprovePaymentOperation",
            "CompleteEstateMandatoryReviewOperation",
            "CreateBusinessAccountOperation",
            "CreatePersonalAccountOperation",
            "DelegateEstateCapabilityOperation",
            "DepositOperation",
            "DisburseEstateOperation",
            "FreezeEstateAccountOperation",
            "GrantAccountAuthorizationOperation",
            "InitiateBusinessPaymentOperation",
            "NotifyDeathEstateOperation",
            "OpenEstateCaseOperation",
            "RecognizeEstateExecutorOperation",
            "RejectPaymentOperation",
            "ReleaseEstateOperation",
            "RequestEstateEmergencyAccessOperation",
            "ReverseJournalOperation",
            "RevokeAccountAuthorizationOperation",
            "RevokeEstateCapabilityOperation",
            "RevokeEstateEmergencyAccessOperation",
            "SendMoneyOperation",
            "ViewRestrictedEstateOperation",
            "WithdrawOperation",
        ])
    );
}

fn assert_capability_inventory(members: &[ApplicationSchemaMember]) {
    assert_eq!(
        names(members, application_capability_name),
        expected(&[
            "ApproveEstateEmergencyAccessCapability",
            "CompleteEstateMandatoryReviewCapability",
            "DelegateEstateCapability",
            "DisburseEstateCapability",
            "FreezeEstateAccountCapability",
            "NotifyDeathEstateCapability",
            "OpenEstateCaseCapability",
            "RecognizeEstateExecutorCapability",
            "ReleaseEstateCapability",
            "RequestEstateEmergencyAccessCapability",
            "RevokeEstateCapability",
            "RevokeEstateEmergencyAccessCapability",
            "ViewEstateAdministrationCapability",
            "ViewEstateEmergencyProtectionCapability",
            "ViewEstateIdentityVerificationCapability",
            "ViewEstateLegalComplianceCapability",
            "ViewEstateMandatoryReviewCapability",
        ])
    );
}

fn assert_application_query_inventory(members: &[ApplicationSchemaMember]) {
    assert_eq!(
        names(members, application_query_name),
        expected(&[
            "account_activity",
            "account_authorized_users",
            "account_detail",
            "account_discovery",
            "account_summary",
            "estate_case_overview",
            "estate_customer_identity",
            "estate_governance_context",
            "institution_audit",
            "payment_detail",
            "pending_payments",
        ])
    );
}

fn assert_account_creation_programs(members: &[ApplicationSchemaMember]) {
    assert_program(
        members,
        "CreatePersonalAccountOperation",
        &[
            "create:Account",
            "link:InstitutionAccount:Institution->Account",
            "link:PersonalOwner:Principal->Account",
            "write:Account/Identity/AccountIdentity",
            "write:Account/AccountProfile/AccountDisplayName",
            "write:Account/AccountProfile/Kind",
            "write:Account/AccountState/AccountingRevision",
            "write:Account/AccountState/Status",
        ],
    );
    assert_program(
        members,
        "CreateBusinessAccountOperation",
        &[
            "create:Account",
            "link:BusinessAccount:Business->Account",
            "link:InstitutionAccount:Institution->Account",
            "write:Account/Identity/AccountIdentity",
            "write:Account/AccountProfile/AccountDisplayName",
            "write:Account/AccountProfile/Kind",
            "write:Account/AccountState/AccountingRevision",
            "write:Account/AccountState/Status",
        ],
    );
}

fn assert_money_programs(members: &[ApplicationSchemaMember]) {
    for operation in [
        "ApplyOpeningFundingOperation",
        "DepositOperation",
        "ReverseJournalOperation",
        "SendMoneyOperation",
        "WithdrawOperation",
    ] {
        assert_money_program(members, operation);
    }
}

fn assert_payment_and_authorization_programs(members: &[ApplicationSchemaMember]) {
    assert_program(
        members,
        "InitiateBusinessPaymentOperation",
        &[
            "create:PaymentIntent",
            "link:PaymentBusiness:PaymentIntent->Business",
            "link:PaymentDestination:PaymentIntent->Account",
            "link:PaymentInitiator:Principal->PaymentIntent",
            "link:PaymentSource:PaymentIntent->Account",
            "write:PaymentIntent/PaymentIdentity/PaymentIdentityField",
            "write:PaymentIntent/PaymentState/PaymentStatusField",
            "write:PaymentIntent/PaymentValue/PaymentAmount",
        ],
    );
    assert_program(
        members,
        "ApprovePaymentOperation",
        &[
            "create:Approval",
            "create:JournalEntry",
            "create:Posting",
            "emit:AccountActivityEffect",
            "link:ApprovalPrincipal:Approval->Principal",
            "link:JournalPosting:JournalEntry->Posting",
            "link:PaymentApproval:PaymentIntent->Approval",
            "link:PostingAccount:Posting->Account",
            "write:Account/AccountState/AccountingRevision",
            "write:JournalEntry/JournalIdentity/JournalIdentityField",
            "write:JournalEntry/JournalState/JournalPurpose",
            "write:PaymentIntent/PaymentState/PaymentStatusField",
            "write:Posting/PostingIdentity/PostingIdentityField",
            "write:Posting/PostingValue/PostingAccountSequence",
            "write:Posting/PostingValue/PostingAmount",
            "write:Posting/PostingValue/Purpose",
        ],
    );
    assert_program(
        members,
        "RejectPaymentOperation",
        &[
            "create:Approval",
            "link:ApprovalPrincipal:Approval->Principal",
            "link:PaymentApproval:PaymentIntent->Approval",
            "write:PaymentIntent/PaymentState/PaymentStatusField",
        ],
    );
    assert_program(
        members,
        "GrantAccountAuthorizationOperation",
        &[
            "create:AccountAuthorization",
            "link:AccountAuthorizedUser:Principal->AccountAuthorization",
            "link:AuthorizationAccount:AccountAuthorization->Account",
            "write:AccountAuthorization/AuthorizationIdentity/AccountAuthorizationIdentity",
            "write:AccountAuthorization/AuthorizationScope/AuthorizationRole",
        ],
    );
    assert_program(
        members,
        "RevokeAccountAuthorizationOperation",
        &[
            "delete:AccountAuthorization",
            "unlink:AccountAuthorizedUser:Principal->AccountAuthorization",
            "unlink:AuthorizationAccount:AccountAuthorization->Account",
        ],
    );
}

#[test]
fn bank_schema_source_has_no_raw_query_descriptor_or_dynamic_key_lane() {
    let schema_sources = [
        include_str!("../src/schema/entities.rs"),
        include_str!("../src/schema/authentication.rs"),
        include_str!("../src/schema/decision_read_manifest.rs"),
        include_str!("../src/schema/fields.rs"),
        include_str!("../src/schema/governance.rs"),
        include_str!("../src/schema/manifest.rs"),
        include_str!("../src/schema/operations.rs"),
        include_str!("../src/schema/program_manifest.rs"),
        include_str!("../src/schema/relations.rs"),
        include_str!("../src/schema/values.rs"),
    ]
    .join("\n");
    for forbidden in [
        "from_schema_identifier(",
        "from_schema_identifiers(",
        "ApplicationEntityRef::<",
        "ApplicationFieldRef::<",
        "DynamicApplication",
    ] {
        assert!(
            !schema_sources.contains(forbidden),
            "bank schema contains forbidden raw lane: {forbidden}"
        );
    }

    let manifest = include_str!("../Cargo.toml");
    for forbidden_dependency in [
        "worth-query-declaration",
        "worth-query-installation",
        "worth-query-execution",
        "worth-query-replay",
        "worth-runtime-bridge",
        "worth-relational",
    ] {
        assert!(
            !manifest.contains(forbidden_dependency),
            "bank-domain crosses audience boundary through {forbidden_dependency}"
        );
    }
}
