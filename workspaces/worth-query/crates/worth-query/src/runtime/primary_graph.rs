use worth_query_declaration::facade::application_schema::{
    ApplicationSchema, ApplicationSchemaDeclaration,
};
use worth_query_execution::facade::primary_graph::WorthQueryPrimaryGraphPublication;
use worth_query_installation::facade::{
    WorthQueryInstalledApplicationSchema, WorthQueryInstalledApplicationSchemaDenial,
    WorthQueryInstalledPrincipalBinding,
};

use super::WorthQueryRuntime;

impl WorthQueryRuntime {
    /// Returns installation evidence for the execution-owned primary graph.
    ///
    /// The receipt is descriptive. It does not expose graph mutation authority
    /// or raw Relational access.
    pub fn primary_graph_publication(&self) -> Option<&WorthQueryPrimaryGraphPublication> {
        self.primary_graph_publication.as_ref()
    }

    /// Binds a typed schema declaration to this runtime's exact installed
    /// package generation.
    pub fn installed_application_schema<Schema>(
        &self,
        declaration: ApplicationSchemaDeclaration<Schema>,
    ) -> Result<
        WorthQueryInstalledApplicationSchema<Schema>,
        WorthQueryInstalledApplicationSchemaDenial,
    >
    where
        Schema: ApplicationSchema,
    {
        self.execution_runtime
            .installed_packages()
            .bind_application_schema(declaration)
    }

    /// Resolves one admitted external identity through the installed primary
    /// graph. Authentication remains admission-owned; this step grants only
    /// application-principal identity.
    pub fn resolve_authenticated_principal<Schema, Binding, Mapping, Principal, PrincipalIdentity>(
        &self,
        installed_binding: &WorthQueryInstalledPrincipalBinding<
            Schema,
            Binding,
            Mapping,
            Principal,
            PrincipalIdentity,
        >,
        external: worth_query_admission::facade::authenticated_principal::WorthQueryAuthenticatedExternalPrincipal<
            Schema,
        >,
        scope: &worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope,
        mode: worth_query_execution::facade::primary_graph::WorthQueryPrincipalResolutionMode,
    ) -> Result<
        worth_query_execution::facade::primary_graph::WorthQueryAuthenticatedPrincipal<
            Schema,
            Principal,
            PrincipalIdentity,
        >,
        worth_query_execution::facade::primary_graph::WorthQueryPrincipalResolutionDenial,
    >
    where
        Schema: ApplicationSchema,
        PrincipalIdentity: worth_query_installation::facade::TypedApplicationIdentityValue,
    {
        self.execution_runtime.resolve_authenticated_principal(
            installed_binding,
            external,
            scope,
            mode,
        )
    }

    pub fn validate_authenticated_principal<Schema, Principal, PrincipalIdentity>(
        &self,
        principal: &worth_query_execution::facade::primary_graph::WorthQueryAuthenticatedPrincipal<
            Schema,
            Principal,
            PrincipalIdentity,
        >,
        scope: &worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope,
    ) -> Result<(), worth_query_execution::facade::primary_graph::WorthQueryPrincipalResolutionDenial>
    where
        Schema: ApplicationSchema,
    {
        self.execution_runtime
            .validate_authenticated_principal(principal, scope)
    }
}
