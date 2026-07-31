use worth_query_declaration::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryInfluenceContract,
    ApplicationQueryLaneEligibility, ApplicationQueryResultFieldRef,
    ApplicationQueryResultRelationRef, ApplicationQueryResultShapeBuilder,
    ForwardResultTraversal, ManyResults,
};
use worth_query_declaration::worth_query_application_query;

use super::application_queries::AccountSummaryParameters;
use super::{
    Account, AccountAllActivity, AccountLabel, AccountPolicy, AccountStatus, Activity,
    ActivityFacts, ActivityIdentity, ActivitySequence, CapabilityDisclosure,
    IdentityExecutionSchema, TouchAccountCapability,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationDisclosed, WorthQueryApplicationProjection,
    WorthQueryApplicationProjectionDenial, WorthQueryApplicationProjectionRow,
};

pub struct StatusSlot;
pub struct LabelSlot;
pub struct ActivitiesSlot;
pub struct ActivityIdentitySlot;
pub struct ActivitySequenceSlot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernedAccountOmissionResult {
    status: WorthQueryApplicationDisclosed<String>,
    label: WorthQueryApplicationDisclosed<String>,
    activities: WorthQueryApplicationDisclosed<usize>,
}

impl GovernedAccountOmissionResult {
    pub const fn status(&self) -> &WorthQueryApplicationDisclosed<String> {
        &self.status
    }

    pub const fn label(&self) -> &WorthQueryApplicationDisclosed<String> {
        &self.label
    }

    pub const fn activities(&self) -> &WorthQueryApplicationDisclosed<usize> {
        &self.activities
    }
}

worth_query_application_query!(
    pub GovernedAccountOmissionQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result GovernedAccountOmissionResult,
    scope Account,
    name "governed_account_omission"
);

pub(super) fn governed_account_omission_definition() -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    GovernedAccountOmissionQuery,
    AccountSummaryParameters,
    GovernedAccountOmissionResult,
    Account,
> {
    let activity = ApplicationQueryResultShapeBuilder::<
        IdentityExecutionSchema,
        GovernedAccountOmissionQuery,
        Activity,
        (),
    >::new(Activity::reference())
    .field(activity_identity())
    .field(activity_sequence());
    let shape = ApplicationQueryResultShapeBuilder::<
        IdentityExecutionSchema,
        GovernedAccountOmissionQuery,
        Account,
        GovernedAccountOmissionResult,
    >::new(Account::reference())
    .field(status())
    .field(label())
    .relation(activities(), activity)
    .build();
    let disclosure = ApplicationQueryDisclosureContract::governed_by(
        "account-omission",
        TouchAccountCapability::reference(),
    )
    .disclose_field_by(
        status(),
        CapabilityDisclosure::AccountActivity,
        ApplicationQueryInfluenceContract::forbid_all(),
    )
    .disclose_field_by(
        label(),
        CapabilityDisclosure::PrivateLabel,
        ApplicationQueryInfluenceContract::forbid_all(),
    )
    .disclose_relation_by(
        activities(),
        CapabilityDisclosure::PrivateLabel,
        ApplicationQueryInfluenceContract::forbid_all(),
    )
    .disclose_field_by(
        activity_identity(),
        CapabilityDisclosure::PrivateLabel,
        ApplicationQueryInfluenceContract::forbid_all(),
    )
    .disclose_field_by(
        activity_sequence(),
        CapabilityDisclosure::PrivateLabel,
        ApplicationQueryInfluenceContract::forbid_all(),
    );
    ApplicationQueryDefinitionBuilder::public(
        GovernedAccountOmissionQuery::reference(),
        Account::reference(),
        Account::reference(),
        shape,
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(1, 1, 4),
        disclosure,
        ApplicationQueryBasisSupport::current_and_pinned().with_preview(),
        ApplicationQueryLaneEligibility::one_shot()
            .with_historical()
            .with_preview(),
    )
    .build()
    .unwrap()
}

impl WorthQueryApplicationProjection<IdentityExecutionSchema, GovernedAccountOmissionQuery>
    for GovernedAccountOmissionResult
{
    fn project(
        row: &WorthQueryApplicationProjectionRow<
            '_,
            IdentityExecutionSchema,
            GovernedAccountOmissionQuery,
        >,
    ) -> Result<Self, WorthQueryApplicationProjectionDenial> {
        let activities = match row.disclosed_many(activities())? {
            WorthQueryApplicationDisclosed::Disclosed(rows) => {
                WorthQueryApplicationDisclosed::Disclosed(rows.len())
            }
            WorthQueryApplicationDisclosed::Omitted(omission) => {
                WorthQueryApplicationDisclosed::Omitted(omission)
            }
        };
        Ok(Self {
            status: row.disclosed_field(status())?,
            label: row.disclosed_field(label())?,
            activities,
        })
    }
}

fn status() -> ApplicationQueryResultFieldRef<
    GovernedAccountOmissionQuery,
    StatusSlot,
    IdentityExecutionSchema,
    Account,
    AccountPolicy,
    AccountStatus,
    String,
    worth_query_declaration::facade::application_schema::ReadWrite,
    worth_query_declaration::facade::application_schema::EqualityPredicate,
    worth_query_declaration::facade::application_schema::NoApplicationCurrency,
> {
    ApplicationQueryResultFieldRef::new("status", AccountStatus::reference())
}

fn label() -> ApplicationQueryResultFieldRef<
    GovernedAccountOmissionQuery,
    LabelSlot,
    IdentityExecutionSchema,
    Account,
    AccountPolicy,
    AccountLabel,
    String,
    worth_query_declaration::facade::application_schema::ReadWrite,
    worth_query_declaration::facade::application_schema::EqualityPredicate,
    worth_query_declaration::facade::application_schema::NoApplicationCurrency,
> {
    ApplicationQueryResultFieldRef::new("label", AccountLabel::reference())
}

fn activity_identity() -> ApplicationQueryResultFieldRef<
    GovernedAccountOmissionQuery,
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
    GovernedAccountOmissionQuery,
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
    GovernedAccountOmissionQuery,
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
