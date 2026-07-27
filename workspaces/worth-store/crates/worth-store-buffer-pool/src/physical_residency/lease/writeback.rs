use crate::physical_residency::pool::PoolInner;
use crate::physical_residency::pool_ownership::FrameWritebackCleanAuthority;
use crate::{
    PhysicalFrameKey, PhysicalResidencyDenial, PhysicalResidencyIncarnation,
    WriteBehindResidencyGrant,
};
use std::sync::Arc;

#[derive(Debug)]
pub struct PhysicalWritebackClaim {
    pub(crate) owner: Arc<PoolInner>,
    pub(crate) writebehind: WriteBehindResidencyGrant,
    pub(crate) bytes: Vec<Arc<Vec<u8>>>,
    pub(crate) range_postures: Vec<crate::PhysicalWritebackRangePosture>,
    pub(crate) armed: bool,
}

impl PhysicalWritebackClaim {
    pub fn frames(&self) -> &[PhysicalFrameKey] {
        self.writebehind.frames()
    }

    pub fn frame_bytes(&self, index: usize) -> Option<&[u8]> {
        self.bytes.get(index).map(|bytes| bytes.as_slice())
    }

    pub fn range_posture(&self, index: usize) -> Option<crate::PhysicalWritebackRangePosture> {
        self.range_postures.get(index).copied()
    }

    pub fn store_identity(
        &self,
    ) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.owner.store_identity()
    }

    pub fn pool_incarnation(&self) -> PhysicalResidencyIncarnation {
        self.owner.incarnation()
    }

    pub const fn writebehind_grant(&self) -> &WriteBehindResidencyGrant {
        &self.writebehind
    }

    pub fn complete_writeback(
        mut self,
        authority: &FrameWritebackCleanAuthority,
    ) -> Result<(), PhysicalResidencyDenial> {
        if !authority.authorizes(&self.owner) {
            return Err(PhysicalResidencyDenial::WritebackCleanAuthorityMismatch);
        }
        self.owner.complete_writeback_claim(self.frames())?;
        self.armed = false;
        Ok(())
    }
}

impl Drop for PhysicalWritebackClaim {
    fn drop(&mut self) {
        if self.armed {
            self.owner.release_writeback_claim(self.frames());
        }
    }
}
