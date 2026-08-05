use crate::data::resource::{
    LoweredResourceDescriptor, ResourceNodeId, ResourceRevalidationReport,
};

use crate::data::temporal::{ReadyTemporalWake, TemporalWakeRetirementReason};

use super::super::super::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn active_resource_stale_after_wake_for_node(
        &self,
        node: ResourceNodeId,
    ) -> Option<crate::data::temporal::TemporalWakeId> {
        self.resource.active_stale_after_wake_for_node(node)
    }

    pub fn admit_stale_after_resource_revalidation(
        &mut self,
        node: ResourceNodeId,
        ready_wake: ReadyTemporalWake,
    ) -> Result<ResourceRevalidationReport, crate::data::error::SignalError> {
        if !self.graph.is_alive(node.node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot stale-after revalidate non-live resource node {}",
                node.node()
            )));
        }

        self.telemetry
            .resource
            .resource_revalidation_policy_decision_count += 1;
        let validation = self
            .resource
            .validate_stale_after_resource_revalidation(node, &ready_wake);
        let wake_id = ready_wake.id();
        let matched_active_wake = self
            .resource
            .active_stale_after_wake_for_node(node)
            .is_some_and(|active| active == wake_id);
        if matched_active_wake {
            self.retire_temporal_wake(wake_id, TemporalWakeRetirementReason::Consumed)?;
            self.resource.clear_stale_after_wake_for_node(node);
        }
        if let Some(class) = validation {
            return Ok(self.resource.deny_resource_revalidation_for_report(
                node,
                class,
                &mut self.telemetry.resource,
            ));
        }

        self.telemetry
            .resource
            .resource_stale_after_revalidation_count += 1;
        let current_tick = self.clock_basis().current_tick();
        let revalidation_descriptor = self.resource.descriptor_for_node(node);
        let timeout_plan = revalidation_descriptor
            .map(|descriptor| descriptor.timeout_decision_plan().clone())
            .unwrap_or_else(LoweredResourceDescriptor::default_timeout_decision_plan);
        let revalidation_decision_digest = revalidation_descriptor
            .map(|descriptor| {
                descriptor
                    .revalidation_decision_plan()
                    .decision_digest()
                    .clone()
            })
            .unwrap_or_else(|| {
                crate::data::resource::ResourcePolicyDigest::new(
                    "resource-policy-revalidation-plan:undeclared",
                )
            });
        let prepared_revalidation = match self.resource.prepare_stale_after_resource_revalidation(
            node,
            ready_wake,
            revalidation_decision_digest,
            &mut self.telemetry.resource,
        ) {
            Ok(prepared) => prepared,
            Err(report) => return Ok(report),
        };
        let resolved_timeout =
            self.resolve_timeout_admission(node, &timeout_plan, current_tick, None)?;
        let prior_timeout_wake = self.resource.active_timeout_wake_for_node(node);
        let scheduled_timeout_wake = resolved_timeout
            .as_ref()
            .map(|resolved| self.schedule_resource_timeout_wake(node, resolved))
            .transpose()?;
        let scheduled_timeout_admission = resolved_timeout.map(|resolved| {
            resolved.bind_scheduled_wake(
                scheduled_timeout_wake
                    .as_ref()
                    .expect("resolved timeout must schedule one temporal wake")
                    .id(),
            )
        });
        let report = self.resource.admit_prepared_resource_revalidation(
            prepared_revalidation,
            self.graph.current_branch().id,
            current_tick,
            scheduled_timeout_admission,
            &mut self.telemetry.resource,
        );
        self.reconcile_resource_revalidation_wakes(
            &report,
            node,
            prior_timeout_wake,
            None,
            scheduled_timeout_wake,
        )?;
        Ok(report)
    }
}
