//! Move-only installed operation admission authority.

use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

use worth_query_admission::facade::authenticated_principal::{
    WorthQueryRequestInterruption, WorthQueryRequestScope,
};
use worth_query_installation::facade::{
    ApplicationSchemaBindingIdentity, WorthQueryCanonicalWorkPhases,
    WorthQueryCompiledApplicationOperationContracts,
};
use worth_relational::facade::authorization::RelationalAuthorizationObservationCounters;

use super::{
    WorthQueryCommitAuthorizationBasis, WorthQueryOperationAuthorizationDenial,
    WorthQueryOperationAuthorizationDenialKind, WorthQueryProviderAuthorizationDecisionFacts,
    WorthQueryRetainedAuthorizationDecisionFacts, WorthQueryRetainedCapabilityAuthorization,
};
use crate::domain_computation::primary_graph::WorthQueryBoundMutationPreconditions;

mod elevation_request;
mod graph_work_inspection;

static NEXT_OPERATION_ADMISSION_IDENTITY: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Eq, PartialEq)]
pub(in crate::domain_computation) struct WorthQueryOperationAdmissionIdentity(u64);

impl WorthQueryOperationAdmissionIdentity {
    pub(super) fn mint() -> Option<Self> {
        Self::mint_from(&NEXT_OPERATION_ADMISSION_IDENTITY)
    }

    pub(super) fn resource_binding_identity(self) -> Arc<str> {
        Arc::from(format!("worth-query-application-admission:{}", self.0))
    }

    fn mint_from(counter: &AtomicU64) -> Option<Self> {
        counter
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .ok()
            .map(Self)
    }
}

use super::WorthQueryOperationScopeBinding;

pub(super) enum WorthQueryOperationAuthorizationBasis<Input> {
    Conventional,
    Capability {
        input: Input,
    },
    ElevationRequest {
        input: Input,
        binding: super::WorthQueryElevationRequestBinding,
    },
}

/// Query-owned proof that one installed operation was authorized for one exact
/// current principal and typed scope.
///
/// The proof is move-only, has private fields, and is not deserializable.
/// Descriptive identities, token claims, or decision enums cannot mint it.
///
/// ```compile_fail
/// use worth_query_execution::facade::primary_graph::WorthQueryAdmittedApplicationOperation;
///
/// let _: WorthQueryAdmittedApplicationOperation<(), (), (), ()> =
///     serde_json::from_str("{}").unwrap();
/// ```
pub struct WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope> {
    runtime_authority:
        crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    binding_identity: ApplicationSchemaBindingIdentity,
    operation: String,
    operation_authority_identity: Arc<str>,
    admission_identity: WorthQueryOperationAdmissionIdentity,
    resource_binding_identity: Arc<str>,
    operation_scope_binding: WorthQueryOperationScopeBinding,
    canonical_work: WorthQueryCanonicalWorkPhases,
    scope_entity_id: worth_relational::facade::identity::EntityId,
    scope_entity_kind: worth_relational::facade::identity::KindId,
    scope_entity_name: String,
    authentication_valid_until: Instant,
    request_scope: WorthQueryRequestScope,
    contracts: WorthQueryCompiledApplicationOperationContracts,
    mutation_preconditions: WorthQueryBoundMutationPreconditions,
    authorization: Option<WorthQueryRetainedAuthorizationDecisionFacts>,
    authorization_basis: WorthQueryOperationAuthorizationBasis<Input>,
    graph_work: crate::domain_computation::provider_session::WorthQueryManagedGraphWorkSession,
    _marker: PhantomData<fn(Input) -> (Schema, Operation, Scope)>,
}

impl<Schema, Operation, Input, Scope>
    WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>
{
    pub(super) fn mint(
        admission_identity: WorthQueryOperationAdmissionIdentity,
        runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
        binding_identity: ApplicationSchemaBindingIdentity,
        operation: String,
        operation_authority_identity: String,
        operation_scope_binding: WorthQueryOperationScopeBinding,
        scope_entity_id: worth_relational::facade::identity::EntityId,
        scope_entity_kind: worth_relational::facade::identity::KindId,
        scope_entity_name: String,
        authentication_valid_until: Instant,
        request_scope: WorthQueryRequestScope,
        contracts: WorthQueryCompiledApplicationOperationContracts,
        mutation_preconditions: WorthQueryBoundMutationPreconditions,
        authorization_admission_work: worth_query_installation::facade::WorthQueryCanonicalWorkEvidence,
        authorization: WorthQueryRetainedAuthorizationDecisionFacts,
        authorization_basis: WorthQueryOperationAuthorizationBasis<Input>,
        graph_work: crate::domain_computation::provider_session::WorthQueryManagedGraphWorkSession,
    ) -> Self {
        let resource_binding_identity = admission_identity.resource_binding_identity();
        let canonical_work = WorthQueryCanonicalWorkPhases::new(
            contracts.canonical_work(),
            mutation_preconditions
                .canonical_work()
                .combine(authorization_admission_work),
        );
        Self {
            runtime_authority,
            binding_identity,
            operation,
            operation_authority_identity: operation_authority_identity.into(),
            admission_identity,
            resource_binding_identity,
            operation_scope_binding,
            canonical_work,
            scope_entity_id,
            scope_entity_kind,
            scope_entity_name,
            authentication_valid_until,
            request_scope,
            contracts,
            mutation_preconditions,
            authorization: Some(authorization),
            authorization_basis,
            graph_work,
            _marker: PhantomData,
        }
    }

    pub fn binding_identity(&self) -> &ApplicationSchemaBindingIdentity {
        &self.binding_identity
    }

    pub(in crate::domain_computation) fn belongs_to(
        &self,
        runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
        binding_identity: &ApplicationSchemaBindingIdentity,
    ) -> bool {
        self.runtime_authority == runtime_authority && &self.binding_identity == binding_identity
    }

    pub(in crate::domain_computation) fn validate_projection_authority(
        &self,
        runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
        binding_identity: &ApplicationSchemaBindingIdentity,
    ) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        if self.runtime_authority != runtime_authority {
            return Err(WorthQueryOperationAuthorizationDenial::new(
                WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
                &self.operation,
            ));
        }
        if &self.binding_identity != binding_identity {
            return Err(WorthQueryOperationAuthorizationDenial::new(
                WorthQueryOperationAuthorizationDenialKind::StaleInstalledSchema,
                &self.operation,
            ));
        }
        self.validate_current_authority()
    }

    pub(in crate::domain_computation) const fn runtime_authority(
        &self,
    ) -> crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity {
        self.runtime_authority
    }

    pub(in crate::domain_computation) const fn admission_identity(
        &self,
    ) -> WorthQueryOperationAdmissionIdentity {
        self.admission_identity
    }

    pub(in crate::domain_computation) const fn scope_entity_id(
        &self,
    ) -> worth_relational::facade::identity::EntityId {
        self.scope_entity_id
    }

    pub(in crate::domain_computation) const fn scope_entity_kind(
        &self,
    ) -> worth_relational::facade::identity::KindId {
        self.scope_entity_kind
    }

    pub(in crate::domain_computation) fn scope_entity_name(&self) -> &str {
        &self.scope_entity_name
    }

    pub fn operation(&self) -> &str {
        &self.operation
    }

    pub fn allowed_graph_contract(&self) -> &WorthQueryCompiledApplicationOperationContracts {
        &self.contracts
    }

    pub(in crate::domain_computation) const fn mutation_preconditions(
        &self,
    ) -> &WorthQueryBoundMutationPreconditions {
        &self.mutation_preconditions
    }

    pub const fn canonical_work(&self) -> WorthQueryCanonicalWorkPhases {
        self.canonical_work
    }

    pub fn authorization_requirement_count(&self) -> usize {
        self.authorization.as_ref().map_or(
            0,
            WorthQueryRetainedAuthorizationDecisionFacts::policy_count,
        )
    }

    pub fn authorization_decision_fact_count(&self) -> usize {
        self.authorization.as_ref().map_or(
            0,
            WorthQueryRetainedAuthorizationDecisionFacts::exact_fact_count,
        )
    }

    pub const fn capability_input(&self) -> Option<&Input> {
        match &self.authorization_basis {
            WorthQueryOperationAuthorizationBasis::Conventional => None,
            WorthQueryOperationAuthorizationBasis::Capability { input }
            | WorthQueryOperationAuthorizationBasis::ElevationRequest { input, .. } => Some(input),
        }
    }

    pub fn installed_capability_authority_identity(&self) -> Option<&str> {
        self.authorization
            .as_ref()?
            .capability_authorization()
            .map(WorthQueryRetainedCapabilityAuthorization::capability_authority_identity)
    }

    pub fn capability_time_timeline(
        &self,
    ) -> Option<
        worth_query_declaration::facade::application_capability::ApplicationCapabilityValidityTimeline,
    >{
        self.authorization
            .as_ref()?
            .capability_authorization()
            .map(WorthQueryRetainedCapabilityAuthorization::timeline)
    }

    pub fn capability_time_sample(&self) -> Option<&worth_foundational::facade::AspectValue> {
        self.authorization
            .as_ref()?
            .capability_authorization()
            .map(WorthQueryRetainedCapabilityAuthorization::sampled_value)
    }

    pub fn relational_counters(&self) -> RelationalAuthorizationObservationCounters {
        self.authorization
            .as_ref()
            .map(WorthQueryRetainedAuthorizationDecisionFacts::relational_counters)
            .unwrap_or_default()
    }

    pub fn signal_dependency_count(&self) -> usize {
        self.authorization
            .as_ref()
            .map(WorthQueryRetainedAuthorizationDecisionFacts::signal_dependency_count)
            .unwrap_or(0)
    }

    pub(in crate::domain_computation) fn authorization_mut(
        &mut self,
    ) -> Option<&mut WorthQueryRetainedAuthorizationDecisionFacts> {
        self.authorization.as_mut()
    }

    pub(in crate::domain_computation) fn authorization(
        &self,
    ) -> Option<&WorthQueryRetainedAuthorizationDecisionFacts> {
        self.authorization.as_ref()
    }

    pub(in crate::domain_computation) fn take_authorization_dependencies(
        &mut self,
        bridge: &worth_runtime_bridge::facade::BridgeAuthorizationRuntime,
    ) -> Result<
        (
            WorthQueryProviderAuthorizationDecisionFacts,
            WorthQueryCommitAuthorizationBasis,
        ),
        WorthQueryOperationAuthorizationDenial,
    > {
        let authorization = self.authorization.take().ok_or_else(|| {
            WorthQueryOperationAuthorizationDenial::new(
                WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                &self.operation,
            )
        })?;
        if !authorization.bridge_is_retained(bridge) {
            return Err(WorthQueryOperationAuthorizationDenial::new(
                WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                &self.operation,
            ));
        }
        Ok(authorization.into_provider_parts(self.admission_identity))
    }

    /// Descriptive fingerprint of the exact installed operation authority
    /// retained by this proof. The string does not itself grant authority.
    pub fn installed_operation_fingerprint(&self) -> &str {
        &self.operation_authority_identity
    }

    pub(in crate::domain_computation) fn retain_installed_operation_fingerprint(&self) -> Arc<str> {
        Arc::clone(&self.operation_authority_identity)
    }

    pub(in crate::domain_computation) fn retain_resource_binding_identity(&self) -> Arc<str> {
        Arc::clone(&self.resource_binding_identity)
    }

    /// Revalidates the time and cancellation authority carried from the exact
    /// authentication request that minted this operation admission.
    ///
    /// Later governed phases must call this rather than accepting a detached
    /// timestamp or caller assertion.
    pub fn validate_current_authority(&self) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        if let Some(interruption) = self.request_scope.interruption() {
            let kind = match interruption {
                WorthQueryRequestInterruption::Cancelled => {
                    WorthQueryOperationAuthorizationDenialKind::Cancelled
                }
                WorthQueryRequestInterruption::DeadlineExceeded => {
                    WorthQueryOperationAuthorizationDenialKind::DeadlineExceeded
                }
            };
            return Err(WorthQueryOperationAuthorizationDenial::new(
                kind,
                &self.operation,
            ));
        }
        if Instant::now() >= self.authentication_valid_until {
            return Err(WorthQueryOperationAuthorizationDenial::new(
                WorthQueryOperationAuthorizationDenialKind::ExpiredAuthentication,
                &self.operation,
            ));
        }
        Ok(())
    }

    /// Stable identity of the authenticated runtime, installed operation,
    /// principal, and typed scope. It intentionally excludes snapshot identity
    /// so an equivalent authorized retry can retain one idempotency intent.
    pub const fn operation_scope_binding(&self) -> &WorthQueryOperationScopeBinding {
        &self.operation_scope_binding
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicU64;

    use super::WorthQueryOperationAdmissionIdentity;

    #[test]
    fn operation_admission_identity_exhaustion_cannot_wrap() {
        let counter = AtomicU64::new(u64::MAX);
        assert!(WorthQueryOperationAdmissionIdentity::mint_from(&counter).is_none());
        assert_eq!(counter.load(std::sync::atomic::Ordering::Relaxed), u64::MAX);
    }
}
