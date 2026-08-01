use worth_query_declaration::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
    ApplicationQueryOrderingDirection, ApplicationQueryResultFieldRef,
    ApplicationQueryResultShapeBuilder, ApplicationQueryRootPath,
};
use worth_query_declaration::worth_query_application_query;

use super::AccountSummaryParameters;
use crate::domain_computation::primary_graph::tests::fixture::{
    Account, AccountAllActivity, AccountPrimaryActivity, AccountSecondaryActivity, AccountStatus,
    Activity, ActivityFacts, ActivitySequence, IdentityExecutionSchema, ViewAccount,
};

pub struct ActivitySequenceResultSlot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivitySequenceResult {
    sequence: u64,
}

impl ActivitySequenceResult {
    pub(in crate::domain_computation::primary_graph::tests) const fn sequence(&self) -> u64 {
        self.sequence
    }
}

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
    ApplicationQueryDefinitionBuilder::requires_ability(
        CrossRootQuery::reference(),
        Activity::reference(),
        Account::reference(),
        shape,
        ApplicationQueryCardinality::Many,
        ApplicationQueryDependencyCeiling::bounded(1, 3, 1),
        ApplicationQueryDisclosureContract::public(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
        ViewAccount::reference(),
    )
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

fn activity_sequence_result_field() -> ApplicationQueryResultFieldRef<
    CrossRootQuery,
    ActivitySequenceResultSlot,
    IdentityExecutionSchema,
    Activity,
    ActivityFacts,
    ActivitySequence,
    u64,
    worth_query_declaration::facade::application_schema::ReadOnly,
    worth_query_declaration::facade::application_schema::NoEqualityPredicate,
    worth_query_declaration::facade::application_schema::NoApplicationCurrency,
> {
    ApplicationQueryResultFieldRef::new("sequence", ActivitySequence::reference())
}
