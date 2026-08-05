use worth_query_decl::facade::application_query::{
    ApplicationQueryResultFieldRef, ApplicationQueryResultRelationRef, ExactlyOneResult,
    ForwardResultTraversal,
};
use worth_query_decl::facade::application_schema::{
    EqualityPredicate, NoApplicationCurrency, ReadOnly, ReadWrite,
};

use crate::{
    model::{AccountId, AccountName},
    schema::{
        Account, AccountDisplayName, AccountIdentity, AccountProfile, AccountState, AccountStatus,
        BankSchema, EstateAccount, EstateCase, Identity, Status,
    },
};

use super::emergency_account_details::EstateEmergencyAccountDetailsQuery;

pub(super) struct AccountRelationSlot;
pub(super) struct AccountIdentitySlot;
pub(super) struct AccountNameSlot;
pub(super) struct AccountStatusSlot;

pub(super) fn estate_account() -> ApplicationQueryResultRelationRef<
    EstateEmergencyAccountDetailsQuery,
    AccountRelationSlot,
    BankSchema,
    EstateAccount,
    EstateCase,
    Account,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one("account", EstateAccount::reference())
}

pub(super) fn account_identity() -> ApplicationQueryResultFieldRef<
    EstateEmergencyAccountDetailsQuery,
    AccountIdentitySlot,
    BankSchema,
    Account,
    Identity,
    AccountIdentity,
    AccountId,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
> {
    ApplicationQueryResultFieldRef::new("account", AccountIdentity::reference())
}

pub(super) fn account_name() -> ApplicationQueryResultFieldRef<
    EstateEmergencyAccountDetailsQuery,
    AccountNameSlot,
    BankSchema,
    Account,
    AccountProfile,
    AccountDisplayName,
    AccountName,
    ReadWrite,
    EqualityPredicate,
    NoApplicationCurrency,
> {
    ApplicationQueryResultFieldRef::new("display_name", AccountDisplayName::reference())
}

pub(super) fn account_status() -> ApplicationQueryResultFieldRef<
    EstateEmergencyAccountDetailsQuery,
    AccountStatusSlot,
    BankSchema,
    Account,
    AccountState,
    Status,
    AccountStatus,
    ReadWrite,
    EqualityPredicate,
    NoApplicationCurrency,
> {
    ApplicationQueryResultFieldRef::new("status", Status::reference())
}
