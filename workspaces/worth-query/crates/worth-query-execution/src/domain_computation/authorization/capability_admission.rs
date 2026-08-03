//! Non-observing preparation of a typed capability request.

use worth_foundational::facade::CanonicalDigestId;
use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;
use worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest;
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationCapability,
};

use super::admission::admit_request;
use super::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
    WorthQueryPreparedApplicationCapabilityAccess,
};
use crate::domain_computation::primary_graph::{
    WorthQueryAuthenticatedPrincipal, WorthQueryPrimaryGraphApplicationRuntime,
};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn prepare_capability_access<Principal, PrincipalIdentity, Capability, Operation, Input>(
        &self,
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
        input: Input,
        request: &WorthQueryRequestScope,
    ) -> Result<
        WorthQueryPreparedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
        WorthQueryOperationAuthorizationDenial,
    >
    where
        Input: ApplicationCapabilityRequest<Schema, Capability>,
    {
        admit_request(request, capability.contract().operation())?;
        validate_capability_static_authority(self, principal, capability)?;
        let installed = self
            .authorization
            .capability_plan(capability)
            .ok_or_else(|| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::PolicyNotInstalled,
                    capability.contract().name(),
                )
            })?;
        if !self.authorization.bridge().matches_installed_policy(
            installed.correspondence,
            &super::bridge_authorization_binding_identity(capability.binding_identity()),
            installed.contract.name(),
            &installed.request.resource_entity,
            installed.contract.operation(),
            &installed.bridge_rules,
        ) {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::PolicyNotInstalled,
                capability.contract().name(),
            ));
        }
        let projection = input.capability_request().map_err(|projection_denial| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected,
                projection_denial.subject(),
            )
        })?;
        super::capability_projection_validation::validate_projected_capability_shape(
            installed,
            &projection,
        )?;
        admit_request(request, capability.contract().operation())?;
        if principal.is_expired() {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::ExpiredAuthentication,
                principal.binding(),
            ));
        }
        Ok(WorthQueryPreparedApplicationCapabilityAccess::mint(
            self.runtime.authority_identity(),
            capability.binding_identity().clone(),
            CanonicalDigestId::new(*capability.identity().bytes()),
            installed.capability_authority_identity.clone(),
            capability.contract().name(),
            std::any::type_name::<Capability>(),
            capability.contract().operation(),
            principal.principal_entity_id(),
            principal.binding(),
            principal.freshness().clone(),
            input,
            projection,
            principal.valid_until(),
            request.clone(),
            capability.lookup_evidence().canonical_work(),
        ))
    }
}

fn validate_capability_static_authority<
    Schema,
    Principal,
    PrincipalIdentity,
    Capability,
    Operation,
    Input,
>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
) -> Result<(), WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
{
    if principal.is_expired() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ExpiredAuthentication,
            principal.binding(),
        ));
    }
    if principal.runtime_authority() != runtime.runtime.authority_identity() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
            capability.contract().name(),
        ));
    }
    if principal.binding_identity() != capability.binding_identity()
        || runtime.installed_schema.binding_identity() != *capability.binding_identity()
    {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::StaleInstalledSchema,
            capability.contract().name(),
        ));
    }
    runtime
        .installed_schema
        .validate_installed_capability(capability)
        .map_err(|installation_denial| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation,
                installation_denial.subject(),
            )
        })
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
