use worth_query_decl::facade::worth_query_application_schema;

use crate::authorization::*;

use super::authentication::*;
use super::decision_read_manifest::install_operation_decision_reads;
use super::entities::*;
use super::estate::install_estate_world;
use super::fields::*;
use super::governance::*;
use super::operations::*;
use super::precondition_manifest::install_operation_preconditions;
use super::program_manifest::install_operation_program;
use super::relations::*;

worth_query_application_schema! {
    pub schema BankSchema {
        owner: "WORTH.bank",
        version: (1, 0),
        members: |schema| {
            let schema = schema
                .entity(Institution::reference())
                .entity(ExternalPrincipalMapping::reference())
                .entity(Principal::reference())
                .entity(Customer::reference())
                .entity(Business::reference())
                .entity(Account::reference())
                .entity(AccountAuthorization::reference())
                .entity(EmployeeAssignment::reference())
                .entity(PaymentIntent::reference())
                .entity(Approval::reference())
                .entity(JournalEntry::reference())
                .entity(Posting::reference())
                .aspect(
                    ExternalPrincipalMapping::reference(),
                    ExternalPrincipalIdentity::reference(),
                )
                .aspect(Principal::reference(), PrincipalIdentity::reference())
                .aspect(Account::reference(), Identity::reference())
                .aspect(
                    Institution::reference(),
                    InstitutionIdentity::reference(),
                )
                .aspect(Business::reference(), BusinessIdentity::reference())
                .aspect(PaymentIntent::reference(), PaymentIdentity::reference())
                .aspect(Account::reference(), AccountProfile::reference())
                .aspect(Account::reference(), AccountState::reference())
                .aspect(
                    AccountAuthorization::reference(),
                    AuthorizationScope::reference(),
                )
                .aspect(
                    AccountAuthorization::reference(),
                    AuthorizationIdentity::reference(),
                )
                .aspect(EmployeeAssignment::reference(), EmployeeScope::reference())
                .aspect(Posting::reference(), PostingValue::reference())
                .aspect(Posting::reference(), PostingIdentity::reference())
                .aspect(JournalEntry::reference(), JournalIdentity::reference())
                .aspect(JournalEntry::reference(), JournalState::reference())
                .aspect(PaymentIntent::reference(), PaymentState::reference())
                .aspect(PaymentIntent::reference(), PaymentValue::reference())
                .field(
                    ExternalPrincipalMapping::reference(),
                    ExternalIdentityKey::reference(),
                )
                .field(Principal::reference(), PrincipalIdentityField::reference())
                .field(
                    ExternalPrincipalMapping::reference(),
                    ExternalMappingStatus::reference(),
                )
                .field(Account::reference(), AccountIdentity::reference())
                .field(
                    Institution::reference(),
                    InstitutionIdentityField::reference(),
                )
                .field(Business::reference(), BusinessIdentityField::reference())
                .field(
                    PaymentIntent::reference(),
                    PaymentIdentityField::reference(),
                )
                .field(Account::reference(), AccountDisplayName::reference())
                .field(Account::reference(), Kind::reference())
                .field(Account::reference(), AccountingRevision::reference())
                .field(Account::reference(), Status::reference())
                .field(
                    AccountAuthorization::reference(),
                    AuthorizationRole::reference(),
                )
                .field(
                    AccountAuthorization::reference(),
                    AccountAuthorizationIdentity::reference(),
                )
                .field(
                    EmployeeAssignment::reference(),
                    EmployeeAssignmentIdentityField::reference(),
                )
                .field(
                    EmployeeAssignment::reference(),
                    AssignmentRole::reference(),
                )
                .field(Posting::reference(), PostingAmount::reference())
                .field(Posting::reference(), PostingAccountSequence::reference())
                .field(Posting::reference(), PostingIdentityField::reference())
                .field(Posting::reference(), Purpose::reference())
                .field(JournalEntry::reference(), JournalIdentityField::reference())
                .field(JournalEntry::reference(), JournalPurpose::reference())
                .field(
                    PaymentIntent::reference(),
                    PaymentStatusField::reference(),
                )
                .field(PaymentIntent::reference(), PaymentAmount::reference())
                .relation(
                    ExternalPrincipal::reference(),
                    ExternalPrincipalMapping::reference(),
                    Principal::reference(),
                )
                .relation(
                    PrincipalCustomer::reference(),
                    Principal::reference(),
                    Customer::reference(),
                )
                .relation(
                    PersonalOwner::reference(),
                    Principal::reference(),
                    Account::reference(),
                )
                .relation(
                    BusinessOwner::reference(),
                    Business::reference(),
                    Principal::reference(),
                )
                .relation(
                    BusinessAccount::reference(),
                    Business::reference(),
                    Account::reference(),
                )
                .relation(
                    AccountAuthorizedUser::reference(),
                    Principal::reference(),
                    AccountAuthorization::reference(),
                )
                .relation(
                    AuthorizationAccount::reference(),
                    AccountAuthorization::reference(),
                    Account::reference(),
                )
                .relation(
                    InstitutionEmployee::reference(),
                    Institution::reference(),
                    EmployeeAssignment::reference(),
                )
                .relation(
                    AssignmentPrincipal::reference(),
                    EmployeeAssignment::reference(),
                    Principal::reference(),
                )
                .relation(
                    InstitutionAccount::reference(),
                    Institution::reference(),
                    Account::reference(),
                )
                .relation(
                    InstitutionCashAccount::reference(),
                    Institution::reference(),
                    Account::reference(),
                )
                .relation(
                    PaymentSource::reference(),
                    PaymentIntent::reference(),
                    Account::reference(),
                )
                .relation(
                    PaymentDestination::reference(),
                    PaymentIntent::reference(),
                    Account::reference(),
                )
                .relation(
                    PaymentBusiness::reference(),
                    PaymentIntent::reference(),
                    Business::reference(),
                )
                .relation(
                    PaymentInitiator::reference(),
                    Principal::reference(),
                    PaymentIntent::reference(),
                )
                .relation(
                    PaymentApproval::reference(),
                    PaymentIntent::reference(),
                    Approval::reference(),
                )
                .relation(
                    ApprovalPrincipal::reference(),
                    Approval::reference(),
                    Principal::reference(),
                )
                .relation(
                    JournalPosting::reference(),
                    JournalEntry::reference(),
                    Posting::reference(),
                )
                .relation(
                    JournalReversal::reference(),
                    JournalEntry::reference(),
                    JournalEntry::reference(),
                )
                .relation(
                    PostingAccount::reference(),
                    Posting::reference(),
                    Account::reference(),
                )
                .principal_binding(BankPrincipalBinding::reference())
                .ability(OpenAccount::reference())
                .ability(DiscoverOwnAccounts::reference())
                .ability(ViewAccount::reference())
                .ability(ViewAccountAccess::reference())
                .ability(ViewPayment::reference())
                .ability(SendPersonalFunds::reference())
                .ability(ManageAccountAccess::reference())
                .ability(InitiateBusinessFunds::reference())
                .ability(ApproveBusinessFunds::reference())
                .ability(ServiceInstitutionAccount::reference())
                .ability(AuditInstitution::reference())
                .ability(ViewEstateCase::reference())
                .operation(CreatePersonalAccountOperation::reference())
                .operation(CreateBusinessAccountOperation::reference())
                .operation(ApplyOpeningFundingOperation::reference())
                .operation(DepositOperation::reference())
                .operation(WithdrawOperation::reference())
                .operation(SendMoneyOperation::reference())
                .operation(InitiateBusinessPaymentOperation::reference())
                .operation(ApprovePaymentOperation::reference())
                .operation(RejectPaymentOperation::reference())
                .operation(GrantAccountAuthorizationOperation::reference())
                .operation(RevokeAccountAuthorizationOperation::reference())
                .operation(ReverseJournalOperation::reference());
            let schema = install_estate_world(schema);
            let schema = install_operation_preconditions(install_operation_decision_reads(
                install_operation_program(schema),
            ))
                .policy(AccountVisibilityPolicy::reference())
                .policy(AccountMutationScopePolicy::reference())
                .policy(EmployeeScopePolicy::reference())
                .policy(DistinctApproverPolicy::reference())
                .currency(UsdCurrency::reference())
                .effect(AccountActivityEffect::reference())
                .application_query(crate::queries::account_authorized_users_definition())
                .application_query(crate::queries::account_discovery_definition())
                .application_query(crate::queries::account_detail_definition())
                .application_query(crate::queries::account_summary_definition())
                .application_query(crate::queries::account_activity_definition())
                .application_query(crate::queries::payment_detail_definition())
                .application_query(crate::queries::pending_payments_definition())
                .application_query(crate::queries::estate_case_overview_definition())
                .application_query(crate::queries::estate_customer_disclosure_definition())
                .application_query(crate::queries::estate_governance_definition())
                .application_query(crate::queries::institution_audit_definition());
            install_ability_policies(install_operation_abilities(schema))
        }
    }
}

fn install_operation_abilities(
    schema: worth_query_decl::facade::application_schema::ApplicationSchemaDeclarationBuilder<
        BankSchema,
    >,
) -> worth_query_decl::facade::application_schema::ApplicationSchemaDeclarationBuilder<BankSchema> {
    schema
        .operation_requires_ability(
            CreatePersonalAccountOperation::reference(),
            OpenAccount::reference(),
        )
        .operation_requires_ability(
            CreateBusinessAccountOperation::reference(),
            OpenAccount::reference(),
        )
        .operation_requires_ability(
            ApplyOpeningFundingOperation::reference(),
            ServiceInstitutionAccount::reference(),
        )
        .operation_requires_ability(
            DepositOperation::reference(),
            ServiceInstitutionAccount::reference(),
        )
        .operation_requires_ability(
            WithdrawOperation::reference(),
            ServiceInstitutionAccount::reference(),
        )
        .operation_requires_ability(
            SendMoneyOperation::reference(),
            SendPersonalFunds::reference(),
        )
        .operation_requires_ability(
            InitiateBusinessPaymentOperation::reference(),
            InitiateBusinessFunds::reference(),
        )
        .operation_requires_ability(
            ApprovePaymentOperation::reference(),
            ApproveBusinessFunds::reference(),
        )
        .operation_requires_ability(
            RejectPaymentOperation::reference(),
            ApproveBusinessFunds::reference(),
        )
        .operation_requires_ability(
            GrantAccountAuthorizationOperation::reference(),
            ManageAccountAccess::reference(),
        )
        .operation_requires_ability(
            RevokeAccountAuthorizationOperation::reference(),
            ManageAccountAccess::reference(),
        )
        .operation_requires_ability(
            ReverseJournalOperation::reference(),
            ServiceInstitutionAccount::reference(),
        )
}
