use std::sync::atomic::AtomicU64;
use worth_query_admission::facade::authenticated_principal::{
    WorthQueryAuthenticatedExternalPrincipal, WorthQueryRequestScope,
};
use worth_query_installation::facade::{
    ApplicationSchema, TypedApplicationIdentityValue, WorthQueryInstalledApplicationSchema,
    WorthQueryInstalledPrincipalBinding,
};

use crate::domain_computation::authorization::WorthQueryAuthorizationClock;
use crate::domain_computation::execution_runtime::{
    WorthQueryExecutionInstallationAuthority, WorthQueryExecutionRuntime,
};

use super::provider::WorthQueryPrimaryGraphProvider;
use super::{
    WorthQueryAuthenticatedPrincipal, WorthQueryPrimaryGraphBootstrap,
    WorthQueryPrimaryGraphInstallationDenial, WorthQueryPrimaryGraphInstallationDenialKind,
    WorthQueryPrimaryGraphPublication, WorthQueryPrincipalResolutionDenial,
    WorthQueryPrincipalResolutionMode,
};
use crate::domain_computation::authorization::WorthQueryInstalledAuthorizationRegistry;

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
    pub(in crate::domain_computation) authorization_clock: WorthQueryAuthorizationClock,
    pub(super) relational_source: worth_relational::facade::bridge::RuntimeBridgeRelationalSource,
    pub(super) bridge: worth_runtime_bridge::facade::RuntimeBridge,
    pub(super) primary_provider: std::sync::Arc<WorthQueryPrimaryGraphProvider>,
    pub(super) primary_graph_authority:
        worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
    pub(super) result_buffers:
        super::application_query::resource_lifecycle::WorthQueryApplicationResultBufferRegistry,
    pub(super) basis_leases:
        super::application_query::resource_lifecycle::WorthQueryApplicationBasisRegistry,
    pub(super) next_preview_session: AtomicU64,
}

impl<Schema> WorthQueryPrimaryGraphBootstrap<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn publish_application_runtime(
        self,
        mut runtime: WorthQueryExecutionRuntime,
        authority: WorthQueryExecutionInstallationAuthority,
        installed_schema: WorthQueryInstalledApplicationSchema<Schema>,
    ) -> Result<
        WorthQueryPrimaryGraphApplicationRuntime<Schema>,
        WorthQueryPrimaryGraphInstallationDenial,
    > {
        runtime
            .installed_packages()
            .validate_application_schema(&installed_schema)
            .map_err(|denial| {
                WorthQueryPrimaryGraphInstallationDenial::new(
                    WorthQueryPrimaryGraphInstallationDenialKind::StaleInstalledSchema,
                    denial.subject(),
                )
            })?;
        let authorization = WorthQueryInstalledAuthorizationRegistry::compile(
            &installed_schema,
            &self.graph.layout,
        )
        .map_err(|denial| {
            WorthQueryPrimaryGraphInstallationDenial::new(
                WorthQueryPrimaryGraphInstallationDenialKind::AuthorizationPolicyRejected,
                denial.subject(),
            )
        })?;
        let publication = self.publish(&mut runtime, &authority)?;
        let graph = runtime
            .retain_primary_graph_integration_handle()
            .expect("publishing the primary graph installs its integration authority");
        let relational_source = graph.relational_bridge_source();
        let bridge = super::managed_bridge::install_application_bridge(
            &installed_schema,
            relational_source.clone(),
        )?;
        let (provider_anchor, primary_provider) = WorthQueryPrimaryGraphProvider::install(graph);
        let primary_graph_authority =
            worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority::install(
                authority.installation_runtime(),
                "primary",
                provider_anchor.provider_identity(),
                true,
                Some("primary"),
                provider_anchor,
            )
            .map_err(|detail| {
                WorthQueryPrimaryGraphInstallationDenial::new(
                    WorthQueryPrimaryGraphInstallationDenialKind::RelationalSchemaRejected,
                    detail,
                )
            })?;
        Ok(WorthQueryPrimaryGraphApplicationRuntime {
            runtime,
            installed_schema,
            publication,
            authorization,
            authorization_clock: WorthQueryAuthorizationClock::system(),
            relational_source,
            bridge,
            primary_provider,
            primary_graph_authority,
            result_buffers: Default::default(),
            basis_leases: Default::default(),
            next_preview_session: AtomicU64::new(1),
        })
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
    pub(crate) fn script_authorization_time(
        &mut self,
        samples: impl IntoIterator<Item = std::time::SystemTime>,
    ) {
        self.authorization_clock = WorthQueryAuthorizationClock::scripted(samples);
    }

    #[cfg(test)]
    pub(crate) fn lose_next_commit_response(&self) {
        self.primary_provider.lose_next_commit_response();
    }

    #[cfg(test)]
    pub(crate) fn reject_next_session_prepare(&self) {
        self.primary_provider.reject_next_session_prepare();
    }

    #[cfg(test)]
    pub(crate) fn reject_next_commit_before_transaction(&self) {
        self.primary_provider
            .reject_next_commit_before_transaction();
    }

    #[cfg(test)]
    pub(crate) fn provider_session_resource_count(&self) -> usize {
        let sessions = self
            .primary_provider
            .sessions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        sessions
            .application_attempts
            .len()
            .saturating_add(sessions.session_overlays.len())
            .saturating_add(sessions.overlays.len())
    }

    #[cfg(test)]
    pub(crate) fn fail_next_index_publication(&self) {
        self.primary_provider.fail_next_index_publication();
    }

    #[cfg(test)]
    pub(crate) fn completed_mutation_work(
        &self,
    ) -> Option<super::provider::WorthQueryPrimaryMutationWorkEvidence> {
        self.primary_provider.completed_mutation_work()
    }

    #[cfg(test)]
    pub(crate) fn skip_next_invariant_owner_execution(&self) {
        self.primary_provider.skip_next_invariant_owner_execution();
    }

    #[cfg(test)]
    pub(crate) fn violate_next_relational_invariant(&self) {
        self.primary_provider.violate_next_relational_invariant();
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
