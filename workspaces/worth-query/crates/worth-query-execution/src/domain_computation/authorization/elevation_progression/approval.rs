use worth_foundational::facade::AspectValue;
use worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest;
use worth_query_declaration::facade::application_schema::TypedMutationPreconditions;
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationOperation,
};
use worth_relational::facade::identity::EntityId;

use super::super::capability_operation_progression::{
    progress_capability_operation, WorthQueryCapabilityOperationProgression,
};
use super::super::capability_registry::{
    WorthQueryElevationLifecycleOperationRole, WorthQueryInstalledCapabilityPlan,
};
use super::super::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryAdmittedApplicationOperation,
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};
use super::approval_binding::WorthQueryElevationApprovalDraft;
use super::context_identity::{resolve_lifecycle_identity, selected_elevation_entity};
use super::operation_role::installed_lifecycle_owner;
use super::transition_contract::{approval_program_targets, lifecycle_decision_reads};
use crate::domain_computation::primary_graph::{
    WorthQueryPrimaryGraphApplicationRuntime, WorthQueryRequestedElevation,
};

mod support_currentness;
use support_currentness::refresh_exact_support;

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
        mut requested: WorthQueryRequestedElevation,
        mut access: WorthQueryAdmittedApplicationCapabilityAccess<
            Schema,
            Capability,
            Operation,
            Input,
        >,
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
        Input: 'static,
    {
        let draft = match bind_approval(self, &requested, &access, operation) {
            Ok(draft) => draft,
            Err(denial) => return Err(approval_denial(requested, denial)),
        };
        let support_sample = match refresh_exact_support(self, requested.binding_mut(), &mut access)
        {
            Ok(sample) => sample,
            Err(denial) => return Err(approval_denial(requested, denial)),
        };
        if let Err(denial) =
            validate_approval_time(requested.binding(), &support_sample, operation.operation())
        {
            return Err(approval_denial(requested, denial));
        }
        let admission = match progress_capability_operation(
            self,
            access,
            operation,
            preconditions,
            WorthQueryCapabilityOperationProgression::ElevationLifecycle,
        ) {
            Ok(admission) => admission,
            Err(denial) => return Err(approval_denial(requested, denial)),
        };
        admission
            .bind_elevation_approval(draft.bind(requested))
            .map_err(|(denial, binding)| approval_denial(binding.into_requested(), denial))
    }
}

fn bind_approval<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    requested: &WorthQueryRequestedElevation,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
) -> Result<WorthQueryElevationApprovalDraft, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
    Input: 'static,
{
    let (capability_identity, installed) = installed_lifecycle_owner(
        runtime,
        access.authorization.installed_capability_identity(),
        operation,
        WorthQueryElevationLifecycleOperationRole::Approve,
    )?;
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
    let (elevation, review) =
        resolve_approval_entities(runtime, access, installed, requested_binding)?;
    Ok(WorthQueryElevationApprovalDraft {
        elevation,
        review,
        approver: access.principal_entity_id,
        approved_status: lifecycle.lifecycle.approved.clone(),
        elevation_entity: elevation_definition.status().entity().to_string(),
        status_field: lifecycle.lifecycle.status.clone(),
        approver_relation: lifecycle.lifecycle.approver_relation,
        reviewer_relation: lifecycle.lifecycle.reviewer_relation,
        required_decision_reads: lifecycle_decision_reads(installed),
        required_program_targets: approval_program_targets(installed),
        lifecycle_effect: super::lifecycle_effect::derive_lifecycle_effect(
            elevation_definition.lifecycle().approve(),
            &access.input,
            installed.contract.name(),
        )?,
    })
}

fn resolve_approval_entities<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    installed: &WorthQueryInstalledCapabilityPlan,
    requested: &super::WorthQueryElevationRequestBinding,
) -> Result<(EntityId, EntityId), WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let definition = installed.contract.elevation().definition().unwrap();
    let elevation = selected_elevation_entity(access, installed)
        .ok_or_else(|| approval_rejected(installed.contract.name()))?;
    let resolved_elevation = resolve_lifecycle_identity(
        runtime,
        access,
        definition.identity(),
        &requested.elevation_identity,
    )
    .ok_or_else(|| approval_rejected(installed.contract.name()))?;
    let review = resolve_lifecycle_identity(
        runtime,
        access,
        definition.review().identity(),
        &requested.review_identity,
    )
    .ok_or_else(|| approval_rejected(installed.contract.name()))?;
    if resolved_elevation == elevation {
        Ok((elevation, review))
    } else {
        Err(approval_rejected(installed.contract.name()))
    }
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
    {
        return Err(approval_rejected(installed.contract.name()));
    }
    Ok(())
}

fn validate_approval_time(
    requested: &super::WorthQueryElevationRequestBinding,
    sample: &super::super::WorthQueryRuntimeTimeSample,
    subject: &str,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    match (sample.value(), &requested.issued_at, &requested.expires_at) {
        (AspectValue::UInt64(now), AspectValue::UInt64(start), AspectValue::UInt64(end))
            if start <= now && now < end =>
        {
            Ok(())
        }
        _ => Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationExpired,
            subject,
        )),
    }
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

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
