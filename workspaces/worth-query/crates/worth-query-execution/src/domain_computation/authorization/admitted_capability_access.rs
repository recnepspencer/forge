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

use super::capability_request_resolution::WorthQueryResolvedCapabilityRequest;
use super::WorthQueryRetainedCapabilityAuthorization;

/// Move-only Query authority proving that one exact capability request was
/// admitted from current graph truth.
///
/// This proof opens no execution surface. A separately installed operation
/// must consume it before an application attempt can begin.
///
/// Authentication is not admitted capability access:
///
/// ```compile_fail
/// use worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest;
/// use worth_query_execution::facade::primary_graph::{
///     WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryAuthenticatedPrincipal,
/// };
///
/// fn cannot_substitute_authentication<Schema, Capability, Operation, Input, Principal, Identity>(
///     principal: WorthQueryAuthenticatedPrincipal<Schema, Principal, Identity>,
/// ) where
///     Input: ApplicationCapabilityRequest<Schema, Capability>,
/// {
///     let _: WorthQueryAdmittedApplicationCapabilityAccess<
///         Schema, Capability, Operation, Input,
///     > = principal;
/// }
/// ```
///
/// A descriptive capability reference is not admitted capability access:
///
/// ```compile_fail
/// use worth_query_declaration::facade::application_capability::{
///     ApplicationCapabilityRef, ApplicationCapabilityRequest,
/// };
/// use worth_query_execution::facade::primary_graph::WorthQueryAdmittedApplicationCapabilityAccess;
///
/// fn cannot_substitute_description<Schema, Capability, Operation, Input>(
///     capability: ApplicationCapabilityRef<Schema, Capability>,
/// ) where
///     Input: ApplicationCapabilityRequest<Schema, Capability>,
/// {
///     let _: WorthQueryAdmittedApplicationCapabilityAccess<
///         Schema, Capability, Operation, Input,
///     > = capability;
/// }
/// ```
///
/// The access proof is move-only:
///
/// ```compile_fail
/// use worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest;
/// use worth_query_execution::facade::primary_graph::WorthQueryAdmittedApplicationCapabilityAccess;
///
/// fn cannot_clone_access<Schema, Capability, Operation, Input>(
///     access: WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
/// ) where
///     Input: ApplicationCapabilityRequest<Schema, Capability>,
/// {
///     let _copied = access.clone();
/// }
/// ```
///
/// The operation marker is exact and cannot be repurposed:
///
/// ```compile_fail
/// use worth_query_declaration::facade::{
///     application_capability::ApplicationCapabilityRequest,
///     application_schema::TypedMutationPreconditions,
/// };
/// use worth_query_execution::facade::primary_graph::{
///     WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryPrimaryGraphApplicationRuntime,
/// };
/// use worth_query_installation::facade::{
///     ApplicationSchema, WorthQueryInstalledApplicationOperation,
/// };
///
/// fn cannot_change_operation<Schema, Capability, Granted, Other, Input>(
///     runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
///     access: WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Granted, Input>,
///     other: &WorthQueryInstalledApplicationOperation<Schema, Other, Input>,
/// ) where
///     Schema: ApplicationSchema,
///     Input: ApplicationCapabilityRequest<Schema, Capability>,
/// {
///     runtime.authorize_capability_operation(
///         access,
///         other,
///         TypedMutationPreconditions::new(),
///     );
/// }
/// ```
pub struct WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    pub(super) runtime_authority:
        crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    pub(super) binding_identity: ApplicationSchemaBindingIdentity,
    pub(super) capability: Arc<str>,
    pub(super) capability_type: Arc<str>,
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
    pub(super) canonical_work: WorthQueryCanonicalWorkEvidence,
    pub(super) authorization: WorthQueryRetainedCapabilityAuthorization,
    pub(super) operation_admission_identity: super::WorthQueryOperationAdmissionIdentity,
    pub(super) graph_work:
        crate::domain_computation::provider_session::WorthQueryManagedGraphWorkSession,
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
        capability: impl Into<Arc<str>>,
        capability_type: impl Into<Arc<str>>,
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
        canonical_work: WorthQueryCanonicalWorkEvidence,
        authorization: WorthQueryRetainedCapabilityAuthorization,
        operation_admission_identity: super::WorthQueryOperationAdmissionIdentity,
        graph_work: crate::domain_computation::provider_session::WorthQueryManagedGraphWorkSession,
    ) -> Self {
        Self {
            runtime_authority,
            binding_identity,
            capability: capability.into(),
            capability_type: capability_type.into(),
            operation: operation.into(),
            principal_entity_id,
            input,
            projection,
            resolved,
            authentication_valid_until,
            request_scope,
            canonical_work,
            authorization,
            operation_admission_identity,
            graph_work,
            _marker: PhantomData,
        }
    }

    pub fn graph_work_session_identity(
        &self,
    ) -> crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity {
        self.graph_work.identity()
    }

    pub fn graph_work_managed_run_identity(
        &self,
    ) -> crate::domain_computation::provider_session::WorthQueryGraphWorkManagedRunIdentity {
        self.graph_work.managed_run_identity()
    }

    pub fn graph_work_branch(&self) -> &worth_relational::facade::history::BranchId {
        self.graph_work.branch().relational()
    }

    pub fn operation(&self) -> &str {
        &self.operation
    }

    pub(in crate::domain_computation) fn capability_name(&self) -> &str {
        &self.capability
    }

    pub(in crate::domain_computation) fn capability_type(&self) -> &str {
        &self.capability_type
    }

    pub(in crate::domain_computation) const fn runtime_authority(
        &self,
    ) -> crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity {
        self.runtime_authority
    }

    pub(in crate::domain_computation) const fn binding_identity(
        &self,
    ) -> &ApplicationSchemaBindingIdentity {
        &self.binding_identity
    }

    pub(in crate::domain_computation) const fn principal_entity_id(
        &self,
    ) -> worth_relational::facade::identity::EntityId {
        self.principal_entity_id
    }

    pub(in crate::domain_computation) fn resource_entity_id(
        &self,
    ) -> worth_relational::facade::identity::EntityId {
        self.resolved.resource.entity_id()
    }

    pub(in crate::domain_computation) fn disclosure_value(
        &self,
    ) -> Option<&worth_foundational::facade::AspectValue> {
        self.projection.field_value()
    }

    pub(in crate::domain_computation) const fn authentication_valid_until(&self) -> Instant {
        self.authentication_valid_until
    }

    pub(in crate::domain_computation) const fn request_scope(&self) -> &WorthQueryRequestScope {
        &self.request_scope
    }

    pub(in crate::domain_computation) fn into_query_authorization(
        self,
    ) -> WorthQueryRetainedCapabilityAuthorization {
        self.authorization
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
        self.authorization.capability_authority_identity()
    }

    pub const fn capability_time_timeline(&self) -> ApplicationCapabilityValidityTimeline {
        self.authorization.timeline()
    }

    pub const fn capability_time_sample(&self) -> &worth_foundational::facade::AspectValue {
        self.authorization.sampled_value()
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
        self.canonical_work
    }
}
