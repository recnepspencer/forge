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
    Account, AccountAllActivity, AccountIdentity, AccountLabel, AccountPolicy, Activity,
    ActivityFacts, ActivityIdentity, ActivitySequence, CapabilityDisclosure,
    IdentityExecutionSchema, TouchAccountCapability,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationDisclosed, WorthQueryApplicationProjection,
    WorthQueryApplicationProjectionDenial, WorthQueryApplicationProjectionRow,
};

pub struct AccountIdentitySlot;
pub struct AccountLabelSlot;
pub struct AccountIdentityParameter;
pub struct ActivitiesSlot;
pub struct ActivityIdentitySlot;
pub struct ActivitySequenceSlot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernedLiveAccountActivityResult {
    account: String,
    label: WorthQueryApplicationDisclosed<String>,
    activities: Vec<(String, u64)>,
}

impl GovernedLiveAccountActivityResult {
    pub(in crate::domain_computation::primary_graph) fn account(&self) -> &str {
        &self.account
    }

    pub(in crate::domain_computation::primary_graph) const fn label(
        &self,
    ) -> &WorthQueryApplicationDisclosed<String> {
        &self.label
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
    .field(account_label())
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
    .use_field_by(
        ActivityIdentity::reference(),
        CapabilityDisclosure::AccountActivity,
        influence.clone(),
    )
    .use_field_by(
        ActivitySequence::reference(),
        CapabilityDisclosure::AccountActivity,
        influence.clone(),
    )
    .disclose_field_by(
        account_identity(),
        CapabilityDisclosure::AccountActivity,
        influence.clone(),
    )
    .disclose_field_by(
        account_label(),
        CapabilityDisclosure::PrivateLabel,
        ApplicationQueryInfluenceContract::forbid_all(),
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
    ApplicationQueryDefinitionBuilder::declare(GovernedLiveAccountActivityQuery::reference())
        .root(Account::reference())
        .scope(Account::reference())
        .result_shape(shape)
        .cardinality(ApplicationQueryCardinality::ExactlyOne)
        .dependency_ceiling(ApplicationQueryDependencyCeiling::bounded(1, 1, 4))
        .disclosure(disclosure)
        .basis_support(ApplicationQueryBasisSupport::current_and_pinned().with_preview())
        .lanes(
            ApplicationQueryLaneEligibility::one_shot()
                .with_historical()
                .with_preview()
                .with_live(),
        )
        .public()
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
            label: row.disclosed_field(account_label())?,
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
    worth_query_declaration::facade::application_schema::NoApplicationUnit,
> {
    ApplicationQueryResultFieldRef::new("account", AccountIdentity::reference())
}

fn account_label() -> ApplicationQueryResultFieldRef<
    GovernedLiveAccountActivityQuery,
    AccountLabelSlot,
    IdentityExecutionSchema,
    Account,
    AccountPolicy,
    AccountLabel,
    String,
    worth_query_declaration::facade::application_schema::ReadWrite,
    worth_query_declaration::facade::application_schema::EqualityPredicate,
    worth_query_declaration::facade::application_schema::NoApplicationUnit,
> {
    ApplicationQueryResultFieldRef::new("label", AccountLabel::reference())
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
    worth_query_declaration::facade::application_schema::NoApplicationUnit,
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
    worth_query_declaration::facade::application_schema::NoApplicationUnit,
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
