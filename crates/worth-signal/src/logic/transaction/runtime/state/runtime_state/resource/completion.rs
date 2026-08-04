use crate::data::resource::{
    AdmittedResourceCompletion, DeniedResourceCompletion, RawCompletionEnvelope,
    ResourceCompletionAdmissionReport, ResourceCompletionBatchAdmissionReport,
    ResourceCompletionCommitReport, ResourceCompletionDenialStagingReport,
    ResourceCompletionRollbackReport, ResourceCompletionStagingReport,
    StagedDeniedResourceCompletionEffect, StagedResourceCompletionEffect,
};

use crate::data::temporal::TemporalWakeRetirementReason;

use super::super::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn admit_resource_completion(
        &mut self,
        completion: RawCompletionEnvelope,
    ) -> ResourceCompletionAdmissionReport {
        self.resource
            .admit_resource_completion(completion, &mut self.telemetry.resource)
    }

    pub fn admit_resource_completion_batch(
        &mut self,
        completions: impl IntoIterator<Item = RawCompletionEnvelope>,
    ) -> ResourceCompletionBatchAdmissionReport {
        self.resource
            .admit_resource_completion_batch(completions, &mut self.telemetry.resource)
    }

    pub fn stage_admitted_resource_completion(
        &mut self,
        admitted: AdmittedResourceCompletion,
    ) -> Result<ResourceCompletionStagingReport, crate::data::error::SignalError> {
        self.resource
            .stage_admitted_resource_completion(admitted, &mut self.telemetry.resource)
    }

    pub fn stage_denied_resource_completion(
        &mut self,
        denied: DeniedResourceCompletion,
    ) -> Result<ResourceCompletionDenialStagingReport, crate::data::error::SignalError> {
        self.resource
            .stage_denied_resource_completion(denied, &mut self.telemetry.resource)
    }

    pub fn rollback_staged_resource_completion(
        &mut self,
        staged: StagedResourceCompletionEffect,
    ) -> ResourceCompletionRollbackReport {
        self.resource
            .rollback_staged_resource_completion(staged, &mut self.telemetry.resource)
    }

    pub fn rollback_staged_denied_resource_completion(
        &mut self,
        staged: StagedDeniedResourceCompletionEffect,
    ) -> ResourceCompletionRollbackReport {
        self.resource
            .rollback_staged_denied_resource_completion(staged, &mut self.telemetry.resource)
    }

    pub fn commit_staged_resource_completion(
        &mut self,
        staged: StagedResourceCompletionEffect,
    ) -> Result<ResourceCompletionCommitReport, crate::data::error::SignalError> {
        let handle = staged.handle();
        if let Some(wake_id) = self.resource.active_timeout_wake_for_handle(handle) {
            self.retire_temporal_wake(wake_id, TemporalWakeRetirementReason::Consumed)?;
        }
        let report = self
            .resource
            .commit_staged_resource_completion(staged, &mut self.telemetry.resource)?;
        let node = report.lifecycle().node();
        let prior_stale_after_wake = self.resource.active_stale_after_wake_for_node(node);
        let scheduled_stale_after_wake = self
            .resource
            .descriptor_for_node(node)
            .and_then(|descriptor| {
                let revalidation_plan = descriptor.revalidation_decision_plan();
                if !revalidation_plan.permits_stale_after_revalidation() {
                    return None;
                }
                descriptor.stale_after_decision_plan().stale_after()
            })
            .map(|stale_after| self.schedule_resource_stale_after_wake(node, stale_after))
            .transpose()?
            .flatten();
        self.retire_superseded_resource_stale_after_wake(
            prior_stale_after_wake,
            scheduled_stale_after_wake.as_ref(),
        )?;
        if let Some(wake) = scheduled_stale_after_wake {
            self.resource.attach_stale_after_wake(node, wake.id());
        }
        Ok(report)
    }
}
