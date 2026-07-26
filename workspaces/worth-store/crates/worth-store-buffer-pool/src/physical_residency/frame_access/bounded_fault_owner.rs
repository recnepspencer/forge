use std::sync::Arc;

use super::{PhysicalFrameFaultError, PhysicalFrameLoadTerminalKind, PhysicalFrameLoadingIdentity};
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
    pub(crate) armed: bool,
}

impl PhysicalBoundedFrameFaultOwner {
    pub const fn loading_identity(&self) -> PhysicalFrameLoadingIdentity {
        self.identity
    }

    pub fn load<E, L, F>(
        mut self,
        discover_length: L,
        fill: F,
    ) -> Result<PhysicalFrameLease, PhysicalFrameFaultError<E>>
    where
        L: FnOnce(u32) -> Result<u32, E>,
        F: FnOnce(&mut [u8]) -> Result<(), E>,
    {
        self.owner.record_source_load();
        let length = match discover_length(self.key.limit()) {
            Ok(length) if length > 0 && length <= self.key.limit() => length,
            Ok(_) => return Err(self.reject_invalid_length()),
            Err(failure) => return Err(self.reject_source(failure)),
        };
        let mut bytes = Vec::new();
        if bytes.try_reserve_exact(length as usize).is_err() {
            let terminal = self.owner.fail_bounded_loading(
                self.key,
                self.identity,
                PhysicalFrameLoadTerminalKind::AllocationFailed,
            );
            self.armed = false;
            return Err(PhysicalFrameFaultError::Residency {
                terminal,
                denial: self
                    .owner
                    .record_denial(PhysicalResidencyDenial::AllocationFailed),
            });
        }
        bytes.resize(length as usize, 0);
        if let Err(failure) = fill(bytes.as_mut_slice()) {
            return Err(self.reject_source(failure));
        }
        match self
            .owner
            .finish_bounded_loading(self.key, self.identity, length, Arc::new(bytes))
        {
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
