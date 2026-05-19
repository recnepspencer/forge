use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::identity::hash_parts;
use crate::memory_workspace::ForgeQueryMutationReceipt;
use crate::runtime::ForgeQueryWriteCommand;
use crate::subscription::SubscriptionActivationInput;

use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeReadmissionReceipt,
    ForgeQueryLowerRuntimeRouteKind, ForgeQueryLowerRuntimeRoutePlan,
    ForgeQueryLowerRuntimeSeamKey,
};
use crate::runtime::{
    LiveViewDeclarationAdmissionReceipt, SignalInvalidationRoutingReceipt,
    SubscriptionActivationReceipt,
};

const LIVE_VIEW_CAPABILITY_LABEL: &str = "live-view-schema-admission";
const WRITE_AUTHORITY_CAPABILITY_LABEL: &str = "write-authority-backend-execution";
const SIGNAL_INVALIDATION_CAPABILITY_LABEL: &str = "signal-invalidation-routing";
const SUBSCRIPTION_ACTIVATION_CAPABILITY_LABEL: &str = "subscription-activation";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveViewDeclarationAdmissionBoundaryReceipt {
    admission_receipt: LiveViewDeclarationAdmissionReceipt,
    readmission_receipt: ForgeQueryLowerRuntimeReadmissionReceipt,
    boundary_execution_receipt: ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
    boundary_envelope: ForgeQueryLowerRuntimeBoundaryEnvelope,
}

impl LiveViewDeclarationAdmissionBoundaryReceipt {
    pub(crate) fn from_request(
        view_name: &str,
        request: &DeclarativeLiveQueryRequest,
        admission_receipt: LiveViewDeclarationAdmissionReceipt,
    ) -> Self {
        let capability_request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
            ForgeQueryLowerRuntimeRouteKind::ReadmissionHandoff,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            LIVE_VIEW_CAPABILITY_LABEL,
            live_view_subject_digest(view_name, request),
        );
        let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
            capability_request,
            admission_receipt.receipt_digest(),
        );
        let readmission_receipt = ForgeQueryLowerRuntimeReadmissionReceipt::new(
            eligibility,
            admission_receipt.receipt_digest(),
        );
        let boundary_execution_receipt =
            ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_readmission_receipt(
                &readmission_receipt,
            );
        let boundary_envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_readmission_receipt(
            ForgeQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
            &readmission_receipt,
            &boundary_execution_receipt,
        );
        Self {
            admission_receipt,
            readmission_receipt,
            boundary_execution_receipt,
            boundary_envelope,
        }
    }

    pub fn admission_receipt(&self) -> &LiveViewDeclarationAdmissionReceipt {
        &self.admission_receipt
    }

    pub fn readmission_receipt(&self) -> &ForgeQueryLowerRuntimeReadmissionReceipt {
        &self.readmission_receipt
    }

    pub fn boundary_execution_receipt(&self) -> &ForgeQueryLowerRuntimeBoundaryExecutionReceipt {
        &self.boundary_execution_receipt
    }

    pub fn boundary_envelope(&self) -> &ForgeQueryLowerRuntimeBoundaryEnvelope {
        &self.boundary_envelope
    }

    pub(crate) fn drift_from_request(
        &self,
        view_name: &str,
        request: &DeclarativeLiveQueryRequest,
    ) -> Option<String> {
        if let Some(message) = self
            .admission_receipt
            .drift_from_request(view_name, request)
        {
            return Some(message);
        }
        let expected_subject = live_view_subject_digest(view_name, request);
        if let Some(message) = self
            .readmission_receipt
            .eligibility()
            .request()
            .drift_from_contract(
                ForgeQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
                ForgeQueryLowerRuntimeRouteKind::ReadmissionHandoff,
                ForgeQueryLowerRuntimeAuthorityOwner::Query,
                LIVE_VIEW_CAPABILITY_LABEL,
                &expected_subject,
            )
        {
            return Some(message);
        }
        self.boundary_execution_receipt
            .drift_from_readmission_receipt(&self.readmission_receipt)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WriteAuthorityExecutionReceipt {
    mutation_receipt: ForgeQueryMutationReceipt,
    route_plan: ForgeQueryLowerRuntimeRoutePlan,
    boundary_execution_receipt: ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
    boundary_envelope: ForgeQueryLowerRuntimeBoundaryEnvelope,
}

impl WriteAuthorityExecutionReceipt {
    pub(crate) fn from_command(
        command: &ForgeQueryWriteCommand,
        mutation_receipt: ForgeQueryMutationReceipt,
    ) -> Self {
        let capability_request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            WRITE_AUTHORITY_CAPABILITY_LABEL,
            write_command_subject_digest(command),
        );
        let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
            capability_request,
            mutation_receipt.commit_identity.clone(),
        );
        let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(eligibility, "mutation-write");
        let boundary_execution_receipt =
            ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
                &route_plan,
                mutation_receipt.commit_identity.clone(),
            );
        let boundary_envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
            ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
            &route_plan,
            &boundary_execution_receipt,
            &mutation_receipt.commit_identity,
        );
        Self {
            mutation_receipt,
            route_plan,
            boundary_execution_receipt,
            boundary_envelope,
        }
    }

    pub fn mutation_receipt(&self) -> &ForgeQueryMutationReceipt {
        &self.mutation_receipt
    }

    pub fn route_plan(&self) -> &ForgeQueryLowerRuntimeRoutePlan {
        &self.route_plan
    }

    pub fn boundary_execution_receipt(&self) -> &ForgeQueryLowerRuntimeBoundaryExecutionReceipt {
        &self.boundary_execution_receipt
    }

    pub fn boundary_envelope(&self) -> &ForgeQueryLowerRuntimeBoundaryEnvelope {
        &self.boundary_envelope
    }

    pub(crate) fn drift_from_command(&self, command: &ForgeQueryWriteCommand) -> Option<String> {
        let expected_subject = write_command_subject_digest(command);
        if let Some(message) = self.route_plan.eligibility().request().drift_from_contract(
            ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            WRITE_AUTHORITY_CAPABILITY_LABEL,
            &expected_subject,
        ) {
            return Some(message);
        }
        self.boundary_execution_receipt
            .drift_from_route_plan(&self.route_plan, &self.mutation_receipt.commit_identity)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignalInvalidationBoundaryReceipt {
    routing_receipt: SignalInvalidationRoutingReceipt,
    route_plan: ForgeQueryLowerRuntimeRoutePlan,
    boundary_execution_receipt: ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
    boundary_envelope: ForgeQueryLowerRuntimeBoundaryEnvelope,
}

impl SignalInvalidationBoundaryReceipt {
    pub(crate) fn from_mutation_receipt(
        mutation_receipt: &ForgeQueryMutationReceipt,
        routing_receipt: SignalInvalidationRoutingReceipt,
    ) -> Self {
        let capability_request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            SIGNAL_INVALIDATION_CAPABILITY_LABEL,
            mutation_receipt.commit_identity.clone(),
        );
        let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
            capability_request,
            routing_receipt.receipt_digest(),
        );
        let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(eligibility, "signal-routing");
        let boundary_execution_receipt =
            ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
                &route_plan,
                routing_receipt.receipt_digest(),
            );
        let boundary_envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
            ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
            &route_plan,
            &boundary_execution_receipt,
            routing_receipt.receipt_digest(),
        );
        Self {
            routing_receipt,
            route_plan,
            boundary_execution_receipt,
            boundary_envelope,
        }
    }

    pub fn routing_receipt(&self) -> &SignalInvalidationRoutingReceipt {
        &self.routing_receipt
    }

    pub fn route_plan(&self) -> &ForgeQueryLowerRuntimeRoutePlan {
        &self.route_plan
    }

    pub fn boundary_execution_receipt(&self) -> &ForgeQueryLowerRuntimeBoundaryExecutionReceipt {
        &self.boundary_execution_receipt
    }

    pub fn boundary_envelope(&self) -> &ForgeQueryLowerRuntimeBoundaryEnvelope {
        &self.boundary_envelope
    }

    pub(crate) fn drift_from_mutation_receipt(
        &self,
        mutation_receipt: &ForgeQueryMutationReceipt,
    ) -> Option<String> {
        if let Some(message) = self
            .routing_receipt
            .drift_from_mutation_receipt(mutation_receipt)
        {
            return Some(message);
        }
        if let Some(message) = self.route_plan.eligibility().request().drift_from_contract(
            ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            SIGNAL_INVALIDATION_CAPABILITY_LABEL,
            &mutation_receipt.commit_identity,
        ) {
            return Some(message);
        }
        self.boundary_execution_receipt
            .drift_from_route_plan(&self.route_plan, self.routing_receipt.receipt_digest())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionActivationBoundaryReceipt {
    activation_receipt: SubscriptionActivationReceipt,
    route_plan: ForgeQueryLowerRuntimeRoutePlan,
    boundary_execution_receipt: ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
    boundary_envelope: ForgeQueryLowerRuntimeBoundaryEnvelope,
}

impl SubscriptionActivationBoundaryReceipt {
    pub(crate) fn from_activation(
        view_name: &str,
        activation: &SubscriptionActivationInput,
        activation_receipt: SubscriptionActivationReceipt,
    ) -> Self {
        let capability_request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::SubscriptionActivation,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            SUBSCRIPTION_ACTIVATION_CAPABILITY_LABEL,
            activation_subject_digest(view_name, activation),
        );
        let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
            capability_request,
            activation_receipt.receipt_digest(),
        );
        let route_plan =
            ForgeQueryLowerRuntimeRoutePlan::new(eligibility, "subscription-activation");
        let boundary_execution_receipt =
            ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
                &route_plan,
                activation_receipt.receipt_digest(),
            );
        let boundary_envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
            ForgeQueryLowerRuntimeSeamKey::SubscriptionActivation,
            &route_plan,
            &boundary_execution_receipt,
            activation_receipt.receipt_digest(),
        );
        Self {
            activation_receipt,
            route_plan,
            boundary_execution_receipt,
            boundary_envelope,
        }
    }

    pub fn activation_receipt(&self) -> &SubscriptionActivationReceipt {
        &self.activation_receipt
    }

    pub fn route_plan(&self) -> &ForgeQueryLowerRuntimeRoutePlan {
        &self.route_plan
    }

    pub fn boundary_execution_receipt(&self) -> &ForgeQueryLowerRuntimeBoundaryExecutionReceipt {
        &self.boundary_execution_receipt
    }

    pub fn boundary_envelope(&self) -> &ForgeQueryLowerRuntimeBoundaryEnvelope {
        &self.boundary_envelope
    }

    pub(crate) fn drift_from_activation(
        &self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Option<String> {
        if let Some(message) = self
            .activation_receipt
            .drift_from_activation(view_name, activation)
        {
            return Some(message);
        }
        let expected_subject = activation_subject_digest(view_name, activation);
        if let Some(message) = self.route_plan.eligibility().request().drift_from_contract(
            ForgeQueryLowerRuntimeSeamKey::SubscriptionActivation,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            SUBSCRIPTION_ACTIVATION_CAPABILITY_LABEL,
            &expected_subject,
        ) {
            return Some(message);
        }
        self.boundary_execution_receipt
            .drift_from_route_plan(&self.route_plan, self.activation_receipt.receipt_digest())
    }
}

fn live_view_subject_digest(view_name: &str, request: &DeclarativeLiveQueryRequest) -> String {
    hash_parts(&[
        "live_view_route_subject_v1".to_string(),
        format!("view:{view_name}"),
        format!("target:{}", request.target()),
        format!("shape:{}", request.view_shape().as_str()),
        format!("projection_count:{}", request.query_projection().len()),
        format!("result_count:{}", request.result_fields().len()),
    ])
}

fn write_command_subject_digest(command: &ForgeQueryWriteCommand) -> String {
    hash_parts(&[
        "write_command_route_subject_v1".to_string(),
        format!("family:{}", command.mutation_family().as_str()),
        format!(
            "collection:{}",
            command.declared_collection_ref().unwrap_or("")
        ),
        format!(
            "entity:{}",
            command.declared_entity_identity_ref().unwrap_or("")
        ),
        format!(
            "aspect_operations:{}",
            command.declared_aspect_operations().len()
        ),
        format!("touched_aspects:{}", command.declared_aspect_paths().len()),
    ])
}

fn activation_subject_digest(view_name: &str, activation: &SubscriptionActivationInput) -> String {
    hash_parts(&[
        "subscription_activation_route_subject_v1".to_string(),
        format!("view:{view_name}"),
        format!("activation:{}", activation.activation_digest()),
        format!(
            "query_declaration:{}",
            activation.query_declaration_digest()
        ),
        format!(
            "bridge_declaration:{}",
            activation.bridge_declaration_digest()
        ),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_plan_drift_rejects_foreign_boundary_receipt() {
        let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            WRITE_AUTHORITY_CAPABILITY_LABEL,
            "subject-a",
        );
        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted(request, "detail-a");
        let plan = ForgeQueryLowerRuntimeRoutePlan::new(eligibility, "route-a");

        let foreign_request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            SIGNAL_INVALIDATION_CAPABILITY_LABEL,
            "subject-b",
        );
        let foreign_eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted(foreign_request, "detail-b");
        let foreign_plan = ForgeQueryLowerRuntimeRoutePlan::new(foreign_eligibility, "route-b");
        let foreign_receipt = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
            &foreign_plan,
            "detail-b",
        );

        let drift = foreign_receipt
            .drift_from_route_plan(&plan, "detail-a")
            .expect("foreign route receipt must drift");

        assert!(drift.contains("boundary execution request digest"));
    }

    #[test]
    fn write_authority_boundary_receipt_carries_boundary_envelope() {
        let command = ForgeQueryWriteCommand::Delete {
            entity_identity: "task-1".to_string(),
        };
        let mutation_receipt = ForgeQueryMutationReceipt {
            commit_identity: "commit-1".to_string(),
            snapshot_token: "snapshot-1".to_string(),
            deltas: Vec::new(),
            bridge_authority: None,
        };
        let receipt = WriteAuthorityExecutionReceipt::from_command(&command, mutation_receipt);

        assert_eq!(
            receipt.boundary_envelope().seam_key(),
            ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution
        );
        assert_eq!(
            receipt.boundary_envelope().boundary_execution_digest(),
            receipt
                .boundary_execution_receipt()
                .boundary_execution_digest()
        );
    }
}
