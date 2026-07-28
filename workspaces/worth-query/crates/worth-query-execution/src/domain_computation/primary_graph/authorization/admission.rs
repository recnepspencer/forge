use sha2::{Digest, Sha256};
use worth_query_admission::facade::authenticated_principal::{
    WorthQueryRequestInterruption, WorthQueryRequestScope,
};
use worth_query_installation::facade::{
    ApplicationSchema, TypedApplicationValue, WorthQueryInstalledApplicationOperation,
};
use worth_relational::facade::authorization::{
    RelationalAuthorizationDecision, RelationalAuthorizationObservationPlan,
};
use worth_runtime_bridge::facade::{
    BridgeAuthorizationDependencyCardinality, BridgeAuthorizationObservation,
    BridgeAuthorizationPathObservation,
};

use super::super::entity_resolution::validate_entity_freshness_at_snapshot;
use super::super::resolution::validate_freshness_at_snapshot;
use super::super::{
    WorthQueryApplicationEntityIdentity, WorthQueryAuthenticatedPrincipal,
    WorthQueryPrimaryGraphApplicationRuntime,
};
use super::admitted_operation::WorthQueryAuthorizationRequirementEvidence;
use super::{
    WorthQueryAdmittedApplicationOperation, WorthQueryOperationAuthorizationDenial,
    WorthQueryOperationAuthorizationDenialKind, WorthQueryOperationScopeFingerprint,
};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn authorize_operation<Principal, PrincipalIdentity, Operation, Input, Scope>(
        &self,
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        scope_identity: &WorthQueryApplicationEntityIdentity<Schema, Scope>,
        operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
        request: &WorthQueryRequestScope,
    ) -> Result<
        WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
        WorthQueryOperationAuthorizationDenial,
    > {
        admit_request(request, operation.operation())?;
        validate_static_authority(self, principal, scope_identity, operation)?;
        let graph = self.runtime.primary_graph().ok_or_else(|| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
                operation.operation(),
            )
        })?;
        let principal_layout = graph
            .layout
            .principal_binding(principal.binding())
            .cloned()
            .ok_or_else(|| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::StalePrincipal,
                    principal.binding(),
                )
            })?;
        let expected_external_identity = principal
            .external_identity()
            .clone()
            .into_foundational_value();
        let result = graph.integration_handle().with_runtime_mut(|runtime| {
            let snapshot = runtime.snapshots().snapshot();
            let result = (|| {
                validate_freshness_at_snapshot(
                    runtime,
                    &snapshot,
                    principal,
                    &principal_layout,
                    &expected_external_identity,
                )
                .map_err(|_| {
                    denial(
                        WorthQueryOperationAuthorizationDenialKind::StalePrincipal,
                        principal.binding(),
                    )
                })?;
                validate_entity_freshness_at_snapshot(runtime, &snapshot, scope_identity).map_err(
                    |_| {
                        denial(
                            WorthQueryOperationAuthorizationDenialKind::StaleScope,
                            scope_identity.entity_name(),
                        )
                    },
                )?;
                self.observe_requirements(
                    runtime,
                    snapshot.clone(),
                    principal,
                    scope_identity,
                    operation,
                )
            })();
            runtime.snapshots().release_snapshot(&snapshot);
            result
        })?;
        admit_request(request, operation.operation())?;
        if principal.is_expired() {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::ExpiredAuthentication,
                principal.binding(),
            ));
        }
        Ok(WorthQueryAdmittedApplicationOperation::mint(
            operation.binding_identity().clone(),
            operation.operation().to_string(),
            operation.authority_identity().to_string(),
            operation_scope_fingerprint(self, principal, scope_identity, operation),
            principal.valid_until(),
            request.clone(),
            operation.contracts().clone(),
            result,
        ))
    }

    fn observe_requirements<Principal, PrincipalIdentity, Operation, Input, Scope>(
        &self,
        relational: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: worth_relational::facade::snapshots::SnapshotHandle,
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        scope_identity: &WorthQueryApplicationEntityIdentity<Schema, Scope>,
        operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    ) -> Result<
        Vec<WorthQueryAuthorizationRequirementEvidence>,
        WorthQueryOperationAuthorizationDenial,
    > {
        let mut admitted = Vec::with_capacity(operation.contracts().ability_requirements().len());
        for requirement in operation.contracts().ability_requirements() {
            if requirement.scope_entity() != scope_identity.entity_name() {
                return Err(denial(
                    WorthQueryOperationAuthorizationDenialKind::ScopeMismatch,
                    requirement.scope_entity(),
                ));
            }
            let installed = self.authorization.policy(requirement)?;
            if installed.scope_kind != scope_identity.entity_kind() {
                return Err(denial(
                    WorthQueryOperationAuthorizationDenialKind::ScopeMismatch,
                    requirement.scope_entity(),
                ));
            }
            if !self.authorization.bridge().matches_installed_policy(
                installed.correspondence,
                operation.binding_identity(),
                requirement.ability(),
                requirement.scope_entity(),
                requirement.policy(),
                &installed.bridge_paths,
            ) {
                return Err(denial(
                    WorthQueryOperationAuthorizationDenialKind::PolicyNotInstalled,
                    requirement.policy(),
                ));
            }
            let plan = RelationalAuthorizationObservationPlan::try_new(
                snapshot.clone(),
                principal.principal_entity_id(),
                scope_identity.entity_id(),
                installed.principal_kind,
                installed.scope_kind,
                installed.relational_paths.clone(),
                [],
            )
            .map_err(|_| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::InvalidInstalledPolicy,
                    requirement.policy(),
                )
            })?;
            let expected_plan_identity = *plan.identity().bytes();
            let evidence = relational.observe_authorization(plan).map_err(|_| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::RelationalObservationRejected,
                    requirement.policy(),
                )
            })?;
            if evidence.plan_identity().bytes() != &expected_plan_identity {
                return Err(denial(
                    WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                    requirement.policy(),
                ));
            }
            let dependency_identity = *evidence.observation_identity().bytes();
            let bridge_observation = BridgeAuthorizationObservation::new(
                installed.correspondence,
                dependency_identity,
                installed
                    .bridge_paths
                    .iter()
                    .zip(evidence.paths())
                    .map(|(contract, path)| {
                        BridgeAuthorizationPathObservation::new(
                            *contract.identity(),
                            contract.effect(),
                            path.matched(),
                            path.exhaustive(),
                            BridgeAuthorizationDependencyCardinality {
                                entities: path.entities().len(),
                                relations: path.relations().len(),
                                adjacency_lists: path.adjacency_lists().len(),
                                fields: path.fields().len(),
                            },
                        )
                    }),
            );
            let bridge = self
                .authorization
                .bridge()
                .evaluate(bridge_observation)
                .map_err(|_| {
                    denial(
                        WorthQueryOperationAuthorizationDenialKind::BridgeEvaluationRejected,
                        requirement.policy(),
                    )
                })?;
            validate_decision(
                self.authorization.bridge(),
                &evidence,
                &bridge,
                dependency_identity,
                requirement.policy(),
            )?;
            admitted.push(WorthQueryAuthorizationRequirementEvidence {
                relational: evidence,
                bridge,
            });
        }
        Ok(admitted)
    }
}

fn operation_scope_fingerprint<Schema, Principal, PrincipalIdentity, Operation, Input, Scope>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    scope: &WorthQueryApplicationEntityIdentity<Schema, Scope>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
) -> WorthQueryOperationScopeFingerprint {
    let mut hash = Sha256::new();
    hash.update(b"worth-query.operation-scope.v1");
    hash.update(runtime.runtime.authority_identity().as_u64().to_le_bytes());
    for value in [
        operation.binding_identity().package_identity(),
        operation.binding_identity().schema_identity().as_str(),
        operation.authority_identity(),
    ] {
        hash.update(value.len().to_le_bytes());
        hash.update(value.as_bytes());
    }
    hash_entity(&mut hash, principal.principal_entity_id());
    hash_entity(&mut hash, scope.entity_id());
    WorthQueryOperationScopeFingerprint::mint(hash.finalize().into())
}

fn hash_entity(hash: &mut Sha256, entity: worth_relational::facade::identity::EntityId) {
    hash.update(entity.partition_value().to_le_bytes());
    hash.update(entity.local_slot_value().to_le_bytes());
    hash.update(entity.generation_value().to_le_bytes());
}

fn validate_static_authority<Schema, Principal, PrincipalIdentity, Operation, Input, Scope>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    scope: &WorthQueryApplicationEntityIdentity<Schema, Scope>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
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
    let authority = runtime.runtime.authority_identity();
    if principal.runtime_authority() != authority || scope.runtime_authority() != authority {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
            operation.operation(),
        ));
    }
    if principal.binding_identity() != operation.binding_identity()
        || scope.binding_identity() != operation.binding_identity()
        || runtime.installed_schema.binding_identity() != *operation.binding_identity()
    {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::StaleInstalledSchema,
            operation.operation(),
        ));
    }
    runtime
        .runtime
        .installed_packages()
        .validate_application_operation(operation)
        .map_err(|_| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation,
                operation.operation(),
            )
        })
}

fn validate_decision(
    bridge_runtime: &worth_runtime_bridge::facade::BridgeAuthorizationRuntime,
    relational: &worth_relational::facade::authorization::RelationalAuthorizationObservationEvidence,
    bridge: &worth_runtime_bridge::facade::BridgeAuthorizationDecisionEvidence,
    dependency_identity: [u8; 32],
    policy: &str,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    if relational.observation_identity().bytes() != &dependency_identity
        || bridge.dependency_identity() != &dependency_identity
        || !bridge_runtime.retains(bridge)
    {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
            policy,
        ));
    }
    match (relational.decision(), bridge.is_allowed()) {
        (RelationalAuthorizationDecision::Allowed, true) => Ok(()),
        (RelationalAuthorizationDecision::Denied, false) => Err(denial(
            WorthQueryOperationAuthorizationDenialKind::PermissionDenied,
            policy,
        )),
        _ => Err(denial(
            WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
            policy,
        )),
    }
}

fn admit_request(
    scope: &WorthQueryRequestScope,
    subject: &str,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    match scope.interruption() {
        Some(WorthQueryRequestInterruption::Cancelled) => Err(denial(
            WorthQueryOperationAuthorizationDenialKind::Cancelled,
            subject,
        )),
        Some(WorthQueryRequestInterruption::DeadlineExceeded) => Err(denial(
            WorthQueryOperationAuthorizationDenialKind::DeadlineExceeded,
            subject,
        )),
        None => Ok(()),
    }
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
