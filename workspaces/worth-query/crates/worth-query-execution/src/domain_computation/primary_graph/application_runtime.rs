use worth_query_admission::facade::authenticated_principal::{
    WorthQueryAuthenticatedExternalPrincipal, WorthQueryRequestScope,
};
use worth_query_installation::facade::{
    ApplicationSchema, TypedApplicationIdentityValue, WorthQueryInstalledApplicationSchema,
    WorthQueryInstalledPrincipalBinding,
};

use crate::domain_computation::execution_runtime::{
    WorthQueryExecutionInstallationAuthority, WorthQueryExecutionRuntime,
};

use super::authorization::WorthQueryInstalledAuthorizationRegistry;
use super::{
    WorthQueryAuthenticatedPrincipal, WorthQueryPrimaryGraphBootstrap,
    WorthQueryPrimaryGraphInstallationDenial, WorthQueryPrimaryGraphInstallationDenialKind,
    WorthQueryPrimaryGraphPublication, WorthQueryPrincipalResolutionDenial,
    WorthQueryPrincipalResolutionMode,
};

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
    pub(super) runtime: WorthQueryExecutionRuntime,
    pub(super) installed_schema: WorthQueryInstalledApplicationSchema<Schema>,
    publication: WorthQueryPrimaryGraphPublication,
    pub(super) authorization: WorthQueryInstalledAuthorizationRegistry,
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
        Ok(WorthQueryPrimaryGraphApplicationRuntime {
            runtime,
            installed_schema,
            publication,
            authorization,
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
}
