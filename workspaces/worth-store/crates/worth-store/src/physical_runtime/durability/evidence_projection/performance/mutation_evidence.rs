use crate::physical_runtime::{PhysicalMutationAcknowledgment, PhysicalMutationIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalMutationPerformanceEvidence {
    mutation: PhysicalMutationIdentity,
    bytes_requested: u64,
    bytes_completed: u64,
    transfer_count: u64,
    explicit_copy_count: u64,
    copied_bytes: u64,
    peak_scratch_bytes: u64,
}

impl PhysicalMutationPerformanceEvidence {
    pub(in crate::physical_runtime) fn from_acknowledgment(
        acknowledgment: &PhysicalMutationAcknowledgment,
    ) -> Self {
        let observation = acknowledgment.observation();
        Self {
            mutation: acknowledgment.mutation_identity(),
            bytes_requested: observation.bytes_requested(),
            bytes_completed: observation.bytes_completed(),
            transfer_count: observation.transfer_count(),
            explicit_copy_count: observation.explicit_copy_count(),
            copied_bytes: observation.copied_bytes(),
            peak_scratch_bytes: observation.peak_scratch_bytes(),
        }
    }

    pub const fn mutation_identity(self) -> PhysicalMutationIdentity {
        self.mutation
    }
    pub const fn bytes_requested(self) -> u64 {
        self.bytes_requested
    }
    pub const fn bytes_completed(self) -> u64 {
        self.bytes_completed
    }
    pub const fn transfer_count(self) -> u64 {
        self.transfer_count
    }
    pub const fn explicit_copy_count(self) -> u64 {
        self.explicit_copy_count
    }
    pub const fn copied_bytes(self) -> u64 {
        self.copied_bytes
    }
    pub const fn peak_scratch_bytes(self) -> u64 {
        self.peak_scratch_bytes
    }
}
