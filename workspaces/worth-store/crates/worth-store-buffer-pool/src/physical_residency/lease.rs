use super::{PhysicalFrameKey, PhysicalResidencyDenial, PhysicalResidencyIncarnation};
use crate::physical_residency::pool::PoolInner;
use crate::PhysicalSpeculativeWorkKind;
use sha2::{Digest, Sha256};
use std::{ops::Deref, sync::Arc};

pub(crate) mod dirty_replacement_allocation;

use dirty_replacement_allocation::{DirtyReplacementAllocator, ProcessDirtyReplacementAllocator};

mod candidate;

pub use candidate::{
    PhysicalCandidateBatchAdmission, PhysicalCandidateBatchReservation,
    PhysicalCandidateFrameReservation,
};

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
        allocation: &'grant super::OperationAllocationGrant,
    ) -> Result<PhysicalDirtyReplacementReservation<'grant>, PhysicalResidencyDenial> {
        let scope = allocation.scope_for(&self.owner)?;
        let bytes = u64::from(self.key.coordinate().length());
        self.owner
            .reserve_dirty_replacement(scope, self.key, &self.bytes, bytes)?;
        Ok(PhysicalDirtyReplacementReservation {
            owner: Arc::clone(&self.owner),
            lease: Some(self),
            _allocation: allocation,
            bytes,
            armed: true,
        })
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

#[derive(Debug)]
pub struct DirtyPhysicalFrame {
    pub(crate) lease: Option<PhysicalFrameLease>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PhysicalDirtyReplacementError<E> {
    Residency(PhysicalResidencyDenial),
    Fill(E),
}

#[derive(Debug)]
pub struct PhysicalDirtyReplacementReservation<'grant> {
    owner: Arc<PoolInner>,
    lease: Option<PhysicalFrameLease>,
    _allocation: &'grant super::OperationAllocationGrant,
    bytes: u64,
    armed: bool,
}

impl<'grant> PhysicalDirtyReplacementReservation<'grant> {
    pub fn replace<E, F>(
        self,
        fill: F,
    ) -> Result<DirtyPhysicalFrame, PhysicalDirtyReplacementError<E>>
    where
        F: FnOnce(&[u8], &mut [u8]) -> Result<(), E>,
    {
        self.replace_with_allocator(&ProcessDirtyReplacementAllocator, fill)
    }

    pub(crate) fn replace_with_allocator<E, F>(
        mut self,
        allocator: &dyn DirtyReplacementAllocator,
        fill: F,
    ) -> Result<DirtyPhysicalFrame, PhysicalDirtyReplacementError<E>>
    where
        F: FnOnce(&[u8], &mut [u8]) -> Result<(), E>,
    {
        let length = usize::try_from(self.bytes)
            .expect("physical frame lengths are admitted from u32 coordinates");
        let mut replacement = allocator.allocate(length).map_err(|()| {
            self.release_after_allocator_failure();
            PhysicalDirtyReplacementError::Residency(PhysicalResidencyDenial::AllocationFailed)
        })?;
        let lease = self
            .lease
            .as_ref()
            .expect("armed dirty replacement retains its clean lease");
        if let Err(error) = fill(lease.bytes.as_slice(), replacement.as_mut_slice()) {
            self.release_reservation();
            return Err(PhysicalDirtyReplacementError::Fill(error));
        }
        let replacement = Arc::new(replacement);
        let mut lease = self
            .lease
            .take()
            .expect("armed dirty replacement retains its clean lease");
        if let Err(reason) =
            lease
                .owner
                .finish_dirty_replacement(lease.key, &lease.bytes, Arc::clone(&replacement))
        {
            self.release_reservation();
            return Err(PhysicalDirtyReplacementError::Residency(reason));
        }
        lease.bytes = replacement;
        self.release_reservation();
        Ok(DirtyPhysicalFrame { lease: Some(lease) })
    }

    fn release_reservation(&mut self) {
        if self.armed {
            self.owner.release_dirty_replacement(self.bytes);
            self.armed = false;
        }
    }

    fn release_after_allocator_failure(&mut self) {
        if self.armed {
            self.owner.dirty_replacement_allocator_failed(self.bytes);
            self.armed = false;
        }
    }
}

impl Drop for PhysicalDirtyReplacementReservation<'_> {
    fn drop(&mut self) {
        self.release_reservation();
    }
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
}

#[derive(Debug)]
pub struct SpeculativeResidencyGrant {
    pub(crate) owner: Arc<PoolInner>,
    pub(crate) kind: PhysicalSpeculativeWorkKind,
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
    pub const fn kind(&self) -> PhysicalSpeculativeWorkKind {
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
