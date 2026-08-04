use worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest;
use worth_query_declaration::facade::application_schema::TypedMutationPreconditions;
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationOperation,
};

use super::super::capability_operation_progression::progress_capability_operation;
use super::super::capability_registry::WorthQueryElevationLifecycleOperationRole;
use super::super::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryAdmittedApplicationOperation,
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};
use super::context_identity::{selected_elevation_entity, selected_review_entity};
use super::mandatory_review_binding::WorthQueryMandatoryReviewDraft;
use super::operation_role::installed_lifecycle_owner;
use super::transition_contract::{lifecycle_decision_reads, review_program_targets};
use crate::domain_computation::primary_graph::{
    WorthQueryElevationClosureKind, WorthQueryMandatoryReview,
    WorthQueryPrimaryGraphApplicationRuntime,
};

#[derive(Debug)]
pub struct WorthQueryMandatoryReviewAuthorizationDenial {
    denial: WorthQueryOperationAuthorizationDenial,
    mandatory: WorthQueryMandatoryReview,
}

impl WorthQueryMandatoryReviewAuthorizationDenial {
    pub const fn denial(&self) -> &WorthQueryOperationAuthorizationDenial {
        &self.denial
    }

    pub fn into_mandatory_review(self) -> WorthQueryMandatoryReview {
        self.mandatory
    }
}

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn authorize_mandatory_review<Capability, Operation, Input>(
        &self,
        mandatory: WorthQueryMandatoryReview,
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
        WorthQueryMandatoryReviewAuthorizationDenial,
    >
    where
        Input: ApplicationCapabilityRequest<Schema, Capability>,
    {
        let draft = match bind_review(self, &mandatory, &access, operation) {
            Ok(draft) => draft,
            Err(denial) => return Err(review_denial(mandatory, denial)),
        };
        let admission =
            match progress_capability_operation(self, access, operation, preconditions, true) {
                Ok(admission) => admission,
                Err(denial) => return Err(review_denial(mandatory, denial)),
            };
        admission
            .bind_mandatory_review(draft.bind(mandatory))
            .map_err(|(denial, binding)| review_denial(binding.into_mandatory(), denial))
    }
}

fn bind_review<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    mandatory: &WorthQueryMandatoryReview,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
) -> Result<WorthQueryMandatoryReviewDraft, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let (capability_identity, installed) = installed_lifecycle_owner(
        runtime,
        access.authorization.installed_capability_identity(),
        operation,
        WorthQueryElevationLifecycleOperationRole::CompleteReview,
    )?;
    if !mandatory.belongs_to_lifecycle(
        runtime.runtime.authority_identity(),
        access.graph_work.branch().relational(),
        capability_identity,
        installed.capability_authority_identity.as_ref(),
    ) || mandatory.resource() != access.resolved.resource.entity_id()
    {
        return Err(review_rejected(installed.contract.name()));
    }
    let elevation = selected_elevation_entity(access, installed)
        .ok_or_else(|| review_rejected(installed.contract.name()))?;
    if elevation != mandatory.elevation() {
        return Err(review_rejected(installed.contract.name()));
    }
    let review = mandatory.review();
    if selected_review_entity(access, installed) != Some(review) {
        return Err(review_rejected(installed.contract.name()));
    }
    let lifecycle = installed.elevation.as_ref().unwrap();
    let sample = runtime
        .authorization_clock
        .sample(lifecycle.temporal.timeline)
        .map_err(|_| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::TrustedTimeUnavailable,
                installed.contract.name(),
            )
        })?;
    let terminal_status = match mandatory.closure_kind() {
        WorthQueryElevationClosureKind::Revoked => lifecycle.lifecycle.revoked.clone(),
        WorthQueryElevationClosureKind::Expired => lifecycle.lifecycle.expired.clone(),
    };
    let definition = installed.contract.elevation().definition().unwrap();
    Ok(WorthQueryMandatoryReviewDraft {
        elevation,
        review,
        reviewer: access.principal_entity_id,
        reviewed_at: sample.value().clone(),
        terminal_status,
        completed_status: lifecycle.lifecycle.review_completed.clone(),
        review_entity: definition.review().status().entity().to_string(),
        review_status_field: lifecycle.lifecycle.review_status.clone(),
        approver_relation: lifecycle.lifecycle.approver_relation,
        reviewer_relation: lifecycle.lifecycle.reviewer_relation,
        required_decision_reads: lifecycle_decision_reads(installed),
        required_program_targets: review_program_targets(installed),
    })
}

fn review_denial(
    mandatory: WorthQueryMandatoryReview,
    denial: WorthQueryOperationAuthorizationDenial,
) -> WorthQueryMandatoryReviewAuthorizationDenial {
    WorthQueryMandatoryReviewAuthorizationDenial { denial, mandatory }
}

fn review_rejected(subject: impl Into<String>) -> WorthQueryOperationAuthorizationDenial {
    denial(
        WorthQueryOperationAuthorizationDenialKind::MandatoryReviewRejected,
        subject,
    )
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
