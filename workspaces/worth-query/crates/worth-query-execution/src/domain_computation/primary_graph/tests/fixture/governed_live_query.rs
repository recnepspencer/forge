use worth_query_declaration::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryInfluenceContract,
    ApplicationQueryLaneEligibility, ApplicationQueryLiveCauseBinding,
    ApplicationQueryLiveResourceContract, ApplicationQueryOrderingDirection,
    ApplicationQueryParameterRef, ApplicationQueryParameterSet, ApplicationQueryResultFieldRef,
    ApplicationQueryResultRelationRef, ApplicationQueryResultShapeBuilder, ForwardResultTraversal,
    ManyResults,
};
use worth_query_declaration::worth_query_application_query;

use super::application_queries::AccountSummaryParameters;
use super::live_account_query::{LiveActivityEffect, LiveActivityEvent};
use super::{
    Account, AccountAllActivity, AccountIdentity, AccountPolicy, Activity, ActivityFacts,
    ActivityIdentity, ActivitySequence, CapabilityDisclosure, IdentityExecutionSchema,
    TouchAccountCapability,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationProjection, WorthQueryApplicationProjectionDenial,
    WorthQueryApplicationProjectionRow,
};

pub struct AccountIdentitySlot;
pub struct AccountIdentityParameter;
pub struct ActivitiesSlot;
pub struct ActivityIdentitySlot;
pub struct ActivitySequenceSlot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernedLiveAccountActivityResult {
    account: String,
    activities: Vec<(String, u64)>,
}

impl GovernedLiveAccountActivityResult {
    pub(in crate::domain_computation::primary_graph) fn account(&self) -> &str {
        &self.account
    }

    pub(in crate::domain_computation::primary_graph) fn activities(&self) -> &[(String, u64)] {
        &self.activities
    }
}

worth_query_application_query!(
    pub GovernedLiveAccountActivityQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result GovernedLiveAccountActivityResult,
    scope Account,
    name "governed_live_account_activity"
);

pub(in crate::domain_computation::primary_graph) fn governed_live_account_parameters(
    account: impl Into<String>,
) -> ApplicationQueryParameterSet<GovernedLiveAccountActivityQuery> {
    ApplicationQueryParameterSet::new().bind(account_parameter(), account.into())
}

pub(super) fn governed_live_account_definition() -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    GovernedLiveAccountActivityQuery,
    AccountSummaryParameters,
    GovernedLiveAccountActivityResult,
    Account,
> {
    let activity = ApplicationQueryResultShapeBuilder::<
        IdentityExecutionSchema,
        GovernedLiveAccountActivityQuery,
        Activity,
        (),
    >::new(Activity::reference())
    .field(activity_identity())
    .field(activity_sequence());
    let shape = ApplicationQueryResultShapeBuilder::<
        IdentityExecutionSchema,
        GovernedLiveAccountActivityQuery,
        Account,
        GovernedLiveAccountActivityResult,
    >::new(Account::reference())
    .field(account_identity())
    .relation(activities(), activity)
    .build();
    let influence = ApplicationQueryInfluenceContract::permit_all();
    let disclosure = ApplicationQueryDisclosureContract::governed_by(
        "account-activity",
        TouchAccountCapability::reference(),
    )
    .use_field_by(
        AccountIdentity::reference(),
        CapabilityDisclosure::AccountActivity,
        influence.clone(),
    )
    .disclose_field_by(
        account_identity(),
        CapabilityDisclosure::AccountActivity,
        influence.clone(),
    )
    .disclose_relation_by(
        activities(),
        CapabilityDisclosure::AccountActivity,
        influence.clone(),
    )
    .disclose_field_by(
        activity_identity(),
        CapabilityDisclosure::AccountActivity,
        influence.clone(),
    )
    .disclose_field_by(
        activity_sequence(),
        CapabilityDisclosure::AccountActivity,
        influence,
    );
    ApplicationQueryDefinitionBuilder::public(
        GovernedLiveAccountActivityQuery::reference(),
        Account::reference(),
        Account::reference(),
        shape,
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(1, 1, 3),
        disclosure,
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot().with_live(),
    )
    .parameter(account_parameter())
    .where_equal(AccountIdentity::reference(), account_parameter())
    .order_by(
        activity_sequence(),
        ApplicationQueryOrderingDirection::Ascending,
    )
    .continue_by(activities())
    .live_by::<Activity, GovernedLiveAccountActivityCause, _, _, _, _, _, _, _, _>(
        account_identity(),
        activity_identity(),
        ApplicationQueryLiveResourceContract::bounded(4, 2_048, 4_096),
    )
    .build()
    .unwrap()
}

impl WorthQueryApplicationProjection<IdentityExecutionSchema, GovernedLiveAccountActivityQuery>
    for GovernedLiveAccountActivityResult
{
    fn project(
        row: &WorthQueryApplicationProjectionRow<
            '_,
            IdentityExecutionSchema,
            GovernedLiveAccountActivityQuery,
        >,
    ) -> Result<Self, WorthQueryApplicationProjectionDenial> {
        let account = row.field(account_identity())?;
        let activities = row
            .many(activities())?
            .iter()
            .map(|activity| {
                Ok((
                    activity.field(activity_identity())?,
                    activity.field(activity_sequence())?,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            account,
            activities,
        })
    }
}

pub struct GovernedLiveAccountActivityCause;

impl
    ApplicationQueryLiveCauseBinding<
        IdentityExecutionSchema,
        GovernedLiveAccountActivityQuery,
        Account,
        Activity,
    > for GovernedLiveAccountActivityCause
{
    type Effect = LiveActivityEffect;
    type Payload = LiveActivityEvent;
    type ScopeIdentity = String;
    type TargetIdentity = String;

    fn effect() -> worth_query_declaration::facade::application_schema::ApplicationEffectRef<
        IdentityExecutionSchema,
        Self::Effect,
        Self::Payload,
    > {
        LiveActivityEffect::reference()
    }

    fn scope_identity(payload: &Self::Payload) -> Self::ScopeIdentity {
        payload.account().to_owned()
    }

    fn target_identity(payload: &Self::Payload) -> Self::TargetIdentity {
        payload.activity().to_owned()
    }
}

fn account_parameter(
) -> ApplicationQueryParameterRef<GovernedLiveAccountActivityQuery, AccountIdentityParameter, String>
{
    ApplicationQueryParameterRef::from_query_identifier("account")
}

fn account_identity() -> ApplicationQueryResultFieldRef<
    GovernedLiveAccountActivityQuery,
    AccountIdentitySlot,
    IdentityExecutionSchema,
    Account,
    AccountPolicy,
    AccountIdentity,
    String,
    worth_query_declaration::facade::application_schema::ReadOnly,
    worth_query_declaration::facade::application_schema::EqualityPredicate,
    worth_query_declaration::facade::application_schema::NoApplicationCurrency,
> {
    ApplicationQueryResultFieldRef::new("account", AccountIdentity::reference())
}

fn activity_identity() -> ApplicationQueryResultFieldRef<
    GovernedLiveAccountActivityQuery,
    ActivityIdentitySlot,
    IdentityExecutionSchema,
    Activity,
    ActivityFacts,
    ActivityIdentity,
    String,
    worth_query_declaration::facade::application_schema::ReadOnly,
    worth_query_declaration::facade::application_schema::EqualityPredicate,
    worth_query_declaration::facade::application_schema::NoApplicationCurrency,
> {
    ApplicationQueryResultFieldRef::new("activity", ActivityIdentity::reference())
}

fn activity_sequence() -> ApplicationQueryResultFieldRef<
    GovernedLiveAccountActivityQuery,
    ActivitySequenceSlot,
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

fn activities() -> ApplicationQueryResultRelationRef<
    GovernedLiveAccountActivityQuery,
    ActivitiesSlot,
    IdentityExecutionSchema,
    AccountAllActivity,
    Account,
    Activity,
    ForwardResultTraversal,
    ManyResults,
> {
    ApplicationQueryResultRelationRef::forward_many("activities", AccountAllActivity::reference())
}
