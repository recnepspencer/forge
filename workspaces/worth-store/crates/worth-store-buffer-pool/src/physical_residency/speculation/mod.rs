use std::sync::Arc;

use super::{pool::PoolInner, PhysicalSpeculativeWorkKind};

mod admission;
mod admission_attempt;
mod queue_declaration;
mod read_grant;
mod writebehind_grant;

pub(in crate::physical_residency) use admission_attempt::SpeculativeAdmissionAttempt;
pub use queue_declaration::{
    BufferPoolQueueDeclarationContext, BufferPoolQueueGroupingScope,
    BufferPoolQueueWriteDurability, BufferPoolReadQueueExecutionDeclaration,
    BufferPoolReadQueueExecutionKind, BufferPoolWritebackQueueExecutionDeclaration,
};
pub use read_grant::{PrefetchResidencyGrant, ReadAheadFrameGrant, ReadAheadResidencyGrant};
pub use writebehind_grant::WriteBehindResidencyGrant;

#[derive(Debug)]
pub(in crate::physical_residency) struct SpeculativeResidencyPermit {
    owner: Arc<PoolInner>,
    kind: PhysicalSpeculativeWorkKind,
    frames: u32,
}

impl SpeculativeResidencyPermit {
    pub(in crate::physical_residency) fn new(
        owner: Arc<PoolInner>,
        kind: PhysicalSpeculativeWorkKind,
        frames: u32,
    ) -> Self {
        Self {
            owner,
            kind,
            frames,
        }
    }
}

impl Drop for SpeculativeResidencyPermit {
    fn drop(&mut self) {
        self.owner.release_speculative(self.kind, self.frames);
    }
}
