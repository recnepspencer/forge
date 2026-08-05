use crate::data::resource::{
    LoweredResourceDescriptor, ResourceLifecycleClass, ResourceNodeId, ResourceRevalidationReport,
    TerminalStateResourceRevalidationProof,
};

use super::super::super::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn prove_terminal_state_resource_revalidation(
        &mut self,
        node: ResourceNodeId,
    ) -> Result<TerminalStateResourceRevalidationProof, crate::data::error::SignalError> {
        if !self.graph.is_alive(node.node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot mint terminal-state revalidation proof for non-live resource node {}",
                node.node()
            )));
        }
        let lifecycle = self
            .resource
            .lifecycle_summary_for_node(node)
            .map(|summary| summary.lifecycle())
            .unwrap_or(ResourceLifecycleClass::Unrequested);
        self.resource
            .prove_terminal_state_resource_revalidation(node, &mut self.telemetry.resource)
            .ok_or_else(|| {
                crate::data::error::SignalError::invalid_input(format!(
                    "cannot mint terminal-state revalidation proof for resource node {} in lifecycle {:?}",
                    node.node(),
                    lifecycle
                ))
            })
    }

    pub fn revalidate_resource_node_for_terminal_state(
        &mut self,
        proof: TerminalStateResourceRevalidationProof,
    ) -> Result<ResourceRevalidationReport, crate::data::error::SignalError> {
        let resource_node = proof.node();
        if !self.graph.is_alive(resource_node.node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot terminal-state revalidate non-live resource node {}",
                resource_node.node()
            )));
        }

        let current_tick = self.clock_basis().current_tick();
        self.telemetry
            .resource
            .resource_revalidation_policy_decision_count += 1;
        if let Some(class) = self
            .resource
            .validate_terminal_state_resource_revalidation_proof(&proof)
        {
            return Ok(self.resource.deny_resource_revalidation_for_report(
                proof.node(),
                class,
                &mut self.telemetry.resource,
            ));
        }

        self.telemetry
            .resource
            .resource_terminal_state_revalidation_count += 1;
        let prepared_revalidation = match self
            .resource
            .prepare_terminal_state_resource_revalidation(proof, &mut self.telemetry.resource)
        {
            Ok(prepared) => prepared,
            Err(report) => return Ok(report),
        };
        let timeout_plan = self
            .resource
            .descriptor_for_node(resource_node)
            .map(|descriptor| descriptor.timeout_decision_plan().clone())
            .unwrap_or_else(LoweredResourceDescriptor::default_timeout_decision_plan);
        let resolved_timeout =
            self.resolve_timeout_admission(resource_node, &timeout_plan, current_tick, None)?;
        let prior_timeout_wake = self.resource.active_timeout_wake_for_node(resource_node);
        let prior_stale_after_wake = self
            .resource
            .active_stale_after_wake_for_node(resource_node);
        let scheduled_timeout_wake = resolved_timeout
            .as_ref()
            .map(|resolved| self.schedule_resource_timeout_wake(resource_node, resolved))
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
            resource_node,
            prior_timeout_wake,
            prior_stale_after_wake,
            scheduled_timeout_wake,
        )?;
        Ok(report)
    }
}
