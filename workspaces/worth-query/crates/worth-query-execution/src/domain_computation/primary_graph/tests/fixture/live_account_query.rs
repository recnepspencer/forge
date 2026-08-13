use worth_query_declaration::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
    ApplicationQueryLiveCauseBinding, ApplicationQueryLiveResourceContract,
    ApplicationQueryOrderingDirection, ApplicationQueryParameterRef, ApplicationQueryParameterSet,
    ApplicationQueryResultFieldRef, ApplicationQueryResultRelationRef,
    ApplicationQueryResultShapeBuilder, ForwardResultTraversal, ManyResults,
};
use worth_query_declaration::{worth_query_application_query, worth_query_effect};

use super::application_queries::AccountSummaryParameters;
use super::{
    Account, AccountAllActivity, AccountIdentity, AccountPolicy, Activity, ActivityFacts,
    ActivityIdentity, ActivitySequence, IdentityExecutionSchema, ViewAccount,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveActivityEvent {
    account: String,
    activity: String,
}

impl LiveActivityEvent {
    pub(in crate::domain_computation::primary_graph) fn new(
        account: impl Into<String>,
        activity: impl Into<String>,
    ) -> Self {
        Self {
            account: account.into(),
            activity: activity.into(),
        }
    }

    pub(super) fn account(&self) -> &str {
        &self.account
    }

    pub(super) fn activity(&self) -> &str {
        &self.activity
    }
}

impl worth_query_declaration::facade::application_schema::ApplicationEffectPayload
    for LiveActivityEvent
{
    fn retained_bytes(&self) -> u64 {
        u64::try_from(std::mem::size_of::<Self>())
            .unwrap_or(u64::MAX)
            .saturating_add(u64::try_from(self.account.capacity()).unwrap_or(u64::MAX))
            .saturating_add(u64::try_from(self.activity.capacity()).unwrap_or(u64::MAX))
    }
}

worth_query_effect!(
    pub LiveActivityEffect(LiveActivityEvent) in IdentityExecutionSchema
);

pub struct AccountIdentitySlot;
pub struct AccountIdentityParameter;
pub struct ActivitiesSlot;
pub struct ActivityIdentitySlot;
pub struct ActivitySequenceSlot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveAccountActivityResult {
    account: String,
    activities: Vec<(String, u64)>,
}

impl LiveAccountActivityResult {
    pub(in crate::domain_computation::primary_graph) fn account(&self) -> &str {
        &self.account
    }

    pub(in crate::domain_computation::primary_graph) fn activities(&self) -> &[(String, u64)] {
        &self.activities
    }
}

worth_query_application_query!(
    pub LiveAccountActivityQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result LiveAccountActivityResult,
    scope Account,
    name "live_account_activity"
);

pub(in crate::domain_computation::primary_graph) fn live_account_parameter(
) -> ApplicationQueryParameterRef<LiveAccountActivityQuery, AccountIdentityParameter, String> {
    ApplicationQueryParameterRef::from_query_identifier("account")
}

pub(in crate::domain_computation::primary_graph) fn live_account_parameters(
    account: impl Into<String>,
) -> ApplicationQueryParameterSet<LiveAccountActivityQuery> {
    ApplicationQueryParameterSet::new().bind(live_account_parameter(), account.into())
}

pub(super) fn live_account_activity_definition() -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    LiveAccountActivityQuery,
    AccountSummaryParameters,
    LiveAccountActivityResult,
    Account,
> {
    let activity = ApplicationQueryResultShapeBuilder::<
        IdentityExecutionSchema,
        LiveAccountActivityQuery,
        Activity,
        (),
    >::new(Activity::reference())
    .field(activity_identity())
    .field(activity_sequence());
    let shape = ApplicationQueryResultShapeBuilder::<
        IdentityExecutionSchema,
        LiveAccountActivityQuery,
        Account,
        LiveAccountActivityResult,
    >::new(Account::reference())
    .field(account_identity())
    .relation(activities(), activity)
    .build();
    ApplicationQueryDefinitionBuilder::declare(LiveAccountActivityQuery::reference())
        .root(Account::reference())
        .scope(Account::reference())
        .result_shape(shape)
        .cardinality(ApplicationQueryCardinality::ExactlyOne)
        .dependency_ceiling(ApplicationQueryDependencyCeiling::bounded(1, 1, 3))
        .disclosure(ApplicationQueryDisclosureContract::public())
        .basis_support(ApplicationQueryBasisSupport::current_and_pinned())
        .lanes(ApplicationQueryLaneEligibility::one_shot().with_live())
        .requires_ability(ViewAccount::reference())
        .parameter(live_account_parameter())
        .where_equal(AccountIdentity::reference(), live_account_parameter())
        .order_by(
            activity_sequence(),
            ApplicationQueryOrderingDirection::Ascending,
        )
        .continue_by(activities())
        .live_by::<Activity, LiveAccountActivityCause, _, _, _, _, _, _, _, _>(
            account_identity(),
            activity_identity(),
            ApplicationQueryLiveResourceContract::bounded(4, 2_048, 4_096),
        )
        .build()
        .unwrap()
}

impl
    crate::domain_computation::primary_graph::WorthQueryApplicationProjection<
        IdentityExecutionSchema,
        LiveAccountActivityQuery,
    > for LiveAccountActivityResult
{
    fn project(
        row: &crate::domain_computation::primary_graph::WorthQueryApplicationProjectionRow<
            '_,
            IdentityExecutionSchema,
            LiveAccountActivityQuery,
        >,
    ) -> Result<Self, crate::domain_computation::primary_graph::WorthQueryApplicationProjectionDenial>
    {
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

pub struct LiveAccountActivityCause;

impl
    ApplicationQueryLiveCauseBinding<
        IdentityExecutionSchema,
        LiveAccountActivityQuery,
        Account,
        Activity,
    > for LiveAccountActivityCause
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
        payload.account.clone()
    }

    fn target_identity(payload: &Self::Payload) -> Self::TargetIdentity {
        payload.activity.clone()
    }
}

fn account_identity() -> ApplicationQueryResultFieldRef<
    LiveAccountActivityQuery,
    AccountIdentitySlot,
    IdentityExecutionSchema,
    Account,
    AccountPolicy,
    AccountIdentity,
    String,
    worth_query_declaration::facade::application_schema::ReadOnly,
    worth_query_declaration::facade::application_schema::EqualityPredicate,
    worth_query_declaration::facade::application_schema::NoApplicationUnit,
> {
    ApplicationQueryResultFieldRef::new("account", AccountIdentity::reference())
}

fn activity_identity() -> ApplicationQueryResultFieldRef<
    LiveAccountActivityQuery,
    ActivityIdentitySlot,
    IdentityExecutionSchema,
    Activity,
    ActivityFacts,
    ActivityIdentity,
    String,
    worth_query_declaration::facade::application_schema::ReadOnly,
    worth_query_declaration::facade::application_schema::EqualityPredicate,
    worth_query_declaration::facade::application_schema::NoApplicationUnit,
> {
    ApplicationQueryResultFieldRef::new("activity", ActivityIdentity::reference())
}

fn activity_sequence() -> ApplicationQueryResultFieldRef<
    LiveAccountActivityQuery,
    ActivitySequenceSlot,
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

fn activities() -> ApplicationQueryResultRelationRef<
    LiveAccountActivityQuery,
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
