use worth_query_admission::facade::{
    application_query::{
        admit_application_query_parameters, WorthQueryAdmittedApplicationQueryParameters,
    },
    authenticated_principal::{WorthQueryRequestInterruption, WorthQueryRequestScope},
};
use worth_query_declaration::facade::{
    application_query::{ApplicationQueryDisclosurePosture, ApplicationQueryParameterSet},
    application_schema::ApplicationSchema,
};
use worth_query_installation::facade::{
    TypedApplicationValue, WorthQueryInstalledApplicationQuery,
};

use super::{
    control_validation::validate_controls, WorthQueryApplicationAuthorizationWorkEvidence,
    WorthQueryApplicationQueryAccessContext, WorthQueryApplicationQueryAdmissionDenial,
    WorthQueryApplicationQueryAdmissionDenialKind, WorthQueryApplicationQueryControls,
};
use crate::domain_computation::primary_graph::{
    authorization::{
        WorthQueryPrincipalCurrentnessDependency, WorthQueryRetainedAuthorizationDecisionFacts,
    },
    entity_resolution::validate_entity_freshness_at_snapshot,
    resolution::validate_freshness_at_snapshot,
    WorthQueryPrimaryGraphApplicationRuntime, WorthQueryPrincipalResolutionDenialKind,
};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub(super) fn prepare_application_query_admission<
        'a,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >(
        &'a self,
        query: &'a WorthQueryInstalledApplicationQuery<
            Schema,
            Query,
            Parameters,
            QueryResult,
            Scope,
        >,
        access: &WorthQueryApplicationQueryAccessContext<
            'a,
            Schema,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
        parameters: ApplicationQueryParameterSet<Query>,
        controls: WorthQueryApplicationQueryControls<'a, Schema>,
    ) -> Result<
        (
            WorthQueryAdmittedApplicationQueryParameters,
            WorthQueryApplicationQueryControls<'a, Schema>,
            WorthQueryRetainedAuthorizationDecisionFacts,
            WorthQueryApplicationAuthorizationWorkEvidence,
        ),
        WorthQueryApplicationQueryAdmissionDenial,
    > {
        validate_admission_request(controls.request_scope(), query.name())?;
        self.validate_installed_query(query)?;
        let (authorization, authorization_work) =
            self.validate_access(query, access, controls.request_scope())?;
        validate_controls(query, &controls)?;
        validate_disclosure(query)?;
        let parameters =
            admit_application_query_parameters(query, parameters).map_err(|denial| {
                WorthQueryApplicationQueryAdmissionDenial::new(
                    WorthQueryApplicationQueryAdmissionDenialKind::Parameter(denial.kind()),
                    denial.parameter(),
                )
            })?;
        Ok((parameters, controls, authorization, authorization_work))
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

    fn validate_access<Principal, PrincipalIdentity, Scope, Query, Parameters, QueryResult>(
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
    ) -> Result<
        (
            WorthQueryRetainedAuthorizationDecisionFacts,
            WorthQueryApplicationAuthorizationWorkEvidence,
        ),
        WorthQueryApplicationQueryAdmissionDenial,
    > {
        self.validate_authenticated_principal(access.principal(), request)
            .map_err(|denial| {
                WorthQueryApplicationQueryAdmissionDenial::new(
                    map_principal_denial(denial.kind()),
                    denial.binding(),
                )
            })?;
        let scope = access.scope();
        let authority = self.runtime.authority_identity();
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
        let principal_currentness =
            WorthQueryPrincipalCurrentnessDependency::capture(principal, &principal_layout);
        let policy = graph.integration_handle().with_runtime_mut(|runtime| {
            let snapshot = runtime.snapshots().snapshot();
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
                self.observe_query_authorization(runtime, snapshot.clone(), query, access)
                    .map_err(map_authorization_denial)
            });
            runtime.snapshots().release_snapshot(&snapshot);
            result
        })?;
        let work = WorthQueryApplicationAuthorizationWorkEvidence::from_dependencies(&policy);
        Ok((
            WorthQueryRetainedAuthorizationDecisionFacts::new(principal_currentness, policy),
            work,
        ))
    }
}

fn map_authorization_denial(
    authorization: crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenial,
) -> WorthQueryApplicationQueryAdmissionDenial {
    WorthQueryApplicationQueryAdmissionDenial::new(
        WorthQueryApplicationQueryAdmissionDenialKind::Authorization(authorization.kind()),
        authorization.subject(),
    )
}

fn validate_disclosure<Schema, Query, Parameters, QueryResult, Scope>(
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
) -> Result<(), WorthQueryApplicationQueryAdmissionDenial> {
    match query.disclosure().posture() {
        ApplicationQueryDisclosurePosture::Public => Ok(()),
        ApplicationQueryDisclosurePosture::InstalledPolicyRequired
        | ApplicationQueryDisclosurePosture::PhaseSevenGovernanceRequired => Err(denial(
            WorthQueryApplicationQueryAdmissionDenialKind::DisclosureGovernanceRequired,
            query.disclosure().classification(),
        )),
    }
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

fn map_principal_denial(
    kind: WorthQueryPrincipalResolutionDenialKind,
) -> WorthQueryApplicationQueryAdmissionDenialKind {
    match kind {
        WorthQueryPrincipalResolutionDenialKind::ForeignRuntime => {
            WorthQueryApplicationQueryAdmissionDenialKind::ForeignPrincipal
        }
        WorthQueryPrincipalResolutionDenialKind::Cancelled => {
            WorthQueryApplicationQueryAdmissionDenialKind::Cancelled
        }
        WorthQueryPrincipalResolutionDenialKind::DeadlineExceeded => {
            WorthQueryApplicationQueryAdmissionDenialKind::DeadlineExceeded
        }
        _ => WorthQueryApplicationQueryAdmissionDenialKind::StalePrincipal,
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
