use worth_query_decl::facade::worth_query_application_schema;

use super::entities::*;
use super::fields::*;
use super::governance::*;
use super::operations::*;
use super::program_manifest::install_operation_program;
use super::relations::*;

worth_query_application_schema! {
    pub schema BankSchema {
        owner: bank,
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
                .entity(IdempotencyRecord::reference())
                .aspect(Account::reference(), Identity::reference())
                .aspect(Account::reference(), AccountProfile::reference())
                .aspect(Account::reference(), AccountState::reference())
                .aspect(
                    AccountAuthorization::reference(),
                    AuthorizationScope::reference(),
                )
                .aspect(EmployeeAssignment::reference(), EmployeeScope::reference())
                .aspect(Posting::reference(), PostingValue::reference())
                .aspect(PaymentIntent::reference(), PaymentState::reference())
                .field(Account::reference(), AccountIdentity::reference())
                .field(Account::reference(), AccountDisplayName::reference())
                .field(Account::reference(), Kind::reference())
                .field(Account::reference(), AvailableBalance::reference())
                .field(Account::reference(), Status::reference())
                .field(
                    AccountAuthorization::reference(),
                    AuthorizationRole::reference(),
                )
                .field(
                    EmployeeAssignment::reference(),
                    AssignmentRole::reference(),
                )
                .field(Posting::reference(), PostingAmount::reference())
                .field(Posting::reference(), Purpose::reference())
                .field(
                    PaymentIntent::reference(),
                    PaymentStatusField::reference(),
                )
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
                    PostingAccount::reference(),
                    Posting::reference(),
                    Account::reference(),
                )
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
            install_operation_program(schema)
                .policy(AccountVisibilityPolicy::reference())
                .policy(AccountMutationScopePolicy::reference())
                .policy(EmployeeScopePolicy::reference())
                .policy(DistinctApproverPolicy::reference())
                .currency(UsdCurrency::reference())
                .effect(AccountActivityEffect::reference())
        }
    }
}
