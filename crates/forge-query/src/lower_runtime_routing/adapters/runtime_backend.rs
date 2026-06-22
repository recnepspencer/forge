use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::memory_workspace::ForgeQueryMutationReceipt;
use crate::runtime::ForgeQueryWriteCommand;
use crate::subscription::SubscriptionActivationInput;

use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeReadmissionReceipt,
    ForgeQueryLowerRuntimeRetainedEvidenceIdentity, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeRouteSubjectIdentity,
    ForgeQueryLowerRuntimeSeamKey,
};
use crate::runtime::{
    LiveViewDeclarationAdmissionReceipt, SignalInvalidationRoutingReceipt,
    SubscriptionActivationReceipt,
};

use self::subject_digest::{
    activation_subject_identity, live_view_subject_identity, signal_invalidation_subject_identity,
    write_command_subject_identity,
};

mod subject_digest;

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
            live_view_subject_identity(view_name, request),
        );
        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
                capability_request,
                admission_receipt.receipt_identity(),
            );
        let readmission_receipt = ForgeQueryLowerRuntimeReadmissionReceipt::new(
            eligibility,
            &ForgeQueryLowerRuntimeRetainedEvidenceIdentity::from_evidence_identity(
                "live-view-declaration-admission",
                admission_receipt.receipt_identity(),
            ),
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
        let expected_subject = live_view_subject_identity(view_name, request);
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
            write_command_subject_identity(command),
        );
        let commit_evidence_identity = mutation_receipt.commit_identity.evidence_identity();
        let retained_evidence_identity =
            ForgeQueryLowerRuntimeRetainedEvidenceIdentity::from_evidence_identity(
                "write-authority-commit",
                &commit_evidence_identity,
            );
        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
                capability_request,
                &commit_evidence_identity,
            );
        let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(
            eligibility,
            ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
                "write-authority-route",
                &commit_evidence_identity,
            ),
        );
        let boundary_execution_receipt =
            ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan_with_retained_evidence_identity(
                &route_plan,
                &retained_evidence_identity,
            );
        let boundary_envelope =
            ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan_with_retained_evidence_identity(
                ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
                &route_plan,
                &boundary_execution_receipt,
                &retained_evidence_identity,
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
        let expected_subject = write_command_subject_identity(command);
        if let Some(message) = self.route_plan.eligibility().request().drift_from_contract(
            ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            WRITE_AUTHORITY_CAPABILITY_LABEL,
            &expected_subject,
        ) {
            return Some(message);
        }
        let commit_evidence_identity = self.mutation_receipt.commit_identity.evidence_identity();
        let retained_evidence_identity =
            ForgeQueryLowerRuntimeRetainedEvidenceIdentity::from_evidence_identity(
                "write-authority-commit",
                &commit_evidence_identity,
            );
        self.boundary_execution_receipt
            .drift_from_route_plan_with_retained_evidence_identity(
                &self.route_plan,
                &retained_evidence_identity,
            )
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
        _mutation_receipt: &ForgeQueryMutationReceipt,
        routing_receipt: SignalInvalidationRoutingReceipt,
    ) -> Self {
        let capability_request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            SIGNAL_INVALIDATION_CAPABILITY_LABEL,
            signal_invalidation_subject_identity(&routing_receipt),
        );
        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
                capability_request,
                routing_receipt.receipt_identity(),
            );
        let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(
            eligibility,
            ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
                "signal-invalidation-route",
                routing_receipt.receipt_identity(),
            ),
        );
        let retained_evidence_identity =
            ForgeQueryLowerRuntimeRetainedEvidenceIdentity::from_evidence_identity(
                "signal-invalidation-routing",
                routing_receipt.receipt_identity(),
            );
        let boundary_execution_receipt =
            ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan_with_retained_evidence_identity(
                &route_plan,
                &retained_evidence_identity,
            );
        let boundary_envelope =
            ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan_with_retained_evidence_identity(
                ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
                &route_plan,
                &boundary_execution_receipt,
                &retained_evidence_identity,
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
            &signal_invalidation_subject_identity(&self.routing_receipt),
        ) {
            return Some(message);
        }
        self.boundary_execution_receipt
            .drift_from_route_plan_with_retained_evidence_identity(
                &self.route_plan,
                &ForgeQueryLowerRuntimeRetainedEvidenceIdentity::from_evidence_identity(
                    "signal-invalidation-routing",
                    self.routing_receipt.receipt_identity(),
                ),
            )
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
            activation_subject_identity(view_name, activation, &activation_receipt),
        );
        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
                capability_request,
                activation_receipt.receipt_identity(),
            );
        let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(
            eligibility,
            ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
                "subscription-activation-route",
                activation_receipt.receipt_identity(),
            ),
        );
        let retained_evidence_identity =
            ForgeQueryLowerRuntimeRetainedEvidenceIdentity::from_evidence_identity(
                "subscription-activation",
                activation_receipt.receipt_identity(),
            );
        let boundary_execution_receipt =
            ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan_with_retained_evidence_identity(
                &route_plan,
                &retained_evidence_identity,
            );
        let boundary_envelope =
            ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan_with_retained_evidence_identity(
                ForgeQueryLowerRuntimeSeamKey::SubscriptionActivation,
                &route_plan,
                &boundary_execution_receipt,
                &retained_evidence_identity,
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
        let expected_subject =
            activation_subject_identity(view_name, activation, &self.activation_receipt);
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
            .drift_from_route_plan_with_retained_evidence_identity(
                &self.route_plan,
                &ForgeQueryLowerRuntimeRetainedEvidenceIdentity::from_evidence_identity(
                    "subscription-activation",
                    self.activation_receipt.receipt_identity(),
                ),
            )
    }
}

#[cfg(test)]
mod tests;
