use serde::{Deserialize, Serialize};

use super::super::performance::ResourceBoundaryPerformanceEnvelope;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceReplayReconstructionReport {
    descriptor_width: u32,
    lifecycle_summary_width: u32,
    denied_completion_width: u32,
    retained_retry_lineage_width: u32,
    in_flight_width: u32,
    retained_history_unavailable_count: u32,
    denied_completion_unavailable_count: u32,
    retry_lineage_unavailable_count: u32,
    descriptor_digest: String,
    lifecycle_digest: String,
    output_continuity_digest: String,
    denied_completion_digest: String,
    retry_lineage_digest: String,
    in_flight_digest: String,
    replay_digest: String,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceReplayReconstructionReport {
    pub(crate) fn new(
        descriptor_width: u32,
        lifecycle_summary_width: u32,
        denied_completion_width: u32,
        retained_retry_lineage_width: u32,
        in_flight_width: u32,
        retained_history_unavailable_count: u32,
        denied_completion_unavailable_count: u32,
        retry_lineage_unavailable_count: u32,
        descriptor_digest: String,
        lifecycle_digest: String,
        output_continuity_digest: String,
        denied_completion_digest: String,
        retry_lineage_digest: String,
        in_flight_digest: String,
        replay_digest: String,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            descriptor_width,
            lifecycle_summary_width,
            denied_completion_width,
            retained_retry_lineage_width,
            in_flight_width,
            retained_history_unavailable_count,
            denied_completion_unavailable_count,
            retry_lineage_unavailable_count,
            descriptor_digest,
            lifecycle_digest,
            output_continuity_digest,
            denied_completion_digest,
            retry_lineage_digest,
            in_flight_digest,
            replay_digest,
            performance,
        }
    }

    pub fn descriptor_width(&self) -> u32 {
        self.descriptor_width
    }

    pub fn lifecycle_summary_width(&self) -> u32 {
        self.lifecycle_summary_width
    }

    pub fn denied_completion_width(&self) -> u32 {
        self.denied_completion_width
    }

    pub fn retained_retry_lineage_width(&self) -> u32 {
        self.retained_retry_lineage_width
    }

    pub fn in_flight_width(&self) -> u32 {
        self.in_flight_width
    }

    pub fn retained_history_unavailable_count(&self) -> u32 {
        self.retained_history_unavailable_count
    }

    pub fn denied_completion_unavailable_count(&self) -> u32 {
        self.denied_completion_unavailable_count
    }

    pub fn retry_lineage_unavailable_count(&self) -> u32 {
        self.retry_lineage_unavailable_count
    }

    pub fn descriptor_digest(&self) -> &str {
        &self.descriptor_digest
    }

    pub fn lifecycle_digest(&self) -> &str {
        &self.lifecycle_digest
    }

    pub fn output_continuity_digest(&self) -> &str {
        &self.output_continuity_digest
    }

    pub fn denied_completion_digest(&self) -> &str {
        &self.denied_completion_digest
    }

    pub fn retry_lineage_digest(&self) -> &str {
        &self.retry_lineage_digest
    }

    pub fn in_flight_digest(&self) -> &str {
        &self.in_flight_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}
