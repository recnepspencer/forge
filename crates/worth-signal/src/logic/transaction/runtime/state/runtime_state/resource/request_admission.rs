use crate::data::resource::{
    LoweredResourceDescriptor, ResourceRequestAdmissionReport, ResourceRequestIntent,
};

use super::super::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn admit_resource_request(
        &mut self,
        intent: ResourceRequestIntent,
    ) -> Result<ResourceRequestAdmissionReport, crate::data::error::SignalError> {
        let resource_node = intent.node();
        if !self.graph.is_alive(intent.node().node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot admit resource request for non-live owner {}",
                intent.node().node()
            )));
        }

        let current_tick = self.clock_basis().current_tick();
        let timeout_plan = self
            .resource
            .descriptor_for_node(resource_node)
            .map(|descriptor| descriptor.timeout_decision_plan().clone())
            .unwrap_or_else(|| LoweredResourceDescriptor::default_timeout_decision_plan());
        let prior_stale_after_wake = self
            .resource
            .active_stale_after_wake_for_node(resource_node);
        let prior_retry_wake = self.resource.pending_retry_wake_for_node(resource_node);
        let resolved_timeout = self.resolve_timeout_admission(
            resource_node,
            &timeout_plan,
            current_tick,
            intent.transaction_deadline(),
        )?;
        let prior_timeout_wake = self.resource.active_timeout_wake_for_node(resource_node);
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
        self.retire_superseded_resource_timeout_wake(
            prior_timeout_wake,
            scheduled_timeout_wake.as_ref(),
        )?;
        self.retire_superseded_resource_stale_after_wake(prior_stale_after_wake, None)?;
        self.retire_superseded_resource_retry_wake(prior_retry_wake)?;
        let _ = self.resource.clear_pending_retry_for_node(resource_node);
        let report = match self.resource.admit_resource_request(
            intent,
            self.graph.current_branch().id,
            current_tick,
            true,
            scheduled_timeout_admission,
            &mut self.telemetry.resource,
        ) {
            Ok(report) => report,
            Err(err) => {
                if let Some(wake) = scheduled_timeout_wake.as_ref() {
                    self.dispose_resource_timeout_wake(wake);
                }
                return Err(err);
            }
        };

        Ok(report)
    }
}
