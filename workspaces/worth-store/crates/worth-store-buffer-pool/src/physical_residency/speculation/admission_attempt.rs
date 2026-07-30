use std::sync::Arc;

use super::SpeculativeResidencyPermit;
use crate::physical_residency::pool::PoolInner;
use crate::{
    PhysicalOperationAllocationScope, PhysicalResidencyDenial, PhysicalSpeculativeWorkKind,
};

/// One authenticated speculative admission request.
///
/// Dropping an unresolved attempt records exactly one kind denial. Successful
/// admission consumes the attempt and returns the only permit that can release
/// the admitted frames.
pub(in crate::physical_residency) struct SpeculativeAdmissionAttempt {
    owner: Arc<PoolInner>,
    kind: PhysicalSpeculativeWorkKind,
    resolved: bool,
}

impl SpeculativeAdmissionAttempt {
    pub(in crate::physical_residency) fn new(
        owner: Arc<PoolInner>,
        kind: PhysicalSpeculativeWorkKind,
    ) -> Self {
        Self {
            owner,
            kind,
            resolved: false,
        }
    }

    pub(in crate::physical_residency) fn admit(
        mut self,
        scope: PhysicalOperationAllocationScope,
        frames: u32,
    ) -> Result<SpeculativeResidencyPermit, PhysicalResidencyDenial> {
        self.owner.reserve_speculative(scope, self.kind, frames)?;
        self.resolved = true;
        Ok(SpeculativeResidencyPermit::new(
            Arc::clone(&self.owner),
            self.kind,
            frames,
        ))
    }
}

impl Drop for SpeculativeAdmissionAttempt {
    fn drop(&mut self) {
        if !self.resolved {
            self.owner.record_speculative_admission_denial(self.kind);
        }
    }
}
