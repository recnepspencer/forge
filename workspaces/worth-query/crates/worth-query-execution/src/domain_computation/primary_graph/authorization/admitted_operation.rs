use std::marker::PhantomData;
use std::time::Instant;

use worth_query_admission::facade::authenticated_principal::{
    WorthQueryRequestInterruption, WorthQueryRequestScope,
};
use worth_query_installation::facade::{
    ApplicationSchemaBindingIdentity, WorthQueryCompiledApplicationOperationContracts,
};
use worth_relational::facade::authorization::{
    RelationalAuthorizationObservationCounters, RelationalAuthorizationObservationEvidence,
};
use worth_runtime_bridge::facade::BridgeAuthorizationDecisionEvidence;

use super::{WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind};

static NEXT_OPERATION_ADMISSION_IDENTITY: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(1);

#[derive(Clone, Copy, Eq, PartialEq)]
pub(in crate::domain_computation::primary_graph) struct WorthQueryOperationAdmissionIdentity(u64);

impl WorthQueryOperationAdmissionIdentity {
    fn mint() -> Self {
        Self(NEXT_OPERATION_ADMISSION_IDENTITY.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

pub(in crate::domain_computation::primary_graph) struct WorthQueryAuthorizationCommitDependency {
    pub(in crate::domain_computation::primary_graph) relational:
        RelationalAuthorizationObservationEvidence,
    pub(in crate::domain_computation::primary_graph) bridge: BridgeAuthorizationDecisionEvidence,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WorthQueryOperationScopeFingerprint([u8; 32]);

impl WorthQueryOperationScopeFingerprint {
    pub(super) const fn mint(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Descriptive canonical bytes. Possessing them grants no Query authority.
    pub const fn bytes(&self) -> &[u8; 32] {
        &self.0
    }
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
    operation_authority_identity: String,
    admission_identity: WorthQueryOperationAdmissionIdentity,
    operation_scope_fingerprint: WorthQueryOperationScopeFingerprint,
    scope_entity_id: worth_relational::facade::identity::EntityId,
    scope_entity_kind: worth_relational::facade::identity::KindId,
    scope_entity_name: String,
    authentication_valid_until: Instant,
    request_scope: WorthQueryRequestScope,
    contracts: WorthQueryCompiledApplicationOperationContracts,
    requirements: Vec<WorthQueryAuthorizationCommitDependency>,
    _marker: PhantomData<fn(Input) -> (Schema, Operation, Scope)>,
}

impl<Schema, Operation, Input, Scope>
    WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>
{
    pub(super) fn mint(
        runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
        binding_identity: ApplicationSchemaBindingIdentity,
        operation: String,
        operation_authority_identity: String,
        operation_scope_fingerprint: WorthQueryOperationScopeFingerprint,
        scope_entity_id: worth_relational::facade::identity::EntityId,
        scope_entity_kind: worth_relational::facade::identity::KindId,
        scope_entity_name: String,
        authentication_valid_until: Instant,
        request_scope: WorthQueryRequestScope,
        contracts: WorthQueryCompiledApplicationOperationContracts,
        requirements: Vec<WorthQueryAuthorizationCommitDependency>,
    ) -> Self {
        Self {
            runtime_authority,
            binding_identity,
            operation,
            operation_authority_identity,
            admission_identity: WorthQueryOperationAdmissionIdentity::mint(),
            operation_scope_fingerprint,
            scope_entity_id,
            scope_entity_kind,
            scope_entity_name,
            authentication_valid_until,
            request_scope,
            contracts,
            requirements,
            _marker: PhantomData,
        }
    }

    pub fn binding_identity(&self) -> &ApplicationSchemaBindingIdentity {
        &self.binding_identity
    }

    pub(in crate::domain_computation::primary_graph) fn belongs_to(
        &self,
        runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
        binding_identity: &ApplicationSchemaBindingIdentity,
    ) -> bool {
        self.runtime_authority == runtime_authority && &self.binding_identity == binding_identity
    }

    pub(in crate::domain_computation::primary_graph) fn validate_projection_authority(
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

    pub(in crate::domain_computation::primary_graph) const fn runtime_authority(
        &self,
    ) -> crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity {
        self.runtime_authority
    }

    pub(in crate::domain_computation::primary_graph) const fn admission_identity(
        &self,
    ) -> WorthQueryOperationAdmissionIdentity {
        self.admission_identity
    }

    pub(in crate::domain_computation::primary_graph) const fn scope_entity_id(
        &self,
    ) -> worth_relational::facade::identity::EntityId {
        self.scope_entity_id
    }

    pub(in crate::domain_computation::primary_graph) const fn scope_entity_kind(
        &self,
    ) -> worth_relational::facade::identity::KindId {
        self.scope_entity_kind
    }

    pub(in crate::domain_computation::primary_graph) fn scope_entity_name(&self) -> &str {
        &self.scope_entity_name
    }

    pub fn operation(&self) -> &str {
        &self.operation
    }

    pub fn allowed_graph_contract(&self) -> &WorthQueryCompiledApplicationOperationContracts {
        &self.contracts
    }

    pub fn authorization_requirement_count(&self) -> usize {
        self.requirements.len()
    }

    pub fn relational_counters(&self) -> RelationalAuthorizationObservationCounters {
        self.requirements.iter().fold(
            RelationalAuthorizationObservationCounters::default(),
            |mut total, requirement| {
                let counters = requirement.relational.counters();
                total.paths_evaluated += counters.paths_evaluated;
                total.adjacency_lists_read += counters.adjacency_lists_read;
                total.adjacency_edges_inspected += counters.adjacency_edges_inspected;
                total.relation_records_inspected += counters.relation_records_inspected;
                total.entity_records_inspected += counters.entity_records_inspected;
                total.predicate_fields_inspected += counters.predicate_fields_inspected;
                total.maximum_frontier_width = total
                    .maximum_frontier_width
                    .max(counters.maximum_frontier_width);
                total.reconstructive_graph_scans += counters.reconstructive_graph_scans;
                total.reconstructive_relation_records_scanned +=
                    counters.reconstructive_relation_records_scanned;
                total
            },
        )
    }

    pub fn signal_dependency_count(&self) -> usize {
        self.requirements
            .iter()
            .map(|requirement| {
                let counters = requirement.bridge.counters();
                counters.entities_depended_on
                    + counters.relations_depended_on
                    + counters.adjacency_lists_depended_on
                    + counters.fields_depended_on
            })
            .sum()
    }

    pub(in crate::domain_computation::primary_graph) fn take_authorization_dependencies(
        &mut self,
        bridge: &worth_runtime_bridge::facade::BridgeAuthorizationRuntime,
    ) -> Result<Vec<WorthQueryAuthorizationCommitDependency>, WorthQueryOperationAuthorizationDenial>
    {
        if self.requirements.iter().any(|requirement| {
            !bridge.retains(&requirement.bridge)
                || requirement.bridge.dependency_identity()
                    != requirement.relational.observation_identity().bytes()
                || !requirement.bridge.is_allowed()
        }) {
            return Err(WorthQueryOperationAuthorizationDenial::new(
                WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                &self.operation,
            ));
        }
        Ok(std::mem::take(&mut self.requirements))
    }

    /// Descriptive fingerprint of the exact installed operation authority
    /// retained by this proof. The string does not itself grant authority.
    pub fn installed_operation_fingerprint(&self) -> &str {
        &self.operation_authority_identity
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
    pub const fn operation_scope_fingerprint(&self) -> WorthQueryOperationScopeFingerprint {
        self.operation_scope_fingerprint
    }
}
