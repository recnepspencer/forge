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
#[path = "fixture/live_activity_operation.rs"]
mod live_activity_operation;
pub(in crate::domain_computation::primary_graph) use live_activity_operation::{
    PublishLiveActivityInput, PublishLiveActivityOperation,
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
#[path = "fixture/schema_declaration.rs"]
mod schema_declaration;
#[path = "fixture/world_authentication.rs"]
mod world_authentication;
#[path = "fixture/world_installation.rs"]
mod world_installation;
pub(in crate::domain_computation::primary_graph) use schema_declaration::IdentityExecutionSchema;
pub(in crate::domain_computation::primary_graph) use world_installation::{
    installed_authorization_world, installed_authorization_world_on_branch,
    installed_authorization_world_with_label, installed_authorization_world_with_resource_profile,
    installed_blocked_authorization_world, installed_capability_authorization_world,
    installed_capability_authorization_world_on_branch, installed_capability_live_world,
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
    worth_query_ability, worth_query_aspect, worth_query_effect, worth_query_entity,
    worth_query_field, worth_query_operation, worth_query_operation_emits,
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
    pub AccountBlocked in IdentityExecutionSchema,
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

pub(super) fn external_identity(subject: &str) -> WorthQueryExternalPrincipalIdentity {
    WorthQueryExternalPrincipalIdentity::new("https://issuer.example", subject).unwrap()
}

pub(in crate::domain_computation::primary_graph) fn live_scope() -> WorthQueryRequestScope {
    let source = WorthQueryCancellationSource::new();
    WorthQueryRequestScope::new(Instant::now() + Duration::from_secs(60), source.token())
}
