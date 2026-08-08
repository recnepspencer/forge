//! Currentness-bound capability policy observation for one admitted graph-work session.

use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityRequest, ApplicationCapabilityRequestProjection,
};
use worth_query_installation::facade::{
    ApplicationSchema, TypedApplicationValue, WorthQueryInstalledApplicationCapability,
};

use super::capability_registry::WorthQueryInstalledCapabilityPlan;
use super::capability_request_resolution::{
    resolve_capability_request, WorthQueryResolvedCapabilityRequest,
};
use super::delegation_admission::observe_capability;
use super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
    WorthQueryPrincipalCurrentnessDependency, WorthQueryRetainedCapabilityAuthorization,
    WorthQueryRuntimeTimeSample,
};
use crate::domain_computation::primary_graph::{
    validate_freshness_at_snapshot, WorthQueryApprovedElevation, WorthQueryAuthenticatedPrincipal,
    WorthQueryPrimaryGraphApplicationRuntime,
};
use crate::domain_computation::provider_session::WorthQueryManagedGraphWorkSession;

pub(super) struct WorthQueryCapabilityObservationContext<
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
    pub(super) principal:
        &'a WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    pub(super) capability:
        &'a WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
    pub(super) installed: &'a WorthQueryInstalledCapabilityPlan,
    pub(super) projection: &'a ApplicationCapabilityRequestProjection<
        Schema,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Context,
    >,
    pub(super) approved: Option<&'a WorthQueryApprovedElevation>,
    pub(super) graph_work: &'a WorthQueryManagedGraphWorkSession,
    pub(super) sample: &'a WorthQueryRuntimeTimeSample,
}

pub(super) struct WorthQueryAdmittedCapabilityObservation<Schema, Scope> {
    pub(super) resolved: WorthQueryResolvedCapabilityRequest<Schema, Scope>,
    pub(super) authorization: WorthQueryRetainedCapabilityAuthorization,
}

pub(super) fn observe_current_capability<
    Schema,
    Principal,
    PrincipalIdentity,
    Capability,
    Operation,
    Input,
>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    context: WorthQueryCapabilityObservationContext<
        '_,
        Schema,
        Principal,
        PrincipalIdentity,
        Capability,
        Operation,
        Input,
    >,
) -> Result<
    WorthQueryAdmittedCapabilityObservation<
        Schema,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
    >,
    WorthQueryOperationAuthorizationDenial,
>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let graph = runtime.runtime.primary_graph().ok_or_else(|| {
        denial(
            WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
            context.capability.contract().operation(),
        )
    })?;
    if context.approved.is_some_and(|approved| {
        !approved.belongs_to_lifecycle(
            runtime.runtime.authority_identity(),
            context.graph_work.branch().relational(),
            *context.capability.identity().bytes(),
            &context.installed.capability_authority_identity,
        )
    }) {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationApprovalRejected,
            context.capability.contract().name(),
        ));
    }
    let session_identity = context.graph_work.identity();
    let principal_layout = graph
        .layout()
        .principal_binding(context.principal.binding())
        .cloned()
        .ok_or_else(|| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::StalePrincipal,
                context.principal.binding(),
            )
        })?;
    let expected_external_identity = context
        .principal
        .external_identity()
        .clone()
        .into_foundational_value();
    let principal_currentness = WorthQueryPrincipalCurrentnessDependency::capture(
        session_identity,
        context.principal,
        &principal_layout,
    );
    let snapshot = context
        .graph_work
        .mutation_snapshot()
        .expect("a capability session owns its admitted snapshot")
        .clone();
    let handle = context
        .graph_work
        .mutation_handle()
        .expect("a capability session owns its graph handle")
        .clone();
    let (resolved, revalidation, observed) = handle.with_runtime_mut(|relational| {
        validate_freshness_at_snapshot(
            relational,
            &snapshot,
            context.principal,
            &principal_layout,
            &expected_external_identity,
        )
        .map_err(|_| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::StalePrincipal,
                context.principal.binding(),
            )
        })?;
        let resolved = resolve_capability_request(
            relational,
            &snapshot,
            graph.layout(),
            &runtime.installed_schema,
            context.projection,
            runtime.runtime.authority_identity(),
        )?;
        let revalidation = WorthQueryRetainedCapabilityRequest::capture(
            *context.capability.identity().bytes(),
            context.principal.principal_entity_id(),
            context.projection,
            &resolved,
        );
        if context.approved.is_some_and(|approved| {
            !approved.support_remains_current_in(
                relational,
                &snapshot,
                runtime.authorization.bridge(),
            )
        }) {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::StaleAuthorization,
                context.capability.contract().name(),
            ));
        }
        let observed = observe_capability(
            session_identity,
            relational,
            snapshot.clone(),
            runtime.authorization.bridge(),
            context.installed,
            &revalidation,
            context.sample,
            None,
            context
                .approved
                .map(WorthQueryApprovedElevation::support_decision),
        )?;
        Ok((resolved, revalidation, observed))
    })?;
    let (decision, grant) = observed.into_parts();
    validate_approved_use(
        &context,
        runtime.runtime.authority_identity(),
        &resolved,
        &revalidation,
        grant,
    )?;
    Ok(WorthQueryAdmittedCapabilityObservation {
        resolved,
        authorization: WorthQueryRetainedCapabilityAuthorization::new(
            principal_currentness,
            decision,
            context.installed.capability_authority_identity.clone(),
            grant,
            revalidation,
            context.sample.clone(),
        ),
    })
}

fn validate_approved_use<Schema, Principal, PrincipalIdentity, Capability, Operation, Input>(
    context: &WorthQueryCapabilityObservationContext<
        '_,
        Schema,
        Principal,
        PrincipalIdentity,
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
    let Some(approved) = context.approved else {
        return Ok(());
    };
    let elevation = resolved.elevation.ok_or_else(|| {
        denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationProjectionRejected,
            context.capability.contract().name(),
        )
    })?;
    if !approved.admits_active_use(
        runtime_authority,
        context.graph_work.branch().relational(),
        *context.capability.identity().bytes(),
        &context.installed.capability_authority_identity,
        revalidation,
        elevation,
        grant,
    ) {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationApprovalRejected,
            context.capability.contract().name(),
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
