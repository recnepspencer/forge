#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CertifiedSegmentSegment2DPerformanceCounters {
    segment_pairs_evaluated: usize,
    projection_receipts_consumed: usize,
    orientation_receipts_consumed: usize,
    predicate_escalation_records_consumed: usize,
    basis_digest_part_count: usize,
}

impl CertifiedSegmentSegment2DPerformanceCounters {
    pub(crate) const fn certified(basis_digest_part_count: usize) -> Self {
        Self {
            segment_pairs_evaluated: 1,
            projection_receipts_consumed: 4,
            orientation_receipts_consumed: 4,
            predicate_escalation_records_consumed: 4,
            basis_digest_part_count,
        }
    }

    pub fn segment_pairs_evaluated(&self) -> usize {
        self.segment_pairs_evaluated
    }

    pub fn projection_receipts_consumed(&self) -> usize {
        self.projection_receipts_consumed
    }

    pub fn orientation_receipts_consumed(&self) -> usize {
        self.orientation_receipts_consumed
    }

    pub fn predicate_escalation_records_consumed(&self) -> usize {
        self.predicate_escalation_records_consumed
    }

    pub fn basis_digest_part_count(&self) -> usize {
        self.basis_digest_part_count
    }
}
