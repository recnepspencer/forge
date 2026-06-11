#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DirtyPlanarCleanFailCounters {
    topology_clean_fail_receipts: usize,
    clean_fail_boundary_receipts: usize,
    recovery_receipts: usize,
    transform_posture_receipts: usize,
    diagnostic_receipts: usize,
    user_outcome_receipts: usize,
}

impl DirtyPlanarCleanFailCounters {
    pub(crate) fn from_input(input: DirtyPlanarCleanFailCounterInput) -> Self {
        Self {
            topology_clean_fail_receipts: input.topology_clean_fail_receipts,
            clean_fail_boundary_receipts: input.clean_fail_boundary_receipts,
            recovery_receipts: input.recovery_receipts,
            transform_posture_receipts: input.transform_posture_receipts,
            diagnostic_receipts: input.diagnostic_receipts,
            user_outcome_receipts: input.user_outcome_receipts,
        }
    }

    pub fn topology_clean_fail_receipts(self) -> usize {
        self.topology_clean_fail_receipts
    }

    pub fn clean_fail_boundary_receipts(self) -> usize {
        self.clean_fail_boundary_receipts
    }

    pub fn recovery_receipts(self) -> usize {
        self.recovery_receipts
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
}

pub(crate) struct DirtyPlanarCleanFailCounterInput {
    pub(crate) topology_clean_fail_receipts: usize,
    pub(crate) clean_fail_boundary_receipts: usize,
    pub(crate) recovery_receipts: usize,
    pub(crate) transform_posture_receipts: usize,
    pub(crate) diagnostic_receipts: usize,
    pub(crate) user_outcome_receipts: usize,
}
