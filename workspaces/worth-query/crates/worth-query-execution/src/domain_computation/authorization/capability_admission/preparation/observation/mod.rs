//! Currentness-bound capability observation owned by capability admission.

use worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest;
use worth_query_installation::facade::{
    ApplicationSchema, TypedApplicationValue, WorthQueryInstalledApplicationCapability,
};

use super::super::super::delegation_admission::observe_capability;
use super::super::super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::super::super::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
    WorthQueryPrincipalCurrentnessDependency, WorthQueryRetainedCapabilityAuthorization,
};
use super::PreparedCapabilityAdmission;
use crate::domain_computation::primary_graph::{
    validate_freshness_at_snapshot, WorthQueryApprovedElevation, WorthQueryPrincipalResolutionMode,
};

mod admitted_access;
mod request_resolution;

pub(super) use admitted_access::admit_observed_capability;
pub use admitted_access::WorthQueryAdmittedApplicationCapabilityAccess;
pub(in crate::domain_computation::authorization) use admitted_access::{
    progress_capability_operation, WorthQueryCapabilityOperationProgression,
    WorthQueryDelegationResolvedRequest,
};
use request_resolution::resolve_capability_request;
pub(in crate::domain_computation::authorization) use request_resolution::{
    WorthQueryCapabilityContextKey, WorthQueryResolvedCapabilityRequest,
};
pub(super) struct ObservedCapabilityAdmission<
    'a,
    Schema,
    Principal,
    PrincipalIdentity,
    Capability,
    Operation,
    Input,
> where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    prepared: PreparedCapabilityAdmission<
        'a,
        Schema,
        Principal,
        PrincipalIdentity,
        Capability,
        Operation,
        Input,
    >,
    resolved: WorthQueryResolvedCapabilityRequest<
        Schema,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
    >,
    authorization: WorthQueryRetainedCapabilityAuthorization,
    _seal: ObservedSeal,
}

struct ObservedSeal;

pub(super) fn observe_current_capability<
    'a,
    Schema,
    Principal,
    PrincipalIdentity,
    Capability,
    Operation,
    Input,
>(
    mut prepared: PreparedCapabilityAdmission<
        'a,
        Schema,
        Principal,
        PrincipalIdentity,
        Capability,
        Operation,
        Input,
    >,
) -> Result<
    ObservedCapabilityAdmission<
        'a,
        Schema,
        Principal,
        PrincipalIdentity,
        Capability,
        Operation,
        Input,
    >,
    WorthQueryOperationAuthorizationDenial,
>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let runtime = prepared.runtime();
    let principal = prepared.principal();
    let capability = prepared.capability();
    let approved = prepared.approved();
    let graph = runtime.runtime.primary_graph().ok_or_else(|| {
        denial(
            WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
            capability.contract().operation(),
        )
    })?;
    if approved.is_some_and(|approved| {
        !approved.belongs_to_lifecycle(
            runtime.runtime.authority_identity(),
            prepared.graph_work().branch().relational(),
            *capability.identity().bytes(),
            &prepared.installed().capability_authority_identity,
        )
    }) {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationApprovalRejected,
            capability.contract().name(),
        ));
    }
    let session_identity = prepared.graph_work().identity();
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
    let snapshot = prepared
        .graph_work()
        .mutation_snapshot()
        .expect("a capability session owns its admitted snapshot")
        .clone();
    let handle = prepared
        .graph_work()
        .mutation_handle()
        .expect("a capability session owns its graph handle")
        .clone();
    let entity_resolution = graph.retain_entity_resolution_context();
    let (resolved, revalidation, observed) = handle.with_runtime_mut(|relational| {
        validate_freshness_at_snapshot(
            relational,
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
        let resolution_truth = entity_resolution
            .at_snapshot(
                relational,
                &snapshot,
                WorthQueryPrincipalResolutionMode::Ordinary,
            )
            .map_err(|_| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected,
                    capability.contract().name(),
                )
            })?;
        let resolved = resolve_capability_request(
            &resolution_truth,
            &runtime.installed_schema,
            prepared.projection(),
        )?;
        let revalidation = WorthQueryRetainedCapabilityRequest::capture(
            *capability.identity().bytes(),
            principal.principal_entity_id(),
            prepared.projection(),
            &resolved,
        );
        if approved.is_some_and(|approved| {
            !approved.support_remains_current_in(
                relational,
                &snapshot,
                runtime.authorization.bridge(),
            )
        }) {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::StaleAuthorization,
                capability.contract().name(),
            ));
        }
        let observed = observe_capability(
            session_identity,
            relational,
            snapshot.clone(),
            runtime.authorization.bridge(),
            prepared.installed(),
            &revalidation,
            prepared.sample(),
            None,
            approved.map(WorthQueryApprovedElevation::support_decision),
        )?;
        Ok((resolved, revalidation, observed))
    })?;
    let (decision, grant) = observed.into_parts();
    validate_approved_use(
        approved,
        capability,
        &prepared,
        runtime.runtime.authority_identity(),
        &resolved,
        &revalidation,
        grant,
    )?;
    super::super::super::admission::admit_request(
        &prepared.request_scope,
        prepared.capability.contract().operation(),
    )?;
    if prepared.principal.is_expired() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ExpiredAuthentication,
            prepared.principal.binding(),
        ));
    }
    prepared.record_admission_decisions();
    let authorization = WorthQueryRetainedCapabilityAuthorization::new(
        principal_currentness,
        decision,
        prepared.installed().capability_authority_identity.clone(),
        grant,
        revalidation,
        prepared.sample().clone(),
    );
    Ok(ObservedCapabilityAdmission {
        prepared,
        resolved,
        authorization,
        _seal: ObservedSeal,
    })
}

fn validate_approved_use<Schema, Capability, Operation, Input>(
    approved: Option<&WorthQueryApprovedElevation>,
    capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
    prepared: &PreparedCapabilityAdmission<
        '_,
        Schema,
        impl Sized,
        impl Sized,
        Capability,
        Operation,
        Input,
    >,
    runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    resolved: &WorthQueryResolvedCapabilityRequest<
        Schema,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
    >,
    revalidation: &WorthQueryRetainedCapabilityRequest,
    grant: worth_relational::facade::identity::EntityId,
) -> Result<(), WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let Some(approved) = approved else {
        return Ok(());
    };
    let elevation = resolved.elevation().ok_or_else(|| {
        denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationProjectionRejected,
            capability.contract().name(),
        )
    })?;
    if !approved.admits_active_use(
        runtime_authority,
        prepared.graph_work().branch().relational(),
        *capability.identity().bytes(),
        &prepared.installed().capability_authority_identity,
        revalidation,
        elevation,
        grant,
    ) {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationApprovalRejected,
            capability.contract().name(),
        ));
    }
    Ok(())
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
