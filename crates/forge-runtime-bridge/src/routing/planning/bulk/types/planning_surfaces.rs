#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBulkPlanningCounters {
    bulk_workload_count: usize,
    bulk_routed_item_count: usize,
    bulk_normalized_workload_width: usize,
    bulk_packet_count: usize,
    bulk_packet_entry_count: usize,
    bulk_reduction_input_count: usize,
    bulk_reduction_output_count: usize,
    bulk_fallback_count: usize,
    bulk_packet_queue_depth_peak: usize,
    bulk_reducer_input_buffer_peak: usize,
    bulk_replay_mismatch_count: usize,
    bulk_unsupported_path_count: usize,
    bulk_serial_required_count: usize,
    bulk_parallel_legal_count: usize,
    bulk_parallel_profitable_count: usize,
    bulk_parallel_preparation_admitted_count: usize,
    bulk_parallel_preparation_rejected_count: usize,
    bulk_parallel_fallback_to_serial_count: usize,
}

impl BridgeBulkPlanningCounters {
    pub(crate) fn new(
        bulk_routed_item_count: usize,
        bulk_normalized_workload_width: usize,
        bulk_packet_count: usize,
        bulk_packet_entry_count: usize,
        bulk_reduction_input_count: usize,
        bulk_reduction_output_count: usize,
        bulk_fallback_count: usize,
        bulk_packet_queue_depth_peak: usize,
        bulk_reducer_input_buffer_peak: usize,
        bulk_replay_mismatch_count: usize,
        bulk_unsupported_path_count: usize,
        legality_class: BridgeParallelLegalityClass,
        profitability_class: BridgeParallelProfitabilityClass,
        admission_class: BridgeParallelAdmissionClass,
    ) -> Self {
        let parallel_legal = matches!(
            legality_class,
            BridgeParallelLegalityClass::ParallelPreparationLegal
        );
        let parallel_profitable = matches!(
            profitability_class,
            BridgeParallelProfitabilityClass::Profitable
        );
        let parallel_admitted = matches!(
            admission_class,
            BridgeParallelAdmissionClass::ParallelPreparationAdmitted
        );
        let parallel_rejected = matches!(
            admission_class,
            BridgeParallelAdmissionClass::ParallelPreparationRejected
        );
        Self {
            bulk_workload_count: 1,
            bulk_routed_item_count,
            bulk_normalized_workload_width,
            bulk_packet_count,
            bulk_packet_entry_count,
            bulk_reduction_input_count,
            bulk_reduction_output_count,
            bulk_fallback_count,
            bulk_packet_queue_depth_peak,
            bulk_reducer_input_buffer_peak,
            bulk_replay_mismatch_count,
            bulk_unsupported_path_count,
            bulk_serial_required_count: usize::from(!parallel_admitted),
            bulk_parallel_legal_count: usize::from(parallel_legal),
            bulk_parallel_profitable_count: usize::from(parallel_profitable),
            bulk_parallel_preparation_admitted_count: usize::from(parallel_admitted),
            bulk_parallel_preparation_rejected_count: usize::from(parallel_rejected),
            bulk_parallel_fallback_to_serial_count: usize::from(
                parallel_legal && !parallel_profitable,
            ),
        }
    }

    pub(crate) fn zero() -> Self {
        Self::new(
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            BridgeParallelLegalityClass::SerialOnly,
            BridgeParallelProfitabilityClass::NotApplicable,
            BridgeParallelAdmissionClass::SerialRequired,
        )
    }

    pub fn bulk_workload_count(&self) -> usize {
        self.bulk_workload_count
    }
    pub fn bulk_routed_item_count(&self) -> usize {
        self.bulk_routed_item_count
    }
    pub fn bulk_normalized_workload_width(&self) -> usize {
        self.bulk_normalized_workload_width
    }
    pub fn bulk_packet_count(&self) -> usize {
        self.bulk_packet_count
    }
    pub fn bulk_packet_entry_count(&self) -> usize {
        self.bulk_packet_entry_count
    }
    pub fn bulk_reduction_input_count(&self) -> usize {
        self.bulk_reduction_input_count
    }
    pub fn bulk_reduction_output_count(&self) -> usize {
        self.bulk_reduction_output_count
    }
    pub fn bulk_fallback_count(&self) -> usize {
        self.bulk_fallback_count
    }
    pub fn bulk_packet_queue_depth_peak(&self) -> usize {
        self.bulk_packet_queue_depth_peak
    }
    pub fn bulk_reducer_input_buffer_peak(&self) -> usize {
        self.bulk_reducer_input_buffer_peak
    }
    pub fn bulk_replay_mismatch_count(&self) -> usize {
        self.bulk_replay_mismatch_count
    }
    pub fn bulk_unsupported_path_count(&self) -> usize {
        self.bulk_unsupported_path_count
    }
    pub fn bulk_serial_required_count(&self) -> usize {
        self.bulk_serial_required_count
    }
    pub fn bulk_parallel_legal_count(&self) -> usize {
        self.bulk_parallel_legal_count
    }
    pub fn bulk_parallel_profitable_count(&self) -> usize {
        self.bulk_parallel_profitable_count
    }
    pub fn bulk_parallel_preparation_admitted_count(&self) -> usize {
        self.bulk_parallel_preparation_admitted_count
    }
    pub fn bulk_parallel_preparation_rejected_count(&self) -> usize {
        self.bulk_parallel_preparation_rejected_count
    }
    pub fn bulk_parallel_fallback_to_serial_count(&self) -> usize {
        self.bulk_parallel_fallback_to_serial_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeLocalityFootprint {
    branch_scope_count: usize,
    snapshot_scope_count: usize,
    publication_scope_count: usize,
    digest: Arc<str>,
}

impl BridgeLocalityFootprint {
    pub(crate) fn new(
        branch_scope_count: usize,
        snapshot_scope_count: usize,
        publication_scope_count: usize,
    ) -> Self {
        let publication_scope_count = publication_scope_count.max(1);
        let basis = format!(
            "bridge-locality-footprint|branch-scope-count={}|snapshot-scope-count={}|publication-scope-count={}",
            branch_scope_count, snapshot_scope_count, publication_scope_count
        );
        Self {
            branch_scope_count,
            snapshot_scope_count,
            publication_scope_count,
            digest: digest_string("bridge-locality-footprint", &basis),
        }
    }

    pub fn branch_scope_count(&self) -> usize {
        self.branch_scope_count
    }

    pub fn snapshot_scope_count(&self) -> usize {
        self.snapshot_scope_count
    }

    pub fn publication_scope_count(&self) -> usize {
        self.publication_scope_count
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

use super::*;
