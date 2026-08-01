use worth_query_declaration::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryInfluenceContract,
    ApplicationQueryLaneEligibility, ApplicationQueryOrderingDirection,
    ApplicationQueryParameterRef, ApplicationQueryReference, ApplicationQueryResultFieldRef,
    ApplicationQueryResultShapeBuilder,
};
use worth_query_declaration::worth_query_application_query;

use super::{
    Account, AccountLabel, AccountStatus, IdentityExecutionSchema, TouchAccountCapability,
    ViewAccount,
};

#[path = "application_queries/cross_root.rs"]
mod cross_root;

pub(in crate::domain_computation::primary_graph::tests) use cross_root::{
    cross_root_definition, CrossRootQuery,
};

pub struct AccountSummaryParameters;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountSummaryResult {
    pub(super) status: String,
    pub(super) label: String,
}
pub struct StatusParameter;
pub struct StatusResultSlot;
pub struct LabelResultSlot;

impl AccountSummaryResult {
    pub(in crate::domain_computation::primary_graph::tests) fn status(&self) -> &str {
        &self.status
    }

    pub(in crate::domain_computation::primary_graph::tests) fn label(&self) -> &str {
        &self.label
    }
}

impl<Query: 'static>
    crate::domain_computation::primary_graph::WorthQueryApplicationProjection<
        IdentityExecutionSchema,
        Query,
    > for AccountSummaryResult
{
    fn project(
        row: &crate::domain_computation::primary_graph::WorthQueryApplicationProjectionRow<
            '_,
            IdentityExecutionSchema,
            Query,
        >,
    ) -> Result<Self, crate::domain_computation::primary_graph::WorthQueryApplicationProjectionDenial>
    {
        Ok(Self {
            status: row.field(status_result_field::<Query>())?,
            label: row.field(label_result_field::<Query>())?,
        })
    }
}

worth_query_application_query!(
    pub AccountSummaryQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result AccountSummaryResult,
    scope Account,
    name "account_summary"
);
worth_query_application_query!(
    pub ScopedAccountSummaryQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result AccountSummaryResult,
    scope Account,
    name "scoped_account_summary"
);
worth_query_application_query!(
    pub GovernedAccountSummaryQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result AccountSummaryResult,
    scope Account,
    name "governed_account_summary"
);
worth_query_application_query!(
    pub OrderedAccountSummaryQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result AccountSummaryResult,
    scope Account,
    name "ordered_account_summary"
);
pub(in crate::domain_computation::primary_graph::tests) fn status_parameter<Query>(
) -> ApplicationQueryParameterRef<Query, StatusParameter, String> {
    ApplicationQueryParameterRef::from_query_identifier("status")
}

pub(in crate::domain_computation::primary_graph::tests) fn status_result_field<Query>(
) -> ApplicationQueryResultFieldRef<
    Query,
    StatusResultSlot,
    IdentityExecutionSchema,
    Account,
    super::AccountPolicy,
    AccountStatus,
    String,
    worth_query_declaration::facade::application_schema::ReadWrite,
    worth_query_declaration::facade::application_schema::EqualityPredicate,
    worth_query_declaration::facade::application_schema::NoApplicationCurrency,
> {
    ApplicationQueryResultFieldRef::new("status", AccountStatus::reference())
}

pub(in crate::domain_computation::primary_graph::tests) fn label_result_field<Query>(
) -> ApplicationQueryResultFieldRef<
    Query,
    LabelResultSlot,
    IdentityExecutionSchema,
    Account,
    super::AccountPolicy,
    AccountLabel,
    String,
    worth_query_declaration::facade::application_schema::ReadWrite,
    worth_query_declaration::facade::application_schema::EqualityPredicate,
    worth_query_declaration::facade::application_schema::NoApplicationCurrency,
> {
    ApplicationQueryResultFieldRef::new("label", AccountLabel::reference())
}

pub(super) fn account_summary_definition() -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    AccountSummaryQuery,
    AccountSummaryParameters,
    AccountSummaryResult,
    Account,
> {
    definition(
        AccountSummaryQuery::reference(),
        ApplicationQueryDisclosureContract::public(),
        false,
        true,
        true,
    )
}

pub(super) fn scoped_account_summary_definition() -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    ScopedAccountSummaryQuery,
    AccountSummaryParameters,
    AccountSummaryResult,
    Account,
> {
    let shape = ApplicationQueryResultShapeBuilder::<
        IdentityExecutionSchema,
        ScopedAccountSummaryQuery,
        Account,
        AccountSummaryResult,
    >::new(Account::reference())
    .field(status_result_field::<ScopedAccountSummaryQuery>())
    .field(label_result_field::<ScopedAccountSummaryQuery>())
    .build();
    ApplicationQueryDefinitionBuilder::requires_ability(
        ScopedAccountSummaryQuery::reference(),
        Account::reference(),
        Account::reference(),
        shape,
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(0, 0, 2),
        ApplicationQueryDisclosureContract::public(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
        ViewAccount::reference(),
    )
    .build()
    .unwrap()
}

pub(super) fn governed_account_summary_definition() -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    GovernedAccountSummaryQuery,
    AccountSummaryParameters,
    AccountSummaryResult,
    Account,
> {
    definition(
        GovernedAccountSummaryQuery::reference(),
        ApplicationQueryDisclosureContract::governed_by(
            "account-holder",
            TouchAccountCapability::reference(),
        )
        .use_field_by(
            AccountStatus::reference(),
            super::CapabilityDisclosure::AccountActivity,
            ApplicationQueryInfluenceContract::permit_all(),
        )
        .disclose_field_by(
            status_result_field::<GovernedAccountSummaryQuery>(),
            super::CapabilityDisclosure::AccountActivity,
            ApplicationQueryInfluenceContract::forbid_all(),
        )
        .disclose_field_by(
            label_result_field::<GovernedAccountSummaryQuery>(),
            super::CapabilityDisclosure::AccountActivity,
            ApplicationQueryInfluenceContract::forbid_all(),
        ),
        false,
        false,
        false,
    )
}

pub(super) fn ordered_account_summary_definition() -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    OrderedAccountSummaryQuery,
    AccountSummaryParameters,
    AccountSummaryResult,
    Account,
> {
    definition(
        OrderedAccountSummaryQuery::reference(),
        ApplicationQueryDisclosureContract::public(),
        true,
        false,
        false,
    )
}

fn definition<Query: 'static>(
    reference: ApplicationQueryReference<
        IdentityExecutionSchema,
        Query,
        AccountSummaryParameters,
        AccountSummaryResult,
        Account,
    >,
    disclosure: ApplicationQueryDisclosureContract,
    ordered: bool,
    requires_view: bool,
    advanced_read_lanes: bool,
) -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    Query,
    AccountSummaryParameters,
    AccountSummaryResult,
    Account,
> {
    let shape = ApplicationQueryResultShapeBuilder::<
        IdentityExecutionSchema,
        Query,
        Account,
        AccountSummaryResult,
    >::new(Account::reference())
    .field(status_result_field::<Query>())
    .field(label_result_field::<Query>())
    .build();
    let basis_support = if advanced_read_lanes {
        ApplicationQueryBasisSupport::current_and_pinned().with_preview()
    } else {
        ApplicationQueryBasisSupport::current_and_pinned()
    };
    let lanes = if advanced_read_lanes {
        ApplicationQueryLaneEligibility::one_shot()
            .with_historical()
            .with_preview()
    } else {
        ApplicationQueryLaneEligibility::one_shot()
    };
    let builder = if requires_view {
        ApplicationQueryDefinitionBuilder::requires_ability(
            reference,
            Account::reference(),
            Account::reference(),
            shape,
            ApplicationQueryCardinality::Many,
            ApplicationQueryDependencyCeiling::bounded(0, 0, 2),
            disclosure,
            basis_support,
            lanes,
            ViewAccount::reference(),
        )
    } else {
        ApplicationQueryDefinitionBuilder::public(
            reference,
            Account::reference(),
            Account::reference(),
            shape,
            ApplicationQueryCardinality::Many,
            ApplicationQueryDependencyCeiling::bounded(0, 0, 2),
            disclosure,
            basis_support,
            lanes,
        )
    }
    .parameter(status_parameter())
    .where_equal(AccountStatus::reference(), status_parameter());
    let builder = if ordered {
        builder.order_by(
            label_result_field::<Query>(),
            ApplicationQueryOrderingDirection::Ascending,
        )
    } else {
        builder
    };
    builder.build().unwrap()
}
