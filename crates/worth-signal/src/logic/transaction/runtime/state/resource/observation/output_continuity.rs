use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::logic::transaction::runtime::state::resource) enum ResourceTerminalVisibilityCause {
    Rejection,
    Timeout,
    Cancellation,
    Supersession,
}
impl ResourceRuntimeState {
    pub(in crate::logic::transaction::runtime::state::resource) fn pending_output_continuity_for_node_optional(
        &self,
        node: ResourceNodeId,
        descriptor_id: ResourceDescriptorId,
        telemetry: Option<&mut ResourceTelemetry>,
    ) -> ResourceOutputContinuity {
        let continuity = match (
            self.descriptors.get(&descriptor_id),
            self.current_lifecycle_summary(node),
        ) {
            (Some(descriptor), Some(current))
                if current.output_continuity() != ResourceOutputContinuity::NoPriorOutput =>
            {
                if descriptor
                    .output_continuity_decision_plan()
                    .preserves_previous_output_while_pending()
                {
                    ResourceOutputContinuity::PriorOutputPreserved
                } else {
                    ResourceOutputContinuity::OutputUnavailableByPolicy
                }
            }
            _ => ResourceOutputContinuity::NoPriorOutput,
        };
        self.record_output_continuity_decision_optional(continuity, telemetry);
        continuity
    }

    pub(in crate::logic::transaction::runtime::state::resource) fn classify_terminal_output_continuity_for_node_optional(
        &self,
        node: ResourceNodeId,
        descriptor_id: ResourceDescriptorId,
        cause: ResourceTerminalVisibilityCause,
        telemetry: Option<&mut ResourceTelemetry>,
    ) -> (ResourceOutputContinuity, bool) {
        let prior_output_exists = self
            .current_lifecycle_summary(node)
            .map(|summary| summary.output_continuity() != ResourceOutputContinuity::NoPriorOutput)
            .unwrap_or(false);
        if !prior_output_exists {
            return (ResourceOutputContinuity::NoPriorOutput, false);
        }
        let descriptor = self
            .descriptors
            .get(&descriptor_id)
            .expect("output continuity classification requires a declared descriptor");
        let plan = descriptor.output_continuity_decision_plan();
        let preserves = match cause {
            ResourceTerminalVisibilityCause::Rejection => {
                plan.preserves_previous_output_after_rejection()
            }
            ResourceTerminalVisibilityCause::Timeout => {
                plan.preserves_previous_output_after_timeout()
            }
            ResourceTerminalVisibilityCause::Cancellation => {
                plan.preserves_previous_output_after_cancellation()
            }
            ResourceTerminalVisibilityCause::Supersession => {
                plan.preserves_previous_output_after_supersession()
            }
        };
        let continuity = if preserves {
            ResourceOutputContinuity::PriorOutputPreserved
        } else {
            ResourceOutputContinuity::OutputUnavailableByPolicy
        };
        self.record_output_continuity_decision_optional(continuity, telemetry);
        (continuity, true)
    }

    pub(in crate::logic::transaction::runtime::state::resource) fn record_output_continuity_decision_optional(
        &self,
        continuity: ResourceOutputContinuity,
        telemetry: Option<&mut ResourceTelemetry>,
    ) {
        let Some(telemetry) = telemetry else {
            return;
        };
        telemetry.resource_output_continuity_decision_count += 1;
        match continuity {
            ResourceOutputContinuity::PriorOutputPreserved => {
                telemetry.resource_previous_output_preserved_count += 1;
            }
            ResourceOutputContinuity::OutputUnavailableByPolicy => {
                telemetry.resource_previous_output_hidden_count += 1;
            }
            _ => {}
        }
    }
}
