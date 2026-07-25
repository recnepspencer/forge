use super::{PhysicalFrameKey, PhysicalResidencyDenial, PhysicalResidencyIncarnation};
use crate::physical_residency::pool::PoolInner;
use crate::SpeculativePhysicalWorkKind;
use sha2::{Digest, Sha256};
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

    pub fn replace_with_dirty_candidate(
        mut self,
        bytes: Vec<u8>,
    ) -> Result<DirtyPhysicalFrame, PhysicalResidencyDenial> {
        if bytes.len() != self.key.coordinate().length() as usize {
            return Err(self
                .owner
                .record_denial(PhysicalResidencyDenial::FrameLengthMismatch));
        }
        let replacement = Arc::new(bytes);
        self.owner.replace_clean_lease_with_dirty(
            self.key,
            &self.bytes,
            Arc::clone(&replacement),
        )?;
        self.bytes = replacement;
        Ok(DirtyPhysicalFrame { lease: Some(self) })
    }
}

impl Deref for PhysicalFrameLease {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.bytes.as_slice()
    }
}

#[derive(Debug)]
pub struct PhysicalCandidateFrameReservation {
    pub(crate) owner: Arc<PoolInner>,
    pub(crate) key: PhysicalFrameKey,
    pub(crate) armed: bool,
}

#[derive(Debug)]
pub struct PhysicalCandidateBatchReservation {
    pub(crate) owner: Arc<PoolInner>,
    pub(crate) keys: std::collections::VecDeque<PhysicalFrameKey>,
    pub(crate) armed: bool,
}

impl PhysicalCandidateBatchReservation {
    pub fn reserve_next(
        &mut self,
        key: PhysicalFrameKey,
    ) -> Result<PhysicalCandidateFrameReservation, PhysicalResidencyDenial> {
        if self.keys.front().copied() != Some(key) {
            return Err(self
                .owner
                .record_denial(PhysicalResidencyDenial::FrameLengthMismatch));
        }
        self.owner.reserve_next_candidate(key)?;
        self.keys.pop_front();
        Ok(PhysicalCandidateFrameReservation::new(
            Arc::clone(&self.owner),
            key,
        ))
    }
}

impl Drop for PhysicalCandidateBatchReservation {
    fn drop(&mut self) {
        if self.armed {
            self.owner.finish_candidate_batch();
            self.armed = false;
        }
    }
}

impl PhysicalCandidateFrameReservation {
    pub const fn key(&self) -> PhysicalFrameKey {
        self.key
    }

    pub fn admit(mut self, bytes: Vec<u8>) -> Result<DirtyPhysicalFrame, PhysicalResidencyDenial> {
        if bytes.len() != self.key.coordinate().length() as usize {
            return Err(self
                .owner
                .record_denial(PhysicalResidencyDenial::FrameLengthMismatch));
        }
        let bytes = Arc::new(bytes);
        let frame = self.owner.finish_candidate(self.key, bytes)?;
        self.armed = false;
        Ok(frame)
    }
}

impl Drop for PhysicalCandidateFrameReservation {
    fn drop(&mut self) {
        if self.armed {
            self.owner.cancel_candidate(self.key);
        }
    }
}

impl Drop for PhysicalFrameLease {
    fn drop(&mut self) {
        self.owner.release_pin(self.key);
    }
}

#[derive(Debug)]
pub struct DirtyPhysicalFrame {
    pub(crate) lease: Option<PhysicalFrameLease>,
}

impl DirtyPhysicalFrame {
    pub fn bytes(&self) -> &[u8] {
        self.lease
            .as_ref()
            .expect("dirty frame lease is present")
            .bytes
            .as_slice()
    }

    pub fn publish_clean(
        mut self,
        receipt: &worth_store_physical_backend::CompletedArtifactRangeWrite,
    ) -> Result<PhysicalFrameLease, PhysicalResidencyDenial> {
        let lease = self.lease.take().expect("dirty frame lease is present");
        validate_completed_write(&lease.owner, lease.key, lease.bytes.as_slice(), receipt)?;
        lease.owner.publish_clean(lease.key)?;
        Ok(lease)
    }

    pub fn discard_candidate(mut self) -> Result<(), PhysicalResidencyDenial> {
        let lease = self.lease.take().expect("dirty frame lease is present");
        lease.owner.discard_dirty_candidate(lease.key)
    }

    #[cfg(test)]
    pub(crate) fn publish_clean_for_pool_test(
        mut self,
    ) -> Result<PhysicalFrameLease, PhysicalResidencyDenial> {
        let lease = self.lease.take().expect("dirty frame lease is present");
        lease.owner.publish_clean(lease.key)?;
        Ok(lease)
    }
}

#[derive(Debug)]
pub struct SpeculativeResidencyGrant {
    pub(crate) owner: Arc<PoolInner>,
    pub(crate) kind: SpeculativePhysicalWorkKind,
    pub(crate) frames: u32,
}

#[derive(Debug)]
pub struct PhysicalWritebackClaim {
    pub(crate) owner: Arc<PoolInner>,
    pub(crate) frames: Vec<PhysicalFrameKey>,
    pub(crate) bytes: Vec<Arc<Vec<u8>>>,
    pub(crate) armed: bool,
}

impl PhysicalWritebackClaim {
    pub fn frames(&self) -> &[PhysicalFrameKey] {
        &self.frames
    }

    pub fn frame_bytes(&self, index: usize) -> Option<&[u8]> {
        self.bytes.get(index).map(|bytes| bytes.as_slice())
    }

    pub fn store_identity(
        &self,
    ) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.owner.store_identity()
    }

    pub fn pool_incarnation(&self) -> PhysicalResidencyIncarnation {
        self.owner.incarnation()
    }

    pub fn publish_clean(
        mut self,
        receipt: &worth_store_physical_backend::CompletedArtifactRangeWrite,
    ) -> Result<(), PhysicalResidencyDenial> {
        let [frame] = self.frames.as_slice() else {
            return Err(self
                .owner
                .record_denial(PhysicalResidencyDenial::WriteBackReceiptMismatch));
        };
        let [bytes] = self.bytes.as_slice() else {
            return Err(self
                .owner
                .record_denial(PhysicalResidencyDenial::WriteBackReceiptMismatch));
        };
        validate_completed_write(&self.owner, *frame, bytes.as_slice(), receipt)?;
        self.owner.complete_writeback_claim(&self.frames)?;
        self.armed = false;
        Ok(())
    }
}

fn validate_completed_write(
    owner: &PoolInner,
    frame: PhysicalFrameKey,
    bytes: &[u8],
    receipt: &worth_store_physical_backend::CompletedArtifactRangeWrite,
) -> Result<(), PhysicalResidencyDenial> {
    let digest: [u8; 32] = Sha256::digest(bytes).into();
    if receipt.store() != owner.store_identity()
        || receipt.coordinate() != frame.coordinate()
        || receipt.completed_bytes() != bytes.len() as u64
        || receipt.payload_digest() != digest
    {
        return Err(owner.record_denial(PhysicalResidencyDenial::WriteBackReceiptMismatch));
    }
    Ok(())
}

impl Drop for PhysicalWritebackClaim {
    fn drop(&mut self) {
        if self.armed {
            self.owner.release_writeback_claim(&self.frames);
        }
    }
}

impl SpeculativeResidencyGrant {
    pub const fn kind(&self) -> SpeculativePhysicalWorkKind {
        self.kind
    }

    pub const fn frames(&self) -> u32 {
        self.frames
    }
}

impl Drop for SpeculativeResidencyGrant {
    fn drop(&mut self) {
        self.owner.release_speculative(self.kind, self.frames);
    }
}
