use super::super::policy::registry::resource_policy_resolution_signal_error;
use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

impl ResourceRuntimeState {
    pub fn declare_resource_node(
        &mut self,
        declaration: ResourceNodeDeclaration,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<ResourceDeclarationReport, crate::data::error::SignalError> {
        let node = declaration.node();
        if self.descriptors_by_node.contains_key(&node) {
            telemetry.resource_duplicate_declaration_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "resource node {} already has a lowered resource descriptor",
                node.node()
            )));
        }

        telemetry.resource_policy_resolution_count += 1;
        let validated_policy_declaration =
            match ValidatedResourcePolicyDeclaration::from_declaration(
                &declaration,
                &self.policy_registry,
            ) {
                Ok(validated) => validated,
                Err(err) => {
                    telemetry.resource_policy_resolution_denial_count += 1;
                    return Err(resource_policy_resolution_signal_error(err));
                }
            };
        let frozen_policy_descriptors =
            match FrozenResourcePolicyDescriptorSet::from_validated_declaration(
                &validated_policy_declaration,
                &self.policy_registry,
            ) {
                Ok(frozen) => frozen,
                Err(err) => {
                    telemetry.resource_policy_resolution_denial_count += 1;
                    return Err(resource_policy_resolution_signal_error(err));
                }
            };
        let lowered_policy_bundle =
            LoweredResourcePolicyBundle::from_frozen_descriptors(&frozen_policy_descriptors);
        let descriptor_id = self.issue_descriptor_id();
        let descriptor = match LoweredResourceDescriptor::from_validated_policy_declaration(
            descriptor_id,
            ResourceDescriptorVersion::INITIAL,
            &validated_policy_declaration,
            lowered_policy_bundle,
        ) {
            Ok(descriptor) => descriptor,
            Err(err) => {
                telemetry.resource_policy_resolution_denial_count += 1;
                return Err(resource_policy_resolution_signal_error(err));
            }
        };
        self.descriptors_by_node.insert(node, descriptor_id);
        self.descriptors.insert(descriptor_id, descriptor);
        let ordinal = self.issue_lifecycle_ordinal();
        let lifecycle = ResourceLifecycleSummary::new(
            node,
            ResourceLifecycleClass::Unrequested,
            ResourceOutputContinuity::NoPriorOutput,
            ordinal,
        );
        let transition = ResourceLifecycleTransition::new(
            node,
            ResourceLifecycleClass::Unrequested,
            ResourceLifecycleClass::Unrequested,
            ResourceLifecycleTransitionKind::DeclarationInitialized,
            ordinal,
            ResourceOutputContinuity::NoPriorOutput,
        );
        self.lifecycle_by_node.insert(node, lifecycle);

        telemetry.resource_declaration_lowering_count += 1;
        telemetry.resource_descriptor_count = self.descriptors.len() as u64;
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::declaration_lowering(1),
        );

        Ok(ResourceDeclarationReport::new(
            descriptor_id,
            lifecycle,
            transition,
            performance,
        ))
    }
}
