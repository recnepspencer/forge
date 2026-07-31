use worth_query_declaration::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryInfluenceContract,
    ApplicationQueryLaneEligibility, ApplicationQueryResultShapeBuilder,
};
use worth_query_declaration::worth_query_application_query;

use super::application_queries::{
    label_result_field, status_parameter, status_result_field, AccountSummaryParameters,
    AccountSummaryResult,
};
use super::{
    Account, AccountStatus, CapabilityDisclosure, IdentityExecutionSchema,
    TouchAccountCapability,
};

worth_query_application_query!(
    pub IncompleteDisclosureQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result AccountSummaryResult,
    scope Account,
    name "incomplete_disclosure"
);

worth_query_application_query!(
    pub ForbiddenInfluenceQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result AccountSummaryResult,
    scope Account,
    name "forbidden_influence"
);

pub(super) fn incomplete_disclosure_definition() -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    IncompleteDisclosureQuery,
    AccountSummaryParameters,
    AccountSummaryResult,
    Account,
> {
    let shape = shape::<IncompleteDisclosureQuery>();
    let disclosure = ApplicationQueryDisclosureContract::governed_by(
        "incomplete",
        TouchAccountCapability::reference(),
    )
    .disclose_field_by(
        status_result_field::<IncompleteDisclosureQuery>(),
        CapabilityDisclosure::AccountActivity,
        ApplicationQueryInfluenceContract::forbid_all(),
    );
    definition(
        IncompleteDisclosureQuery::reference(),
        shape,
        disclosure,
        false,
    )
}

pub(super) fn forbidden_influence_definition() -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    ForbiddenInfluenceQuery,
    AccountSummaryParameters,
    AccountSummaryResult,
    Account,
> {
    let shape = shape::<ForbiddenInfluenceQuery>();
    let disclosure = ApplicationQueryDisclosureContract::governed_by(
        "forbidden-influence",
        TouchAccountCapability::reference(),
    )
    .use_field_by(
        AccountStatus::reference(),
        CapabilityDisclosure::AccountActivity,
        ApplicationQueryInfluenceContract::forbid_all(),
    )
    .disclose_field_by(
        status_result_field::<ForbiddenInfluenceQuery>(),
        CapabilityDisclosure::AccountActivity,
        ApplicationQueryInfluenceContract::forbid_all(),
    )
    .disclose_field_by(
        label_result_field::<ForbiddenInfluenceQuery>(),
        CapabilityDisclosure::AccountActivity,
        ApplicationQueryInfluenceContract::forbid_all(),
    );
    definition(
        ForbiddenInfluenceQuery::reference(),
        shape,
        disclosure,
        true,
    )
}

fn shape<Query: 'static>() -> worth_query_declaration::facade::application_query::ApplicationQueryResultShape {
    ApplicationQueryResultShapeBuilder::<
        IdentityExecutionSchema,
        Query,
        Account,
        AccountSummaryResult,
    >::new(Account::reference())
    .field(status_result_field::<Query>())
    .field(label_result_field::<Query>())
    .build()
}

fn definition<Query: 'static>(
    reference: worth_query_declaration::facade::application_query::ApplicationQueryReference<
        IdentityExecutionSchema,
        Query,
        AccountSummaryParameters,
        AccountSummaryResult,
        Account,
    >,
    shape: worth_query_declaration::facade::application_query::ApplicationQueryResultShape,
    disclosure: ApplicationQueryDisclosureContract,
    predicate: bool,
) -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    Query,
    AccountSummaryParameters,
    AccountSummaryResult,
    Account,
> {
    let builder = ApplicationQueryDefinitionBuilder::public(
        reference,
        Account::reference(),
        Account::reference(),
        shape,
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(0, 0, 2),
        disclosure,
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
    );
    if predicate {
        builder
            .parameter(status_parameter())
            .where_equal(AccountStatus::reference(), status_parameter())
            .build()
            .unwrap()
    } else {
        builder.build().unwrap()
    }
}
