use worth_foundational::facade::AspectValue;
use worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest;
use worth_query_declaration::facade::application_schema::TypedMutationPreconditions;
use worth_query_installation::facade::{
    ApplicationOperationDecisionReadTarget, ApplicationOperationProgramTarget, ApplicationSchema,
    WorthQueryInstalledApplicationOperation,
};

use super::super::capability_operation_progression::progress_capability_operation;
use super::super::capability_registry::{
    WorthQueryElevationLifecycleOperationRole, WorthQueryInstalledCapabilityPlan,
};
use super::super::capability_request_resolution::WorthQueryCapabilityContextKey;
use super::super::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryAdmittedApplicationOperation,
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};
use super::approval_binding::WorthQueryElevationApprovalBinding;
use crate::domain_computation::primary_graph::{
    resolve_at_snapshot, WorthQueryPrimaryGraphApplicationRuntime,
    WorthQueryPrincipalResolutionMode, WorthQueryRequestedElevation,
};

#[derive(Debug)]
pub struct WorthQueryElevationApprovalAuthorizationDenial {
    denial: WorthQueryOperationAuthorizationDenial,
    requested: WorthQueryRequestedElevation,
}

impl WorthQueryElevationApprovalAuthorizationDenial {
    pub const fn denial(&self) -> &WorthQueryOperationAuthorizationDenial {
        &self.denial
    }

    pub fn into_requested(self) -> WorthQueryRequestedElevation {
        self.requested
    }
}

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn authorize_elevation_approval<Capability, Operation, Input>(
        &self,
        requested: WorthQueryRequestedElevation,
        access: WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
        operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
        preconditions: TypedMutationPreconditions<
            Schema,
            Operation,
            <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        >,
    ) -> Result<
        WorthQueryAdmittedApplicationOperation<
            Schema,
            Operation,
            Input,
            <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        >,
        WorthQueryElevationApprovalAuthorizationDenial,
    >
    where
        Input: ApplicationCapabilityRequest<Schema, Capability>,
    {
        let binding = match bind_approval(self, &requested, &access, operation) {
            Ok(binding) => binding,
            Err(denial) => return Err(approval_denial(requested, denial)),
        };
        let admission =
            match progress_capability_operation(self, access, operation, preconditions, true) {
                Ok(admission) => admission,
                Err(denial) => return Err(approval_denial(requested, denial)),
            };
        admission
            .bind_elevation_approval(binding)
            .map_err(|denial| approval_denial(requested, denial))
    }
}

fn bind_approval<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    requested: &WorthQueryRequestedElevation,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
) -> Result<WorthQueryElevationApprovalBinding, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let (capability_identity, installed) = installed_approval_lifecycle(runtime, operation)?;
    let requested_binding = requested.binding();
    validate_receipt_authority(runtime, capability_identity, installed, requested, access)?;
    let lifecycle = installed
        .elevation
        .as_ref()
        .ok_or_else(|| approval_rejected(installed.contract.name()))?;
    let elevation_definition = installed
        .contract
        .elevation()
        .definition()
        .ok_or_else(|| approval_rejected(installed.contract.name()))?;
    let elevation = selected_context_entity(access, installed, true)?;
    let review = resolve_identity(
        runtime,
        access,
        installed
            .contract
            .elevation()
            .definition()
            .unwrap()
            .review()
            .identity(),
        &requested_binding.review_identity,
    )?;
    if resolve_identity(
        runtime,
        access,
        installed
            .contract
            .elevation()
            .definition()
            .unwrap()
            .identity(),
        &requested_binding.elevation_identity,
    )? != elevation
        || access.principal_entity_id == requested_binding.requester()
    {
        return Err(approval_rejected(installed.contract.name()));
    }
    validate_approval_time(runtime, installed, requested_binding)?;
    Ok(WorthQueryElevationApprovalBinding {
        requested: requested_binding.clone(),
        request_commit: requested.commit_receipt().clone(),
        elevation,
        review,
        approver: access.principal_entity_id,
        approved_status: lifecycle.lifecycle.approved.clone(),
        elevation_entity: elevation_definition.status().entity().to_string(),
        status_field: lifecycle.lifecycle.status.clone(),
        approver_relation: lifecycle.lifecycle.approver_relation,
        required_decision_reads: required_decision_reads(installed),
        required_program_targets: required_program_targets(installed),
    })
}

fn installed_approval_lifecycle<'runtime, Schema, Operation, Input>(
    runtime: &'runtime WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
) -> Result<
    ([u8; 32], &'runtime WorthQueryInstalledCapabilityPlan),
    WorthQueryOperationAuthorizationDenial,
>
where
    Schema: ApplicationSchema,
{
    let Some((capability, role)) = runtime
        .authorization
        .elevation_lifecycle_operation::<Operation, Input>(operation.operation())
        .map_err(|()| stale_operation(operation.operation()))?
    else {
        return Err(role_mismatch(operation.operation()));
    };
    if role != WorthQueryElevationLifecycleOperationRole::Approve {
        return Err(role_mismatch(operation.operation()));
    }
    let installed = runtime
        .authorization
        .capability_plan_by_identity(&capability)
        .filter(|plan| plan.elevation.is_some())
        .ok_or_else(|| stale_operation(operation.operation()))?;
    Ok((capability, installed))
}

fn validate_receipt_authority<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    capability_identity: [u8; 32],
    installed: &WorthQueryInstalledCapabilityPlan,
    requested: &WorthQueryRequestedElevation,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
) -> Result<(), WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let binding = requested.binding();
    if binding.runtime_authority != runtime.runtime.authority_identity()
        || binding.branch != *access.graph_work.branch().relational()
        || requested.commit_receipt().terminal().branch() != &binding.branch
        || binding.capability_identity != capability_identity
        || binding.capability_authority_identity.as_ref()
            != installed.capability_authority_identity.as_ref()
        || binding.upper_bound.resource() != access.resolved.resource.entity_id()
    {
        return Err(approval_rejected(installed.contract.name()));
    }
    Ok(())
}

fn selected_context_entity<Schema, Capability, Operation, Input>(
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    installed: &WorthQueryInstalledCapabilityPlan,
    elevation: bool,
) -> Result<worth_relational::facade::identity::EntityId, WorthQueryOperationAuthorizationDenial>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let lifecycle = installed
        .contract
        .elevation()
        .definition()
        .unwrap()
        .lifecycle();
    let slot = if elevation {
        lifecycle.elevation_slot()
    } else {
        lifecycle.review_slot()
    };
    access
        .resolved
        .context
        .get(&WorthQueryCapabilityContextKey {
            context: slot.context().to_string(),
            context_type: slot.context_type().to_string(),
            slot: slot.slot().to_string(),
            slot_type: slot.slot_type().to_string(),
            entity: slot.entity().to_string(),
        })
        .copied()
        .ok_or_else(|| approval_rejected(installed.contract.name()))
}

fn resolve_identity<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    field: &worth_query_declaration::facade::application_capability::ApplicationCapabilityFieldBinding,
    value: &AspectValue,
) -> Result<worth_relational::facade::identity::EntityId, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let graph = runtime
        .runtime
        .primary_graph()
        .ok_or_else(|| approval_rejected(access.operation()))?;
    let layout = graph
        .layout()
        .equality_field(field.entity(), field.aspect(), field.field())
        .ok_or_else(|| approval_rejected(field.field()))?;
    access
        .graph_work
        .mutation_handle()
        .ok_or_else(|| approval_rejected(field.field()))?
        .with_runtime(|relational| {
            resolve_at_snapshot(
                relational,
                access.graph_work.mutation_snapshot().unwrap(),
                layout,
                value.clone(),
                WorthQueryPrincipalResolutionMode::Ordinary,
                runtime.runtime.authority_identity(),
                runtime.installed_schema.binding_identity(),
                field.entity(),
                field.field(),
            )
        })
        .map(|evidence| evidence.entity_id)
        .map_err(|_| approval_rejected(field.field()))
}

fn validate_approval_time<Schema>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    installed: &WorthQueryInstalledCapabilityPlan,
    requested: &super::WorthQueryElevationRequestBinding,
) -> Result<(), WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
{
    let timeline = installed.elevation.as_ref().unwrap().temporal.timeline;
    let sample = runtime.authorization_clock.sample(timeline).map_err(|_| {
        denial(
            WorthQueryOperationAuthorizationDenialKind::TrustedTimeUnavailable,
            installed.contract.name(),
        )
    })?;
    match (sample.value(), &requested.issued_at, &requested.expires_at) {
        (AspectValue::UInt64(now), AspectValue::UInt64(start), AspectValue::UInt64(end))
            if start <= now && now < end =>
        {
            Ok(())
        }
        _ => Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationExpired,
            installed.contract.name(),
        )),
    }
}

fn required_decision_reads(
    installed: &WorthQueryInstalledCapabilityPlan,
) -> Vec<ApplicationOperationDecisionReadTarget> {
    let elevation = installed.contract.elevation().definition().unwrap();
    let review = elevation.review();
    let mut reads = [
        elevation.identity(),
        elevation.reason(),
        elevation.status(),
        elevation.validity().not_before(),
        elevation.validity().not_after(),
        review.identity(),
        review.status(),
    ]
    .into_iter()
    .map(|field| ApplicationOperationDecisionReadTarget::Field {
        entity: field.entity().to_string(),
        aspect: field.aspect().to_string(),
        field: field.field().to_string(),
    })
    .collect::<Vec<_>>();
    reads.extend(
        [
            elevation.requester(),
            elevation.approver(),
            elevation.grant(),
            review.relation(),
        ]
        .into_iter()
        .map(
            |relation| ApplicationOperationDecisionReadTarget::Relation {
                relation: relation.relation().to_string(),
                from: relation.from().to_string(),
                to: relation.to().to_string(),
            },
        ),
    );
    reads
}

fn required_program_targets(
    installed: &WorthQueryInstalledCapabilityPlan,
) -> Vec<ApplicationOperationProgramTarget> {
    let elevation = installed.contract.elevation().definition().unwrap();
    vec![
        ApplicationOperationProgramTarget::Write {
            entity: elevation.status().entity().to_string(),
            aspect: elevation.status().aspect().to_string(),
            field: elevation.status().field().to_string(),
        },
        ApplicationOperationProgramTarget::Link {
            relation: elevation.approver().relation().to_string(),
            from: elevation.approver().from().to_string(),
            to: elevation.approver().to().to_string(),
        },
    ]
}

fn approval_denial(
    requested: WorthQueryRequestedElevation,
    denial: WorthQueryOperationAuthorizationDenial,
) -> WorthQueryElevationApprovalAuthorizationDenial {
    WorthQueryElevationApprovalAuthorizationDenial { denial, requested }
}

fn approval_rejected(subject: impl Into<String>) -> WorthQueryOperationAuthorizationDenial {
    denial(
        WorthQueryOperationAuthorizationDenialKind::ElevationApprovalRejected,
        subject,
    )
}

fn role_mismatch(subject: impl Into<String>) -> WorthQueryOperationAuthorizationDenial {
    denial(
        WorthQueryOperationAuthorizationDenialKind::ElevationLifecycleRoleMismatch,
        subject,
    )
}

fn stale_operation(subject: impl Into<String>) -> WorthQueryOperationAuthorizationDenial {
    denial(
        WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation,
        subject,
    )
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
