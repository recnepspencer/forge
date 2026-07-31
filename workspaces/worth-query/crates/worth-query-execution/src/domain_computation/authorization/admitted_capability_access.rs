//! Attempt-bound capability access authority.

use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Instant;

use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;
use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityRequest, ApplicationCapabilityRequestProjection,
    ApplicationCapabilityValidityTimeline,
};
use worth_query_installation::facade::{
    ApplicationSchemaBindingIdentity, WorthQueryCanonicalWorkEvidence,
};
use worth_relational::facade::authorization::RelationalAuthorizationObservationCounters;

use super::capability_currentness::WorthQueryCapabilityCurrentnessAuthority;
use super::capability_request_resolution::WorthQueryResolvedCapabilityRequest;
use super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::WorthQueryRetainedAuthorizationDecisionFacts;

/// Move-only Query authority proving that one exact capability request was
/// admitted from current graph truth.
///
/// This proof opens no execution surface. A separately installed operation
/// must consume it before an application attempt can begin.
pub struct WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    pub(super) runtime_authority:
        crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    pub(super) binding_identity: ApplicationSchemaBindingIdentity,
    pub(super) operation: Arc<str>,
    pub(super) principal_entity_id: worth_relational::facade::identity::EntityId,
    pub(super) input: Input,
    pub(super) projection: ApplicationCapabilityRequestProjection<
        Schema,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Context,
    >,
    pub(super) resolved: WorthQueryResolvedCapabilityRequest<
        Schema,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
    >,
    pub(super) authentication_valid_until: Instant,
    pub(super) request_scope: WorthQueryRequestScope,
    pub(super) currentness: WorthQueryCapabilityCurrentnessAuthority,
    pub(super) revalidation: WorthQueryRetainedCapabilityRequest,
    pub(super) authorization: WorthQueryRetainedAuthorizationDecisionFacts,
    _marker: PhantomData<fn() -> (Capability, Operation)>,
}

impl<Schema, Capability, Operation, Input>
    WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    #[allow(clippy::too_many_arguments)]
    pub(super) fn mint(
        runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
        binding_identity: ApplicationSchemaBindingIdentity,
        operation: impl Into<Arc<str>>,
        principal_entity_id: worth_relational::facade::identity::EntityId,
        input: Input,
        projection: ApplicationCapabilityRequestProjection<
            Schema,
            <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
            <Input as ApplicationCapabilityRequest<Schema, Capability>>::Context,
        >,
        resolved: WorthQueryResolvedCapabilityRequest<
            Schema,
            <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        >,
        authentication_valid_until: Instant,
        request_scope: WorthQueryRequestScope,
        currentness: WorthQueryCapabilityCurrentnessAuthority,
        revalidation: WorthQueryRetainedCapabilityRequest,
        authorization: WorthQueryRetainedAuthorizationDecisionFacts,
    ) -> Self {
        Self {
            runtime_authority,
            binding_identity,
            operation: operation.into(),
            principal_entity_id,
            input,
            projection,
            resolved,
            authentication_valid_until,
            request_scope,
            currentness,
            revalidation,
            authorization,
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

    pub fn installed_capability_authority_identity(&self) -> &str {
        self.currentness.capability_authority_identity()
    }

    pub const fn capability_time_timeline(&self) -> ApplicationCapabilityValidityTimeline {
        self.currentness.timeline()
    }

    pub const fn capability_time_sample(&self) -> &worth_foundational::facade::AspectValue {
        self.currentness.sampled_value()
    }

    pub fn authorization_decision_fact_count(&self) -> usize {
        self.authorization.exact_fact_count()
    }

    pub fn relational_counters(&self) -> RelationalAuthorizationObservationCounters {
        self.authorization.relational_counters()
    }

    pub fn signal_dependency_count(&self) -> usize {
        self.authorization.signal_dependency_count()
    }

    pub const fn admission_canonical_work(&self) -> WorthQueryCanonicalWorkEvidence {
        WorthQueryCanonicalWorkEvidence::zero()
    }
}
