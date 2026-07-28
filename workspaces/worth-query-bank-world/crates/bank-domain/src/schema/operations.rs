use worth_query_decl::facade::{
    worth_query_operation, worth_query_operation_creates, worth_query_operation_deletes,
    worth_query_operation_emits, worth_query_operation_links, worth_query_operation_unlinks,
    worth_query_operation_writes,
};

use crate::model::{
    AccountAuthorizationId, AccountId, AccountName, BankPrincipalId, BusinessId, CustomerRole,
    InstitutionId, JournalEntryId, Money, PaymentId, USD,
};

use super::entities::{
    Account, AccountAuthorization, Approval, JournalEntry, PaymentIntent, Posting,
};
use super::fields::{
    AccountDisplayName, AuthorizationRole, Kind, PaymentStatusField, PostingAmount, Purpose, Status,
};
use super::governance::AccountActivityEffect;
use super::relations::{
    AccountAuthorizedUser, ApprovalPrincipal, AuthorizationAccount, BusinessAccount,
    InstitutionAccount, JournalPosting, PaymentApproval, PaymentDestination, PaymentInitiator,
    PaymentSource, PersonalOwner, PostingAccount,
};
use super::BankSchema;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreatePersonalAccount {
    pub institution: InstitutionId,
    pub owner: BankPrincipalId,
    pub display_name: AccountName,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateBusinessAccount {
    pub institution: InstitutionId,
    pub business: BusinessId,
    pub display_name: AccountName,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplyOpeningFunding {
    pub institution: InstitutionId,
    pub account: AccountId,
    pub amount: Money<USD>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deposit {
    pub institution: InstitutionId,
    pub account: AccountId,
    pub amount: Money<USD>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Withdraw {
    pub institution: InstitutionId,
    pub account: AccountId,
    pub amount: Money<USD>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SendMoney {
    pub from: AccountId,
    pub recipient: BankPrincipalId,
    pub amount: Money<USD>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitiateBusinessPayment {
    pub business: BusinessId,
    pub from: AccountId,
    pub recipient: BankPrincipalId,
    pub amount: Money<USD>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApprovePayment {
    pub payment: PaymentId,
    pub approver: BankPrincipalId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RejectPayment {
    pub payment: PaymentId,
    pub rejecting_principal: BankPrincipalId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrantAccountAuthorization {
    pub account: AccountId,
    pub principal: BankPrincipalId,
    pub role: CustomerRole,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevokeAccountAuthorization {
    pub authorization: AccountAuthorizationId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReversalReason {
    Duplicate,
    OperatorCorrection,
    ExternalReturn,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReverseJournal {
    pub journal: JournalEntryId,
    pub reason: ReversalReason,
}

worth_query_operation!(pub CreatePersonalAccountOperation(CreatePersonalAccount) in BankSchema);
worth_query_operation!(pub CreateBusinessAccountOperation(CreateBusinessAccount) in BankSchema);
worth_query_operation!(pub ApplyOpeningFundingOperation(ApplyOpeningFunding) in BankSchema);
worth_query_operation!(pub DepositOperation(Deposit) in BankSchema);
worth_query_operation!(pub WithdrawOperation(Withdraw) in BankSchema);
worth_query_operation!(pub SendMoneyOperation(SendMoney) in BankSchema);
worth_query_operation!(
    pub InitiateBusinessPaymentOperation(InitiateBusinessPayment) in BankSchema
);
worth_query_operation!(pub ApprovePaymentOperation(ApprovePayment) in BankSchema);
worth_query_operation!(pub RejectPaymentOperation(RejectPayment) in BankSchema);
worth_query_operation!(
    pub GrantAccountAuthorizationOperation(GrantAccountAuthorization) in BankSchema
);
worth_query_operation!(
    pub RevokeAccountAuthorizationOperation(RevokeAccountAuthorization) in BankSchema
);
worth_query_operation!(pub ReverseJournalOperation(ReverseJournal) in BankSchema);

worth_query_operation_writes!(
    CreatePersonalAccountOperation => [AccountDisplayName, Kind, Status]
);
worth_query_operation_writes!(
    CreateBusinessAccountOperation => [AccountDisplayName, Kind, Status]
);
worth_query_operation_creates!(CreatePersonalAccountOperation => [Account]);
worth_query_operation_creates!(CreateBusinessAccountOperation => [Account]);
worth_query_operation_links!(
    CreatePersonalAccountOperation => [PersonalOwner, InstitutionAccount]
);
worth_query_operation_links!(
    CreateBusinessAccountOperation => [BusinessAccount, InstitutionAccount]
);

worth_query_operation_writes!(
    ApplyOpeningFundingOperation => [PostingAmount, Purpose]
);
worth_query_operation_writes!(DepositOperation => [PostingAmount, Purpose]);
worth_query_operation_writes!(WithdrawOperation => [PostingAmount, Purpose]);
worth_query_operation_writes!(SendMoneyOperation => [PostingAmount, Purpose]);
worth_query_operation_writes!(ReverseJournalOperation => [PostingAmount, Purpose]);
worth_query_operation_creates!(ApplyOpeningFundingOperation => [JournalEntry, Posting]);
worth_query_operation_creates!(DepositOperation => [JournalEntry, Posting]);
worth_query_operation_creates!(WithdrawOperation => [JournalEntry, Posting]);
worth_query_operation_creates!(SendMoneyOperation => [JournalEntry, Posting]);
worth_query_operation_creates!(ReverseJournalOperation => [JournalEntry, Posting]);
worth_query_operation_links!(
    ApplyOpeningFundingOperation => [JournalPosting, PostingAccount]
);
worth_query_operation_links!(DepositOperation => [JournalPosting, PostingAccount]);
worth_query_operation_links!(WithdrawOperation => [JournalPosting, PostingAccount]);
worth_query_operation_links!(SendMoneyOperation => [JournalPosting, PostingAccount]);
worth_query_operation_links!(ReverseJournalOperation => [JournalPosting, PostingAccount]);

worth_query_operation_creates!(InitiateBusinessPaymentOperation => [PaymentIntent]);
worth_query_operation_links!(
    InitiateBusinessPaymentOperation => [PaymentSource, PaymentDestination, PaymentInitiator]
);
worth_query_operation_creates!(ApprovePaymentOperation => [Approval]);
worth_query_operation_links!(
    ApprovePaymentOperation => [PaymentApproval, ApprovalPrincipal]
);
worth_query_operation_writes!(ApprovePaymentOperation => [PaymentStatusField]);
worth_query_operation_writes!(RejectPaymentOperation => [PaymentStatusField]);
worth_query_operation_creates!(
    GrantAccountAuthorizationOperation => [AccountAuthorization]
);
worth_query_operation_writes!(
    GrantAccountAuthorizationOperation => [AuthorizationRole]
);
worth_query_operation_links!(
    GrantAccountAuthorizationOperation => [AccountAuthorizedUser, AuthorizationAccount]
);
worth_query_operation_unlinks!(
    RevokeAccountAuthorizationOperation => [AccountAuthorizedUser, AuthorizationAccount]
);
worth_query_operation_deletes!(
    RevokeAccountAuthorizationOperation => [AccountAuthorization]
);

worth_query_operation_emits!(ApplyOpeningFundingOperation => [AccountActivityEffect]);
worth_query_operation_emits!(DepositOperation => [AccountActivityEffect]);
worth_query_operation_emits!(WithdrawOperation => [AccountActivityEffect]);
worth_query_operation_emits!(SendMoneyOperation => [AccountActivityEffect]);
worth_query_operation_emits!(ApprovePaymentOperation => [AccountActivityEffect]);
worth_query_operation_emits!(ReverseJournalOperation => [AccountActivityEffect]);
