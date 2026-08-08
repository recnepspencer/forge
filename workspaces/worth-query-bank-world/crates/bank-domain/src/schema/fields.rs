use worth_query_decl::facade::{worth_query_aspect, worth_query_field};

use crate::model::{
    AccountAuthorizationId, AccountId, AccountJournalRevision, AccountName, BusinessId,
    CustomerRole, EmployeeAssignmentId, EmployeeRole, InstitutionId, JournalEntryId, Money,
    PaymentId, PostingId, SignedMoney, USD,
};

use super::entities::{
    Account, AccountAuthorization, Business, EmployeeAssignment, Institution, JournalEntry,
    PaymentIntent, Posting,
};
use super::governance::UsdCurrency;
use super::values::{AccountKind, AccountStatus, PaymentStatus, PostingPurpose};
use super::BankSchema;

worth_query_aspect!(pub Identity in BankSchema, Account);
worth_query_aspect!(pub InstitutionIdentity in BankSchema, Institution);
worth_query_aspect!(pub BusinessIdentity in BankSchema, Business);
worth_query_aspect!(pub PaymentIdentity in BankSchema, PaymentIntent);
worth_query_aspect!(pub AccountProfile in BankSchema, Account);
worth_query_aspect!(pub AccountState in BankSchema, Account);
worth_query_aspect!(pub AuthorizationScope in BankSchema, AccountAuthorization);
worth_query_aspect!(pub AuthorizationIdentity in BankSchema, AccountAuthorization);
worth_query_aspect!(pub EmployeeScope in BankSchema, EmployeeAssignment);
worth_query_aspect!(pub PostingValue in BankSchema, Posting);
worth_query_aspect!(pub PostingIdentity in BankSchema, Posting);
worth_query_aspect!(pub JournalIdentity in BankSchema, JournalEntry);
worth_query_aspect!(pub JournalState in BankSchema, JournalEntry);
worth_query_aspect!(pub PaymentState in BankSchema, PaymentIntent);
worth_query_aspect!(pub PaymentValue in BankSchema, PaymentIntent);

worth_query_field!(
    pub AccountIdentity in BankSchema, Account, Identity:
    AccountId, read_only, equality
);
worth_query_field!(
    pub InstitutionIdentityField in BankSchema, Institution, InstitutionIdentity:
    InstitutionId, read_only, equality
);
worth_query_field!(
    pub BusinessIdentityField in BankSchema, Business, BusinessIdentity:
    BusinessId, read_only, equality
);
worth_query_field!(
    pub PaymentIdentityField in BankSchema, PaymentIntent, PaymentIdentity:
    PaymentId, read_only, equality
);
worth_query_field!(
    pub AccountDisplayName in BankSchema, Account, AccountProfile:
    AccountName, read_write, equality
);
worth_query_field!(
    pub Kind in BankSchema, Account, AccountProfile:
    AccountKind, read_write, equality
);
worth_query_field!(
    pub AccountingRevision in BankSchema, Account, AccountState:
    AccountJournalRevision, read_write, equality
);
worth_query_field!(
    pub Status in BankSchema, Account, AccountState:
    AccountStatus, read_write, equality
);
worth_query_field!(
    pub AuthorizationRole in BankSchema, AccountAuthorization, AuthorizationScope:
    CustomerRole, read_write, equality
);
worth_query_field!(
    pub AccountAuthorizationIdentity in BankSchema, AccountAuthorization, AuthorizationIdentity:
    AccountAuthorizationId, read_only, equality
);
worth_query_field!(
    pub EmployeeAssignmentIdentityField in BankSchema, EmployeeAssignment, EmployeeScope:
    EmployeeAssignmentId, read_only, equality
);
worth_query_field!(
    pub AssignmentRole in BankSchema, EmployeeAssignment, EmployeeScope:
    EmployeeRole, read_write, equality
);
worth_query_field!(
    pub PostingAmount in BankSchema, Posting, PostingValue:
    SignedMoney<USD>, unit UsdCurrency, read_write, no_equality
);
worth_query_field!(
    pub PostingAccountSequence in BankSchema, Posting, PostingValue:
    AccountJournalRevision, read_write, equality
);
worth_query_field!(
    pub PostingIdentityField in BankSchema, Posting, PostingIdentity:
    PostingId, read_only, equality
);
worth_query_field!(
    pub Purpose in BankSchema, Posting, PostingValue:
    PostingPurpose, read_write, equality
);
worth_query_field!(
    pub PaymentStatusField in BankSchema, PaymentIntent, PaymentState:
    PaymentStatus, read_write, equality
);
worth_query_field!(
    pub JournalIdentityField in BankSchema, JournalEntry, JournalIdentity:
    JournalEntryId, read_only, equality
);
worth_query_field!(
    pub JournalPurpose in BankSchema, JournalEntry, JournalState:
    PostingPurpose, read_write, equality
);
worth_query_field!(
    pub PaymentAmount in BankSchema, PaymentIntent, PaymentValue:
    Money<USD>, unit UsdCurrency, read_write, no_equality
);
