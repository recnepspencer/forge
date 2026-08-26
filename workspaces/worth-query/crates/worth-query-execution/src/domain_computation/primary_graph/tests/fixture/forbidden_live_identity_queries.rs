use worth_query_declaration::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryInfluenceContract,
    ApplicationQueryLaneEligibility, ApplicationQueryLiveCauseBinding,
    ApplicationQueryLiveResourceContract, ApplicationQueryObservableInfluence,
    ApplicationQueryOrderingDirection, ApplicationQueryParameterRef, ApplicationQueryParameterSet,
    ApplicationQueryResultFieldRef, ApplicationQueryResultRelationRef,
    ApplicationQueryResultShapeBuilder, ForwardResultTraversal, ManyResults,
};
use worth_query_declaration::worth_query_application_query;

use super::application_queries::AccountSummaryParameters;
use super::live_account_query::{LiveActivityEffect, LiveActivityEvent};
use super::{
    Account, AccountAllActivity, AccountIdentity, AccountPolicy, Activity, ActivityFacts,
    ActivityIdentity, CapabilityDisclosure, IdentityExecutionSchema, TouchAccountCapability,
};

pub struct AccountIdentityParameter;
pub struct AccountIdentitySlot;
pub struct ActivitiesSlot;
pub struct ActivityIdentitySlot;

worth_query_declaration::worth_query_portable_type!(AccountIdentitySlot => "worth.query.test.execution.forbidden_live.account_identity_slot.v1");
worth_query_declaration::worth_query_portable_type!(ActivitiesSlot => "worth.query.test.execution.forbidden_live.activities_slot.v1");
worth_query_declaration::worth_query_portable_type!(ActivityIdentitySlot => "worth.query.test.execution.forbidden_live.activity_identity_slot.v1");

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForbiddenLiveIdentityResult;
worth_query_declaration::worth_query_portable_type!(ForbiddenLiveIdentityResult => "worth.query.test.execution.forbidden_live.result.v1");

worth_query_application_query!(
    pub ForbiddenLiveScopeIdentityQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result ForbiddenLiveIdentityResult,
    scope Account,
    name "forbidden_live_scope_identity"
);

worth_query_application_query!(
    pub ForbiddenLiveTargetIdentityQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result ForbiddenLiveIdentityResult,
    scope Account,
    name "forbidden_live_target_identity"
);

pub(super) fn forbidden_live_scope_identity_definition() -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    ForbiddenLiveScopeIdentityQuery,
    AccountSummaryParameters,
    ForbiddenLiveIdentityResult,
    Account,
> {
    definition::<ForbiddenLiveScopeIdentityQuery, ForbiddenLiveScopeIdentityCause>(
        ForbiddenLiveScopeIdentityQuery::reference(),
        influence_without_live_membership(),
        ApplicationQueryInfluenceContract::permit_all(),
    )
}

pub(super) fn forbidden_live_target_identity_definition() -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    ForbiddenLiveTargetIdentityQuery,
    AccountSummaryParameters,
    ForbiddenLiveIdentityResult,
    Account,
> {
    definition::<ForbiddenLiveTargetIdentityQuery, ForbiddenLiveTargetIdentityCause>(
        ForbiddenLiveTargetIdentityQuery::reference(),
        ApplicationQueryInfluenceContract::permit_all(),
        influence_without_live_membership(),
    )
}

pub(in crate::domain_computation::primary_graph) fn forbidden_live_identity_parameters<
    Query: worth_query_declaration::facade::application_query::ApplicationQueryMarkerIdentity,
>() -> ApplicationQueryParameterSet<Query> {
    ApplicationQueryParameterSet::new().bind(account_parameter::<Query>(), "account-1".to_owned())
}

fn definition<
    Query: worth_query_declaration::facade::application_query::ApplicationQueryMarkerIdentity + 'static,
    Cause,
>(
    reference: worth_query_declaration::facade::application_query::ApplicationQueryReference<
        IdentityExecutionSchema,
        Query,
        AccountSummaryParameters,
        ForbiddenLiveIdentityResult,
        Account,
    >,
    scope_influence: ApplicationQueryInfluenceContract,
    target_influence: ApplicationQueryInfluenceContract,
) -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    Query,
    AccountSummaryParameters,
    ForbiddenLiveIdentityResult,
    Account,
>
where
    Cause: ApplicationQueryLiveCauseBinding<
        IdentityExecutionSchema,
        Query,
        Account,
        Activity,
        ScopeIdentity = String,
        TargetIdentity = String,
    >,
{
    let activity =
        ApplicationQueryResultShapeBuilder::<IdentityExecutionSchema, Query, Activity, ()>::new(
            Activity::reference(),
        )
        .field(activity_identity::<Query>());
    let shape = ApplicationQueryResultShapeBuilder::<
        IdentityExecutionSchema,
        Query,
        Account,
        ForbiddenLiveIdentityResult,
    >::new(Account::reference())
    .field(account_identity::<Query>())
    .relation(activities::<Query>(), activity)
    .build();
    let disclosure = ApplicationQueryDisclosureContract::governed_by(
        "forbidden-live-identity",
        TouchAccountCapability::reference(),
    )
    .use_field_by(
        AccountIdentity::reference(),
        CapabilityDisclosure::AccountActivity,
        scope_influence,
    )
    .use_field_by(
        ActivityIdentity::reference(),
        CapabilityDisclosure::AccountActivity,
        target_influence,
    )
    .disclose_field_by(
        account_identity::<Query>(),
        CapabilityDisclosure::AccountActivity,
        ApplicationQueryInfluenceContract::permit_all(),
    )
    .disclose_relation_by(
        activities::<Query>(),
        CapabilityDisclosure::AccountActivity,
        ApplicationQueryInfluenceContract::permit_all(),
    )
    .disclose_field_by(
        activity_identity::<Query>(),
        CapabilityDisclosure::AccountActivity,
        ApplicationQueryInfluenceContract::permit_all(),
    );
    ApplicationQueryDefinitionBuilder::declare(reference)
        .root(Account::reference())
        .scope(Account::reference())
        .result_shape(shape)
        .cardinality(ApplicationQueryCardinality::ExactlyOne)
        .dependency_ceiling(ApplicationQueryDependencyCeiling::bounded(1, 1, 2))
        .disclosure(disclosure)
        .basis_support(ApplicationQueryBasisSupport::current_and_pinned())
        .lanes(ApplicationQueryLaneEligibility::one_shot().with_live())
        .public()
        .parameter(account_parameter::<Query>())
        .where_equal(AccountIdentity::reference(), account_parameter::<Query>())
        .order_by(
            activity_identity::<Query>(),
            ApplicationQueryOrderingDirection::Ascending,
        )
        .continue_by(activities::<Query>())
        .live_by::<Activity, Cause, _, _, _, _, _, _, _, _>(
            account_identity::<Query>(),
            activity_identity::<Query>(),
            ApplicationQueryLiveResourceContract::bounded(4, 2_048, 4_096),
        )
        .build()
        .unwrap()
}

fn influence_without_live_membership() -> ApplicationQueryInfluenceContract {
    ApplicationQueryInfluenceContract::permit([
        ApplicationQueryObservableInfluence::RowPresence,
        ApplicationQueryObservableInfluence::Ordering,
        ApplicationQueryObservableInfluence::Pagination,
        ApplicationQueryObservableInfluence::Count,
        ApplicationQueryObservableInfluence::HistoricalMembership,
        ApplicationQueryObservableInfluence::Preview,
    ])
}

pub struct ForbiddenLiveScopeIdentityCause;
pub struct ForbiddenLiveTargetIdentityCause;
worth_query_declaration::worth_query_portable_type!(ForbiddenLiveScopeIdentityCause => "worth.query.test.execution.forbidden_live.scope_cause.v1");
worth_query_declaration::worth_query_portable_type!(ForbiddenLiveTargetIdentityCause => "worth.query.test.execution.forbidden_live.target_cause.v1");

macro_rules! live_cause {
    ($cause:ty, $query:ty) => {
        impl ApplicationQueryLiveCauseBinding<IdentityExecutionSchema, $query, Account, Activity>
            for $cause
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
    };
}

live_cause!(
    ForbiddenLiveScopeIdentityCause,
    ForbiddenLiveScopeIdentityQuery
);
live_cause!(
    ForbiddenLiveTargetIdentityCause,
    ForbiddenLiveTargetIdentityQuery
);

fn account_parameter<
    Query: worth_query_declaration::facade::application_query::ApplicationQueryMarkerIdentity,
>() -> ApplicationQueryParameterRef<Query, AccountIdentityParameter, String> {
    ApplicationQueryParameterRef::from_query_identifier("account")
}

fn account_identity<
    Query: worth_query_declaration::facade::application_query::ApplicationQueryMarkerIdentity,
>() -> ApplicationQueryResultFieldRef<
    Query,
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

fn activity_identity<
    Query: worth_query_declaration::facade::application_query::ApplicationQueryMarkerIdentity,
>() -> ApplicationQueryResultFieldRef<
    Query,
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

fn activities<
    Query: worth_query_declaration::facade::application_query::ApplicationQueryMarkerIdentity,
>() -> ApplicationQueryResultRelationRef<
    Query,
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
