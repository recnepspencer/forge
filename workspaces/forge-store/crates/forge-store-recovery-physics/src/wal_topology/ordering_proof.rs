use crate::{LogSequenceNumber, WalSegmentGeneration};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalFrameOrderingProof {
    expected_generation: WalSegmentGeneration,
    candidate_count: usize,
    accepted_segment_count: usize,
    ordered_range_count: usize,
    range_adjacency_check_count: usize,
    first_lsn: LogSequenceNumber,
    end_lsn: LogSequenceNumber,
}

impl WalFrameOrderingProof {
    pub(crate) const fn new(
        expected_generation: WalSegmentGeneration,
        candidate_count: usize,
        accepted_segment_count: usize,
        ordered_range_count: usize,
        range_adjacency_check_count: usize,
        first_lsn: LogSequenceNumber,
        end_lsn: LogSequenceNumber,
    ) -> Self {
        Self {
            expected_generation,
            candidate_count,
            accepted_segment_count,
            ordered_range_count,
            range_adjacency_check_count,
            first_lsn,
            end_lsn,
        }
    }

    pub const fn expected_generation(&self) -> WalSegmentGeneration {
        self.expected_generation
    }

    pub const fn candidate_count(&self) -> usize {
        self.candidate_count
    }

    pub const fn accepted_segment_count(&self) -> usize {
        self.accepted_segment_count
    }

    pub const fn ordered_range_count(&self) -> usize {
        self.ordered_range_count
    }

    pub const fn range_adjacency_check_count(&self) -> usize {
        self.range_adjacency_check_count
    }

    pub const fn first_lsn(&self) -> LogSequenceNumber {
        self.first_lsn
    }

    pub const fn end_lsn(&self) -> LogSequenceNumber {
        self.end_lsn
    }
}
