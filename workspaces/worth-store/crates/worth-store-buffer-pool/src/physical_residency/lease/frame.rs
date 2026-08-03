use super::PhysicalDirtyReplacementReservation;
use crate::physical_residency::pool::PoolInner;
use crate::physical_residency::PhysicalFrameKey;
use crate::PhysicalResidencyDenial;
use std::{ops::Deref, sync::Arc};

#[derive(Debug)]
pub struct PhysicalFrameLease {
    pub(crate) owner: Arc<PoolInner>,
    pub(crate) key: PhysicalFrameKey,
    pub(crate) bytes: Arc<Vec<u8>>,
}

impl PhysicalFrameLease {
    pub const fn key(&self) -> PhysicalFrameKey {
        self.key
    }

    pub fn copy_range_into(&self, range: std::ops::Range<usize>, target: &mut [u8]) {
        assert_eq!(
            range.len(),
            target.len(),
            "copy target must match frame range"
        );
        target.copy_from_slice(&self.bytes[range]);
        self.owner.record_copy(target.len() as u64);
    }

    pub fn begin_dirty_replacement<'grant>(
        self,
        allocation: &'grant super::super::ForegroundWriteAllocationGrant,
    ) -> Result<PhysicalDirtyReplacementReservation<'grant>, PhysicalResidencyDenial> {
        let bytes = u64::from(self.key.coordinate().length());
        let allocation_use = allocation.reserve_use(&self.owner, bytes)?;
        self.owner.reserve_dirty_replacement(
            allocation_use.scope(),
            self.key,
            &self.bytes,
            bytes,
        )?;
        Ok(PhysicalDirtyReplacementReservation::new(
            self,
            allocation_use,
            bytes,
        ))
    }
}

impl Deref for PhysicalFrameLease {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.bytes.as_slice()
    }
}

impl Drop for PhysicalFrameLease {
    fn drop(&mut self) {
        self.owner.release_pin(self.key);
    }
}
