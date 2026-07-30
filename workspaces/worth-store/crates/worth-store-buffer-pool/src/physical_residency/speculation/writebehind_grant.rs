use std::sync::Arc;

use super::SpeculativeResidencyPermit;
use crate::{
    physical_residency::pool::PoolInner, ForegroundWriteAllocationGrant, PhysicalFrameKey,
    PhysicalResidencyIncarnation, PhysicalSpeculativeWorkKind,
};

/// Exact dirty-frame authority retained for the lifetime of a writeback claim.
#[derive(Debug)]
pub struct WriteBehindResidencyGrant {
    pub(super) permit: SpeculativeResidencyPermit,
    pub(super) allocation: ForegroundWriteAllocationGrant,
    pub(super) frames: Box<[PhysicalFrameKey]>,
}

impl WriteBehindResidencyGrant {
    pub(in crate::physical_residency) fn new(
        owner: Arc<PoolInner>,
        allocation: ForegroundWriteAllocationGrant,
        frames: Box<[PhysicalFrameKey]>,
    ) -> Self {
        let count = frames.len() as u32;
        Self {
            permit: SpeculativeResidencyPermit::new(
                owner,
                PhysicalSpeculativeWorkKind::WriteBehind,
                count,
            ),
            allocation,
            frames,
        }
    }

    pub fn frames(&self) -> &[PhysicalFrameKey] {
        &self.frames
    }

    pub fn store_identity(
        &self,
    ) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.permit.owner.store_identity()
    }

    pub fn pool_incarnation(&self) -> PhysicalResidencyIncarnation {
        self.permit.owner.incarnation()
    }

    pub fn allocation_bytes(&self) -> u64 {
        self.allocation.bytes()
    }
}
