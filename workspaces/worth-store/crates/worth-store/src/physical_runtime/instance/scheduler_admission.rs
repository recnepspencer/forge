use worth_store_io_scheduler::{
    IoSchedulerBackendCapabilityAdmission, IoSchedulerBackendCapabilityDenial,
    IoSchedulerBackendCapabilityRequirement,
};
use worth_store_physical_backend::QualifiedFilesystemMedia;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) enum RecordSchedulerReservationDenial {
    Admission(
        worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundAdmissionDenial,
    ),
}

/// Store-owned admission route from qualified media evidence into scheduler
/// capability. It owns no media and cannot execute a physical effect.
#[derive(Clone)]
pub(in crate::physical_runtime) struct PhysicalSchedulerAdmissionOwner {
    buffered_file: IoSchedulerBackendCapabilityAdmission,
    foreground:
        worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundCapacity,
}

impl PhysicalSchedulerAdmissionOwner {
    pub(super) fn new(
        media: &QualifiedFilesystemMedia,
        capacity: crate::physical_runtime::PhysicalWorkCapacity,
    ) -> Result<Self, IoSchedulerBackendCapabilityDenial> {
        let owner = Self {
            buffered_file: admit(media, IoSchedulerBackendCapabilityRequirement::BufferedFile)?,
            foreground: worth_store_io_scheduler::foreground_reservation::
                PhysicalInstanceForegroundCapacity::new(foreground_capacity(capacity))
                .expect("an admitted physical-work profile has nonzero scheduler capacity"),
        };
        Ok(owner)
    }

    pub(in crate::physical_runtime) fn admit(
        &self,
        media: &QualifiedFilesystemMedia,
        requirement: IoSchedulerBackendCapabilityRequirement,
    ) -> Result<IoSchedulerBackendCapabilityAdmission, IoSchedulerBackendCapabilityDenial> {
        admit(media, requirement)
    }

    pub(in crate::physical_runtime) fn record_read(
        &self,
        security: &worth_store_io_scheduler::IoSchedulerSecurityScopeAdmission,
        bytes: u64,
    ) -> Result<
        (
            worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundReservation,
            IoSchedulerBackendCapabilityAdmission,
        ),
        RecordSchedulerReservationDenial,
    > {
        let lane = worth_store_io_scheduler::foreground_reservation::
            ForegroundLaneDeclaration::buffered_file_internal_foreground_read()
            .expect("buffered record reads are a Store-owned lane")
            .with_latency_envelope(
                worth_store_io_scheduler::foreground_reservation::ForegroundLatencyEnvelope::
                    bounded_interference("physical-record-read", 8),
            )
            .with_budget(read_budget(bytes));
        self.reserve_record_lane(lane, security)
            .map_err(RecordSchedulerReservationDenial::Admission)
    }

    pub(in crate::physical_runtime) fn record_metadata(
        &self,
        security: &worth_store_io_scheduler::IoSchedulerSecurityScopeAdmission,
    ) -> Result<
        (
            worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundReservation,
            IoSchedulerBackendCapabilityAdmission,
        ),
        RecordSchedulerReservationDenial,
    > {
        let lane = worth_store_io_scheduler::foreground_reservation::ForegroundLaneDeclaration::
            artifact_metadata_read()
            .with_latency_envelope(
                worth_store_io_scheduler::foreground_reservation::ForegroundLatencyEnvelope::
                    bounded_interference("physical-record-metadata", 8),
            )
            .with_budget(metadata_budget());
        self.reserve_record_lane(lane, security)
            .map_err(RecordSchedulerReservationDenial::Admission)
    }

    pub(in crate::physical_runtime) fn record_write(
        &self,
        security: &worth_store_io_scheduler::IoSchedulerSecurityScopeAdmission,
        bytes: u64,
        synchronization: bool,
        publication: bool,
    ) -> Result<
        (
            worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundReservation,
            IoSchedulerBackendCapabilityAdmission,
        ),
        RecordSchedulerReservationDenial,
    > {
        let lane = worth_store_io_scheduler::foreground_reservation::ForegroundLaneDeclaration::
            ordinary_page_write()
            .with_latency_envelope(
                worth_store_io_scheduler::foreground_reservation::ForegroundLatencyEnvelope::
                    bounded_interference("physical-record-publication", 8),
            )
            .with_budget(write_budget(bytes, synchronization, publication));
        self.reserve_record_lane(lane, security)
            .map_err(RecordSchedulerReservationDenial::Admission)
    }

    pub(in crate::physical_runtime) fn record_publication_effect(
        &self,
        security: &worth_store_io_scheduler::IoSchedulerSecurityScopeAdmission,
    ) -> Result<
        (
            worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundReservation,
            IoSchedulerBackendCapabilityAdmission,
        ),
        RecordSchedulerReservationDenial,
    > {
        let lane = worth_store_io_scheduler::foreground_reservation::ForegroundLaneDeclaration::
            ordinary_page_write()
            .with_latency_envelope(
                worth_store_io_scheduler::foreground_reservation::ForegroundLatencyEnvelope::
                    bounded_interference("physical-record-publication-effect", 8),
            )
            .with_budget(publication_effect_budget());
        self.reserve_record_lane(lane, security)
            .map_err(RecordSchedulerReservationDenial::Admission)
    }

    pub(in crate::physical_runtime) fn reserve_record_lane(
        &self,
        lane: worth_store_io_scheduler::foreground_reservation::ForegroundLaneDeclaration,
        security: &worth_store_io_scheduler::IoSchedulerSecurityScopeAdmission,
    ) -> Result<
        (
            worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundReservation,
            IoSchedulerBackendCapabilityAdmission,
        ),
        worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundAdmissionDenial,
    > {
        let reservation = self
            .foreground
            .reserve(lane, &self.buffered_file, security)?;
        Ok((reservation, self.buffered_file))
    }

    pub(in crate::physical_runtime) fn capacity_snapshot(
        &self,
    ) -> worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundCapacitySnapshot
    {
        self.foreground.snapshot()
    }
}

fn admit(
    media: &QualifiedFilesystemMedia,
    requirement: IoSchedulerBackendCapabilityRequirement,
) -> Result<IoSchedulerBackendCapabilityAdmission, IoSchedulerBackendCapabilityDenial> {
    let claim = media
        .scheduler_capability_claim(
            requirement.capability_kind(),
            requirement.required_evidence(),
        )
        .map_err(IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied)?;
    worth_store_io_scheduler::admit_backend_capability_for_scheduler_qualified_claim(
        claim,
        requirement,
    )
}

fn read_budget(
    bytes: u64,
) -> worth_store_io_scheduler::foreground_reservation::ForegroundResourceBudget {
    use worth_store_io_scheduler::{
        BandwidthToken, CacheResidencyHint, QueueSlot, ReadAheadWindow, WorkerPermit,
    };
    worth_store_io_scheduler::foreground_reservation::ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).expect("one queue slot is nonzero"))
        .with_bandwidth(BandwidthToken::bytes(bytes).expect("record coordinates are nonempty"))
        .with_read_ahead(ReadAheadWindow::pages(1).expect("one read-ahead page is nonzero"))
        .with_worker_permits(WorkerPermit::new(1).expect("one worker permit is nonzero"))
        .with_cache_residency(CacheResidencyHint::frames(1).expect("one frame hint is nonzero"))
}

fn metadata_budget() -> worth_store_io_scheduler::foreground_reservation::ForegroundResourceBudget {
    use worth_store_io_scheduler::{QueueSlot, WorkerPermit};
    worth_store_io_scheduler::foreground_reservation::ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).expect("one queue slot is nonzero"))
        .with_worker_permits(WorkerPermit::new(1).expect("one worker permit is nonzero"))
}

fn write_budget(
    bytes: u64,
    synchronization: bool,
    publication: bool,
) -> worth_store_io_scheduler::foreground_reservation::ForegroundResourceBudget {
    use worth_store_io_scheduler::{
        BandwidthToken, DirtyPageBudget, FlushPermit, QueueSlot, SyncDebt, WorkerPermit,
        WriteBackWindow,
    };
    let budget = worth_store_io_scheduler::foreground_reservation::ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).expect("one queue slot is nonzero"))
        .with_bandwidth(BandwidthToken::bytes(bytes).expect("record coordinates are nonempty"))
        .with_write_back(WriteBackWindow::pages(1).expect("one writeback page is nonzero"))
        .with_dirty_pages(DirtyPageBudget::pages(1).expect("one dirty page is nonzero"))
        .with_worker_permits(WorkerPermit::new(1).expect("one worker permit is nonzero"));
    let budget = if synchronization {
        budget.with_flush_permits(FlushPermit::new(1).expect("one flush permit is nonzero"))
    } else {
        budget
    };
    if publication {
        budget.with_sync_debt(SyncDebt::units(1).expect("one sync-debt unit is nonzero"))
    } else {
        budget
    }
}

fn publication_effect_budget(
) -> worth_store_io_scheduler::foreground_reservation::ForegroundResourceBudget {
    // Publication effects consume the ordinary write lane even when the exact
    // backend operation is a barrier or namespace mutation rather than a byte
    // range. One token is the scheduler's minimum bounded transfer unit; the
    // flush and publication flags carry the additional durability resources.
    write_budget(1, true, true)
}

fn foreground_capacity(
    capacity: crate::physical_runtime::PhysicalWorkCapacity,
) -> worth_store_io_scheduler::foreground_reservation::ForegroundResourceBudget {
    use worth_store_io_scheduler::{
        BandwidthToken, CacheResidencyHint, DirtyPageBudget, FlushPermit, QueueSlot,
        ReadAheadWindow, ReclaimPermit, SyncDebt, WorkerPermit, WriteBackWindow,
    };
    let commands = u64::try_from(capacity.commands()).expect("usize fits the scheduler counter");
    let bytes =
        u64::try_from(capacity.total_semantic_bytes()).expect("usize fits the scheduler counter");
    worth_store_io_scheduler::foreground_reservation::ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(commands).expect("work capacity is nonzero"))
        .with_bandwidth(BandwidthToken::bytes(bytes).expect("semantic capacity is nonzero"))
        .with_flush_permits(FlushPermit::new(commands).expect("work capacity is nonzero"))
        .with_sync_debt(SyncDebt::units(commands).expect("work capacity is nonzero"))
        .with_read_ahead(ReadAheadWindow::pages(commands).expect("work capacity is nonzero"))
        .with_write_back(WriteBackWindow::pages(commands).expect("work capacity is nonzero"))
        .with_dirty_pages(DirtyPageBudget::pages(commands).expect("work capacity is nonzero"))
        .with_worker_permits(WorkerPermit::new(commands).expect("work capacity is nonzero"))
        .with_cache_residency(
            CacheResidencyHint::frames(commands).expect("work capacity is nonzero"),
        )
        .with_reclaim_permits(ReclaimPermit::new(commands).expect("work capacity is nonzero"))
}
