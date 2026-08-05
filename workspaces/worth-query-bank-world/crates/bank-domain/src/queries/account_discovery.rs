use worth_query_decl::facade::{
    application_query::{
        ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
        ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
        ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
        ApplicationQueryOrderingDirection, ApplicationQueryResultFieldRef,
        ApplicationQueryResultShapeBuilder, ApplicationQueryRootPath,
    },
    application_schema::{EqualityPredicate, NoApplicationCurrency, ReadOnly},
    worth_query_application_query,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationProjection, WorthQueryApplicationProjectionDenial,
    WorthQueryApplicationProjectionRow,
};

use crate::{
    authorization::DiscoverOwnAccounts,
    model::AccountId,
    reads::VisibleAccount,
    schema::{
        Account, AccountAuthorizedUser, AccountIdentity, AuthorizationAccount, BankSchema,
        BusinessAccount, BusinessOwner, Identity, PersonalOwner, Principal,
    },
};

pub struct AccountDiscoveryQueryParameters;
pub struct AccountIdentitySlot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccountDiscoveryRequest;

pub const fn accounts() -> AccountDiscoveryRequest {
    AccountDiscoveryRequest
}

worth_query_application_query!(
    pub AccountDiscoveryQuery in BankSchema,
    parameters AccountDiscoveryQueryParameters,
    result VisibleAccount,
    scope Principal,
    name "account_discovery"
);

pub fn account_discovery_definition() -> ApplicationQueryDefinition<
    BankSchema,
    AccountDiscoveryQuery,
    AccountDiscoveryQueryParameters,
    VisibleAccount,
    Principal,
> {
    let identity = account_identity();
    let shape = ApplicationQueryResultShapeBuilder::new(Account::reference())
        .field(identity)
        .build();
    ApplicationQueryDefinitionBuilder::requires_ability(
        AccountDiscoveryQuery::reference(),
        Account::reference(),
        Principal::reference(),
        shape,
        ApplicationQueryCardinality::Many,
        ApplicationQueryDependencyCeiling::bounded(2, 5, 1),
        ApplicationQueryDisclosureContract::public(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
        DiscoverOwnAccounts::reference(),
    )
    .root_path(
        ApplicationQueryRootPath::from(Principal::reference()).forward(PersonalOwner::reference()),
    )
    .root_path(
        ApplicationQueryRootPath::from(Principal::reference())
            .forward(AccountAuthorizedUser::reference())
            .forward(AuthorizationAccount::reference()),
    )
    .root_path(
        ApplicationQueryRootPath::from(Principal::reference())
            .reverse(BusinessOwner::reference())
            .forward(BusinessAccount::reference()),
    )
    .order_by(identity, ApplicationQueryOrderingDirection::Ascending)
    .build()
    .expect("bank account discovery query is statically canonical")
}

impl WorthQueryApplicationProjection<BankSchema, AccountDiscoveryQuery> for VisibleAccount {
    fn project(
        row: &WorthQueryApplicationProjectionRow<'_, BankSchema, AccountDiscoveryQuery>,
    ) -> Result<Self, WorthQueryApplicationProjectionDenial> {
        Ok(Self::new(row.field(account_identity())?))
    }
}

fn account_identity() -> ApplicationQueryResultFieldRef<
    AccountDiscoveryQuery,
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
