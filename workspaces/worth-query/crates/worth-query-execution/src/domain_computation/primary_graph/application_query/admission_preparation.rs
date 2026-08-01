use worth_query_admission::facade::{
    application_query::{
        admit_application_query_parameters, WorthQueryAdmittedApplicationQueryParameters,
    },
    authenticated_principal::{WorthQueryRequestInterruption, WorthQueryRequestScope},
};
use worth_query_declaration::facade::{
    application_query::ApplicationQueryParameterSet, application_schema::ApplicationSchema,
};
use worth_query_installation::facade::{
    TypedApplicationValue, WorthQueryInstalledApplicationQuery,
    WorthQueryInstalledApplicationQueryAuthorization,
};

use super::{
    control_validation::validate_controls, WorthQueryApplicationAuthorizationWorkEvidence,
    WorthQueryApplicationQueryAccessContext, WorthQueryApplicationQueryAdmissionDenial,
    WorthQueryApplicationQueryAdmissionDenialKind, WorthQueryApplicationQueryControls,
};
use crate::domain_computation::authorization::{
    WorthQueryPrincipalCurrentnessDependency, WorthQueryRetainedAuthorizationDecisionFacts,
};
use crate::domain_computation::primary_graph::{
    entity_resolution::validate_entity_freshness_at_snapshot,
    resolution::validate_freshness_at_snapshot, WorthQueryPrimaryGraphApplicationRuntime,
};
use crate::domain_computation::provider_session::record_ability_authorization_completion;

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub(super) fn validate_application_query_access<
        Principal,
        PrincipalIdentity,
        Scope,
        Query,
        Parameters,
        QueryResult,
    >(
        &self,
        query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
        access: &WorthQueryApplicationQueryAccessContext<
            '_,
            Schema,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
    ) -> Result<(), WorthQueryApplicationQueryAdmissionDenial> {
        if access.principal().is_expired() {
            return Err(denial(
                WorthQueryApplicationQueryAdmissionDenialKind::StalePrincipal,
                access.principal().binding(),
            ));
        }
        let authority = self.runtime.authority_identity();
        if access.principal().runtime_authority() != authority {
            return Err(denial(
                WorthQueryApplicationQueryAdmissionDenialKind::ForeignPrincipal,
                access.principal().binding(),
            ));
        }
        if access.principal().binding_identity() != query.binding_identity()
            || access.principal().binding_identity() != &self.installed_schema.binding_identity()
        {
            return Err(denial(
                WorthQueryApplicationQueryAdmissionDenialKind::StalePrincipal,
                access.principal().binding(),
            ));
        }
        let scope = access.scope();
        if scope.runtime_authority() != authority {
            return Err(denial(
                WorthQueryApplicationQueryAdmissionDenialKind::ForeignScope,
                query.name(),
            ));
        }
        if scope.binding_identity() != query.binding_identity()
            || scope.binding_identity() != &self.installed_schema.binding_identity()
        {
            return Err(denial(
                WorthQueryApplicationQueryAdmissionDenialKind::StaleScope,
                query.name(),
            ));
        }
        if scope.entity_name() != query.scope_entity() {
            return Err(denial(
                WorthQueryApplicationQueryAdmissionDenialKind::ScopeTypeMismatch,
                scope.entity_name(),
            ));
        }
        Ok(())
    }

    pub(super) fn prepare_application_query_admission<'a, Query, Parameters, QueryResult, Scope>(
        &'a self,
        query: &'a WorthQueryInstalledApplicationQuery<
            Schema,
            Query,
            Parameters,
            QueryResult,
            Scope,
        >,
        parameters: ApplicationQueryParameterSet<Query>,
        controls: WorthQueryApplicationQueryControls<'a, Schema>,
    ) -> Result<
        (
            WorthQueryAdmittedApplicationQueryParameters,
            WorthQueryApplicationQueryControls<'a, Schema>,
        ),
        WorthQueryApplicationQueryAdmissionDenial,
    > {
        validate_admission_request(controls.request_scope(), query.name())?;
        self.validate_installed_query(query)?;
        validate_controls(query, &controls)?;
        let parameters =
            admit_application_query_parameters(query, parameters).map_err(|denial| {
                WorthQueryApplicationQueryAdmissionDenial::new(
                    WorthQueryApplicationQueryAdmissionDenialKind::Parameter(denial.kind()),
                    denial.parameter(),
                )
            })?;
        Ok((parameters, controls))
    }

    fn validate_installed_query<Query, Parameters, QueryResult, Scope>(
        &self,
        query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    ) -> Result<(), WorthQueryApplicationQueryAdmissionDenial> {
        self.runtime
            .installed_packages()
            .validate_application_schema(&self.installed_schema)
            .map_err(|denial| {
                WorthQueryApplicationQueryAdmissionDenial::new(
                    WorthQueryApplicationQueryAdmissionDenialKind::InstalledQuery(
                        map_schema_denial(denial.kind()),
                    ),
                    query.name(),
                )
            })?;
        self.installed_schema
            .validate_installed_query(query)
            .map_err(|denial| {
                WorthQueryApplicationQueryAdmissionDenial::new(
                    WorthQueryApplicationQueryAdmissionDenialKind::InstalledQuery(denial.kind()),
                    denial.subject(),
                )
            })
    }

    pub(super) fn validate_access_in_session<
        Principal,
        PrincipalIdentity,
        Scope,
        Query,
        Parameters,
        QueryResult,
    >(
        &self,
        query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
        access: &WorthQueryApplicationQueryAccessContext<
            '_,
            Schema,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
        request: &WorthQueryRequestScope,
        session: &mut super::WorthQueryApplicationQueryGraphWorkSession,
    ) -> Result<
        (
            WorthQueryRetainedAuthorizationDecisionFacts,
            WorthQueryApplicationAuthorizationWorkEvidence,
        ),
        WorthQueryApplicationQueryAdmissionDenial,
    > {
        validate_admission_request(request, query.name())?;
        self.validate_application_query_access(query, access)?;
        let scope = access.scope();
        let graph = self.runtime.primary_graph().ok_or_else(|| {
            denial(
                WorthQueryApplicationQueryAdmissionDenialKind::StaleScope,
                query.name(),
            )
        })?;
        let principal = access.principal();
        let principal_layout = graph
            .layout
            .principal_binding(principal.binding())
            .cloned()
            .ok_or_else(|| {
                denial(
                    WorthQueryApplicationQueryAdmissionDenialKind::StalePrincipal,
                    principal.binding(),
                )
            })?;
        let expected_external_identity = principal
            .external_identity()
            .clone()
            .into_foundational_value();
        let principal_currentness = WorthQueryPrincipalCurrentnessDependency::capture(
            *session.identity(),
            principal,
            &principal_layout,
            session.branch_affinity().relational_branch().clone(),
        );
        let branch = session.branch_affinity().relational_branch().clone();
        let handle = graph.integration_handle();
        let policy = handle.with_runtime_mut(|runtime| {
            let version = handle
                .ensure_primary_indexes_current(runtime, &branch)
                .map_err(|detail| authorization_basis_denial(query.name(), detail))?;
            let basis = runtime
                .snapshots()
                .admit_execution_basis(&branch, version)
                .map_err(|denial| authorization_basis_denial(query.name(), denial.detail()))?;
            let snapshot = basis.snapshot_handle().clone();
            let result = validate_freshness_at_snapshot(
                runtime,
                &snapshot,
                principal,
                &principal_layout,
                &expected_external_identity,
            )
            .map_err(|_| {
                denial(
                    WorthQueryApplicationQueryAdmissionDenialKind::StalePrincipal,
                    principal.binding(),
                )
            })
            .and_then(|()| {
                validate_entity_freshness_at_snapshot(runtime, &snapshot, scope).map_err(|_| {
                    denial(
                        WorthQueryApplicationQueryAdmissionDenialKind::StaleScope,
                        query.name(),
                    )
                })
            })
            .and_then(|()| {
                self.observe_query_authorization(
                    *session.identity(),
                    runtime,
                    snapshot.clone(),
                    query,
                    access,
                )
                .map_err(map_authorization_denial)
            });
            if !basis.release().released() {
                return Err(authorization_basis_denial(
                    query.name(),
                    "current authorization basis did not release",
                ));
            }
            result
        })?;
        let work = WorthQueryApplicationAuthorizationWorkEvidence::from_dependencies(&policy);
        let authorization = match query.authorization() {
            WorthQueryInstalledApplicationQueryAuthorization::Public => {
                WorthQueryRetainedAuthorizationDecisionFacts::principal(principal_currentness)
            }
            WorthQueryInstalledApplicationQueryAuthorization::Ability(_) => {
                record_ability_authorization_completion(session, &policy)
                    .map_err(|_| inconsistent_query_authorization(query.name()))?;
                WorthQueryRetainedAuthorizationDecisionFacts::abilities(
                    principal_currentness,
                    policy,
                )
            }
        };
        Ok((authorization, work))
    }
}

fn authorization_basis_denial(
    subject: &str,
    detail: &str,
) -> WorthQueryApplicationQueryAdmissionDenial {
    denial(
        WorthQueryApplicationQueryAdmissionDenialKind::RuntimeSupportUnavailable,
        format!("{subject}: {detail}"),
    )
}

fn inconsistent_query_authorization(subject: &str) -> WorthQueryApplicationQueryAdmissionDenial {
    WorthQueryApplicationQueryAdmissionDenial::new(
        WorthQueryApplicationQueryAdmissionDenialKind::Authorization(
            crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
        ),
        subject,
    )
}

fn map_authorization_denial(
    authorization: crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenial,
) -> WorthQueryApplicationQueryAdmissionDenial {
    WorthQueryApplicationQueryAdmissionDenial::new(
        WorthQueryApplicationQueryAdmissionDenialKind::Authorization(authorization.kind()),
        authorization.subject(),
    )
}

pub(super) fn validate_admission_request(
    request: &WorthQueryRequestScope,
    subject: &str,
) -> Result<(), WorthQueryApplicationQueryAdmissionDenial> {
    match request.interruption() {
        Some(WorthQueryRequestInterruption::Cancelled) => Err(denial(
            WorthQueryApplicationQueryAdmissionDenialKind::Cancelled,
            subject,
        )),
        Some(WorthQueryRequestInterruption::DeadlineExceeded) => Err(denial(
            WorthQueryApplicationQueryAdmissionDenialKind::DeadlineExceeded,
            subject,
        )),
        None => Ok(()),
    }
}

fn map_schema_denial(
    kind: worth_query_installation::facade::WorthQueryInstalledApplicationSchemaDenialKind,
) -> worth_query_installation::facade::WorthQueryApplicationQueryInstallationDenialKind {
    use worth_query_installation::facade::{
        WorthQueryApplicationQueryInstallationDenialKind as Query,
        WorthQueryInstalledApplicationSchemaDenialKind as Schema,
    };
    match kind {
        Schema::ForeignRuntime => Query::ForeignRuntime,
        Schema::StaleGeneration => Query::StaleGeneration,
        Schema::PackageIdentityChanged => Query::PackageIdentityChanged,
        Schema::AuthorityMismatch => Query::AuthorityMismatch,
        _ => Query::SchemaMeaningChanged,
    }
}

fn denial(
    kind: WorthQueryApplicationQueryAdmissionDenialKind,
    subject: impl Into<String>,
) -> WorthQueryApplicationQueryAdmissionDenial {
    WorthQueryApplicationQueryAdmissionDenial::new(kind, subject)
}
