use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use worth_store_buffer_pool::{
    DirtyPhysicalFrame, PhysicalCandidateBatchReservation, PhysicalFrameKey, PhysicalResidencyPool,
};
use worth_store_physical_format::RecordFrameCoordinate;

use super::super::RecordAppendDenial;
use super::candidate_frame_residency::{
    CandidateFrame, CandidateFrameCoordinate, CandidateFramePhysicalWrite,
    CandidateFramePublicationPort, CandidateFrameResidencySession, CandidateFrameRole,
    CandidateFrameSet, CandidateFrameWriteCompletion, ResidentCandidateFrame,
};

#[derive(Default)]
pub(in crate::physical_runtime::record_serving) struct CandidateFrameCounterCells {
    submissions: AtomicU64,
    declared_frames: AtomicU64,
    declared_bytes: AtomicU64,
    retained_frames: AtomicU64,
    retained_bytes: AtomicU64,
}

pub(in crate::physical_runtime::record_serving) struct BoundedCandidateFramePublisher {
    pool: PhysicalResidencyPool,
    counters: Arc<CandidateFrameCounterCells>,
}

impl BoundedCandidateFramePublisher {
    pub(in crate::physical_runtime::record_serving) fn new(
        pool: PhysicalResidencyPool,
        counters: Arc<CandidateFrameCounterCells>,
    ) -> Self {
        Self { pool, counters }
    }
}

impl CandidateFramePublicationPort for BoundedCandidateFramePublisher {
    fn begin(
        &self,
        candidate: &CandidateFrameSet,
    ) -> Result<Box<dyn CandidateFrameResidencySession>, RecordAppendDenial> {
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
                RecordAppendDenial::ResidencyUnavailable(
                    worth_store_buffer_pool::PhysicalResidencyDenial::AllocationFailed,
                )
            })?;
        for declaration in candidate.declarations() {
            let coordinate = declaration.coordinate();
            let physical_coordinate = RecordFrameCoordinate::new(
                coordinate.artifact(),
                coordinate.offset(),
                declaration.length(),
            )
            .ok_or(RecordAppendDenial::ResidencyUnavailable(
                worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
            ))?;
            keys.push(PhysicalFrameKey::new(
                self.pool.store_identity(),
                physical_coordinate,
            ));
        }
        let reservations = self
            .pool
            .reserve_candidate_frames(&keys)
            .map_err(RecordAppendDenial::ResidencyUnavailable)?;
        Ok(Box::new(BoundedCandidateFrameSession {
            pool: self.pool.clone(),
            counters: Arc::clone(&self.counters),
            reservations,
        }))
    }
}

struct BoundedCandidateFrameSession {
    pool: PhysicalResidencyPool,
    counters: Arc<CandidateFrameCounterCells>,
    reservations: PhysicalCandidateBatchReservation,
}

impl CandidateFrameResidencySession for BoundedCandidateFrameSession {
    fn retain(
        &mut self,
        frame: CandidateFrame,
    ) -> Result<Box<dyn ResidentCandidateFrame>, RecordAppendDenial> {
        let role = frame.role();
        let coordinate = frame.coordinate();
        let bytes = frame.into_bytes();
        let length = u32::try_from(bytes.len()).map_err(|_| {
            RecordAppendDenial::ResidencyUnavailable(
                worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
            )
        })?;
        let physical_coordinate =
            RecordFrameCoordinate::new(coordinate.artifact(), coordinate.offset(), length).ok_or(
                RecordAppendDenial::ResidencyUnavailable(
                    worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
                ),
            )?;
        let key = PhysicalFrameKey::new(self.pool.store_identity(), physical_coordinate);
        let resident = self
            .reservations
            .reserve_next(key)
            .map_err(RecordAppendDenial::ResidencyUnavailable)?
            .admit(bytes)
            .map_err(RecordAppendDenial::ResidencyUnavailable)?;
        self.counters.retained_frames.fetch_add(1, Ordering::AcqRel);
        self.counters
            .retained_bytes
            .fetch_add(u64::from(length), Ordering::AcqRel);
        Ok(Box::new(BoundedResidentCandidateFrame {
            role,
            coordinate,
            resident,
        }))
    }

    fn prepare_catalog_cutover(
        &mut self,
        target: CandidateFrameCoordinate,
        length: u32,
    ) -> Result<(), RecordAppendDenial> {
        let target = RecordFrameCoordinate::new(target.artifact(), target.offset(), length).ok_or(
            RecordAppendDenial::ResidencyUnavailable(
                worth_store_buffer_pool::PhysicalResidencyDenial::FrameLengthMismatch,
            ),
        )?;
        self.pool
            .invalidate_clean(PhysicalFrameKey::new(self.pool.store_identity(), target))
            .map_err(RecordAppendDenial::ResidencyUnavailable)
    }
}

struct BoundedResidentCandidateFrame {
    role: CandidateFrameRole,
    coordinate: CandidateFrameCoordinate,
    resident: DirtyPhysicalFrame,
}

impl ResidentCandidateFrame for BoundedResidentCandidateFrame {
    fn role(&self) -> CandidateFrameRole {
        self.role
    }
    fn coordinate(&self) -> CandidateFrameCoordinate {
        self.coordinate
    }
    fn bytes(&self) -> &[u8] {
        self.resident.bytes()
    }

    fn publish_clean(
        self: Box<Self>,
        physical: &CandidateFramePhysicalWrite,
    ) -> Result<CandidateFrameWriteCompletion, RecordAppendDenial> {
        let receipt = physical
            .receipt()
            .ok_or(RecordAppendDenial::ResidencyUnavailable(
                worth_store_buffer_pool::PhysicalResidencyDenial::WriteBackReceiptMismatch,
            ))?;
        let bytes = self.resident.bytes().len() as u64;
        self.resident
            .publish_clean(receipt)
            .map_err(RecordAppendDenial::ResidencyUnavailable)?;
        Ok(CandidateFrameWriteCompletion::retained(bytes))
    }
}

impl CandidateFrameCounterCells {
    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::record_serving) fn submissions(&self) -> u64 {
        self.submissions.load(Ordering::Acquire)
    }
    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::record_serving) fn declared_frames(&self) -> u64 {
        self.declared_frames.load(Ordering::Acquire)
    }
    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::record_serving) fn declared_bytes(&self) -> u64 {
        self.declared_bytes.load(Ordering::Acquire)
    }
    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::record_serving) fn retained_frames(&self) -> u64 {
        self.retained_frames.load(Ordering::Acquire)
    }
    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::record_serving) fn retained_bytes(&self) -> u64 {
        self.retained_bytes.load(Ordering::Acquire)
    }
}
