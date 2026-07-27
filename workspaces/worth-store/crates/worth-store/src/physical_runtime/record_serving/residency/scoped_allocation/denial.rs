use crate::physical_runtime::{
    LifecycleGeneration, PhysicalRecordPressureEvidence, PhysicalRecordResidencyFailure,
    PhysicalRecordResidencyFailureKind, PhysicalRecordResidencyFailureReason,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalScopedAllocationFailure {
    failure: PhysicalRecordResidencyFailure,
    pressure: Option<PhysicalRecordPressureEvidence>,
}

impl PhysicalScopedAllocationFailure {
    pub(super) fn from_denial(
        denial: worth_store_buffer_pool::PhysicalResidencyDenial,
        generation: LifecycleGeneration,
    ) -> Self {
        let failure = denial.into();
        let pressure = PhysicalRecordPressureEvidence::from_store_failure(failure, generation);
        Self { failure, pressure }
    }

    pub const fn kind(self) -> PhysicalRecordResidencyFailureKind {
        self.failure.kind()
    }

    pub const fn reason(self) -> PhysicalRecordResidencyFailureReason {
        self.failure.reason()
    }

    pub const fn pressure(self) -> Option<PhysicalRecordPressureEvidence> {
        self.pressure
    }
}
