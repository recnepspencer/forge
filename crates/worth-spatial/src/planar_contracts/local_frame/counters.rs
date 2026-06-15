#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarLocalFramePerformanceCounters {
    local_frame_derivations: usize,
    retained_precision_receipts_consumed: usize,
    normalization_basis_count: usize,
    basis_digest_part_count: usize,
}

impl PlanarLocalFramePerformanceCounters {
    pub(crate) const fn certified(basis_digest_part_count: usize) -> Self {
        Self {
            local_frame_derivations: 1,
            retained_precision_receipts_consumed: 1,
            normalization_basis_count: 1,
            basis_digest_part_count,
        }
    }

    pub fn local_frame_derivations(&self) -> usize {
        self.local_frame_derivations
    }

    pub fn retained_precision_receipts_consumed(&self) -> usize {
        self.retained_precision_receipts_consumed
    }

    pub fn normalization_basis_count(&self) -> usize {
        self.normalization_basis_count
    }

    pub fn basis_digest_part_count(&self) -> usize {
        self.basis_digest_part_count
    }
}
