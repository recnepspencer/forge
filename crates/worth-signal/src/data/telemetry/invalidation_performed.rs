use serde::{Deserialize, Serialize};

/// One exact row in Signal's performed invalidation counter contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum InvalidationPerformedCounter {
    SourceOutputDeltasConsumed,
    DirectSubscriberEdgesExamined,
    ReverseIndexBucketProbes,
    ReverseIndexCandidatesReturned,
    CandidatesRejectedByAspectContract,
    CandidatesRejectedByScope,
    CandidatesRejectedByComparator,
    DirectSettlementsProduced,
    WorkItemsAdmitted,
    WorkItemsMerged,
    ReadyItemsEnqueued,
    ReadyItemsPopped,
    StaleWorkRejected,
    NodesEvaluated,
    ProducedDeltasEmitted,
    PropagationStops,
    NonSemanticNodeVisits,
    MaximumReadyFrontierWidth,
    RetainedReadyFrontierWidth,
    TopologyRevisionRevalidations,
    RejectedTopologyMutations,
    BatchLocalAllocations,
    PeakBatchMemoryItems,
    RecoveryReconstructionWork,
}

impl InvalidationPerformedCounter {
    pub const ALL: [Self; 24] = [
        Self::SourceOutputDeltasConsumed,
        Self::DirectSubscriberEdgesExamined,
        Self::ReverseIndexBucketProbes,
        Self::ReverseIndexCandidatesReturned,
        Self::CandidatesRejectedByAspectContract,
        Self::CandidatesRejectedByScope,
        Self::CandidatesRejectedByComparator,
        Self::DirectSettlementsProduced,
        Self::WorkItemsAdmitted,
        Self::WorkItemsMerged,
        Self::ReadyItemsEnqueued,
        Self::ReadyItemsPopped,
        Self::StaleWorkRejected,
        Self::NodesEvaluated,
        Self::ProducedDeltasEmitted,
        Self::PropagationStops,
        Self::NonSemanticNodeVisits,
        Self::MaximumReadyFrontierWidth,
        Self::RetainedReadyFrontierWidth,
        Self::TopologyRevisionRevalidations,
        Self::RejectedTopologyMutations,
        Self::BatchLocalAllocations,
        Self::PeakBatchMemoryItems,
        Self::RecoveryReconstructionWork,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Self::SourceOutputDeltasConsumed => "source_output_deltas_consumed",
            Self::DirectSubscriberEdgesExamined => "direct_subscriber_edges_examined",
            Self::ReverseIndexBucketProbes => "reverse_index_bucket_probes",
            Self::ReverseIndexCandidatesReturned => "reverse_index_candidates_returned",
            Self::CandidatesRejectedByAspectContract => "candidates_rejected_by_aspect_contract",
            Self::CandidatesRejectedByScope => "candidates_rejected_by_scope",
            Self::CandidatesRejectedByComparator => "candidates_rejected_by_comparator",
            Self::DirectSettlementsProduced => "direct_settlements_produced",
            Self::WorkItemsAdmitted => "work_items_admitted",
            Self::WorkItemsMerged => "work_items_merged",
            Self::ReadyItemsEnqueued => "ready_items_enqueued",
            Self::ReadyItemsPopped => "ready_items_popped",
            Self::StaleWorkRejected => "stale_work_rejected",
            Self::NodesEvaluated => "nodes_evaluated",
            Self::ProducedDeltasEmitted => "produced_deltas_emitted",
            Self::PropagationStops => "propagation_stops",
            Self::NonSemanticNodeVisits => "non_semantic_node_visits",
            Self::MaximumReadyFrontierWidth => "maximum_ready_frontier_width",
            Self::RetainedReadyFrontierWidth => "retained_ready_frontier_width",
            Self::TopologyRevisionRevalidations => "topology_revision_revalidations",
            Self::RejectedTopologyMutations => "rejected_topology_mutations",
            Self::BatchLocalAllocations => "batch_local_allocations",
            Self::PeakBatchMemoryItems => "peak_batch_memory_items",
            Self::RecoveryReconstructionWork => "recovery_reconstruction_work",
        }
    }

    pub(crate) const fn index(self) -> usize {
        self as usize
    }
}

/// Immutable observation of work actually performed by Signal.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalInvalidationRealizedCounters {
    values: [u64; 24],
}

impl SignalInvalidationRealizedCounters {
    pub const fn from_values(values: [u64; 24]) -> Self {
        Self { values }
    }

    pub const fn value(self, counter: InvalidationPerformedCounter) -> u64 {
        self.values[counter.index()]
    }

    pub const fn values(self) -> [u64; 24] {
        self.values
    }

    pub const fn work_items_admitted(self) -> u64 {
        self.value(InvalidationPerformedCounter::WorkItemsAdmitted)
    }

    pub const fn work_items_merged(self) -> u64 {
        self.value(InvalidationPerformedCounter::WorkItemsMerged)
    }

    pub const fn ready_items_enqueued(self) -> u64 {
        self.value(InvalidationPerformedCounter::ReadyItemsEnqueued)
    }

    pub const fn ready_items_popped(self) -> u64 {
        self.value(InvalidationPerformedCounter::ReadyItemsPopped)
    }

    pub const fn maximum_ready_frontier_width(self) -> u64 {
        self.value(InvalidationPerformedCounter::MaximumReadyFrontierWidth)
    }

    pub const fn retained_ready_frontier_width(self) -> u64 {
        self.value(InvalidationPerformedCounter::RetainedReadyFrontierWidth)
    }
}
