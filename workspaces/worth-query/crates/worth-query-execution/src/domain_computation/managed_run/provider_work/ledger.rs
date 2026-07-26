use std::sync::Arc;

use crate::domain_computation::artifact_owner::WorthQueryArtifactOccurrenceSnapshot;
use crate::domain_computation::{
    WorthQueryGraphProviderStepReport, WorthQueryProviderExecutionReleaseEvidence,
};

use super::evidence::WorthQueryManagedProviderWorkEvidenceParts;
use super::{
    WorthQueryManagedProviderExecutionReleaseSummary, WorthQueryManagedProviderWorkEvidence,
};

pub(crate) struct WorthQueryManagedProviderWorkLedger {
    provider_session_identity: Arc<str>,
    issued_call_count: usize,
    abandoned_call_count: usize,
    interrupted_call_count: usize,
    admitted_receipt_count: usize,
    completed_work_units: u64,
    attempted_effect_count: u64,
    applied_effect_count: u64,
    produced_artifact_count: usize,
    retained_artifact_count: usize,
    disposed_artifact_count: usize,
    peak_scratch_bytes: usize,
    retained_bytes: usize,
    non_artifact_retained_bytes: usize,
    checkpoint_available: bool,
    provider_step_attempt_count: usize,
    safe_point_request_lookup_count: usize,
    pressure_classification_count: usize,
    output_capacity_classification_count: usize,
    queue_request_lookup_count: usize,
    queue_state_mutation_count: usize,
    last_safe_point: Option<super::super::WorthQueryManagedSafePointObservation>,
    last_step_failure:
        Option<crate::domain_computation::WorthQueryGraphProviderStepFailureEvidence>,
    provider_execution_release: WorthQueryManagedProviderExecutionReleaseSummary,
}

impl WorthQueryManagedProviderWorkLedger {
    pub(crate) fn new(provider_session_identity: impl Into<Arc<str>>) -> Self {
        Self {
            provider_session_identity: provider_session_identity.into(),
            issued_call_count: 0,
            abandoned_call_count: 0,
            interrupted_call_count: 0,
            admitted_receipt_count: 0,
            completed_work_units: 0,
            attempted_effect_count: 0,
            applied_effect_count: 0,
            produced_artifact_count: 0,
            retained_artifact_count: 0,
            disposed_artifact_count: 0,
            peak_scratch_bytes: 0,
            retained_bytes: 0,
            non_artifact_retained_bytes: 0,
            checkpoint_available: false,
            provider_step_attempt_count: 0,
            safe_point_request_lookup_count: 0,
            pressure_classification_count: 0,
            output_capacity_classification_count: 0,
            queue_request_lookup_count: 0,
            queue_state_mutation_count: 0,
            last_safe_point: None,
            last_step_failure: None,
            provider_execution_release: WorthQueryManagedProviderExecutionReleaseSummary::default(),
        }
    }

    pub(crate) fn begin_step_call(&mut self) {
        self.issued_call_count = self.issued_call_count.saturating_add(1);
    }

    pub(crate) fn admit_step(&mut self, report: &WorthQueryGraphProviderStepReport) {
        self.completed_work_units = self
            .completed_work_units
            .saturating_add(report.completed_work_units());
        self.attempted_effect_count = self
            .attempted_effect_count
            .saturating_add(report.attempted_effect_count());
        self.applied_effect_count = self
            .applied_effect_count
            .saturating_add(report.applied_effect_count());
        self.peak_scratch_bytes = self
            .peak_scratch_bytes
            .max(usize::try_from(report.peak_scratch_bytes()).unwrap_or(usize::MAX));
        let artifacts = report.artifact_evidence();
        self.retained_bytes = usize::try_from(report.retained_bytes()).unwrap_or(usize::MAX);
        self.non_artifact_retained_bytes = self
            .retained_bytes
            .saturating_sub(artifacts.retained_bytes());
        self.produced_artifact_count = self
            .produced_artifact_count
            .saturating_add(artifacts.produced_artifact_count());
        self.retained_artifact_count = artifacts.retained_artifact_count();
        self.disposed_artifact_count = self
            .disposed_artifact_count
            .saturating_add(artifacts.disposed_artifact_count());
        self.checkpoint_available |= report.checkpoint_available();
        self.last_step_failure = report.failure().cloned();
    }

    pub(crate) fn complete_step_call(&mut self) {
        self.admitted_receipt_count = self.admitted_receipt_count.saturating_add(1);
    }

    pub(crate) fn record_provider_step_attempt(&mut self) {
        self.provider_step_attempt_count = self.provider_step_attempt_count.saturating_add(1);
    }

    pub(crate) fn record_safe_point(
        &mut self,
        observation: &super::super::WorthQueryManagedSafePointObservation,
    ) {
        let counters = observation.counters();
        self.safe_point_request_lookup_count = self
            .safe_point_request_lookup_count
            .saturating_add(counters.exact_signal_request_lookup_count());
        self.pressure_classification_count = self
            .pressure_classification_count
            .saturating_add(counters.pressure_classification_count());
        self.last_safe_point = Some(observation.clone());
    }

    pub(crate) fn record_provider_step_admission(
        &mut self,
        counters: super::super::provider_step_admission::WorthQueryProviderStepAdmissionCounters,
    ) {
        self.output_capacity_classification_count = self
            .output_capacity_classification_count
            .saturating_add(counters.output_capacity_classification_count());
    }

    pub(crate) fn record_queue_mutation(
        &mut self,
        counters: worth_runtime_bridge::facade::BridgeManagedQueueMutationCounters,
    ) {
        self.queue_request_lookup_count = self
            .queue_request_lookup_count
            .saturating_add(counters.exact_signal_request_lookup_count());
        self.queue_state_mutation_count = self
            .queue_state_mutation_count
            .saturating_add(counters.queue_state_mutation_count());
    }

    pub(crate) fn record_provider_execution_release(
        &mut self,
        evidence: &WorthQueryProviderExecutionReleaseEvidence,
    ) {
        self.provider_execution_release.record(evidence);
    }

    pub(crate) fn release_retained_bytes(&mut self, retained_bytes: usize) -> bool {
        let Some(total_remaining) = self.retained_bytes.checked_sub(retained_bytes) else {
            return false;
        };
        let Some(non_artifact_remaining) =
            self.non_artifact_retained_bytes.checked_sub(retained_bytes)
        else {
            return false;
        };
        self.retained_bytes = total_remaining;
        self.non_artifact_retained_bytes = non_artifact_remaining;
        true
    }

    pub(crate) fn settle_artifacts(&mut self, snapshot: WorthQueryArtifactOccurrenceSnapshot) {
        self.produced_artifact_count = snapshot.produced_artifact_count();
        self.retained_artifact_count = snapshot.retained_artifact_count();
        self.disposed_artifact_count = snapshot.disposed_artifact_count();
        self.retained_bytes = self
            .non_artifact_retained_bytes
            .saturating_add(snapshot.retained_bytes());
    }

    pub(crate) fn interrupt_step_call(&mut self) {
        self.interrupted_call_count = self.interrupted_call_count.saturating_add(1);
    }

    pub(crate) fn abandon(&mut self) {
        self.abandoned_call_count = self.abandoned_call_count.saturating_add(1);
    }

    pub(crate) fn has_uncertainty(&self) -> bool {
        self.abandoned_call_count != 0
    }

    pub(crate) fn rebind_provider_session(
        mut self,
        provider_session_identity: impl Into<Arc<str>>,
    ) -> Self {
        self.provider_session_identity = provider_session_identity.into();
        self
    }

    pub(crate) fn snapshot(&self) -> WorthQueryManagedProviderWorkEvidence {
        WorthQueryManagedProviderWorkEvidence::from_parts(
            WorthQueryManagedProviderWorkEvidenceParts {
                provider_session_identity: Arc::clone(&self.provider_session_identity),
                issued_call_count: self.issued_call_count,
                abandoned_call_count: self.abandoned_call_count,
                interrupted_call_count: self.interrupted_call_count,
                admitted_receipt_count: self.admitted_receipt_count,
                completed_work_units: self.completed_work_units,
                attempted_effect_count: self.attempted_effect_count,
                applied_effect_count: self.applied_effect_count,
                produced_artifact_count: self.produced_artifact_count,
                retained_artifact_count: self.retained_artifact_count,
                disposed_artifact_count: self.disposed_artifact_count,
                peak_scratch_bytes: self.peak_scratch_bytes,
                retained_bytes: self.retained_bytes,
                checkpoint_available: self.checkpoint_available,
                provider_step_attempt_count: self.provider_step_attempt_count,
                safe_point_request_lookup_count: self.safe_point_request_lookup_count,
                pressure_classification_count: self.pressure_classification_count,
                output_capacity_classification_count: self.output_capacity_classification_count,
                queue_request_lookup_count: self.queue_request_lookup_count,
                queue_state_mutation_count: self.queue_state_mutation_count,
                last_safe_point: self.last_safe_point.clone(),
                last_step_failure: self.last_step_failure.clone(),
                provider_execution_release: self.provider_execution_release.clone(),
            },
        )
    }

    pub(crate) fn into_evidence(self) -> WorthQueryManagedProviderWorkEvidence {
        self.snapshot()
    }
}
