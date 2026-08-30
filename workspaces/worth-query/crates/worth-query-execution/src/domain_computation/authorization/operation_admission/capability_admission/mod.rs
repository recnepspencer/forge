//! Current capability request admission progression.

use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;
use worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest;
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationCapability,
};

use crate::domain_computation::authorization::admission::admit_request;
use crate::domain_computation::authorization::WorthQueryOperationAuthorizationDenial;
use crate::domain_computation::primary_graph::{
    WorthQueryApprovedElevation, WorthQueryAuthenticatedPrincipal,
    WorthQueryPrimaryGraphApplicationRuntime,
};

mod preparation;

pub use preparation::WorthQueryAdmittedApplicationCapabilityAccess;
use preparation::{complete_capability_admission, prepare_capability_admission};
pub(in crate::domain_computation::authorization) use preparation::{
    WorthQueryCapabilityContextKey, WorthQueryResolvedCapabilityRequest,
};
pub(in crate::domain_computation::authorization) use preparation::{
    WorthQueryCurrentCapabilityObservation, WorthQueryDelegationResolvedRequest,
    WorthQueryExactCapabilityObservationContext,
};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn admit_capability_access<Principal, PrincipalIdentity, Capability, Operation, Input>(
        &self,
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
        input: Input,
        request: &WorthQueryRequestScope,
    ) -> Result<
        WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
        WorthQueryOperationAuthorizationDenial,
    >
    where
        Operation: 'static,
        Input: ApplicationCapabilityRequest<Schema, Capability>
            + worth_query_declaration::facade::portable_identity::WorthQueryPortableType
            + 'static,
    {
        self.admit_capability_access_inner(principal, capability, input, request, None)
    }

    pub fn admit_approved_elevation_access<
        Principal,
        PrincipalIdentity,
        Capability,
        Operation,
        Input,
    >(
        &self,
        approved: &WorthQueryApprovedElevation,
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
        input: Input,
        request: &WorthQueryRequestScope,
    ) -> Result<
        WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
        WorthQueryOperationAuthorizationDenial,
    >
    where
        Operation: 'static,
        Input: ApplicationCapabilityRequest<Schema, Capability>
            + worth_query_declaration::facade::portable_identity::WorthQueryPortableType
            + 'static,
    {
        self.admit_capability_access_inner(principal, capability, input, request, Some(approved))
    }

    fn admit_capability_access_inner<Principal, PrincipalIdentity, Capability, Operation, Input>(
        &self,
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
        input: Input,
        request: &WorthQueryRequestScope,
        approved: Option<&WorthQueryApprovedElevation>,
    ) -> Result<
        WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
        WorthQueryOperationAuthorizationDenial,
    >
    where
        Operation: 'static,
        Input: ApplicationCapabilityRequest<Schema, Capability>
            + worth_query_declaration::facade::portable_identity::WorthQueryPortableType
            + 'static,
    {
        admit_request(request, capability.contract().operation())?;
        let prepared =
            prepare_capability_admission(self, principal, capability, input, request, approved)?;
        complete_capability_admission(prepared)
    }
}
