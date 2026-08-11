use worth_query_declaration::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryInfluenceContract,
    ApplicationQueryLaneEligibility, ApplicationQueryResultFieldRef,
    ApplicationQueryResultShapeBuilder, ApplicationQueryRootPath,
};
use worth_query_declaration::worth_query_application_query;

use super::application_queries::AccountSummaryParameters;
use super::{
    Account, AccountAllActivity, AccountLabel, Activity, ActivityFacts, ActivitySequence,
    CapabilityDisclosure, IdentityExecutionSchema, TouchAccountCapability,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationProjection, WorthQueryApplicationProjectionDenial,
    WorthQueryApplicationProjectionRow,
};

pub struct RootGuardSequenceSlot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RootGuardResult {
    sequence: u64,
}

worth_query_application_query!(
    pub GovernedRootGuardQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result RootGuardResult,
    scope Account,
    name "governed_root_guard"
);

worth_query_application_query!(
    pub ForbiddenRootGuardQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result RootGuardResult,
    scope Account,
    name "forbidden_root_guard"
);

pub(super) fn governed_root_guard_definition() -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    GovernedRootGuardQuery,
    AccountSummaryParameters,
    RootGuardResult,
    Account,
> {
    definition(
        GovernedRootGuardQuery::reference(),
        ApplicationQueryInfluenceContract::permit_all(),
    )
}

pub(super) fn forbidden_root_guard_definition() -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    ForbiddenRootGuardQuery,
    AccountSummaryParameters,
    RootGuardResult,
    Account,
> {
    definition(
        ForbiddenRootGuardQuery::reference(),
        ApplicationQueryInfluenceContract::forbid_all(),
    )
}

fn definition<Query: 'static>(
    reference: worth_query_declaration::facade::application_query::ApplicationQueryReference<
        IdentityExecutionSchema,
        Query,
        AccountSummaryParameters,
        RootGuardResult,
        Account,
    >,
    guard_influence: ApplicationQueryInfluenceContract,
) -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    Query,
    AccountSummaryParameters,
    RootGuardResult,
    Account,
> {
    let shape = ApplicationQueryResultShapeBuilder::<
        IdentityExecutionSchema,
        Query,
        Activity,
        RootGuardResult,
    >::new(Activity::reference())
    .field(sequence::<Query>())
    .build();
    let disclosure = ApplicationQueryDisclosureContract::governed_by(
        "root-guard",
        TouchAccountCapability::reference(),
    )
    .use_field_by(
        AccountLabel::reference(),
        CapabilityDisclosure::AccountActivity,
        guard_influence,
    )
    .disclose_field_by(
        sequence::<Query>(),
        CapabilityDisclosure::AccountActivity,
        ApplicationQueryInfluenceContract::permit_all(),
    );
    ApplicationQueryDefinitionBuilder::declare(reference)
        .root(Activity::reference())
        .scope(Account::reference())
        .result_shape(shape)
        .cardinality(ApplicationQueryCardinality::Many)
        .dependency_ceiling(ApplicationQueryDependencyCeiling::bounded(1, 1, 1))
        .disclosure(disclosure)
        .basis_support(ApplicationQueryBasisSupport::current_and_pinned())
        .lanes(ApplicationQueryLaneEligibility::one_shot())
        .public()
        .root_path(
            ApplicationQueryRootPath::from(Account::reference())
                .where_equal(AccountLabel::reference(), "guard-match".to_owned())
                .forward(AccountAllActivity::reference()),
        )
        .build()
        .unwrap()
}

impl<Query: 'static> WorthQueryApplicationProjection<IdentityExecutionSchema, Query>
    for RootGuardResult
{
    fn project(
        row: &WorthQueryApplicationProjectionRow<'_, IdentityExecutionSchema, Query>,
    ) -> Result<Self, WorthQueryApplicationProjectionDenial> {
        Ok(Self {
            sequence: row.field(sequence::<Query>())?,
        })
    }
}

fn sequence<Query: 'static>() -> ApplicationQueryResultFieldRef<
    Query,
    RootGuardSequenceSlot,
    IdentityExecutionSchema,
    Activity,
    ActivityFacts,
    ActivitySequence,
    u64,
    worth_query_declaration::facade::application_schema::ReadOnly,
    worth_query_declaration::facade::application_schema::NoEqualityPredicate,
    worth_query_declaration::facade::application_schema::NoApplicationUnit,
> {
    ApplicationQueryResultFieldRef::new("sequence", ActivitySequence::reference())
}
