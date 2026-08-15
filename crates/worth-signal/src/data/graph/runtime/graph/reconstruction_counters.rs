use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug, Default)]
pub(crate) struct ReconstructionCounters {
    hot_path_artifact_reconstruction_count: Arc<AtomicU64>,
    explicit_cold_materialization_request_count: Arc<AtomicU64>,
    retained_forensic_read_count: Arc<AtomicU64>,
    cold_explanation_reconstruction_count: Arc<AtomicU64>,
    cold_provenance_reconstruction_count: Arc<AtomicU64>,
    retained_artifact_read_count: Arc<AtomicU64>,
    reconstructed_artifact_read_count: Arc<AtomicU64>,
    checkpoint_reconstruction_count: Arc<AtomicU64>,
    denied_reconstruction_by_budget_count: Arc<AtomicU64>,
    denied_reconstruction_by_tier_count: Arc<AtomicU64>,
    denied_reconstruction_explanation_api_count: Arc<AtomicU64>,
    denied_reconstruction_provenance_api_count: Arc<AtomicU64>,
}

impl ReconstructionCounters {
    pub(crate) fn record_hot_path_artifact_reconstruction(&self) {
        self.hot_path_artifact_reconstruction_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn hot_path_artifact_reconstruction_count(&self) -> u64 {
        self.hot_path_artifact_reconstruction_count
            .load(Ordering::Relaxed)
    }

    pub(crate) fn record_explicit_cold_materialization_request(&self) {
        self.explicit_cold_materialization_request_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn explicit_cold_materialization_request_count(&self) -> u64 {
        self.explicit_cold_materialization_request_count
            .load(Ordering::Relaxed)
    }

    pub(crate) fn record_retained_forensic_read(&self) {
        self.retained_forensic_read_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn retained_forensic_read_count(&self) -> u64 {
        self.retained_forensic_read_count.load(Ordering::Relaxed)
    }

    pub(crate) fn record_cold_explanation_reconstruction(&self) {
        self.cold_explanation_reconstruction_count
            .fetch_add(1, Ordering::Relaxed);
        self.reconstructed_artifact_read_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn cold_explanation_reconstruction_count(&self) -> u64 {
        self.cold_explanation_reconstruction_count
            .load(Ordering::Relaxed)
    }

    pub(crate) fn record_cold_provenance_reconstruction(&self) {
        self.cold_provenance_reconstruction_count
            .fetch_add(1, Ordering::Relaxed);
        self.reconstructed_artifact_read_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn cold_provenance_reconstruction_count(&self) -> u64 {
        self.cold_provenance_reconstruction_count
            .load(Ordering::Relaxed)
    }

    pub(crate) fn reconstructed_artifact_read_count(&self) -> u64 {
        self.reconstructed_artifact_read_count
            .load(Ordering::Relaxed)
    }

    pub(crate) fn record_checkpoint_reconstruction(&self) {
        self.checkpoint_reconstruction_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn checkpoint_reconstruction_count(&self) -> u64 {
        self.checkpoint_reconstruction_count.load(Ordering::Relaxed)
    }

    pub(crate) fn record_retained_artifact_read(&self) {
        self.retained_artifact_read_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn retained_artifact_read_count(&self) -> u64 {
        self.retained_artifact_read_count.load(Ordering::Relaxed)
    }

    pub(crate) fn record_denied_reconstruction_by_budget(&self, explanation_api: bool) {
        self.denied_reconstruction_by_budget_count
            .fetch_add(1, Ordering::Relaxed);
        if explanation_api {
            self.denied_reconstruction_explanation_api_count
                .fetch_add(1, Ordering::Relaxed);
        } else {
            self.denied_reconstruction_provenance_api_count
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    pub(crate) fn denied_reconstruction_by_budget_count(&self) -> u64 {
        self.denied_reconstruction_by_budget_count
            .load(Ordering::Relaxed)
    }

    pub(crate) fn record_denied_reconstruction_by_tier(&self, explanation_api: bool) {
        self.denied_reconstruction_by_tier_count
            .fetch_add(1, Ordering::Relaxed);
        if explanation_api {
            self.denied_reconstruction_explanation_api_count
                .fetch_add(1, Ordering::Relaxed);
        } else {
            self.denied_reconstruction_provenance_api_count
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    pub(crate) fn denied_reconstruction_by_tier_count(&self) -> u64 {
        self.denied_reconstruction_by_tier_count
            .load(Ordering::Relaxed)
    }

    pub(crate) fn denied_reconstruction_explanation_api_count(&self) -> u64 {
        self.denied_reconstruction_explanation_api_count
            .load(Ordering::Relaxed)
    }

    pub(crate) fn denied_reconstruction_provenance_api_count(&self) -> u64 {
        self.denied_reconstruction_provenance_api_count
            .load(Ordering::Relaxed)
    }
}

impl Clone for ReconstructionCounters {
    fn clone(&self) -> Self {
        Self {
            hot_path_artifact_reconstruction_count: Arc::new(AtomicU64::new(
                self.hot_path_artifact_reconstruction_count(),
            )),
            explicit_cold_materialization_request_count: Arc::new(AtomicU64::new(
                self.explicit_cold_materialization_request_count(),
            )),
            retained_forensic_read_count: Arc::new(AtomicU64::new(
                self.retained_forensic_read_count(),
            )),
            cold_explanation_reconstruction_count: Arc::new(AtomicU64::new(
                self.cold_explanation_reconstruction_count(),
            )),
            cold_provenance_reconstruction_count: Arc::new(AtomicU64::new(
                self.cold_provenance_reconstruction_count(),
            )),
            retained_artifact_read_count: Arc::new(AtomicU64::new(
                self.retained_artifact_read_count(),
            )),
            reconstructed_artifact_read_count: Arc::new(AtomicU64::new(
                self.reconstructed_artifact_read_count(),
            )),
            checkpoint_reconstruction_count: Arc::new(AtomicU64::new(
                self.checkpoint_reconstruction_count(),
            )),
            denied_reconstruction_by_budget_count: Arc::new(AtomicU64::new(
                self.denied_reconstruction_by_budget_count(),
            )),
            denied_reconstruction_by_tier_count: Arc::new(AtomicU64::new(
                self.denied_reconstruction_by_tier_count(),
            )),
            denied_reconstruction_explanation_api_count: Arc::new(AtomicU64::new(
                self.denied_reconstruction_explanation_api_count(),
            )),
            denied_reconstruction_provenance_api_count: Arc::new(AtomicU64::new(
                self.denied_reconstruction_provenance_api_count(),
            )),
        }
    }
}
