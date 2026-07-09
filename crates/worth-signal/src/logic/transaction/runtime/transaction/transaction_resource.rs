use crate::data::error::SignalError;
use crate::data::resource::{
    AdmittedResourceCompletion, DeniedResourceCompletion, ResourceCompletionCommitReport,
    ResourceCompletionDenialStagingReport, ResourceCompletionRollbackReport,
    ResourceCompletionStagingReport, StagedDeniedResourceCompletionEffect,
    StagedResourceCompletionEffect,
};
use crate::data::temporal::TemporalWakeRetirementReason;

use super::transaction_types::SignalTransaction;

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn stage_admitted_resource_completion(
        &mut self,
        admitted: AdmittedResourceCompletion,
    ) -> Result<ResourceCompletionStagingReport, SignalError> {
        self.resource
            .stage_admitted_resource_completion(admitted, &mut self.telemetry.resource)
    }

    pub fn stage_denied_resource_completion(
        &mut self,
        denied: DeniedResourceCompletion,
    ) -> Result<ResourceCompletionDenialStagingReport, SignalError> {
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
    ) -> Result<ResourceCompletionCommitReport, SignalError> {
        self.rollback_packets
            .capture_resource_baseline_if_needed(self.resource);
        self.rollback_packets
            .capture_temporal_baseline_if_needed(self.temporal);

        let node = staged.node().node();
        let handle = staged.handle();
        if let Some(wake_id) = self.resource.active_timeout_wake_for_handle(handle) {
            let retired = self.temporal.retire_wake(
                wake_id,
                TemporalWakeRetirementReason::Consumed,
                &mut self.telemetry.temporal,
            )?;
            self.scratch.temporal.record_retired_wake(retired);
        }

        let report = self
            .resource
            .commit_staged_resource_completion(staged, &mut self.telemetry.resource)?;
        self.stage_resource_lifecycle_observation(node);
        Ok(report)
    }

    fn stage_resource_lifecycle_observation(&mut self, node: crate::data::handle::NodeId) {
        let before_candidate_count = self.scratch.observations.staged_candidate_count();
        let before_classified_count = self.scratch.observations.classified_event_count();
        let matched = self
            .scratch
            .observations
            .classify_resource_lifecycle_change(self.observations, node);
        let after_candidate_count = self.scratch.observations.staged_candidate_count();
        let after_classified_count = self.scratch.observations.classified_event_count();

        self.telemetry.transaction.staged_observation_match_count += matched as u64;
        self.telemetry
            .transaction
            .staged_observation_candidate_count +=
            after_candidate_count.saturating_sub(before_candidate_count) as u64;
        self.telemetry.transaction.classified_observation_count +=
            after_classified_count.saturating_sub(before_classified_count) as u64;
        self.telemetry
            .transaction
            .observation_classification_breadth += u64::from(matched > 0);
    }
}
