use worth_query_decl::facade::{worth_query_aspect, worth_query_field};

use crate::model::{CustomerRole, EmployeeRole, Money, USD};

use super::entities::{Account, AccountAuthorization, EmployeeAssignment, PaymentIntent, Posting};
use super::governance::UsdCurrency;
use super::values::{AccountKind, AccountStatus, PaymentStatus, PostingPurpose};
use super::BankSchema;

worth_query_aspect!(pub Identity in BankSchema, Account);
worth_query_aspect!(pub AccountProfile in BankSchema, Account);
worth_query_aspect!(pub AccountState in BankSchema, Account);
worth_query_aspect!(pub AuthorizationScope in BankSchema, AccountAuthorization);
worth_query_aspect!(pub EmployeeScope in BankSchema, EmployeeAssignment);
worth_query_aspect!(pub PostingValue in BankSchema, Posting);
worth_query_aspect!(pub PaymentState in BankSchema, PaymentIntent);

worth_query_field!(
    pub AccountIdentity in BankSchema, Account, Identity:
    u64, read_only, equality
);
worth_query_field!(
    pub AccountDisplayName in BankSchema, Account, AccountProfile:
    String, read_write, equality
);
worth_query_field!(
    pub Kind in BankSchema, Account, AccountProfile:
    AccountKind, read_write, equality
);
worth_query_field!(
    pub AvailableBalance in BankSchema, Account, AccountState:
    Money<USD>, currency UsdCurrency, read_only, no_equality
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
    pub AssignmentRole in BankSchema, EmployeeAssignment, EmployeeScope:
    EmployeeRole, read_write, equality
);
worth_query_field!(
    pub PostingAmount in BankSchema, Posting, PostingValue:
    Money<USD>, currency UsdCurrency, read_write, no_equality
);
worth_query_field!(
    pub Purpose in BankSchema, Posting, PostingValue:
    PostingPurpose, read_write, equality
);
worth_query_field!(
    pub PaymentStatusField in BankSchema, PaymentIntent, PaymentState:
    PaymentStatus, read_write, equality
);
