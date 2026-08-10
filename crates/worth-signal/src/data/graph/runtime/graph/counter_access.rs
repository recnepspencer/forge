use super::SignalGraph;

impl SignalGraph {
    pub(crate) fn record_hot_path_artifact_reconstruction(&self) {
        self.observation
            .reconstruction_counters
            .record_hot_path_artifact_reconstruction();
    }

    pub(crate) fn hot_path_artifact_reconstruction_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .hot_path_artifact_reconstruction_count()
    }

    pub(crate) fn record_explicit_cold_materialization_request(&self) {
        self.observation
            .reconstruction_counters
            .record_explicit_cold_materialization_request();
    }

    pub(crate) fn explicit_cold_materialization_request_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .explicit_cold_materialization_request_count()
    }

    pub(crate) fn record_retained_forensic_read(&self) {
        self.observation
            .reconstruction_counters
            .record_retained_forensic_read();
    }

    pub(crate) fn retained_forensic_read_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .retained_forensic_read_count()
    }

    pub(crate) fn record_cold_explanation_reconstruction(&self) {
        self.observation
            .reconstruction_counters
            .record_cold_explanation_reconstruction();
    }

    pub(crate) fn cold_explanation_reconstruction_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .cold_explanation_reconstruction_count()
    }

    pub(crate) fn record_cold_provenance_reconstruction(&self) {
        self.observation
            .reconstruction_counters
            .record_cold_provenance_reconstruction();
    }

    pub(crate) fn cold_provenance_reconstruction_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .cold_provenance_reconstruction_count()
    }

    pub(crate) fn record_retained_artifact_read(&self) {
        self.observation
            .reconstruction_counters
            .record_retained_artifact_read();
    }

    pub(crate) fn retained_artifact_read_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .retained_artifact_read_count()
    }

    pub(crate) fn reconstructed_artifact_read_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .reconstructed_artifact_read_count()
    }

    pub(crate) fn record_denied_reconstruction_by_budget(&self, explanation_api: bool) {
        self.observation
            .reconstruction_counters
            .record_denied_reconstruction_by_budget(explanation_api);
    }

    pub(crate) fn denied_reconstruction_by_budget_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .denied_reconstruction_by_budget_count()
    }

    pub(crate) fn record_denied_reconstruction_by_tier(&self, explanation_api: bool) {
        self.observation
            .reconstruction_counters
            .record_denied_reconstruction_by_tier(explanation_api);
    }

    pub(crate) fn denied_reconstruction_by_tier_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .denied_reconstruction_by_tier_count()
    }

    pub(crate) fn denied_reconstruction_explanation_api_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .denied_reconstruction_explanation_api_count()
    }

    pub(crate) fn denied_reconstruction_provenance_api_count(&self) -> u64 {
        self.observation
            .reconstruction_counters
            .denied_reconstruction_provenance_api_count()
    }
}
