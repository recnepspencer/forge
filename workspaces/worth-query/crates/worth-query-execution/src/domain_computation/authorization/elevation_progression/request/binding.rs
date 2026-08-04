use std::sync::Arc;

use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityElevationDefinition, ApplicationCapabilityElevationRequestProjection,
    ApplicationCapabilityRequest,
};
use worth_query_installation::facade::{ApplicationOperationProgramTarget, ApplicationSchema};

use super::super::super::capability_registry::WorthQueryInstalledCapabilityPlan;
use super::super::super::capability_request_resolution::{
    resolve_capability_request, resolve_erased_selector,
};
use super::super::super::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryOperationAuthorizationDenial,
    WorthQueryOperationAuthorizationDenialKind,
};
use super::super::request_binding::WorthQueryElevationRequestBinding;
use super::super::WorthQueryElevationUpperBound;
use super::{denial, projection_denial};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

pub(super) fn bind_request<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    capability_identity: [u8; 32],
    installed: &WorthQueryInstalledCapabilityPlan,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    proposed: &ApplicationCapabilityElevationRequestProjection<
        Schema,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Context,
    >,
) -> Result<WorthQueryElevationRequestBinding, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let upper_bound =
        resolve_upper_bound(runtime, capability_identity, installed, access, proposed)?;
    let elevation = installed
        .elevation
        .as_ref()
        .ok_or_else(|| projection_denial(installed.contract.name()))?;
    let interval = runtime
        .authorization_clock
        .sample_interval(elevation.temporal.timeline, proposed.duration())
        .map_err(|_| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::TrustedTimeUnavailable,
                installed.contract.name(),
            )
        })?;
    let lifecycle = &elevation.lifecycle;
    Ok(WorthQueryElevationRequestBinding {
        runtime_authority: runtime.runtime.authority_identity(),
        branch: access.graph_work.branch().relational().clone(),
        capability_identity,
        capability_authority_identity: Arc::clone(&installed.capability_authority_identity),
        upper_bound,
        elevation_kind: elevation.elevation_kind,
        review_kind: lifecycle.review_kind,
        elevation_key: proposed.elevation_key().to_string(),
        elevation_identity_field: lifecycle.identity.clone(),
        elevation_identity: proposed.elevation_identity().value().clone(),
        reason_field: lifecycle.reason.clone(),
        reason: proposed.reason().value().clone(),
        status_field: lifecycle.status.clone(),
        requested_status: lifecycle.requested.clone(),
        not_before_field: elevation.temporal.not_before.clone(),
        issued_at: interval.issued.value().clone(),
        not_after_field: elevation.temporal.not_after.clone(),
        expires_at: interval.expires,
        review_key: proposed.review_key().to_string(),
        review_identity_field: lifecycle.review_identity.clone(),
        review_identity: proposed.review_identity().value().clone(),
        review_status_field: lifecycle.review_status.clone(),
        review_required_status: lifecycle.review_required.clone(),
        requester_relation: lifecycle.requester_relation,
        grant_relation: lifecycle.grant_relation,
        review_relation: lifecycle.review_relation,
        required_program_targets: required_program_targets(
            installed
                .contract
                .elevation()
                .definition()
                .expect("installed elevation plan retains its governed declaration"),
        ),
    })
}

fn resolve_upper_bound<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    capability_identity: [u8; 32],
    installed: &WorthQueryInstalledCapabilityPlan,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    proposed: &ApplicationCapabilityElevationRequestProjection<
        Schema,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Context,
    >,
) -> Result<WorthQueryElevationUpperBound, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let graph = runtime.runtime.primary_graph().ok_or_else(|| {
        denial(
            WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
            access.operation(),
        )
    })?;
    let snapshot = access
        .graph_work
        .mutation_snapshot()
        .ok_or_else(|| projection_denial(access.operation()))?
        .clone();
    let handle = access
        .graph_work
        .mutation_handle()
        .ok_or_else(|| projection_denial(access.operation()))?
        .clone();
    let (target, grant) = handle.with_runtime(|relational| {
        let target = resolve_capability_request(
            relational,
            &snapshot,
            graph.layout(),
            &runtime.installed_schema,
            proposed.target(),
            runtime.runtime.authority_identity(),
        )?;
        let grant = resolve_erased_selector(
            relational,
            &snapshot,
            graph.layout(),
            &runtime.installed_schema,
            proposed.grant(),
            runtime.runtime.authority_identity(),
        )?;
        Ok((target, grant))
    })?;
    if target.resource.entity_id() != access.resolved.resource.entity_id()
        || grant.entity_kind != installed.grant_kind
        || grant.entity_id != access.authorization.grant()
    {
        return Err(projection_denial(installed.contract.name()));
    }
    Ok(WorthQueryElevationUpperBound::capture(
        capability_identity,
        access.principal_entity_id,
        proposed.target(),
        &target,
        grant.entity_id,
    ))
}

fn required_program_targets(
    elevation: &ApplicationCapabilityElevationDefinition,
) -> Vec<ApplicationOperationProgramTarget> {
    let review = elevation.review();
    let mut targets = vec![
        ApplicationOperationProgramTarget::Create {
            entity: elevation.identity().entity().to_string(),
        },
        ApplicationOperationProgramTarget::Create {
            entity: review.identity().entity().to_string(),
        },
    ];
    targets.extend(
        [
            elevation.identity(),
            elevation.reason(),
            elevation.status(),
            elevation.validity().not_before(),
            elevation.validity().not_after(),
            review.identity(),
            review.status(),
        ]
        .into_iter()
        .map(|field| ApplicationOperationProgramTarget::Write {
            entity: field.entity().to_string(),
            aspect: field.aspect().to_string(),
            field: field.field().to_string(),
        }),
    );
    targets.extend(
        [elevation.requester(), elevation.grant(), review.relation()]
            .into_iter()
            .map(|relation| ApplicationOperationProgramTarget::Link {
                relation: relation.relation().to_string(),
                from: relation.from().to_string(),
                to: relation.to().to_string(),
            }),
    );
    targets
}
