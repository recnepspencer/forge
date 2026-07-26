use std::sync::Arc;

use worth_store_buffer_pool::{
    OperationAllocationGrant, PhysicalOperationAllocationScope, PhysicalResidencyCounters,
    PhysicalResidencyDenial, PhysicalResidencyLimits, PhysicalResidencyPool,
    PhysicalResidencyShutdown, PhysicalWritebackClaim,
};
use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordFrameCoordinate};

pub(in crate::physical_runtime::record_serving) use super::candidate_frame_residency::{
    CandidateFrame, CandidateFrameCoordinate, CandidateFrameDeclaration,
    CandidateFramePublicationPort, CandidateFrameRole, CandidateFrameSet,
    CandidateFrameWriteFailure, StoreCandidateFramePublicationSession,
};
pub(in crate::physical_runtime::record_serving) use super::frame_loading::FrameLoadPort;

use super::candidate_frame_publishers::{
    BoundedCandidateFramePublisher, CandidateFrameCounterCells,
};
use super::frame_loading::BoundedFrameLoader;

#[derive(Clone)]
pub(in crate::physical_runtime) struct RecordFramePorts {
    pool: PhysicalResidencyPool,
    loader: BoundedFrameLoader,
    publisher: BoundedCandidateFramePublisher,
    #[cfg(feature = "certification-test-authority")]
    candidate_counters: Arc<CandidateFrameCounterCells>,
}

impl RecordFramePorts {
    pub(in crate::physical_runtime) fn bounded(
        store: StableStoreIdentity,
        limits: PhysicalResidencyLimits,
    ) -> Result<Self, PhysicalResidencyDenial> {
        let pool = PhysicalResidencyPool::open(store, limits)?;
        let candidate_counters = Arc::new(CandidateFrameCounterCells::default());
        Ok(Self {
            loader: BoundedFrameLoader::new(pool.clone()),
            publisher: BoundedCandidateFramePublisher::new(
                pool.clone(),
                Arc::clone(&candidate_counters),
            ),
            pool,
            #[cfg(feature = "certification-test-authority")]
            candidate_counters,
        })
    }

    pub(in crate::physical_runtime::record_serving) const fn loader(
        &self,
    ) -> &(dyn FrameLoadPort + Send + Sync) {
        &self.loader
    }
    pub(in crate::physical_runtime::record_serving) const fn publisher(
        &self,
    ) -> &(dyn CandidateFramePublicationPort + Send + Sync) {
        &self.publisher
    }

    pub(in crate::physical_runtime) fn begin_operation(
        &self,
        scope: PhysicalOperationAllocationScope,
        bytes: std::num::NonZeroU64,
    ) -> Result<OperationAllocationGrant, PhysicalResidencyDenial> {
        self.pool.begin_operation(scope, bytes)
    }

    pub(in crate::physical_runtime) fn counters(&self) -> PhysicalResidencyCounters {
        self.pool.counters()
    }
    pub(in crate::physical_runtime) fn allocation_events(
        &self,
    ) -> worth_store_buffer_pool::PhysicalResidencyAllocationEventObserver {
        self.pool.allocation_events()
    }
    pub(in crate::physical_runtime) fn close(&self) -> PhysicalResidencyShutdown {
        self.pool.close()
    }

    pub(in crate::physical_runtime::record_serving) fn drain_unpinned_clean_frames(&self) -> u64 {
        self.pool.drain_unpinned_clean_frames()
    }

    pub(in crate::physical_runtime::record_serving) fn claim_writeback(
        &self,
        coordinate: RecordFrameCoordinate,
    ) -> Result<PhysicalWritebackClaim, PhysicalResidencyDenial> {
        self.pool
            .claim_writeback(vec![worth_store_buffer_pool::PhysicalFrameKey::new(
                self.pool.store_identity(),
                coordinate,
            )])
    }

    pub(in crate::physical_runtime::record_serving) fn writeback_declaration(
        &self,
        coordinate: RecordFrameCoordinate,
        grouping: worth_store_buffer_pool::BufferPoolQueueGroupingScope,
        flush_epoch: u64,
        resource_shape: worth_store_contracts::QueueProducerResourceShape,
    ) -> Result<worth_store_buffer_pool::BufferPoolQueueExecutionDeclaration, PhysicalResidencyDenial>
    {
        worth_store_buffer_pool::BufferPoolQueueExecutionDeclaration::write_back(
            &self.pool,
            worth_store_buffer_pool::PhysicalFrameKey::new(self.pool.store_identity(), coordinate),
            grouping,
            flush_epoch,
            resource_shape,
        )
    }

    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::record_serving) fn admit_dirty_for_certification(
        &self,
        coordinate: RecordFrameCoordinate,
        bytes: Vec<u8>,
    ) -> Result<(), PhysicalResidencyDenial> {
        let key =
            worth_store_buffer_pool::PhysicalFrameKey::new(self.pool.store_identity(), coordinate);
        let allocation_bytes =
            worth_store_buffer_pool::PhysicalResidencyPool::candidate_batch_operation_bytes(
                std::num::NonZeroUsize::MIN,
            )
            .expect("one candidate batch has a representable operation demand");
        let allocation = self.pool.begin_operation(
            PhysicalOperationAllocationScope::ForegroundWrite,
            allocation_bytes,
        )?;
        drop(self.pool.admit_dirty(&allocation, key, bytes)?);
        Ok(())
    }

    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::record_serving) fn writeback_declaration_for_certification(
        &self,
        coordinate: RecordFrameCoordinate,
        grouping: worth_store_buffer_pool::BufferPoolQueueGroupingScope,
        flush_epoch: u64,
        resource_shape: worth_store_contracts::QueueProducerResourceShape,
    ) -> Result<worth_store_buffer_pool::BufferPoolQueueExecutionDeclaration, PhysicalResidencyDenial>
    {
        self.writeback_declaration(coordinate, grouping, flush_epoch, resource_shape)
    }

    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::record_serving) fn observer(&self) -> FramePortCounterObserver {
        FramePortCounterObserver {
            pool: self.pool.clone(),
            candidate_counters: Arc::clone(&self.candidate_counters),
        }
    }

    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::record_serving) fn reject_next_candidate_publication(&self) {
        self.candidate_counters.reject_next_publication();
    }
}

#[cfg(feature = "certification-test-authority")]
pub struct FramePortCounterObserver {
    pool: PhysicalResidencyPool,
    candidate_counters: Arc<CandidateFrameCounterCells>,
}

#[cfg(feature = "certification-test-authority")]
impl FramePortCounterObserver {
    pub fn snapshot(&self) -> FramePortCounterSnapshot {
        FramePortCounterSnapshot {
            residency: self.pool.counters(),
            candidate_submissions: self.candidate_counters.submissions(),
            declared_candidate_frames: self.candidate_counters.declared_frames(),
            declared_candidate_bytes: self.candidate_counters.declared_bytes(),
            candidate_frames: self.candidate_counters.retained_frames(),
            candidate_bytes: self.candidate_counters.retained_bytes(),
        }
    }
}

#[cfg(feature = "certification-test-authority")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FramePortCounterSnapshot {
    residency: PhysicalResidencyCounters,
    candidate_submissions: u64,
    declared_candidate_frames: u64,
    declared_candidate_bytes: u64,
    candidate_frames: u64,
    candidate_bytes: u64,
}

#[cfg(feature = "certification-test-authority")]
impl FramePortCounterSnapshot {
    pub const fn loads(self) -> u64 {
        self.residency.source_loads()
    }
    pub const fn wrapper_loads(self) -> u64 {
        0
    }
    pub const fn candidate_submissions(self) -> u64 {
        self.candidate_submissions
    }
    pub const fn declared_candidate_frames(self) -> u64 {
        self.declared_candidate_frames
    }
    pub const fn declared_candidate_bytes(self) -> u64 {
        self.declared_candidate_bytes
    }
    pub const fn candidate_frames(self) -> u64 {
        self.candidate_frames
    }
    pub const fn candidate_bytes(self) -> u64 {
        self.candidate_bytes
    }
    pub const fn wrapper_frames(self) -> u64 {
        0
    }
    pub const fn peak_retained_candidate_frames(self) -> u64 {
        self.residency.peak_candidate_frames() as u64
    }
    pub const fn residency_hits(self) -> u64 {
        self.residency.hits()
    }
    pub const fn residency_faults(self) -> u64 {
        self.residency.faults()
    }
    pub const fn writebacks(self) -> u64 {
        self.residency.writebacks()
    }
    pub const fn candidate_publications(self) -> u64 {
        self.residency.candidate_publications()
    }
    pub const fn evictions(self) -> u64 {
        self.residency.evictions()
    }
}
