use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use worth_query_admission::facade::authenticated_principal::{
    WorthQueryAuthenticatedExternalPrincipal, WorthQueryRequestScope,
};
use worth_query_installation::facade::{
    ApplicationSchema, TypedApplicationIdentityValue, WorthQueryInstalledApplicationSchema,
    WorthQueryInstalledPrincipalBinding,
};

use crate::domain_computation::application_aftermath::WorthQueryExternalEffectTransport;
use crate::domain_computation::authorization::WorthQueryRuntimeClock;
use crate::domain_computation::execution_runtime::{
    WorthQueryExecutionInstallationAuthority, WorthQueryExecutionRuntime,
};
use crate::domain_computation::managed_run::WorthQueryRecoveryHandleRegistry;
use crate::domain_computation::runtime_time::WorthQueryRuntimeTimeSource;

use super::provider::WorthQueryPrimaryGraphProvider;
use super::{
    authentication_clock::WorthQueryAuthenticationClock, WorthQueryAuthenticatedPrincipal,
    WorthQueryPrimaryGraphBootstrap, WorthQueryPrimaryGraphInstallationDenial,
    WorthQueryPrimaryGraphInstallationDenialKind, WorthQueryPrimaryGraphPublication,
    WorthQueryPrincipalResolutionDenial, WorthQueryPrincipalResolutionMode,
};
use crate::domain_computation::authorization::WorthQueryInstalledAuthorizationRegistry;

mod external_dispatch_attempt;
pub(in crate::domain_computation::primary_graph) mod installation;

pub(in crate::domain_computation) use external_dispatch_attempt::WorthQueryExternalDispatchAttemptOrdinal;

/// Purpose-scoped application runtime published from one typed primary graph.
///
/// Publishing consumes the raw execution root and its installation authority.
/// The resulting value exposes principal admission but no provider-session,
/// ordinary query, mutation, workflow, live, or replay authority.
///
/// ```compile_fail
/// use worth_query_execution::facade::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;
/// use worth_query_installation::facade::ApplicationSchema;
///
/// fn cannot_extract_execution_authority<Schema: ApplicationSchema>(
///     application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
/// ) {
///     let _ = application.installed_packages();
/// }
/// ```
pub struct WorthQueryPrimaryGraphApplicationRuntime<Schema> {
    pub(in crate::domain_computation) runtime: WorthQueryExecutionRuntime,
    pub(in crate::domain_computation) installed_schema:
        WorthQueryInstalledApplicationSchema<Schema>,
    publication: WorthQueryPrimaryGraphPublication,
    pub(in crate::domain_computation) authorization: WorthQueryInstalledAuthorizationRegistry,
    pub(in crate::domain_computation) authorization_clock: Arc<WorthQueryRuntimeClock>,
    authentication_clock: WorthQueryAuthenticationClock,
    pub(super) relational_source: worth_relational::facade::bridge::RuntimeBridgeRelationalSource,
    pub(super) execution_basis_source:
        worth_relational::facade::runtime::RelationalApplicationCommitBasisSource,
    pub(super) bridge: super::managed_bridge::WorthQueryInstalledApplicationBridge,
    pub(super) conditional_operations: std::sync::Mutex<
        super::conditional_operation::WorthQueryConditionalOperationRegistry<Schema>,
    >,
    pub(super) primary_provider: std::sync::Arc<WorthQueryPrimaryGraphProvider>,
    pub(super) primary_graph_authority:
        worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
    pub(super) result_buffers:
        super::application_query::resource_lifecycle::WorthQueryApplicationResultBufferRegistry,
    pub(super) basis_leases:
        super::application_query::resource_lifecycle::WorthQueryApplicationBasisRegistry,
    pub(super) next_preview_session: AtomicU64,
    pub(super) next_external_dispatch_attempt: AtomicU64,
    pub(super) external_effect_transport:
        std::sync::OnceLock<std::sync::Arc<dyn WorthQueryExternalEffectTransport>>,
    /// Instance-local recovery-handle live set (Q8.9 / R8.29).
    pub(crate) recovery_handles: Arc<WorthQueryRecoveryHandleRegistry>,
}

impl<Schema> WorthQueryPrimaryGraphBootstrap<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn publish_application_runtime(
        self,
        runtime: WorthQueryExecutionRuntime,
        authority: WorthQueryExecutionInstallationAuthority,
        installed_schema: WorthQueryInstalledApplicationSchema<Schema>,
    ) -> Result<
        WorthQueryPrimaryGraphApplicationRuntime<Schema>,
        WorthQueryPrimaryGraphInstallationDenial,
    > {
        installation::require_no_conditional_bindings(&runtime, &installed_schema)?;
        installation::publish_application_runtime_with_clock(
            installation::ApplicationRuntimePublication {
                bootstrap: self,
                runtime,
                authority,
                installed_schema,
                authorization_clock: WorthQueryRuntimeClock::system(),
                fault_port: super::provider::fault_port::production_fault_port(),
            },
        )
    }

    /// Begins the sole primary-graph conditional publication progression.
    ///
    /// Complete provider, clock, and reconstruction bindings are accumulated
    /// here before the application runtime can become visible.
    pub fn conditional_application_runtime_installation(
        self,
        runtime: WorthQueryExecutionRuntime,
        authority: WorthQueryExecutionInstallationAuthority,
        installed_schema: WorthQueryInstalledApplicationSchema<Schema>,
    ) -> Result<
        super::conditional_operation::WorthQueryConditionalApplicationRuntimeInstallation<Schema>,
        super::conditional_operation::WorthQueryConditionalRuntimeInstallationDenial,
    > {
        super::conditional_operation::WorthQueryConditionalApplicationRuntimeInstallation::new(
            installation::ApplicationRuntimePublication {
                bootstrap: self,
                runtime,
                authority,
                installed_schema,
                authorization_clock: WorthQueryRuntimeClock::system(),
                fault_port: super::provider::fault_port::production_fault_port(),
            },
        )
    }

    /// Publishes one application runtime with a host-installed trusted-time
    /// mechanism.
    ///
    /// The source is fixed for the lifetime of the returned runtime. It grants
    /// no Query authority and is never exposed to operation callers.
    pub fn publish_application_runtime_with_authorization_time_source(
        self,
        runtime: WorthQueryExecutionRuntime,
        authority: WorthQueryExecutionInstallationAuthority,
        installed_schema: WorthQueryInstalledApplicationSchema<Schema>,
        source: impl WorthQueryRuntimeTimeSource,
    ) -> Result<
        WorthQueryPrimaryGraphApplicationRuntime<Schema>,
        WorthQueryPrimaryGraphInstallationDenial,
    > {
        self.publish_application_runtime_with_ports(
            runtime,
            authority,
            installed_schema,
            source,
            super::provider::fault_port::production_fault_port(),
        )
    }

    pub(in crate::domain_computation::primary_graph) fn publish_application_runtime_with_ports(
        self,
        runtime: WorthQueryExecutionRuntime,
        authority: WorthQueryExecutionInstallationAuthority,
        installed_schema: WorthQueryInstalledApplicationSchema<Schema>,
        source: impl WorthQueryRuntimeTimeSource,
        fault_port: Arc<dyn super::provider::fault_port::WorthQueryPrimaryGraphFaultPort>,
    ) -> Result<
        WorthQueryPrimaryGraphApplicationRuntime<Schema>,
        WorthQueryPrimaryGraphInstallationDenial,
    > {
        installation::publish_application_runtime_with_clock(
            installation::ApplicationRuntimePublication {
                bootstrap: self,
                runtime,
                authority,
                installed_schema,
                authorization_clock: WorthQueryRuntimeClock::from_source(source),
                fault_port,
            },
        )
    }
}

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema> {
    pub(in crate::domain_computation::primary_graph) fn authentication_is_expired(
        &self,
        valid_until: std::time::Instant,
    ) -> bool {
        self.authentication_clock.is_expired(valid_until)
    }

    #[cfg(test)]
    pub(crate) fn fix_authentication_time(&mut self, now: std::time::Instant) {
        self.authentication_clock = WorthQueryAuthenticationClock::fixed(now);
    }
}

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn installed_schema(&self) -> &WorthQueryInstalledApplicationSchema<Schema> {
        &self.installed_schema
    }

    pub fn publication(&self) -> &WorthQueryPrimaryGraphPublication {
        &self.publication
    }

    pub fn result_buffer_observer(
        &self,
    ) -> super::application_query::WorthQueryApplicationResultBufferObserver {
        self.result_buffers.observer()
    }

    pub fn application_query_basis_observer(
        &self,
    ) -> super::application_query::WorthQueryApplicationBasisObserver {
        self.basis_leases.observer()
    }

    pub fn capability_plan_compilation_evidence(
        &self,
    ) -> crate::domain_computation::authorization::WorthQueryCapabilityPlanCompilationEvidence {
        self.authorization.capability_compilation()
    }

    pub(in crate::domain_computation) fn graph_work_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupportSnapshot
    {
        self.primary_provider.application_resource_support()
    }

    pub(in crate::domain_computation) fn graph_work_provider_identity(&self) -> &str {
        self.primary_graph_authority.provider_identity()
    }

    #[cfg(test)]
    pub(crate) fn provider_session_resource_count(&self) -> usize {
        self.primary_provider.application_attempt_resource_count()
    }

    #[cfg(test)]
    pub(crate) fn application_attempt_work(
        &self,
    ) -> super::provider::WorthQueryApplicationAttemptWorkSnapshot {
        self.primary_provider.application_attempt_work()
    }

    pub fn resolve_authenticated_principal<Binding, Mapping, Principal, PrincipalIdentity>(
        &self,
        installed_binding: &WorthQueryInstalledPrincipalBinding<
            Schema,
            Binding,
            Mapping,
            Principal,
            PrincipalIdentity,
        >,
        external: WorthQueryAuthenticatedExternalPrincipal<Schema>,
        scope: &WorthQueryRequestScope,
        mode: WorthQueryPrincipalResolutionMode,
    ) -> Result<
        WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        WorthQueryPrincipalResolutionDenial,
    >
    where
        PrincipalIdentity: TypedApplicationIdentityValue,
    {
        self.runtime
            .resolve_authenticated_principal(installed_binding, external, scope, mode)
    }

    pub fn validate_authenticated_principal<Principal, PrincipalIdentity>(
        &self,
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        scope: &WorthQueryRequestScope,
    ) -> Result<(), WorthQueryPrincipalResolutionDenial> {
        self.runtime
            .validate_authenticated_principal(principal, scope)
    }

    /// Closes ordinary live delivery without closing the authoritative graph.
    /// Later commits retain their compact idempotency causality but no longer
    /// enter this runtime's delivery ring.
    pub fn close_live_delivery(&self) {
        self.primary_provider.live_delivery.close();
    }
}
