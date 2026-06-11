#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpenPlanarPostureCounters {
    topology_receipts: usize,
    unsupported_surface_receipts: usize,
    clean_fail_boundary_receipts: usize,
    transform_posture_receipts: usize,
    diagnostic_receipts: usize,
    user_outcome_receipts: usize,
    bounded_surrogate_rejections: usize,
}

impl OpenPlanarPostureCounters {
    pub(crate) fn from_input(input: OpenPlanarPostureCounterInput) -> Self {
        Self {
            topology_receipts: input.topology_receipts,
            unsupported_surface_receipts: input.unsupported_surface_receipts,
            clean_fail_boundary_receipts: input.clean_fail_boundary_receipts,
            transform_posture_receipts: input.transform_posture_receipts,
            diagnostic_receipts: input.diagnostic_receipts,
            user_outcome_receipts: input.user_outcome_receipts,
            bounded_surrogate_rejections: input.bounded_surrogate_rejections,
        }
    }

    pub fn topology_receipts(self) -> usize {
        self.topology_receipts
    }

    pub fn unsupported_surface_receipts(self) -> usize {
        self.unsupported_surface_receipts
    }

    pub fn clean_fail_boundary_receipts(self) -> usize {
        self.clean_fail_boundary_receipts
    }

    pub fn transform_posture_receipts(self) -> usize {
        self.transform_posture_receipts
    }

    pub fn diagnostic_receipts(self) -> usize {
        self.diagnostic_receipts
    }

    pub fn user_outcome_receipts(self) -> usize {
        self.user_outcome_receipts
    }

    pub fn bounded_surrogate_rejections(self) -> usize {
        self.bounded_surrogate_rejections
    }
}

pub(crate) struct OpenPlanarPostureCounterInput {
    pub(crate) topology_receipts: usize,
    pub(crate) unsupported_surface_receipts: usize,
    pub(crate) clean_fail_boundary_receipts: usize,
    pub(crate) transform_posture_receipts: usize,
    pub(crate) diagnostic_receipts: usize,
    pub(crate) user_outcome_receipts: usize,
    pub(crate) bounded_surrogate_rejections: usize,
}
