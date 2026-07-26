use std::sync::Arc;

use super::{
    PhysicalFrameLoadTerminal, PhysicalFrameLoadTerminalKind, PhysicalFrameLoadingIdentity,
};
use crate::physical_residency::pool::PoolInner;
use crate::physical_residency::{PhysicalFrameKey, PhysicalFrameLease, PhysicalResidencyDenial};

/// Move-owned authority for the sole source load of one admitted frame fault.
#[derive(Debug)]
pub struct PhysicalFrameFaultOwner {
    pub(crate) owner: Arc<PoolInner>,
    pub(crate) key: PhysicalFrameKey,
    pub(crate) identity: PhysicalFrameLoadingIdentity,
    pub(crate) armed: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PhysicalFrameFaultError<E> {
    Residency {
        terminal: PhysicalFrameLoadTerminal,
        denial: PhysicalResidencyDenial,
    },
    Source {
        terminal: PhysicalFrameLoadTerminal,
        failure: E,
    },
}

impl PhysicalFrameFaultOwner {
    pub const fn loading_identity(&self) -> PhysicalFrameLoadingIdentity {
        self.identity
    }

    pub fn reject_before_source(mut self) -> PhysicalFrameLoadTerminal {
        let terminal = self.owner.fail_loading(
            self.key,
            self.identity,
            PhysicalFrameLoadTerminalKind::SourcePreparationFailed,
        );
        self.armed = false;
        terminal
    }

    pub fn load<E, F>(mut self, fill: F) -> Result<PhysicalFrameLease, PhysicalFrameFaultError<E>>
    where
        F: FnOnce(&mut [u8]) -> Result<(), E>,
    {
        let length = self.key.coordinate().length() as usize;
        let mut bytes = Vec::new();
        if bytes.try_reserve_exact(length).is_err() {
            let terminal = self.owner.fail_loading(
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
        bytes.resize(length, 0);
        self.owner.record_source_load();
        if let Err(failure) = fill(bytes.as_mut_slice()) {
            let terminal = self.owner.fail_loading(
                self.key,
                self.identity,
                PhysicalFrameLoadTerminalKind::SourceExecutionFailed,
            );
            self.armed = false;
            return Err(PhysicalFrameFaultError::Source { terminal, failure });
        }
        match self
            .owner
            .finish_loading(self.key, self.identity, Arc::new(bytes))
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
}

impl Drop for PhysicalFrameFaultOwner {
    fn drop(&mut self) {
        if self.armed {
            self.owner.abandon_loading(self.key, self.identity);
        }
    }
}
