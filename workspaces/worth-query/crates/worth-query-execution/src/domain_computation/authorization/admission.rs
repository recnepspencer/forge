use super::bridge_observation::lower_bridge_observation;
use super::{
    WorthQueryAdmittedApplicationOperation, WorthQueryAuthorizationDecisionFact,
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
    WorthQueryPrincipalCurrentnessDependency, WorthQueryRetainedAuthorizationDecisionFacts,
};
use crate::domain_computation::primary_graph::{
    validate_freshness_at_snapshot, WorthQueryApplicationEntityIdentity,
    WorthQueryAuthenticatedPrincipal, WorthQueryPrimaryGraphApplicationRuntime,
    WorthQueryPrincipalResolutionMode,
};
use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;
use worth_query_declaration::facade::application_schema::TypedMutationPreconditions;
use worth_query_installation::facade::{
    ApplicationSchema, ApplicationSchemaBindingIdentity, TypedApplicationValue,
    WorthQueryInstalledAbilityRequirement, WorthQueryInstalledApplicationOperation,
    WorthQueryInstalledApplicationOperationAuthorization,
};
use worth_relational::facade::authorization::RelationalAuthorizationObservationPlan;

mod validation;
pub(super) use validation::{admit_request, operation_scope_binding, validate_static_authority};
use validation::{denial, validate_decision};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn authorize_operation<Principal, PrincipalIdentity, Operation, Input, Scope>(
        &self,
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        scope_identity: &WorthQueryApplicationEntityIdentity<Schema, Scope>,
        operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
        preconditions: TypedMutationPreconditions<Schema, Operation, Scope>,
        request: &WorthQueryRequestScope,
    ) -> Result<
        WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
        WorthQueryOperationAuthorizationDenial,
    > {
        super::operation_progression::progress_conventional_operation(
            self,
            principal,
            scope_identity,
            operation,
            preconditions,
            request,
        )
    }

    pub(in crate::domain_computation::authorization) fn observe_operation_authorization<
        Principal,
        PrincipalIdentity,
        Operation,
        Input,
        Scope,
    >(
        &self,
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        scope_identity: &WorthQueryApplicationEntityIdentity<Schema, Scope>,
        operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
        graph_work: &crate::domain_computation::provider_session::WorthQueryManagedGraphWorkSession,
    ) -> Result<WorthQueryRetainedAuthorizationDecisionFacts, WorthQueryOperationAuthorizationDenial>
    {
        let graph = self.runtime.primary_graph().ok_or_else(|| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
                operation.operation(),
            )
        })?;
        let session_identity = graph_work.identity();
        let principal_layout = graph
            .layout()
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
        let principal_currentness = WorthQueryPrincipalCurrentnessDependency::capture(
            session_identity,
            principal,
            &principal_layout,
        );
        let snapshot = graph_work
            .mutation_snapshot()
            .expect("a mutation session owns its admitted snapshot")
            .clone();
        let handle = graph_work
            .mutation_handle()
            .expect("a mutation session owns its graph handle")
            .clone();
        let entity_resolution = graph.retain_entity_resolution_context();
        let result = handle.with_runtime_mut(|runtime| {
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
            entity_resolution
                .at_snapshot(
                    runtime,
                    &snapshot,
                    WorthQueryPrincipalResolutionMode::Ordinary,
                )
                .and_then(|truth| truth.validate_entity_freshness(scope_identity))
                .map_err(|_| {
                    denial(
                        WorthQueryOperationAuthorizationDenialKind::StaleScope,
                        scope_identity.entity_name(),
                    )
                })?;
            self.observe_authorization_requirements(
                session_identity,
                runtime,
                snapshot,
                principal,
                scope_identity,
                operation.binding_identity(),
                operation.contracts().ability_requirements(),
            )
        })?;
        let authorization = match operation.contracts().authorization() {
            WorthQueryInstalledApplicationOperationAuthorization::Principal
                if result.is_empty() =>
            {
                WorthQueryRetainedAuthorizationDecisionFacts::principal(principal_currentness)
            }
            WorthQueryInstalledApplicationOperationAuthorization::Abilities => {
                WorthQueryRetainedAuthorizationDecisionFacts::abilities(
                    principal_currentness,
                    result,
                )
            }
            WorthQueryInstalledApplicationOperationAuthorization::Principal
            | WorthQueryInstalledApplicationOperationAuthorization::Capability => {
                return Err(denial(
                    WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                    operation.operation(),
                ));
            }
        };
        if !authorization.belongs_to_session(session_identity) {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                operation.operation(),
            ));
        }
        Ok(authorization)
    }

    pub(in crate::domain_computation) fn observe_authorization_requirements<
        Principal,
        PrincipalIdentity,
        Scope,
    >(
        &self,
        session_identity: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
        relational: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: worth_relational::facade::snapshots::SnapshotHandle,
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        scope_identity: &WorthQueryApplicationEntityIdentity<Schema, Scope>,
        binding_identity: &ApplicationSchemaBindingIdentity,
        requirements: &[WorthQueryInstalledAbilityRequirement],
    ) -> Result<Vec<WorthQueryAuthorizationDecisionFact>, WorthQueryOperationAuthorizationDenial>
    {
        let mut admitted = Vec::with_capacity(requirements.len());
        for requirement in requirements {
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
                &super::bridge_authorization_binding_identity(binding_identity),
                requirement.ability(),
                requirement.scope_entity(),
                requirement.policy(),
                &installed.bridge_rules,
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
            let evidence = relational.observe_authorization(plan).map_err(|_| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::RelationalObservationRejected,
                    requirement.policy(),
                )
            })?;
            let dependency_identity = *evidence.observation_identity().bytes();
            let bridge_observation = lower_bridge_observation(
                installed,
                &evidence,
                dependency_identity,
                requirement.policy(),
            )?;
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
            admitted.push(WorthQueryAuthorizationDecisionFact::new(
                session_identity,
                evidence,
                bridge,
            ));
        }
        Ok(admitted)
    }
}
