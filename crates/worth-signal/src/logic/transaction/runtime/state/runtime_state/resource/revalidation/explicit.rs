use crate::data::resource::{
    LoweredResourceDescriptor, ResourceRequestHandle, ResourceRevalidationIntent,
    ResourceRevalidationReport,
};

use super::super::super::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn revalidate_resource_node(
        &mut self,
        intent: ResourceRevalidationIntent,
    ) -> Result<ResourceRevalidationReport, crate::data::error::SignalError> {
        let resource_node = intent.node();
        if !self.graph.is_alive(resource_node.node()) {
            self.with_resource_telemetry(|telemetry| {
                telemetry.resource_non_live_owner_denial_count += 1
            });
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot revalidate resource node for non-live owner {}",
                resource_node.node()
            )));
        }

        let current_tick = self.clock_basis().current_tick();
        let revalidation_descriptor = self.resource.descriptor_for_node(resource_node);
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
        let capture_telemetry = self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        let prepared_revalidation = match self.resource.prepare_explicit_resource_revalidation(
            intent,
            revalidation_decision_digest,
            capture_telemetry.then_some(&mut self.telemetry.resource),
        ) {
            Ok(prepared) => prepared,
            Err(report) => return Ok(report),
        };
        let resolved_timeout = self.resolve_timeout_admission(
            resource_node,
            &timeout_plan,
            current_tick,
            intent.transaction_deadline(),
        )?;
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
            capture_telemetry.then_some(&mut self.telemetry.resource),
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

    pub fn prove_active_resource_revalidation_handle(
        &mut self,
        handle: ResourceRequestHandle,
    ) -> Result<
        crate::data::resource::ActiveResourceRevalidationProof,
        crate::data::error::SignalError,
    > {
        let capture_telemetry = self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        self.resource
            .prove_active_resource_revalidation_handle(
                handle,
                capture_telemetry.then_some(&mut self.telemetry.resource),
            )
            .ok_or_else(|| {
                crate::data::error::SignalError::invalid_input(format!(
                    "cannot mint active revalidation proof for stale or non-active request {}",
                    handle.request_id().get()
                ))
            })
    }

    pub fn force_revalidate_resource_node(
        &mut self,
        proof: crate::data::resource::ActiveResourceRevalidationProof,
    ) -> Result<ResourceRevalidationReport, crate::data::error::SignalError> {
        let resource_node = proof.node();
        if !self.graph.is_alive(resource_node.node()) {
            self.with_resource_telemetry(|telemetry| {
                telemetry.resource_non_live_owner_denial_count += 1
            });
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot force revalidate non-live resource node {}",
                resource_node.node()
            )));
        }

        let current_tick = self.clock_basis().current_tick();
        let capture_telemetry = self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        if capture_telemetry {
            self.telemetry
                .resource
                .resource_revalidation_policy_decision_count += 1;
        }
        self.with_resource_telemetry(|telemetry| telemetry.resource_forced_revalidation_count += 1);
        if let Some(class) = self
            .resource
            .validate_forced_resource_revalidation_proof(&proof)
        {
            return Ok(self.resource.deny_forced_revalidation_for_report(
                proof.node(),
                proof.handle(),
                class,
                capture_telemetry.then_some(&mut self.telemetry.resource),
            ));
        }
        let prepared_revalidation = match self.resource.prepare_forced_resource_revalidation(
            proof,
            capture_telemetry.then_some(&mut self.telemetry.resource),
        ) {
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
            capture_telemetry.then_some(&mut self.telemetry.resource),
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
