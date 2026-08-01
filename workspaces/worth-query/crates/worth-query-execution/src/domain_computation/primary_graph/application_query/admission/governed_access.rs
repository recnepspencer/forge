use worth_query_declaration::facade::{
    application_capability::ApplicationCapabilityRequest, application_schema::ApplicationSchema,
};
use worth_query_installation::facade::WorthQueryInstalledApplicationQuery;

use super::super::{
    admission_preparation::validate_admission_request,
    disclosure::WorthQueryPendingApplicationQueryGovernance,
    WorthQueryApplicationQueryAccessContext, WorthQueryApplicationQueryAdmissionDenial,
    WorthQueryApplicationQueryAdmissionDenialKind, WorthQueryApplicationQueryControls,
    WorthQueryApplicationQueryGraphWorkSession,
};
use crate::domain_computation::authorization::{
    WorthQueryPreparedApplicationCapabilityAccess, WorthQueryPrincipalCurrentnessDependency,
    WorthQueryRetainedCapabilityAuthorization,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;
use crate::domain_computation::provider_session::record_capability_authorization_completion;

pub(in crate::domain_computation::primary_graph::application_query) type WorthQueryGovernanceAdmission<
    Schema,
    Principal,
    PrincipalIdentity,
    Scope,
> = Box<
    dyn for<'access> FnOnce(
            &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
            &WorthQueryApplicationQueryAccessContext<
                'access,
                Schema,
                Principal,
                PrincipalIdentity,
                Scope,
            >,
            &mut WorthQueryApplicationQueryGraphWorkSession,
        ) -> Result<
            WorthQueryPendingApplicationQueryGovernance,
            WorthQueryApplicationQueryAdmissionDenial,
        > + 'static,
>;

pub(in crate::domain_computation::primary_graph::application_query) fn prepare_governed_access<
    'a,
    Schema,
    Query,
    Parameters,
    QueryResult,
    Principal,
    PrincipalIdentity,
    Scope,
    Capability,
    Operation,
    Input,
>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    access: &WorthQueryApplicationQueryAccessContext<
        'a,
        Schema,
        Principal,
        PrincipalIdentity,
        Scope,
    >,
    prepared: WorthQueryPreparedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    controls: &WorthQueryApplicationQueryControls<'_, Schema>,
) -> Result<
    WorthQueryGovernanceAdmission<Schema, Principal, PrincipalIdentity, Scope>,
    WorthQueryApplicationQueryAdmissionDenial,
>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability, Scope = Scope> + 'static,
    Capability: 'static,
    Operation: 'static,
{
    validate_prepared_governance(runtime, query, access, &prepared, controls)?;
    Ok(Box::new(move |runtime, access, session| {
        admit_governance_in_session(runtime, access, prepared, session)
    }))
}

pub(in crate::domain_computation::primary_graph::application_query) fn prepare_retained_governance<
    Schema,
    Principal,
    PrincipalIdentity,
    Scope,
>(
    pending: Option<WorthQueryPendingApplicationQueryGovernance>,
) -> Option<WorthQueryGovernanceAdmission<Schema, Principal, PrincipalIdentity, Scope>>
where
    Schema: ApplicationSchema,
    Schema: 'static,
    Principal: 'static,
    PrincipalIdentity: 'static,
    Scope: 'static,
{
    pending.map(|pending| {
        Box::new(
            move |runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
                  _access: &WorthQueryApplicationQueryAccessContext<
                '_,
                Schema,
                Principal,
                PrincipalIdentity,
                Scope,
            >,
                  session: &mut WorthQueryApplicationQueryGraphWorkSession| {
                let (capability_name, capability_type, disclosure_value, authorization) =
                    pending.into_parts();
                let branch_id = session.branch_affinity().relational_branch().clone();
                let authorization = runtime
                    .readmit_capability_authorization_in_session(
                        authorization,
                        *session.identity(),
                        branch_id,
                        session.basis().snapshot_handle().clone(),
                    )
                    .map_err(map_authorization_denial)?;
                record_capability_authorization_completion(session, &authorization)
                    .map_err(|_| denial(&capability_name))?;
                Ok(WorthQueryPendingApplicationQueryGovernance::new(
                    capability_name,
                    capability_type,
                    disclosure_value,
                    authorization,
                ))
            },
        ) as WorthQueryGovernanceAdmission<Schema, Principal, PrincipalIdentity, Scope>
    })
}

fn validate_prepared_governance<
    Schema,
    Query,
    Parameters,
    QueryResult,
    Principal,
    PrincipalIdentity,
    Scope,
    Capability,
    Operation,
    Input,
>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    access: &WorthQueryApplicationQueryAccessContext<
        '_,
        Schema,
        Principal,
        PrincipalIdentity,
        Scope,
    >,
    prepared: &WorthQueryPreparedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    controls: &WorthQueryApplicationQueryControls<'_, Schema>,
) -> Result<(), WorthQueryApplicationQueryAdmissionDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability, Scope = Scope>,
{
    validate_admission_request(&prepared.request_scope, query.name())?;
    let disclosure = query.disclosure();
    if prepared.runtime_authority != runtime.runtime.authority_identity()
        || prepared.binding_identity != *query.binding_identity()
        || prepared.principal_entity_id != access.principal().principal_entity_id()
        || prepared.authentication_valid_until <= std::time::Instant::now()
        || disclosure.capability_name() != Some(prepared.capability.as_ref())
        || disclosure.capability_type() != Some(prepared.capability_type.as_ref())
    {
        return Err(denial(query.name()));
    }
    validate_admission_request(controls.request_scope(), query.name())
}

fn admit_governance_in_session<
    Schema,
    Principal,
    PrincipalIdentity,
    Scope,
    Capability,
    Operation,
    Input,
>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    access: &WorthQueryApplicationQueryAccessContext<
        '_,
        Schema,
        Principal,
        PrincipalIdentity,
        Scope,
    >,
    prepared: WorthQueryPreparedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    session: &mut WorthQueryApplicationQueryGraphWorkSession,
) -> Result<WorthQueryPendingApplicationQueryGovernance, WorthQueryApplicationQueryAdmissionDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability, Scope = Scope>,
{
    let installed = runtime
        .authorization
        .capability_plan_by_identity(prepared.capability_identity.bytes())
        .filter(|installed| {
            installed.capability_authority_identity.as_ref()
                == prepared.capability_authority_identity.as_ref()
        })
        .ok_or_else(|| denial(prepared.capability.as_ref()))?;
    let graph = runtime
        .runtime
        .primary_graph()
        .ok_or_else(|| denial(prepared.capability.as_ref()))?;
    let principal_layout = graph
        .layout()
        .principal_binding(&prepared.principal_binding)
        .cloned()
        .ok_or_else(|| denial(prepared.principal_binding.as_ref()))?;
    let sample = runtime
        .sample_capability_time(installed)
        .map_err(map_authorization_denial)?;
    let (request, decision, grant) = graph.integration_handle().with_runtime_mut(|relational| {
        let snapshot = session.basis().snapshot_handle().clone();
        if !prepared.principal_freshness.remains_current_in(
            relational,
            &snapshot,
            &principal_layout,
            &prepared.principal_binding,
        ) {
            return Err(denial(prepared.principal_binding.as_ref()));
        }
        let resolved = crate::domain_computation::authorization::resolve_capability_request(
            relational,
            &snapshot,
            graph.layout(),
            &runtime.installed_schema,
            &prepared.projection,
            runtime.runtime.authority_identity(),
        )
        .map_err(map_authorization_denial)?;
        if resolved.resource.entity_id() != access.scope().entity_id() {
            return Err(denial(prepared.capability.as_ref()));
        }
        let request =
            crate::domain_computation::authorization::WorthQueryRetainedCapabilityRequest::capture(
                *prepared.capability_identity.bytes(),
                prepared.principal_entity_id,
                &prepared.projection,
                &resolved,
            );
        let observed = crate::domain_computation::authorization::observe_capability(
            *session.identity(),
            relational,
            snapshot,
            runtime.authorization.bridge(),
            installed,
            &request,
            &sample,
            None,
        )
        .map_err(map_authorization_denial)?;
        let (decision, grant) = observed.into_parts();
        Ok((request, decision, grant))
    })?;
    let principal = WorthQueryPrincipalCurrentnessDependency::capture_retained(
        *session.identity(),
        prepared.principal_binding.clone(),
        principal_layout,
        prepared.principal_freshness.clone(),
        session.branch_affinity().relational_branch().clone(),
    );
    let authorization = WorthQueryRetainedCapabilityAuthorization::new(
        principal,
        decision,
        prepared.capability_authority_identity,
        grant,
        request,
        sample,
    );
    record_capability_authorization_completion(session, &authorization)
        .map_err(|_| denial(prepared.capability.as_ref()))?;
    let disclosure_value = prepared
        .projection
        .field_value()
        .cloned()
        .ok_or_else(|| denial(prepared.capability.as_ref()))?;
    Ok(WorthQueryPendingApplicationQueryGovernance::new(
        prepared.capability.to_string(),
        prepared.capability_type.to_string(),
        disclosure_value,
        authorization,
    ))
}

fn map_authorization_denial(
    denial: crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenial,
) -> WorthQueryApplicationQueryAdmissionDenial {
    WorthQueryApplicationQueryAdmissionDenial::new(
        WorthQueryApplicationQueryAdmissionDenialKind::Authorization(denial.kind()),
        denial.subject(),
    )
}

fn denial(subject: impl Into<String>) -> WorthQueryApplicationQueryAdmissionDenial {
    WorthQueryApplicationQueryAdmissionDenial::new(
        WorthQueryApplicationQueryAdmissionDenialKind::DisclosureAuthorizationMismatch,
        subject,
    )
}
