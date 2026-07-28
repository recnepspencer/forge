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

pub(super) struct WorthQueryAuthorizationRequirementEvidence {
    pub(super) relational: RelationalAuthorizationObservationEvidence,
    pub(super) bridge: BridgeAuthorizationDecisionEvidence,
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
    binding_identity: ApplicationSchemaBindingIdentity,
    operation: String,
    operation_authority_identity: String,
    operation_scope_fingerprint: WorthQueryOperationScopeFingerprint,
    authentication_valid_until: Instant,
    request_scope: WorthQueryRequestScope,
    contracts: WorthQueryCompiledApplicationOperationContracts,
    requirements: Vec<WorthQueryAuthorizationRequirementEvidence>,
    _marker: PhantomData<fn(Input) -> (Schema, Operation, Scope)>,
}

impl<Schema, Operation, Input, Scope>
    WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>
{
    pub(super) fn mint(
        binding_identity: ApplicationSchemaBindingIdentity,
        operation: String,
        operation_authority_identity: String,
        operation_scope_fingerprint: WorthQueryOperationScopeFingerprint,
        authentication_valid_until: Instant,
        request_scope: WorthQueryRequestScope,
        contracts: WorthQueryCompiledApplicationOperationContracts,
        requirements: Vec<WorthQueryAuthorizationRequirementEvidence>,
    ) -> Self {
        Self {
            binding_identity,
            operation,
            operation_authority_identity,
            operation_scope_fingerprint,
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
