use std::time::{Duration, Instant};

#[path = "fixture/authentication.rs"]
mod authentication;
use authentication::authenticate_external;
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
#[path = "fixture/world_authentication.rs"]
mod world_authentication;
#[path = "fixture/world_installation.rs"]
mod world_installation;
pub(in crate::domain_computation::primary_graph) use world_installation::{
    installed_authorization_world, installed_authorization_world_with_label,
    installed_authorization_world_with_resource_profile,
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
            schema
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
                .operation_decision_fact_budget(TouchAccountOperation::reference(), 1)
                .operation_projection_work_budget(TouchAccountOperation::reference(), 32)
                .operation_requires_ability(
                    TouchAccountOperation::reference(),
                    ViewAccount::reference(),
                )
                .operation_write(
                    TouchAccountOperation::reference(),
                    AccountStatus::reference(),
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
                    [worth_query_declaration::facade::application_schema::ApplicationAuthorizationPathBuilder::from_principal(
                        Principal::reference(),
                    )
                    .forward(AccountOwner::reference())
                    .allow(Account::reference())],
                )
                .ability_policy(
                    EditAccount::reference(),
                    AccountAccessPolicy::reference(),
                    [worth_query_declaration::facade::application_schema::ApplicationAuthorizationPathBuilder::from_principal(
                        Principal::reference(),
                    )
                    .forward(AccountOwner::reference())
                    .allow(Account::reference())],
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
        }
    }
}

worth_query_entity!(pub ExternalMapping in IdentityExecutionSchema);
worth_query_entity!(pub Principal in IdentityExecutionSchema);
worth_query_entity!(pub Account in IdentityExecutionSchema);
worth_query_entity!(pub Activity in IdentityExecutionSchema);
worth_query_aspect!(pub ExternalIdentity in IdentityExecutionSchema, ExternalMapping);
worth_query_field!(
    pub ExternalIdentityField in IdentityExecutionSchema, ExternalMapping, ExternalIdentity:
    WorthQueryExternalPrincipalIdentity, read_only, equality
);
worth_query_aspect!(pub PrincipalIdentity in IdentityExecutionSchema, Principal);
worth_query_field!(
    pub PrincipalIdentityField in IdentityExecutionSchema, Principal, PrincipalIdentity:
    u64, read_only, equality
);
worth_query_field!(
    pub MappingStatusField in IdentityExecutionSchema, ExternalMapping, ExternalIdentity:
    WorthQueryPrincipalMappingStatus, read_write, equality
);
worth_query_relation!(
    pub MappingTarget in IdentityExecutionSchema,
    ExternalMapping => Principal
);
worth_query_principal_binding!(
    pub IdentityBinding in IdentityExecutionSchema,
    mapping ExternalMapping {
        identity: ExternalIdentityField,
        status: MappingStatusField,
        target: MappingTarget => Principal,
        principal_identity: PrincipalIdentityField
    }
);
worth_query_aspect!(pub AccountPolicy in IdentityExecutionSchema, Account);
worth_query_field!(
    pub AccountIdentity in IdentityExecutionSchema, Account, AccountPolicy:
    String, read_only, equality
);
worth_query_field!(
    pub AccountStatus in IdentityExecutionSchema, Account, AccountPolicy:
    String, read_write, equality
);
worth_query_field!(
    pub AccountLabel in IdentityExecutionSchema, Account, AccountPolicy:
    String, read_write, equality
);
worth_query_aspect!(pub ActivityFacts in IdentityExecutionSchema, Activity);
worth_query_field!(
    pub ActivityIdentity in IdentityExecutionSchema, Activity, ActivityFacts:
    String, read_only, equality
);
worth_query_field!(
    pub ActivitySequence in IdentityExecutionSchema, Activity, ActivityFacts:
    u64, read_only, no_equality
);
worth_query_relation!(
    pub AccountOwner in IdentityExecutionSchema,
    Principal => Account
);
worth_query_relation!(
    pub AccountPrimaryActivity in IdentityExecutionSchema,
    Account => Activity
);
worth_query_relation!(
    pub AccountSecondaryActivity in IdentityExecutionSchema,
    Account => Activity
);
worth_query_relation!(
    pub AccountAllActivity in IdentityExecutionSchema,
    Account => Activity
);
worth_query_relation!(
    pub ActivityAccount in IdentityExecutionSchema,
    Activity => Account
);
worth_query_ability!(pub ViewAccount scoped_to Account, in IdentityExecutionSchema);
worth_query_ability!(pub EditAccount scoped_to Account, in IdentityExecutionSchema);
worth_query_ability!(pub ManageOwnership scoped_to Principal, in IdentityExecutionSchema);
worth_query_policy!(pub AccountAccessPolicy in IdentityExecutionSchema);
worth_query_effect!(pub AccountActivityEffect(String) in IdentityExecutionSchema);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TouchAccountInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultiTouchInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeOwnershipInput;

worth_query_operation!(
    pub TouchAccountOperation(TouchAccountInput) in IdentityExecutionSchema
);
worth_query_operation!(
    pub MultiTouchOperation(MultiTouchInput) in IdentityExecutionSchema
);
worth_query_operation!(
    pub ChangeOwnershipOperation(ChangeOwnershipInput) in IdentityExecutionSchema
);
worth_query_operation_requires!(TouchAccountOperation => [ViewAccount]);
worth_query_operation_expects_fact!(TouchAccountOperation => [AccountStatus]);
worth_query_operation_requires!(MultiTouchOperation => [ViewAccount, EditAccount]);
worth_query_operation_requires!(ChangeOwnershipOperation => [ManageOwnership]);
worth_query_operation_writes!(TouchAccountOperation => [AccountStatus, AccountLabel]);
worth_query_operation_writes!(MultiTouchOperation => [AccountStatus]);
worth_query_operation_emits!(
    TouchAccountOperation => [AccountActivityEffect, LiveActivityEffect]
);
worth_query_operation_reads!(TouchAccountOperation => [AccountStatus, AccountLabel, AccountOwner]);
worth_query_operation_reads!(MultiTouchOperation => [AccountStatus]);
worth_query_operation_reads!(ChangeOwnershipOperation => [AccountOwner, AccountStatus]);
worth_query_operation_links!(ChangeOwnershipOperation => [AccountOwner]);
worth_query_operation_unlinks!(ChangeOwnershipOperation => [AccountOwner]);

pub(super) type InstalledIdentityBinding = WorthQueryInstalledPrincipalBinding<
    IdentityExecutionSchema,
    IdentityBinding,
    ExternalMapping,
    Principal,
    u64,
>;

pub(super) struct IdentityWorld {
    pub(super) runtime: WorthQueryExecutionRuntime,
    pub(super) schema: WorthQueryInstalledApplicationSchema<IdentityExecutionSchema>,
    pub(super) binding: InstalledIdentityBinding,
    pub(super) publication: WorthQueryPrimaryGraphPublication,
}

pub(in crate::domain_computation::primary_graph) struct AuthorizationWorld {
    pub(in crate::domain_computation::primary_graph) application:
        crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
            IdentityExecutionSchema,
        >,
    pub(in crate::domain_computation::primary_graph) binding: InstalledIdentityBinding,
    pub(in crate::domain_computation::primary_graph) invariant:
        crate::domain_computation::primary_graph::WorthQueryApplicationInvariantProjectionAuthority<
            IdentityExecutionSchema,
        >,
}

pub(super) fn external_identity(subject: &str) -> WorthQueryExternalPrincipalIdentity {
    WorthQueryExternalPrincipalIdentity::new("https://issuer.example", subject).unwrap()
}

pub(in crate::domain_computation::primary_graph) fn live_scope() -> WorthQueryRequestScope {
    let source = WorthQueryCancellationSource::new();
    WorthQueryRequestScope::new(Instant::now() + Duration::from_secs(60), source.token())
}
