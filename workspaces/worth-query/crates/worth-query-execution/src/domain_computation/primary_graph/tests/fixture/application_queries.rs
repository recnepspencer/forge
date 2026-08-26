use worth_query_declaration::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryInfluenceContract,
    ApplicationQueryLaneEligibility, ApplicationQueryOrderingDirection, ApplicationQueryReference,
    ApplicationQueryResultFieldRef, ApplicationQueryResultShapeBuilder, ApplicationQueryRootPath,
};
use worth_query_declaration::worth_query_application_query;

use super::{
    Account, AccountAllActivity, AccountLabel, AccountPrimaryActivity, AccountSecondaryActivity,
    AccountStatus, Activity, ActivityFacts, ActivitySequence, IdentityExecutionSchema,
    TouchAccountCapability, ViewAccount,
};

#[path = "application_queries/parameter_reference.rs"]
mod parameter_reference;
use parameter_reference::activity_sequence_result_field;
pub(crate) use parameter_reference::status_parameter;

pub struct AccountSummaryParameters;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountSummaryResult {
    pub(super) status: String,
    pub(super) label: String,
}
pub struct StatusParameter;
pub struct StatusResultSlot;
pub struct LabelResultSlot;
pub struct ActivitySequenceResultSlot;

worth_query_declaration::worth_query_portable_type!(
    AccountSummaryResult => "worth.query.test.execution.account_summary.result.v1"
);
worth_query_declaration::worth_query_portable_type!(
    StatusResultSlot => "worth.query.test.execution.account_summary.status_slot.v1"
);
worth_query_declaration::worth_query_portable_type!(
    LabelResultSlot => "worth.query.test.execution.account_summary.label_slot.v1"
);
worth_query_declaration::worth_query_portable_type!(
    ActivitySequenceResultSlot => "worth.query.test.execution.activity_sequence.slot.v1"
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivitySequenceResult {
    pub(super) sequence: u64,
}
worth_query_declaration::worth_query_portable_type!(
    ActivitySequenceResult => "worth.query.test.execution.activity_sequence.result.v1"
);

impl ActivitySequenceResult {
    pub(in crate::domain_computation::primary_graph::tests) const fn sequence(&self) -> u64 {
        self.sequence
    }
}

impl AccountSummaryResult {
    pub(in crate::domain_computation::primary_graph::tests) fn status(&self) -> &str {
        &self.status
    }

    pub(in crate::domain_computation::primary_graph::tests) fn label(&self) -> &str {
        &self.label
    }
}

impl<Query: worth_query_declaration::facade::application_query::ApplicationQueryMarkerIdentity>
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
    pub CrossRootQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result ActivitySequenceResult,
    scope Account,
    name "cross_root"
);

impl
    crate::domain_computation::primary_graph::WorthQueryApplicationProjection<
        IdentityExecutionSchema,
        CrossRootQuery,
    > for ActivitySequenceResult
{
    fn project(
        row: &crate::domain_computation::primary_graph::WorthQueryApplicationProjectionRow<
            '_,
            IdentityExecutionSchema,
            CrossRootQuery,
        >,
    ) -> Result<Self, crate::domain_computation::primary_graph::WorthQueryApplicationProjectionDenial>
    {
        Ok(Self {
            sequence: row.field(activity_sequence_result_field())?,
        })
    }
}
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
    worth_query_declaration::facade::application_schema::NoApplicationUnit,
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
    worth_query_declaration::facade::application_schema::NoApplicationUnit,
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
    ApplicationQueryDefinitionBuilder::declare(ScopedAccountSummaryQuery::reference())
        .root(Account::reference())
        .scope(Account::reference())
        .result_shape(shape)
        .cardinality(ApplicationQueryCardinality::ExactlyOne)
        .dependency_ceiling(ApplicationQueryDependencyCeiling::bounded(0, 0, 2))
        .disclosure(ApplicationQueryDisclosureContract::public())
        .basis_support(ApplicationQueryBasisSupport::current_and_pinned())
        .lanes(ApplicationQueryLaneEligibility::one_shot())
        .requires_ability(ViewAccount::reference())
        .build()
        .unwrap()
}

pub(in crate::domain_computation::primary_graph::tests) fn cross_root_definition(
    status: &str,
) -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    CrossRootQuery,
    AccountSummaryParameters,
    ActivitySequenceResult,
    Account,
> {
    let shape = ApplicationQueryResultShapeBuilder::<
        IdentityExecutionSchema,
        CrossRootQuery,
        Activity,
        ActivitySequenceResult,
    >::new(Activity::reference())
    .field(activity_sequence_result_field())
    .build();
    ApplicationQueryDefinitionBuilder::declare(CrossRootQuery::reference())
        .root(Activity::reference())
        .scope(Account::reference())
        .result_shape(shape)
        .cardinality(ApplicationQueryCardinality::Many)
        .dependency_ceiling(ApplicationQueryDependencyCeiling::bounded(1, 3, 1))
        .disclosure(ApplicationQueryDisclosureContract::public())
        .basis_support(ApplicationQueryBasisSupport::current_and_pinned())
        .lanes(ApplicationQueryLaneEligibility::one_shot())
        .requires_ability(ViewAccount::reference())
        .root_path(
            ApplicationQueryRootPath::from(Account::reference())
                .where_equal(AccountStatus::reference(), status.to_string())
                .forward(AccountPrimaryActivity::reference()),
        )
        .root_path(
            ApplicationQueryRootPath::from(Account::reference())
                .where_equal(AccountStatus::reference(), status.to_string())
                .forward(AccountSecondaryActivity::reference()),
        )
        .root_path(
            ApplicationQueryRootPath::from(Account::reference())
                .where_equal(AccountStatus::reference(), status.to_string())
                .forward(AccountAllActivity::reference()),
        )
        .order_by(
            activity_sequence_result_field(),
            ApplicationQueryOrderingDirection::Ascending,
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

fn definition<
    Query: worth_query_declaration::facade::application_query::ApplicationQueryMarkerIdentity + 'static,
>(
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
        ApplicationQueryDefinitionBuilder::declare(reference)
            .root(Account::reference())
            .scope(Account::reference())
            .result_shape(shape)
            .cardinality(ApplicationQueryCardinality::Many)
            .dependency_ceiling(ApplicationQueryDependencyCeiling::bounded(0, 0, 2))
            .disclosure(disclosure)
            .basis_support(basis_support)
            .lanes(lanes)
            .requires_ability(ViewAccount::reference())
    } else {
        ApplicationQueryDefinitionBuilder::declare(reference)
            .root(Account::reference())
            .scope(Account::reference())
            .result_shape(shape)
            .cardinality(ApplicationQueryCardinality::Many)
            .dependency_ceiling(ApplicationQueryDependencyCeiling::bounded(0, 0, 2))
            .disclosure(disclosure)
            .basis_support(basis_support)
            .lanes(lanes)
            .public()
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
