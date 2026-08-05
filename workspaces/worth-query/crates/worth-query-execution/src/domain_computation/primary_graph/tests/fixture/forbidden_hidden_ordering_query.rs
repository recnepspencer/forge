use worth_query_declaration::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryInfluenceContract,
    ApplicationQueryLaneEligibility, ApplicationQueryOrderingDirection,
    ApplicationQueryResultShapeBuilder,
};
use worth_query_declaration::worth_query_application_query;

use super::application_queries::{
    label_result_field, status_result_field, AccountSummaryParameters, AccountSummaryResult,
};
use super::{
    Account, AccountLabel, CapabilityDisclosure, IdentityExecutionSchema, TouchAccountCapability,
};

worth_query_application_query!(
    pub ForbiddenHiddenOrderingQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result AccountSummaryResult,
    scope Account,
    name "forbidden_hidden_ordering"
);

pub(super) fn forbidden_hidden_ordering_definition() -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    ForbiddenHiddenOrderingQuery,
    AccountSummaryParameters,
    AccountSummaryResult,
    Account,
> {
    let shape = ApplicationQueryResultShapeBuilder::<
        IdentityExecutionSchema,
        ForbiddenHiddenOrderingQuery,
        Account,
        AccountSummaryResult,
    >::new(Account::reference())
    .field(status_result_field::<ForbiddenHiddenOrderingQuery>())
    .field(label_result_field::<ForbiddenHiddenOrderingQuery>())
    .build();
    let disclosure = ApplicationQueryDisclosureContract::governed_by(
        "forbidden-hidden-ordering",
        TouchAccountCapability::reference(),
    )
    .use_field_by(
        AccountLabel::reference(),
        CapabilityDisclosure::AccountActivity,
        ApplicationQueryInfluenceContract::forbid_all(),
    )
    .disclose_field_by(
        status_result_field::<ForbiddenHiddenOrderingQuery>(),
        CapabilityDisclosure::AccountActivity,
        ApplicationQueryInfluenceContract::permit_all(),
    )
    .disclose_field_by(
        label_result_field::<ForbiddenHiddenOrderingQuery>(),
        CapabilityDisclosure::AccountActivity,
        ApplicationQueryInfluenceContract::permit_all(),
    );
    ApplicationQueryDefinitionBuilder::public(
        ForbiddenHiddenOrderingQuery::reference(),
        Account::reference(),
        Account::reference(),
        shape,
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(0, 0, 2),
        disclosure,
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
    )
    .order_by(
        label_result_field::<ForbiddenHiddenOrderingQuery>(),
        ApplicationQueryOrderingDirection::Ascending,
    )
    .build()
    .unwrap()
}
