use std::sync::Arc;

use super::{
    allocation::ProcessPhysicalFrameAllocator, PhysicalFrameAllocator, PhysicalFrameFaultError,
    PhysicalFrameLoadTerminalKind, PhysicalFrameLoadingIdentity,
};
use crate::physical_residency::pool::PoolInner;
use crate::physical_residency::{
    PhysicalBoundedFrameKey, PhysicalFrameLease, PhysicalResidencyDenial,
};

/// Move-owned authority for length discovery and byte loading of one bounded fault.
#[derive(Debug)]
pub struct PhysicalBoundedFrameFaultOwner {
    pub(crate) owner: Arc<PoolInner>,
    pub(crate) key: PhysicalBoundedFrameKey,
    pub(crate) identity: PhysicalFrameLoadingIdentity,
    pub(crate) scope: crate::PhysicalOperationAllocationScope,
    pub(crate) armed: bool,
}

impl PhysicalBoundedFrameFaultOwner {
    pub const fn loading_identity(&self) -> PhysicalFrameLoadingIdentity {
        self.identity
    }

    pub fn load<E, L, F>(
        self,
        discover_length: L,
        fill: F,
    ) -> Result<PhysicalFrameLease, PhysicalFrameFaultError<E>>
    where
        L: FnOnce(u32) -> Result<u32, E>,
        F: FnOnce(&mut [u8]) -> Result<(), E>,
    {
        self.load_core(&ProcessPhysicalFrameAllocator, discover_length, |target| {
            fill(target).map(|()| None)
        })
    }

    #[cfg(test)]
    pub(crate) fn load_with_allocator<E, L, F>(
        self,
        allocator: &dyn PhysicalFrameAllocator,
        discover_length: L,
        fill: F,
    ) -> Result<PhysicalFrameLease, PhysicalFrameFaultError<E>>
    where
        L: FnOnce(u32) -> Result<u32, E>,
        F: FnOnce(&mut [u8]) -> Result<(), E>,
    {
        self.load_core(allocator, discover_length, |target| {
            fill(target).map(|()| None)
        })
    }

    pub fn load_observed<E, L, F>(
        self,
        discover_length: L,
        fill: F,
    ) -> Result<PhysicalFrameLease, PhysicalFrameFaultError<E>>
    where
        L: FnOnce(u32) -> Result<u32, E>,
        F: FnOnce(&mut [u8]) -> Result<Option<crate::PhysicalResidencyAllocationOperation>, E>,
    {
        self.load_core(&ProcessPhysicalFrameAllocator, discover_length, fill)
    }

    fn load_core<E, L, F>(
        mut self,
        allocator: &dyn PhysicalFrameAllocator,
        discover_length: L,
        fill: F,
    ) -> Result<PhysicalFrameLease, PhysicalFrameFaultError<E>>
    where
        L: FnOnce(u32) -> Result<u32, E>,
        F: FnOnce(&mut [u8]) -> Result<Option<crate::PhysicalResidencyAllocationOperation>, E>,
    {
        self.owner.record_source_load();
        let length = match discover_length(self.key.limit()) {
            Ok(length) if length > 0 && length <= self.key.limit() => length,
            Ok(_) => return Err(self.reject_invalid_length()),
            Err(failure) => return Err(self.reject_source(failure)),
        };
        let mut bytes = match allocator.allocate(length as usize) {
            Ok(bytes) => bytes,
            Err(()) => {
                return Err(self.reject_allocation(PhysicalResidencyDenial::AllocationFailed))
            }
        };
        let actual = bytes.capacity() as u64;
        let requested = u64::from(self.key.limit());
        if actual > requested {
            return Err(self.reject_allocation(
                PhysicalResidencyDenial::AllocatorExceededReservation { requested, actual },
            ));
        }
        let operation =
            fill(bytes.as_mut_slice()).map_err(|failure| self.reject_source(failure))?;
        self.owner.actualize_allocation(
            crate::physical_residency::PhysicalResidencyAllocationActualization::new(
                crate::PhysicalResidencyDimension::ResidentBytes,
                self.scope,
                crate::physical_residency::PhysicalResidencyRequestedAllocationUnits::new(
                    requested,
                ),
                crate::physical_residency::PhysicalResidencyActualAllocationUnits::new(actual),
            )
            .with_operation(operation),
        );
        match self.owner.finish_bounded_loading(
            self.key,
            self.identity,
            length,
            bytes.into_resident(),
        ) {
            Ok(lease) => {
                self.armed = false;
                Ok(lease)
            }
            Err((denial, terminal)) => {
                self.armed = false;
                Err(PhysicalFrameFaultError::Residency { terminal, denial })
            }
        }
    }

    fn reject_invalid_length<E>(&mut self) -> PhysicalFrameFaultError<E> {
        let terminal = self.owner.fail_bounded_loading(
            self.key,
            self.identity,
            PhysicalFrameLoadTerminalKind::SourcePreparationFailed,
        );
        self.armed = false;
        PhysicalFrameFaultError::Residency {
            terminal,
            denial: self
                .owner
                .record_denial(PhysicalResidencyDenial::FrameLengthMismatch),
        }
    }

    fn reject_allocation<E>(
        &mut self,
        denial: PhysicalResidencyDenial,
    ) -> PhysicalFrameFaultError<E> {
        let terminal = self.owner.fail_bounded_loading(
            self.key,
            self.identity,
            PhysicalFrameLoadTerminalKind::AllocationFailed,
        );
        self.armed = false;
        PhysicalFrameFaultError::Residency {
            terminal,
            denial: self.owner.record_denial(denial),
        }
    }

    fn reject_source<E>(&mut self, failure: E) -> PhysicalFrameFaultError<E> {
        let terminal = self.owner.fail_bounded_loading(
            self.key,
            self.identity,
            PhysicalFrameLoadTerminalKind::SourceExecutionFailed,
        );
        self.armed = false;
        PhysicalFrameFaultError::Source { terminal, failure }
    }
}

impl Drop for PhysicalBoundedFrameFaultOwner {
    fn drop(&mut self) {
        if self.armed {
            self.owner.abandon_bounded_loading(self.key, self.identity);
        }
    }
}
