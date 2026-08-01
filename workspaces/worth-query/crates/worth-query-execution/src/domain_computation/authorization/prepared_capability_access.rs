//! Non-observing preparation of one typed capability request.

use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Instant;

use worth_foundational::facade::CanonicalDigestId;
use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;
use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityRequest, ApplicationCapabilityRequestProjection,
};
use worth_query_installation::facade::{
    ApplicationSchemaBindingIdentity, WorthQueryCanonicalWorkEvidence,
};

/// Move-only, non-observing input for capability authorization inside a
/// consuming read or mutation session.
///
/// Preparation validates installed/static meaning and projects the typed
/// request. It performs no graph read, policy evaluation, or authorization
/// decision and therefore opens no execution surface.
///
/// ```compile_fail
/// use worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest;
/// use worth_query_execution::facade::primary_graph::WorthQueryPreparedApplicationCapabilityAccess;
///
/// fn cannot_clone<Schema, Capability, Operation, Input>(
///     prepared: WorthQueryPreparedApplicationCapabilityAccess<
///         Schema, Capability, Operation, Input,
///     >,
/// ) where
///     Input: ApplicationCapabilityRequest<Schema, Capability>,
/// {
///     let _copy = prepared.clone();
/// }
/// ```
pub struct WorthQueryPreparedApplicationCapabilityAccess<Schema, Capability, Operation, Input>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    pub(in crate::domain_computation) runtime_authority:
        crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    pub(in crate::domain_computation) binding_identity: ApplicationSchemaBindingIdentity,
    pub(in crate::domain_computation) capability_identity: CanonicalDigestId,
    pub(in crate::domain_computation) capability_authority_identity: Arc<str>,
    pub(in crate::domain_computation) capability: Arc<str>,
    pub(in crate::domain_computation) capability_type: Arc<str>,
    pub(in crate::domain_computation) operation: Arc<str>,
    pub(in crate::domain_computation) principal_entity_id:
        worth_relational::facade::identity::EntityId,
    pub(in crate::domain_computation) principal_binding: Arc<str>,
    pub(in crate::domain_computation) principal_freshness:
        crate::domain_computation::primary_graph::WorthQueryPrincipalFreshnessEvidence,
    pub(in crate::domain_computation) input: Input,
    pub(in crate::domain_computation) projection: ApplicationCapabilityRequestProjection<
        Schema,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Context,
    >,
    pub(in crate::domain_computation) authentication_valid_until: Instant,
    pub(in crate::domain_computation) request_scope: WorthQueryRequestScope,
    pub(in crate::domain_computation) canonical_work: WorthQueryCanonicalWorkEvidence,
    _marker: PhantomData<fn() -> (Capability, Operation)>,
}

impl<Schema, Capability, Operation, Input>
    WorthQueryPreparedApplicationCapabilityAccess<Schema, Capability, Operation, Input>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    #[allow(clippy::too_many_arguments)]
    pub(super) fn mint(
        runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
        binding_identity: ApplicationSchemaBindingIdentity,
        capability_identity: CanonicalDigestId,
        capability_authority_identity: impl Into<Arc<str>>,
        capability: impl Into<Arc<str>>,
        capability_type: impl Into<Arc<str>>,
        operation: impl Into<Arc<str>>,
        principal_entity_id: worth_relational::facade::identity::EntityId,
        principal_binding: impl Into<Arc<str>>,
        principal_freshness: crate::domain_computation::primary_graph::WorthQueryPrincipalFreshnessEvidence,
        input: Input,
        projection: ApplicationCapabilityRequestProjection<
            Schema,
            <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
            <Input as ApplicationCapabilityRequest<Schema, Capability>>::Context,
        >,
        authentication_valid_until: Instant,
        request_scope: WorthQueryRequestScope,
        canonical_work: WorthQueryCanonicalWorkEvidence,
    ) -> Self {
        Self {
            runtime_authority,
            binding_identity,
            capability_identity,
            capability_authority_identity: capability_authority_identity.into(),
            capability: capability.into(),
            capability_type: capability_type.into(),
            operation: operation.into(),
            principal_entity_id,
            principal_binding: principal_binding.into(),
            principal_freshness,
            input,
            projection,
            authentication_valid_until,
            request_scope,
            canonical_work,
            _marker: PhantomData,
        }
    }

    pub fn operation(&self) -> &str {
        &self.operation
    }

    pub const fn projected_request(
        &self,
    ) -> &ApplicationCapabilityRequestProjection<
        Schema,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Context,
    > {
        &self.projection
    }

    pub const fn admission_canonical_work(&self) -> WorthQueryCanonicalWorkEvidence {
        self.canonical_work
    }
}
