#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HistoricalCounterSnapshot {
    historical_requested_path_count: usize,
    historical_admitted_path_count: usize,
    historical_resolved_path_count: usize,
    historical_compatibility_check_count: usize,
    predicted_historical_replay_span: usize,
    predicted_historical_reconstruction_scope: usize,
    historical_retained_snapshot_admission_count: usize,
    historical_delta_replay_admission_count: usize,
    historical_full_reconstruction_admission_count: usize,
    historical_path_denial_count: usize,
    historical_hidden_path_substitution_denial_count: usize,
    historical_result_path_metadata_count: usize,
    historical_replay_span_drift_count: usize,
    historical_reconstruction_scope_drift_count: usize,
    history_work_avoided_by_retained_path_count: usize,
    historical_executor_rediscovery_count: usize,
}

impl HistoricalCounterSnapshot {
    pub fn historical_requested_path_count(&self) -> usize {
        self.historical_requested_path_count
    }

    pub fn historical_admitted_path_count(&self) -> usize {
        self.historical_admitted_path_count
    }

    pub fn historical_resolved_path_count(&self) -> usize {
        self.historical_resolved_path_count
    }

    pub fn historical_compatibility_check_count(&self) -> usize {
        self.historical_compatibility_check_count
    }

    pub fn predicted_historical_replay_span(&self) -> usize {
        self.predicted_historical_replay_span
    }

    pub fn predicted_historical_reconstruction_scope(&self) -> usize {
        self.predicted_historical_reconstruction_scope
    }

    pub fn historical_retained_snapshot_admission_count(&self) -> usize {
        self.historical_retained_snapshot_admission_count
    }

    pub fn historical_delta_replay_admission_count(&self) -> usize {
        self.historical_delta_replay_admission_count
    }

    pub fn historical_full_reconstruction_admission_count(&self) -> usize {
        self.historical_full_reconstruction_admission_count
    }

    pub fn historical_path_denial_count(&self) -> usize {
        self.historical_path_denial_count
    }

    pub fn historical_hidden_path_substitution_denial_count(&self) -> usize {
        self.historical_hidden_path_substitution_denial_count
    }

    pub fn historical_result_path_metadata_count(&self) -> usize {
        self.historical_result_path_metadata_count
    }

    pub fn historical_replay_span_drift_count(&self) -> usize {
        self.historical_replay_span_drift_count
    }

    pub fn historical_reconstruction_scope_drift_count(&self) -> usize {
        self.historical_reconstruction_scope_drift_count
    }

    pub fn history_work_avoided_by_retained_path_count(&self) -> usize {
        self.history_work_avoided_by_retained_path_count
    }

    pub fn historical_executor_rediscovery_count(&self) -> usize {
        self.historical_executor_rediscovery_count
    }

    #[cfg(test)]
    pub(crate) fn vocabulary_baseline() -> Self {
        Self {
            historical_requested_path_count: 1,
            historical_admitted_path_count: 0,
            historical_resolved_path_count: 0,
            historical_compatibility_check_count: 1,
            predicted_historical_replay_span: 1,
            predicted_historical_reconstruction_scope: 1,
            historical_retained_snapshot_admission_count: 0,
            historical_delta_replay_admission_count: 0,
            historical_full_reconstruction_admission_count: 0,
            historical_path_denial_count: 0,
            historical_hidden_path_substitution_denial_count: 0,
            historical_result_path_metadata_count: 0,
            historical_replay_span_drift_count: 0,
            historical_reconstruction_scope_drift_count: 0,
            history_work_avoided_by_retained_path_count: 0,
            historical_executor_rediscovery_count: 0,
        }
    }

    pub(crate) fn retained_admission(
        predicted_historical_replay_span: usize,
        predicted_historical_reconstruction_scope: usize,
    ) -> Self {
        Self {
            historical_requested_path_count: 1,
            historical_admitted_path_count: 1,
            historical_resolved_path_count: 0,
            historical_compatibility_check_count: 1,
            predicted_historical_replay_span,
            predicted_historical_reconstruction_scope,
            historical_retained_snapshot_admission_count: 1,
            historical_delta_replay_admission_count: 0,
            historical_full_reconstruction_admission_count: 0,
            historical_path_denial_count: 0,
            historical_hidden_path_substitution_denial_count: 0,
            historical_result_path_metadata_count: 0,
            historical_replay_span_drift_count: 0,
            historical_reconstruction_scope_drift_count: 0,
            history_work_avoided_by_retained_path_count: 0,
            historical_executor_rediscovery_count: 0,
        }
    }

    pub(crate) fn replay_admission(
        predicted_historical_replay_span: usize,
        predicted_historical_reconstruction_scope: usize,
    ) -> Self {
        Self {
            historical_requested_path_count: 1,
            historical_admitted_path_count: 1,
            historical_resolved_path_count: 0,
            historical_compatibility_check_count: 1,
            predicted_historical_replay_span,
            predicted_historical_reconstruction_scope,
            historical_retained_snapshot_admission_count: 0,
            historical_delta_replay_admission_count: 1,
            historical_full_reconstruction_admission_count: 0,
            historical_path_denial_count: 0,
            historical_hidden_path_substitution_denial_count: 0,
            historical_result_path_metadata_count: 0,
            historical_replay_span_drift_count: 0,
            historical_reconstruction_scope_drift_count: 0,
            history_work_avoided_by_retained_path_count: 0,
            historical_executor_rediscovery_count: 0,
        }
    }

    pub(crate) fn reconstruction_admission(
        predicted_historical_replay_span: usize,
        predicted_historical_reconstruction_scope: usize,
    ) -> Self {
        Self {
            historical_requested_path_count: 1,
            historical_admitted_path_count: 1,
            historical_resolved_path_count: 0,
            historical_compatibility_check_count: 1,
            predicted_historical_replay_span,
            predicted_historical_reconstruction_scope,
            historical_retained_snapshot_admission_count: 0,
            historical_delta_replay_admission_count: 0,
            historical_full_reconstruction_admission_count: 1,
            historical_path_denial_count: 0,
            historical_hidden_path_substitution_denial_count: 0,
            historical_result_path_metadata_count: 0,
            historical_replay_span_drift_count: 0,
            historical_reconstruction_scope_drift_count: 0,
            history_work_avoided_by_retained_path_count: 0,
            historical_executor_rediscovery_count: 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn denied(
        predicted_historical_replay_span: usize,
        predicted_historical_reconstruction_scope: usize,
    ) -> Self {
        Self {
            historical_requested_path_count: 1,
            historical_admitted_path_count: 0,
            historical_resolved_path_count: 0,
            historical_compatibility_check_count: 1,
            predicted_historical_replay_span,
            predicted_historical_reconstruction_scope,
            historical_retained_snapshot_admission_count: 0,
            historical_delta_replay_admission_count: 0,
            historical_full_reconstruction_admission_count: 0,
            historical_path_denial_count: 1,
            historical_hidden_path_substitution_denial_count: 0,
            historical_result_path_metadata_count: 0,
            historical_replay_span_drift_count: 0,
            historical_reconstruction_scope_drift_count: 0,
            history_work_avoided_by_retained_path_count: 0,
            historical_executor_rediscovery_count: 0,
        }
    }

    pub(crate) fn with_resolved_metadata(mut self) -> Self {
        self.historical_resolved_path_count = 1;
        self.historical_result_path_metadata_count = 1;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_path_denial(mut self) -> Self {
        self.historical_path_denial_count = 1;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_hidden_path_substitution_denial(mut self) -> Self {
        self.historical_hidden_path_substitution_denial_count = 1;
        self
    }

    pub(crate) fn with_historical_replay_span_drift(mut self, drift_count: usize) -> Self {
        self.historical_replay_span_drift_count = drift_count;
        self
    }

    pub(crate) fn with_historical_reconstruction_scope_drift(mut self, drift_count: usize) -> Self {
        self.historical_reconstruction_scope_drift_count = drift_count;
        self
    }

    pub(crate) fn with_history_work_avoided_by_retained_path_count(
        mut self,
        work_avoided_count: usize,
    ) -> Self {
        self.history_work_avoided_by_retained_path_count = work_avoided_count;
        self
    }
}
