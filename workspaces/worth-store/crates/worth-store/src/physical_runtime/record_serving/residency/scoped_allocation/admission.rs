use std::num::NonZeroU64;

use worth_store_buffer_pool::PhysicalOperationAllocationScope;

use crate::physical_runtime::{
    record_serving::residency::frame_ports::RecordFramePorts, LifecycleGeneration, RuntimeIdentity,
};

use super::{
    BlobPhysicalAllocation, MaintenancePhysicalAllocation, PhysicalScopedAllocationFailure,
    RecoveryPhysicalAllocation, ScrubPhysicalAllocation, VerificationPhysicalAllocation,
};

pub struct PhysicalScopedAllocationAdmission<'runtime> {
    frame_ports: &'runtime RecordFramePorts,
    runtime: RuntimeIdentity,
    generation: LifecycleGeneration,
}

impl<'runtime> PhysicalScopedAllocationAdmission<'runtime> {
    pub(in crate::physical_runtime) const fn new(
        frame_ports: &'runtime RecordFramePorts,
        runtime: RuntimeIdentity,
        generation: LifecycleGeneration,
    ) -> Self {
        Self {
            frame_ports,
            runtime,
            generation,
        }
    }

    pub fn admit_recovery(
        &self,
        bytes: NonZeroU64,
    ) -> Result<RecoveryPhysicalAllocation<'runtime>, PhysicalScopedAllocationFailure> {
        self.admit(PhysicalOperationAllocationScope::Recovery, bytes)
            .map(|grant| {
                RecoveryPhysicalAllocation::bind(
                    grant,
                    self.frame_ports,
                    self.runtime,
                    self.generation,
                )
            })
    }

    pub fn admit_scrub(
        &self,
        bytes: NonZeroU64,
    ) -> Result<ScrubPhysicalAllocation<'runtime>, PhysicalScopedAllocationFailure> {
        self.admit(PhysicalOperationAllocationScope::Scrub, bytes)
            .map(|grant| {
                ScrubPhysicalAllocation::bind(
                    grant,
                    self.frame_ports,
                    self.runtime,
                    self.generation,
                )
            })
    }

    pub fn admit_maintenance(
        &self,
        bytes: NonZeroU64,
    ) -> Result<MaintenancePhysicalAllocation<'runtime>, PhysicalScopedAllocationFailure> {
        self.admit(PhysicalOperationAllocationScope::Maintenance, bytes)
            .map(|grant| {
                MaintenancePhysicalAllocation::bind(
                    grant,
                    self.frame_ports,
                    self.runtime,
                    self.generation,
                )
            })
    }

    pub fn admit_verification(
        &self,
        bytes: NonZeroU64,
    ) -> Result<VerificationPhysicalAllocation<'runtime>, PhysicalScopedAllocationFailure> {
        self.admit(PhysicalOperationAllocationScope::Verification, bytes)
            .map(|grant| {
                VerificationPhysicalAllocation::bind(
                    grant,
                    self.frame_ports,
                    self.runtime,
                    self.generation,
                )
            })
    }

    pub fn admit_blob(
        &self,
        bytes: NonZeroU64,
    ) -> Result<BlobPhysicalAllocation<'runtime>, PhysicalScopedAllocationFailure> {
        self.admit(PhysicalOperationAllocationScope::Blob, bytes)
            .map(|grant| {
                BlobPhysicalAllocation::bind(grant, self.frame_ports, self.runtime, self.generation)
            })
    }

    fn admit(
        &self,
        scope: PhysicalOperationAllocationScope,
        bytes: NonZeroU64,
    ) -> Result<worth_store_buffer_pool::OperationAllocationGrant, PhysicalScopedAllocationFailure>
    {
        self.frame_ports
            .begin_operation(scope, bytes)
            .map_err(|denial| PhysicalScopedAllocationFailure::from_denial(denial, self.generation))
    }
}
