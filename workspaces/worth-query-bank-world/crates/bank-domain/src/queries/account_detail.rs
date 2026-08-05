use worth_query_decl::facade::worth_query_application_query;
use worth_query_decl::facade::{
    application_query::{
        ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
        ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
        ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
        ApplicationQueryResultFieldRef, ApplicationQueryResultRelationRef,
        ApplicationQueryResultShapeBuilder, OptionalOneResult, ReverseResultTraversal,
    },
    application_schema::{EqualityPredicate, NoApplicationCurrency, ReadOnly},
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationProjection, WorthQueryApplicationProjectionDenial,
    WorthQueryApplicationProjectionRow,
};

use crate::authorization::ViewAccount;
use crate::model::{AccountId, BankPrincipalId, BusinessId, InstitutionId};
use crate::reads::AccountDetail;
use crate::schema::{
    Account, AccountKind, BankSchema, Business, BusinessAccount, BusinessIdentity,
    BusinessIdentityField, Institution, InstitutionAccount, InstitutionIdentity,
    InstitutionIdentityField, PersonalOwner, Principal, PrincipalIdentity, PrincipalIdentityField,
};

use super::account_summary_projection::{account_summary_shape, project_account_summary};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccountDetailQueryParameters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccountDetailRequest {
    account: AccountId,
}

impl AccountDetailRequest {
    pub const fn new(account: AccountId) -> Self {
        Self { account }
    }

    pub const fn account(self) -> AccountId {
        self.account
    }
}

pub const fn account_detail(account: AccountId) -> AccountDetailRequest {
    AccountDetailRequest::new(account)
}

worth_query_application_query!(
    pub AccountDetailQuery in BankSchema,
    parameters AccountDetailQueryParameters,
    result AccountDetail,
    scope Account,
    name "account_detail"
);

struct InstitutionSlot;
struct InstitutionIdentitySlot;
struct PersonalOwnerSlot;
struct PrincipalIdentitySlot;
struct BusinessOwnerSlot;
struct BusinessIdentitySlot;

type InstitutionIdentitySelector = ApplicationQueryResultFieldRef<
    AccountDetailQuery,
    InstitutionIdentitySlot,
    BankSchema,
    Institution,
    InstitutionIdentity,
    InstitutionIdentityField,
    InstitutionId,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
>;

type PrincipalIdentitySelector = ApplicationQueryResultFieldRef<
    AccountDetailQuery,
    PrincipalIdentitySlot,
    BankSchema,
    Principal,
    PrincipalIdentity,
    PrincipalIdentityField,
    BankPrincipalId,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
>;

type BusinessIdentitySelector = ApplicationQueryResultFieldRef<
    AccountDetailQuery,
    BusinessIdentitySlot,
    BankSchema,
    Business,
    BusinessIdentity,
    BusinessIdentityField,
    BusinessId,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
>;

pub fn account_detail_definition() -> ApplicationQueryDefinition<
    BankSchema,
    AccountDetailQuery,
    AccountDetailQueryParameters,
    AccountDetail,
    Account,
> {
    let institution =
        ApplicationQueryResultShapeBuilder::<BankSchema, AccountDetailQuery, Institution, ()>::new(
            Institution::reference(),
        )
        .field(institution_identity());
    let personal =
        ApplicationQueryResultShapeBuilder::<BankSchema, AccountDetailQuery, Principal, ()>::new(
            Principal::reference(),
        )
        .field(principal_identity());
    let business =
        ApplicationQueryResultShapeBuilder::<BankSchema, AccountDetailQuery, Business, ()>::new(
            Business::reference(),
        )
        .field(business_identity());
    let shape = account_summary_shape()
        .relation(account_institution(), institution)
        .relation(account_personal_owner(), personal)
        .relation(account_business_owner(), business)
        .build();
    ApplicationQueryDefinitionBuilder::requires_ability(
        AccountDetailQuery::reference(),
        Account::reference(),
        Account::reference(),
        shape,
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(1, 4, 9),
        ApplicationQueryDisclosureContract::public(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
        ViewAccount::reference(),
    )
    .build()
    .expect("bank account detail query is statically canonical")
}

impl WorthQueryApplicationProjection<BankSchema, AccountDetailQuery> for AccountDetail {
    fn project(
        row: &WorthQueryApplicationProjectionRow<'_, BankSchema, AccountDetailQuery>,
    ) -> Result<Self, WorthQueryApplicationProjectionDenial> {
        let summary = project_account_summary(row)?;
        let institution = row
            .one(account_institution())?
            .field(institution_identity())?;
        let personal_owner = row
            .optional(account_personal_owner())?
            .map(|owner| owner.field(principal_identity()))
            .transpose()?;
        let business_owner = row
            .optional(account_business_owner())?
            .map(|owner| owner.field(business_identity()))
            .transpose()?;
        match summary.kind() {
            AccountKind::Personal if personal_owner.is_some() && business_owner.is_none() => {}
            AccountKind::Business if personal_owner.is_none() && business_owner.is_some() => {}
            AccountKind::InstitutionCash | AccountKind::InstitutionSettlement
                if personal_owner.is_none() && business_owner.is_none() => {}
            _ => {
                return Err(WorthQueryApplicationProjectionDenial::reject(
                    "account ownership disagrees with account kind",
                ));
            }
        }
        Ok(AccountDetail::from_projection(
            summary,
            institution,
            personal_owner,
            business_owner,
        ))
    }
}

fn institution_identity() -> InstitutionIdentitySelector {
    ApplicationQueryResultFieldRef::new("institution", InstitutionIdentityField::reference())
}

fn principal_identity() -> PrincipalIdentitySelector {
    ApplicationQueryResultFieldRef::new("principal", PrincipalIdentityField::reference())
}

fn business_identity() -> BusinessIdentitySelector {
    ApplicationQueryResultFieldRef::new("business", BusinessIdentityField::reference())
}

fn account_institution() -> ApplicationQueryResultRelationRef<
    AccountDetailQuery,
    InstitutionSlot,
    BankSchema,
    InstitutionAccount,
    Institution,
    Account,
    ReverseResultTraversal,
    worth_query_decl::facade::application_query::ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::reverse_one("institution", InstitutionAccount::reference())
}

fn account_personal_owner() -> ApplicationQueryResultRelationRef<
    AccountDetailQuery,
    PersonalOwnerSlot,
    BankSchema,
    PersonalOwner,
    Principal,
    Account,
    ReverseResultTraversal,
    OptionalOneResult,
> {
    ApplicationQueryResultRelationRef::reverse_optional(
        "personal_owner",
        PersonalOwner::reference(),
    )
}

fn account_business_owner() -> ApplicationQueryResultRelationRef<
    AccountDetailQuery,
    BusinessOwnerSlot,
    BankSchema,
    BusinessAccount,
    Business,
    Account,
    ReverseResultTraversal,
    OptionalOneResult,
> {
    ApplicationQueryResultRelationRef::reverse_optional(
        "business_owner",
        BusinessAccount::reference(),
    )
}
