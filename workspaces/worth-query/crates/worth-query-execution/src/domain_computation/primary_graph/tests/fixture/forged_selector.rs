use worth_query_declaration::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
    ApplicationQueryResultFieldRef, ApplicationQueryResultRelationRef,
    ApplicationQueryResultShapeBuilder, ExactlyOneResult, ForwardResultTraversal,
};
use worth_query_declaration::worth_query_application_query;

use super::application_queries::{status_parameter, AccountSummaryParameters};
use super::{
    Account, AccountPrimaryActivity, AccountStatus, Activity, ActivityFacts, ActivitySequence,
    IdentityExecutionSchema,
};

pub struct ForgedActivitySlot;
pub struct ForgedSequenceSlot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgedSelectorResult;

worth_query_application_query!(
    pub ForgedSelectorQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result ForgedSelectorResult,
    scope Account,
    name "forged_selector"
);

impl
    crate::domain_computation::primary_graph::WorthQueryApplicationProjection<
        IdentityExecutionSchema,
        ForgedSelectorQuery,
    > for ForgedSelectorResult
{
    fn project(
        row: &crate::domain_computation::primary_graph::WorthQueryApplicationProjectionRow<
            '_,
            IdentityExecutionSchema,
            ForgedSelectorQuery,
        >,
    ) -> Result<Self, crate::domain_computation::primary_graph::WorthQueryApplicationProjectionDenial>
    {
        let activity = row.one(activity())?;
        let _: u64 = activity.field(forged_sequence())?;
        Ok(Self)
    }
}

pub(super) fn forged_selector_definition() -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    ForgedSelectorQuery,
    AccountSummaryParameters,
    ForgedSelectorResult,
    Account,
> {
    let nested = ApplicationQueryResultShapeBuilder::<
        IdentityExecutionSchema,
        ForgedSelectorQuery,
        Activity,
        (),
    >::new(Activity::reference())
    .field(declared_sequence());
    let shape = ApplicationQueryResultShapeBuilder::new(Account::reference())
        .relation(activity(), nested)
        .build();
    ApplicationQueryDefinitionBuilder::public(
        ForgedSelectorQuery::reference(),
        Account::reference(),
        Account::reference(),
        shape,
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(1, 1, 1),
        ApplicationQueryDisclosureContract::public(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
    )
    .parameter(status_parameter())
    .where_equal(AccountStatus::reference(), status_parameter())
    .build()
    .unwrap()
}

fn declared_sequence() -> SequenceSelector {
    ApplicationQueryResultFieldRef::new("sequence", ActivitySequence::reference())
}

fn forged_sequence() -> SequenceSelector {
    ApplicationQueryResultFieldRef::new("invented_output", ActivitySequence::reference())
}

fn activity() -> ApplicationQueryResultRelationRef<
    ForgedSelectorQuery,
    ForgedActivitySlot,
    IdentityExecutionSchema,
    AccountPrimaryActivity,
    Account,
    Activity,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one(
        "primary_activity",
        AccountPrimaryActivity::reference(),
    )
}

type SequenceSelector = ApplicationQueryResultFieldRef<
    ForgedSelectorQuery,
    ForgedSequenceSlot,
    IdentityExecutionSchema,
    Activity,
    ActivityFacts,
    ActivitySequence,
    u64,
    worth_query_declaration::facade::application_schema::ReadOnly,
    worth_query_declaration::facade::application_schema::NoEqualityPredicate,
    worth_query_declaration::facade::application_schema::NoApplicationCurrency,
>;
