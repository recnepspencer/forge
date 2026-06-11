#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProjectPointToCertifiedPlane2DPerformanceCounters {
    projection_derivations: usize,
    local_frame_receipts_consumed: usize,
    local_delta_basis_reads: usize,
    plane_distance_checks: usize,
    basis_digest_part_count: usize,
}

impl ProjectPointToCertifiedPlane2DPerformanceCounters {
    pub(crate) const fn certified(basis_digest_part_count: usize) -> Self {
        Self {
            projection_derivations: 1,
            local_frame_receipts_consumed: 1,
            local_delta_basis_reads: 1,
            plane_distance_checks: 1,
            basis_digest_part_count,
        }
    }

    pub fn projection_derivations(&self) -> usize {
        self.projection_derivations
    }

    pub fn local_frame_receipts_consumed(&self) -> usize {
        self.local_frame_receipts_consumed
    }

    pub fn local_delta_basis_reads(&self) -> usize {
        self.local_delta_basis_reads
    }

    pub fn plane_distance_checks(&self) -> usize {
        self.plane_distance_checks
    }

    pub fn basis_digest_part_count(&self) -> usize {
        self.basis_digest_part_count
    }
}
