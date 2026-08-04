use super::super::ResourceRuntimeState;
use crate::data::node::NodeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;
use crate::data::temporal::ReadyTemporalWake;

impl ResourceRuntimeState {
    pub fn validate_resource_revalidation_intent(
        &self,
        intent: ResourceRevalidationIntent,
    ) -> Option<ResourceRevalidationDenialClass> {
        let node = intent.node();
        if !self.descriptors_by_node.contains_key(&node) {
            return Some(ResourceRevalidationDenialClass::UndeclaredResourceNode);
        }

        match (
            self.active_request_by_node.get(&node).copied(),
            intent.expected_active(),
        ) {
            (Some(_), None) => {
                Some(ResourceRevalidationDenialClass::ActiveRequestRequiresExpectedHandle)
            }
            (Some(active_request_id), Some(expected)) => self
                .in_flight_by_request
                .get(&active_request_id)
                .cloned()
                .filter(|in_flight| in_flight.handle() == expected)
                .filter(|in_flight| in_flight.status() == ResourceInFlightStatus::Active)
                .filter(|in_flight| in_flight.lifecycle() == ResourceLifecycleClass::Pending)
                .map(|_| None)
                .unwrap_or(Some(
                    ResourceRevalidationDenialClass::ExpectedActiveRequestMismatch,
                )),
            (None, Some(_)) => Some(ResourceRevalidationDenialClass::ExpectedActiveRequestMismatch),
            (None, None) => None,
        }
    }
    pub fn prove_active_resource_revalidation_handle(
        &self,
        handle: ResourceRequestHandle,
        telemetry: &mut ResourceTelemetry,
    ) -> Option<ActiveResourceRevalidationProof> {
        telemetry.resource_revalidation_active_handle_proof_check_count += 1;
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let in_flight = self.in_flight_by_request.get(&handle.request_id())?;
        if in_flight.handle() != handle
            || in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return None;
        }
        let node = in_flight.node();
        let active_request_id = self.active_request_by_node.get(&node).copied()?;
        if active_request_id != handle.request_id() {
            return None;
        }
        let descriptor = self.descriptor_for_node(node)?;
        Some(ActiveResourceRevalidationProof::new(
            node,
            handle,
            descriptor
                .revalidation_decision_plan()
                .decision_digest()
                .clone(),
        ))
    }
    pub fn validate_forced_resource_revalidation_proof(
        &self,
        proof: &ActiveResourceRevalidationProof,
    ) -> Option<ResourceRevalidationDenialClass> {
        let Some(descriptor) = self.descriptor_for_node(proof.node()) else {
            return Some(ResourceRevalidationDenialClass::UndeclaredResourceNode);
        };
        if !descriptor
            .revalidation_decision_plan()
            .permits_active_handle_forcing()
        {
            return Some(ResourceRevalidationDenialClass::ForcedRevalidationPolicyDisabled);
        }
        let Some(in_flight) = self.in_flight_by_request.get(&proof.handle().request_id()) else {
            return Some(ResourceRevalidationDenialClass::ActiveHandleProofMismatch);
        };
        if in_flight.handle() != proof.handle()
            || in_flight.node() != proof.node()
            || in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return Some(ResourceRevalidationDenialClass::ActiveHandleProofMismatch);
        }
        let Some(active_request_id) = self.active_request_by_node.get(&proof.node()).copied()
        else {
            return Some(ResourceRevalidationDenialClass::ActiveHandleProofMismatch);
        };
        if active_request_id != proof.handle().request_id()
            || descriptor.revalidation_decision_plan().decision_digest() != proof.decision_digest()
        {
            return Some(ResourceRevalidationDenialClass::ActiveHandleProofMismatch);
        }
        None
    }
    pub fn prove_dependency_change_resource_revalidation(
        &self,
        node: ResourceNodeId,
        node_state: NodeState,
        telemetry: &mut ResourceTelemetry,
    ) -> Option<DependencyChangeResourceRevalidationProof> {
        telemetry.resource_revalidation_dependency_change_proof_check_count += 1;
        let descriptor = self.descriptor_for_node(node)?;
        if !descriptor
            .revalidation_decision_plan()
            .permits_dependency_change_revalidation()
        {
            return None;
        }
        match node_state {
            NodeState::Dirty | NodeState::MaybeStale => {
                Some(DependencyChangeResourceRevalidationProof::new(
                    node,
                    node_state,
                    descriptor
                        .revalidation_decision_plan()
                        .decision_digest()
                        .clone(),
                ))
            }
            NodeState::Clean => None,
        }
    }
    pub fn validate_dependency_change_resource_revalidation_proof(
        &self,
        proof: &DependencyChangeResourceRevalidationProof,
        current_node_state: NodeState,
    ) -> Option<ResourceRevalidationDenialClass> {
        let Some(descriptor) = self.descriptor_for_node(proof.node()) else {
            return Some(ResourceRevalidationDenialClass::UndeclaredResourceNode);
        };
        if !descriptor
            .revalidation_decision_plan()
            .permits_dependency_change_revalidation()
        {
            return Some(
                ResourceRevalidationDenialClass::DependencyChangeRevalidationPolicyDisabled,
            );
        }
        if !matches!(current_node_state, NodeState::Dirty | NodeState::MaybeStale)
            || current_node_state != proof.node_state()
            || descriptor.revalidation_decision_plan().decision_digest() != proof.decision_digest()
        {
            return Some(ResourceRevalidationDenialClass::DependencyChangeProofMismatch);
        }
        None
    }
    pub fn validate_observer_demand_resource_revalidation_proof(
        &self,
        proof: &ObserverDemandResourceRevalidationProof,
    ) -> Option<ResourceRevalidationDenialClass> {
        let Some(descriptor) = self.descriptor_for_node(proof.node()) else {
            return Some(ResourceRevalidationDenialClass::UndeclaredResourceNode);
        };
        if !descriptor
            .revalidation_decision_plan()
            .permits_observer_demand_revalidation()
        {
            return Some(ResourceRevalidationDenialClass::ObserverDemandRevalidationPolicyDisabled);
        }
        if descriptor.revalidation_decision_plan().decision_digest() != proof.decision_digest() {
            return Some(ResourceRevalidationDenialClass::ObserverDemandProofMismatch);
        }
        None
    }
    pub fn prove_terminal_state_resource_revalidation(
        &self,
        node: ResourceNodeId,
        telemetry: &mut ResourceTelemetry,
    ) -> Option<TerminalStateResourceRevalidationProof> {
        telemetry.resource_revalidation_terminal_state_proof_check_count += 1;
        let descriptor = self.descriptor_for_node(node)?;
        if !descriptor
            .revalidation_decision_plan()
            .permits_terminal_state_revalidation()
        {
            return None;
        }
        let lifecycle = self.lifecycle_by_node.get(&node)?.lifecycle();
        let lifecycle_ordinal = self.lifecycle_by_node.get(&node)?.lifecycle_ordinal();
        if !lifecycle.is_terminal() {
            return None;
        }
        Some(TerminalStateResourceRevalidationProof::new(
            node,
            lifecycle,
            lifecycle_ordinal,
            descriptor
                .revalidation_decision_plan()
                .decision_digest()
                .clone(),
        ))
    }
    pub fn validate_terminal_state_resource_revalidation_proof(
        &self,
        proof: &TerminalStateResourceRevalidationProof,
    ) -> Option<ResourceRevalidationDenialClass> {
        let Some(descriptor) = self.descriptor_for_node(proof.node()) else {
            return Some(ResourceRevalidationDenialClass::UndeclaredResourceNode);
        };
        if !descriptor
            .revalidation_decision_plan()
            .permits_terminal_state_revalidation()
        {
            return Some(ResourceRevalidationDenialClass::TerminalStateRevalidationPolicyDisabled);
        }
        let Some(lifecycle) = self.lifecycle_by_node.get(&proof.node()).copied() else {
            return Some(ResourceRevalidationDenialClass::TerminalStateProofMismatch);
        };
        if !lifecycle.lifecycle().is_terminal()
            || lifecycle.lifecycle() != proof.lifecycle()
            || lifecycle.lifecycle_ordinal() != proof.lifecycle_ordinal()
            || descriptor.revalidation_decision_plan().decision_digest() != proof.decision_digest()
        {
            return Some(ResourceRevalidationDenialClass::TerminalStateProofMismatch);
        }
        None
    }
    pub fn prove_fulfilled_lifecycle_resource_revalidation(
        &self,
        node: ResourceNodeId,
        telemetry: &mut ResourceTelemetry,
    ) -> Option<FulfilledLifecycleResourceRevalidationProof> {
        telemetry.resource_revalidation_fulfilled_lifecycle_proof_check_count += 1;
        let descriptor = self.descriptor_for_node(node)?;
        if !descriptor
            .revalidation_decision_plan()
            .permits_fulfilled_lifecycle_revalidation()
        {
            return None;
        }
        let lifecycle = self.lifecycle_by_node.get(&node)?.lifecycle();
        let lifecycle_ordinal = self.lifecycle_by_node.get(&node)?.lifecycle_ordinal();
        if lifecycle != ResourceLifecycleClass::Fulfilled {
            return None;
        }
        Some(FulfilledLifecycleResourceRevalidationProof::new(
            node,
            lifecycle_ordinal,
            descriptor
                .revalidation_decision_plan()
                .decision_digest()
                .clone(),
        ))
    }
    pub fn validate_fulfilled_lifecycle_resource_revalidation_proof(
        &self,
        proof: &FulfilledLifecycleResourceRevalidationProof,
    ) -> Option<ResourceRevalidationDenialClass> {
        let Some(descriptor) = self.descriptor_for_node(proof.node()) else {
            return Some(ResourceRevalidationDenialClass::UndeclaredResourceNode);
        };
        if !descriptor
            .revalidation_decision_plan()
            .permits_fulfilled_lifecycle_revalidation()
        {
            return Some(
                ResourceRevalidationDenialClass::FulfilledLifecycleRevalidationPolicyDisabled,
            );
        }
        let Some(lifecycle) = self.lifecycle_by_node.get(&proof.node()).copied() else {
            return Some(ResourceRevalidationDenialClass::FulfilledLifecycleProofMismatch);
        };
        if lifecycle.lifecycle() != ResourceLifecycleClass::Fulfilled
            || lifecycle.lifecycle_ordinal() != proof.lifecycle_ordinal()
            || descriptor.revalidation_decision_plan().decision_digest() != proof.decision_digest()
        {
            return Some(ResourceRevalidationDenialClass::FulfilledLifecycleProofMismatch);
        }
        None
    }
    pub fn validate_stale_after_resource_revalidation(
        &self,
        node: ResourceNodeId,
        ready_wake: &ReadyTemporalWake,
    ) -> Option<ResourceRevalidationDenialClass> {
        let Some(descriptor) = self.descriptor_for_node(node) else {
            return Some(ResourceRevalidationDenialClass::UndeclaredResourceNode);
        };
        let revalidation_plan = descriptor.revalidation_decision_plan();
        if !revalidation_plan.permits_stale_after_revalidation()
            || !descriptor.stale_after_decision_plan().is_enabled()
        {
            return Some(ResourceRevalidationDenialClass::StaleAfterRevalidationPolicyDisabled);
        }
        if revalidation_plan.stale_after_requires_fulfilled_lifecycle()
            && self
                .lifecycle_by_node
                .get(&node)
                .is_none_or(|lifecycle| lifecycle.lifecycle() != ResourceLifecycleClass::Fulfilled)
        {
            return Some(ResourceRevalidationDenialClass::StaleAfterRequiresFulfilledLifecycle);
        }
        match self.stale_after_wake_by_node.get(&node).copied() {
            Some(wake_id) if wake_id == ready_wake.id() => None,
            _ => Some(ResourceRevalidationDenialClass::StaleAfterWakeMismatch),
        }
    }
}
