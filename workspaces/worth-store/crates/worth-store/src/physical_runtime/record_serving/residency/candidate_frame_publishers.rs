use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use sha2::Digest;
use worth_store_buffer_pool::{
    CandidateFrameCleanAuthority, DirtyPhysicalFrame, PhysicalCandidateBatchReservation,
    PhysicalCandidateFrameKey, PhysicalFrameKey, PhysicalResidencyPool,
};
use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_format::RecordFrameCoordinate;

use super::super::RecordAppendDenial;
use super::candidate_frame_residency::{
    CandidateFrame, CandidateFrameCoordinate, CandidateFramePublicationPort,
    CandidateFrameResidencySession, CandidateFrameResidencySettlement, CandidateFrameRole,
    CandidateFrameSet, CandidateFrameWriteCompletion, ResidentCandidateFrame,
};

#[derive(Default)]
pub(in crate::physical_runtime::record_serving) struct CandidateFrameCounterCells {
    submissions: AtomicU64,
    declared_frames: AtomicU64,
    declared_bytes: AtomicU64,
    retained_frames: AtomicU64,
    retained_bytes: AtomicU64,
    #[cfg(feature = "certification-test-authority")]
    reject_next_publication: std::sync::atomic::AtomicBool,
}

#[derive(Clone)]
pub(in crate::physical_runtime::record_serving) struct BoundedCandidateFramePublisher {
    pool: PhysicalResidencyPool,
    counters: Arc<CandidateFrameCounterCells>,
    candidate_clean: Arc<CandidateFrameCleanAuthority>,
}

impl BoundedCandidateFramePublisher {
    pub(in crate::physical_runtime::record_serving) fn new(
        pool: PhysicalResidencyPool,
        counters: Arc<CandidateFrameCounterCells>,
        candidate_clean: Arc<CandidateFrameCleanAuthority>,
    ) -> Self {
        Self {
            pool,
            counters,
            candidate_clean,
        }
    }
}

impl CandidateFramePublicationPort for BoundedCandidateFramePublisher {
    fn begin<'allocation>(
        &self,
        allocation: &'allocation worth_store_buffer_pool::ForegroundWriteAllocationGrant,
        candidate: &CandidateFrameSet,
    ) -> Result<Box<dyn CandidateFrameResidencySession + 'allocation>, RecordAppendDenial> {
        let candidate_count = std::num::NonZeroUsize::new(candidate.declarations().len())
            .ok_or_else(|| {
                RecordAppendDenial::from_residency(
                    worth_store_buffer_pool::PhysicalResidencyDenial::EmptyCandidateBatch,
                )
            })?;
        let admission = self
            .pool
            .begin_candidate_batch(allocation, candidate_count)
            .map_err(RecordAppendDenial::from_residency)?;
        self.counters.submissions.fetch_add(1, Ordering::AcqRel);
        self.counters
            .declared_frames
            .fetch_add(candidate.frame_count(), Ordering::AcqRel);
        self.counters
            .declared_bytes
            .fetch_add(candidate.total_frame_bytes(), Ordering::AcqRel);
        let mut keys = Vec::new();
        keys.try_reserve_exact(candidate.declarations().len())
            .map_err(|_| {
                RecordAppendDenial::from_residency(
                    worth_store_buffer_pool::PhysicalResidencyDenial::AllocationFailed,
                )
            })?;
        for declaration in candidate.declarations() {
            keys.push(candidate_key(self.pool.store_identity(), *declaration)?);
        }
        let reservations = admission
            .reserve(&keys)
            .map_err(RecordAppendDenial::from_residency)?;
        Ok(Box::new(BoundedCandidateFrameSession {
            pool: self.pool.clone(),
            counters: Arc::clone(&self.counters),
            candidate_clean: Arc::clone(&self.candidate_clean),
            reservations,
        }))
    }
}

struct BoundedCandidateFrameSession<'allocation> {
    pool: PhysicalResidencyPool,
    counters: Arc<CandidateFrameCounterCells>,
    candidate_clean: Arc<CandidateFrameCleanAuthority>,
    reservations: PhysicalCandidateBatchReservation<'allocation>,
}

impl CandidateFrameResidencySession for BoundedCandidateFrameSession<'_> {
    fn retain(
        &mut self,
        frame: CandidateFrame,
    ) -> Result<Box<dyn ResidentCandidateFrame>, RecordAppendDenial> {
        let role = frame.role();
        let coordinate = frame.coordinate();
        let bytes = frame.into_bytes();
        let length = u32::try_from(bytes.len()).map_err(|_| {
            RecordAppendDenial::from_residency(
                worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
            )
        })?;
        let physical_coordinate =
            RecordFrameCoordinate::new(coordinate.artifact(), coordinate.offset(), length).ok_or(
                RecordAppendDenial::from_residency(
                    worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
                ),
            )?;
        let key = PhysicalFrameKey::new(self.pool.store_identity(), physical_coordinate);
        let candidate = if role.is_complete_artifact() {
            PhysicalCandidateFrameKey::complete_artifact(key).ok_or(
                RecordAppendDenial::from_residency(
                    worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
                ),
            )?
        } else {
            PhysicalCandidateFrameKey::fragment(key)
        };
        let resident = self
            .reservations
            .reserve_next(candidate)
            .map_err(RecordAppendDenial::from_residency)?
            .materialize(move |resident| resident.copy_from_slice(&bytes))
            .map_err(RecordAppendDenial::from_residency)?;
        self.counters.retained_frames.fetch_add(1, Ordering::AcqRel);
        self.counters
            .retained_bytes
            .fetch_add(u64::from(length), Ordering::AcqRel);
        Ok(Box::new(BoundedResidentCandidateFrame {
            store: self.pool.store_identity(),
            role,
            coordinate,
            resident,
            candidate_clean: Arc::clone(&self.candidate_clean),
            #[cfg(feature = "certification-test-authority")]
            counters: Arc::clone(&self.counters),
        }))
    }

    fn prepare_catalog_cutover(
        &mut self,
        target: CandidateFrameCoordinate,
        length: u32,
    ) -> Result<(), RecordAppendDenial> {
        let target = RecordFrameCoordinate::new(target.artifact(), target.offset(), length).ok_or(
            RecordAppendDenial::from_residency(
                worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
            ),
        )?;
        self.pool
            .invalidate_clean(PhysicalFrameKey::new(self.pool.store_identity(), target))
            .map_err(RecordAppendDenial::from_residency)
    }
}

fn candidate_key(
    store: StableStoreIdentity,
    declaration: super::candidate_frame_residency::CandidateFrameDeclaration,
) -> Result<PhysicalCandidateFrameKey, RecordAppendDenial> {
    let coordinate = declaration.coordinate();
    let physical_coordinate = RecordFrameCoordinate::new(
        coordinate.artifact(),
        coordinate.offset(),
        declaration.length(),
    )
    .ok_or(RecordAppendDenial::from_residency(
        worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
    ))?;
    let frame = PhysicalFrameKey::new(store, physical_coordinate);
    if declaration.role().is_complete_artifact() {
        PhysicalCandidateFrameKey::complete_artifact(frame).ok_or(
            RecordAppendDenial::from_residency(
                worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
            ),
        )
    } else {
        Ok(PhysicalCandidateFrameKey::fragment(frame))
    }
}

struct BoundedResidentCandidateFrame {
    store: StableStoreIdentity,
    role: CandidateFrameRole,
    coordinate: CandidateFrameCoordinate,
    resident: DirtyPhysicalFrame,
    candidate_clean: Arc<CandidateFrameCleanAuthority>,
    #[cfg(feature = "certification-test-authority")]
    counters: Arc<CandidateFrameCounterCells>,
}

impl ResidentCandidateFrame for BoundedResidentCandidateFrame {
    fn store_identity(&self) -> StableStoreIdentity {
        self.store
    }

    fn role(&self) -> CandidateFrameRole {
        self.role
    }
    fn coordinate(&self) -> CandidateFrameCoordinate {
        self.coordinate
    }
    fn bytes(&self) -> &[u8] {
        self.resident.bytes()
    }

    fn discard(self: Box<Self>) -> Result<(), RecordAppendDenial> {
        self.resident
            .discard_candidate()
            .map_err(RecordAppendDenial::from_residency)
    }

    fn into_dirty(self: Box<Self>) -> Result<DirtyPhysicalFrame, RecordAppendDenial> {
        Ok(self.resident)
    }

    fn publish_clean(
        self: Box<Self>,
        settlement: CandidateFrameResidencySettlement,
    ) -> Result<CandidateFrameWriteCompletion, RecordAppendDenial> {
        let settlement = settlement.settlement();
        #[cfg(feature = "certification-test-authority")]
        if self.counters.take_reject_next_publication() {
            return Err(RecordAppendDenial::from_residency(
                worth_store_buffer_pool::PhysicalResidencyDenial::CandidatePublicationActive,
            ));
        }
        let bytes = self.resident.bytes().len() as u64;
        let length = u32::try_from(bytes).map_err(|_| {
            RecordAppendDenial::from_residency(
                worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
            )
        })?;
        let coordinate = RecordFrameCoordinate::new(
            self.coordinate.artifact(),
            self.coordinate.offset(),
            length,
        )
        .ok_or_else(|| {
            RecordAppendDenial::from_residency(
                worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
            )
        })?;
        let payload_digest: [u8; 32] = sha2::Sha256::digest(self.resident.bytes()).into();
        self.resident
            .complete_candidate_publication(&self.candidate_clean)
            .map_err(RecordAppendDenial::from_residency)?;
        Ok(CandidateFrameWriteCompletion::canonical(
            bytes,
            coordinate,
            payload_digest,
            settlement,
        ))
    }
}

impl CandidateFrameCounterCells {
    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::record_serving) fn reject_next_publication(&self) {
        self.reject_next_publication.store(true, Ordering::Release);
    }

    #[cfg(feature = "certification-test-authority")]
    fn take_reject_next_publication(&self) -> bool {
        self.reject_next_publication.swap(false, Ordering::AcqRel)
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub(in crate::physical_runtime::record_serving) fn submissions(&self) -> u64 {
        self.submissions.load(Ordering::Acquire)
    }
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub(in crate::physical_runtime::record_serving) fn declared_frames(&self) -> u64 {
        self.declared_frames.load(Ordering::Acquire)
    }
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub(in crate::physical_runtime::record_serving) fn declared_bytes(&self) -> u64 {
        self.declared_bytes.load(Ordering::Acquire)
    }
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub(in crate::physical_runtime::record_serving) fn retained_frames(&self) -> u64 {
        self.retained_frames.load(Ordering::Acquire)
    }
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub(in crate::physical_runtime::record_serving) fn retained_bytes(&self) -> u64 {
        self.retained_bytes.load(Ordering::Acquire)
    }
}
