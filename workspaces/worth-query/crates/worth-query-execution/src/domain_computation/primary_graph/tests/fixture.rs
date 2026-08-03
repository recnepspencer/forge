use std::time::{Duration, Instant};

#[path = "fixture/authentication.rs"]
mod authentication;
use authentication::authenticate_external;
#[path = "fixture/capability.rs"]
pub(super) mod capability;
#[path = "fixture/capability_access_fixture.rs"]
mod capability_access_fixture;
#[path = "fixture/capability_seed.rs"]
mod capability_seed;
pub(in crate::domain_computation::primary_graph) use capability_access_fixture::admit_touch_account_capability;
#[path = "fixture/capability_status_mutation.rs"]
mod capability_status_mutation;
pub(super) use capability::{
    CapabilityAction, CapabilityDisclosure, CapabilityIdentity, CapabilityPurpose,
    CapabilityStatus, CapabilityStatusField, CapabilityTouchInput, CapabilityTouchOperation,
    TouchAccountCapability,
};
pub(in crate::domain_computation::primary_graph) use capability_status_mutation::revoke_current_capability;
#[path = "fixture/application_queries.rs"]
mod application_queries;
pub(in crate::domain_computation::primary_graph) use application_queries::AccountSummaryParameters;
pub(super) use application_queries::{
    cross_root_definition, status_parameter, AccountSummaryQuery, AccountSummaryResult,
    CrossRootQuery, GovernedAccountSummaryQuery, OrderedAccountSummaryQuery,
    ScopedAccountSummaryQuery,
};
#[path = "fixture/nested_account.rs"]
mod nested_account;
pub(super) use nested_account::{NestedAccountQuery, NestedAccountResult};
#[path = "fixture/forged_selector.rs"]
mod forged_selector;
pub(super) use forged_selector::{ForgedSelectorQuery, ForgedSelectorResult};
#[path = "fixture/live_account_query.rs"]
mod live_account_query;
pub(in crate::domain_computation::primary_graph) use live_account_query::{
    live_account_parameters, LiveAccountActivityCause, LiveAccountActivityQuery,
    LiveAccountActivityResult, LiveActivityEffect, LiveActivityEvent,
};
#[path = "fixture/governed_live_query.rs"]
mod governed_live_query;
pub(in crate::domain_computation::primary_graph) use governed_live_query::{
    governed_live_account_parameters, GovernedLiveAccountActivityCause,
    GovernedLiveAccountActivityQuery, GovernedLiveAccountActivityResult,
};
#[path = "fixture/governed_omission_query.rs"]
mod governed_omission_query;
pub(in crate::domain_computation::primary_graph) use governed_omission_query::{
    GovernedAccountOmissionQuery, GovernedAccountOmissionResult,
};
#[path = "fixture/invalid_disclosure_queries.rs"]
mod invalid_disclosure_queries;
#[path = "fixture/schema_types.rs"]
mod schema_types;
#[path = "fixture/world_authentication.rs"]
mod world_authentication;
#[path = "fixture/world_installation.rs"]
mod world_installation;
pub(in crate::domain_computation::primary_graph) use schema_types::*;
pub(in crate::domain_computation::primary_graph) use world_installation::{
    installed_authorization_world, installed_authorization_world_with_label,
    installed_authorization_world_with_resource_profile, installed_blocked_authorization_world,
    installed_capability_authorization_world, installed_capability_live_world,
    installed_capability_replacement_world, installed_capability_world_with_label,
    AuthorizationWorld,
};
pub(super) use world_installation::{
    installed_two_principal_authorization_world, installed_world, installed_world_with_policy_fact,
};

use worth_query_admission::facade::authenticated_principal::*;
use worth_query_declaration::facade::authentication::{
    WorthQueryExternalPrincipalIdentity, WorthQueryPrincipalMappingStatus,
};
use worth_query_declaration::{
    worth_query_ability, worth_query_application_schema, worth_query_aspect, worth_query_effect,
    worth_query_entity, worth_query_field, worth_query_operation, worth_query_operation_emits,
    worth_query_operation_expects_fact, worth_query_operation_links, worth_query_operation_reads,
    worth_query_operation_requires, worth_query_operation_unlinks, worth_query_operation_writes,
    worth_query_policy, worth_query_principal_binding, worth_query_relation,
};
use worth_query_installation::facade::{
    WorthQueryInstallationAdmissionProfile, WorthQueryInstallationGeneration,
    WorthQueryInstalledApplicationSchema, WorthQueryInstalledPrincipalBinding,
    WorthQueryPortableDomainIdentity, WorthQueryPortableDomainPackage,
    WorthQueryValidatedPortableDomainPackage,
};

use crate::domain_computation::execution_runtime::{
    WorthQueryApplicationQueryResourceProfile, WorthQueryExecutionRuntime,
    WorthQueryExecutionRuntimeInstaller,
};
use crate::domain_computation::primary_graph::WorthQueryApplicationPrincipalKey;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationEntityKey, WorthQueryApplicationEntitySeed,
    WorthQueryApplicationRelationSeed, WorthQueryPrimaryGraphPublication,
};

worth_query_application_schema! {
    pub schema IdentityExecutionSchema {
        owner: identity_execution_test,
        version: (1, 0),
        members: |schema| {
            let schema = schema
                .entity(ExternalMapping::reference())
                .entity(Principal::reference())
                .entity(Account::reference())
                .entity(Activity::reference())
                .aspect(ExternalMapping::reference(), ExternalIdentity::reference())
                .aspect(Principal::reference(), PrincipalIdentity::reference())
                .field(ExternalMapping::reference(), ExternalIdentityField::reference())
                .field(ExternalMapping::reference(), MappingStatusField::reference())
                .field(Principal::reference(), PrincipalIdentityField::reference())
                .aspect(Account::reference(), AccountPolicy::reference())
                .field(Account::reference(), AccountIdentity::reference())
                .field(Account::reference(), AccountStatus::reference())
                .field(Account::reference(), AccountLabel::reference())
                .aspect(Activity::reference(), ActivityFacts::reference())
                .field(Activity::reference(), ActivityIdentity::reference())
                .field(Activity::reference(), ActivitySequence::reference())
                .relation(
                    MappingTarget::reference(),
                    ExternalMapping::reference(),
                    Principal::reference(),
                )
                .relation(
                    AccountOwner::reference(),
                    Principal::reference(),
                    Account::reference(),
                )
                .relation(
                    AccountBlocked::reference(),
                    Principal::reference(),
                    Account::reference(),
                )
                .relation(
                    AccountPrimaryActivity::reference(),
                    Account::reference(),
                    Activity::reference(),
                )
                .relation(
                    AccountSecondaryActivity::reference(),
                    Account::reference(),
                    Activity::reference(),
                )
                .relation(
                    AccountAllActivity::reference(),
                    Account::reference(),
                    Activity::reference(),
                )
                .relation(
                    ActivityAccount::reference(),
                    Activity::reference(),
                    Account::reference(),
                )
                .principal_binding(IdentityBinding::reference())
                .ability(ViewAccount::reference())
                .ability(EditAccount::reference())
                .ability(ManageOwnership::reference())
                .effect(AccountActivityEffect::reference())
                .effect(LiveActivityEffect::reference())
                .operation(TouchAccountOperation::reference())
                .operation_decision_fact_budget(TouchAccountOperation::reference(), 2)
                .operation_projection_work_budget(TouchAccountOperation::reference(), 32)
                .operation_requires_ability(
                    TouchAccountOperation::reference(),
                    ViewAccount::reference(),
                )
                .operation_write(
                    TouchAccountOperation::reference(),
                    AccountStatus::reference(),
                )
                .operation_write(
                    TouchAccountOperation::reference(),
                    AccountLabel::reference(),
                )
                .operation_emit(
                    TouchAccountOperation::reference(),
                    AccountActivityEffect::reference(),
                )
                .operation_emit(
                    TouchAccountOperation::reference(),
                    LiveActivityEffect::reference(),
                )
                .operation_read_field(
                    TouchAccountOperation::reference(),
                    AccountStatus::reference(),
                )
                .operation_read_field(
                    TouchAccountOperation::reference(),
                    AccountLabel::reference(),
                )
                .operation_expected_fact(
                    TouchAccountOperation::reference(),
                    AccountStatus::reference(),
                )
                .operation(MultiTouchOperation::reference())
                .operation_decision_fact_budget(MultiTouchOperation::reference(), 2)
                .operation_projection_work_budget(MultiTouchOperation::reference(), 32)
                .operation_requires_ability(
                    MultiTouchOperation::reference(),
                    ViewAccount::reference(),
                )
                .operation_requires_ability(
                    MultiTouchOperation::reference(),
                    EditAccount::reference(),
                )
                .operation_write(
                    MultiTouchOperation::reference(),
                    AccountStatus::reference(),
                )
                .operation_read_field(
                    MultiTouchOperation::reference(),
                    AccountStatus::reference(),
                )
                .operation(ChangeOwnershipOperation::reference())
                .operation_decision_fact_budget(ChangeOwnershipOperation::reference(), 2)
                .operation_projection_work_budget(ChangeOwnershipOperation::reference(), 32)
                .operation_requires_ability(
                    ChangeOwnershipOperation::reference(),
                    ManageOwnership::reference(),
                )
                .operation_read_relation(
                    ChangeOwnershipOperation::reference(),
                    AccountOwner::reference(),
                )
                .operation_read_field(
                    ChangeOwnershipOperation::reference(),
                    AccountStatus::reference(),
                )
                .operation_link(
                    ChangeOwnershipOperation::reference(),
                    AccountOwner::reference(),
                )
                .operation_unlink(
                    ChangeOwnershipOperation::reference(),
                    AccountOwner::reference(),
                )
                .policy(AccountAccessPolicy::reference())
                .ability_policy(
                    ViewAccount::reference(),
                    AccountAccessPolicy::reference(),
                    [
                        worth_query_declaration::facade::application_schema::ApplicationAuthorizationPathBuilder::from_principal(
                            Principal::reference(),
                        )
                        .forward(AccountOwner::reference())
                        .allow(Account::reference()),
                        worth_query_declaration::facade::application_schema::ApplicationAuthorizationPathBuilder::from_principal(
                            Principal::reference(),
                        )
                        .forward(AccountBlocked::reference())
                        .deny(Account::reference()),
                    ],
                )
                .ability_policy(
                    EditAccount::reference(),
                    AccountAccessPolicy::reference(),
                    [
                        worth_query_declaration::facade::application_schema::ApplicationAuthorizationPathBuilder::from_principal(
                            Principal::reference(),
                        )
                        .forward(AccountOwner::reference())
                        .allow(Account::reference()),
                        worth_query_declaration::facade::application_schema::ApplicationAuthorizationPathBuilder::from_principal(
                            Principal::reference(),
                        )
                        .forward(AccountBlocked::reference())
                        .deny(Account::reference()),
                    ],
                )
                .ability_policy(
                    ManageOwnership::reference(),
                    AccountAccessPolicy::reference(),
                    [worth_query_declaration::facade::application_schema::ApplicationAuthorizationPathBuilder::from_principal(
                        Principal::reference(),
                    )
                    .allow(Principal::reference())],
                )
                .application_query(application_queries::account_summary_definition())
                .application_query(application_queries::scoped_account_summary_definition())
                .application_query(application_queries::cross_root_definition("open"))
                .application_query(application_queries::governed_account_summary_definition())
                .application_query(application_queries::ordered_account_summary_definition())
                .application_query(nested_account::nested_account_definition())
                .application_query(forged_selector::forged_selector_definition())
                .application_query(live_account_query::live_account_activity_definition())
                .application_query(governed_live_query::governed_live_account_definition())
                .application_query(governed_omission_query::governed_account_omission_definition())
                .application_query(invalid_disclosure_queries::incomplete_disclosure_definition())
                .application_query(invalid_disclosure_queries::forbidden_influence_definition());
            capability::install(schema)
        }
    }
}
