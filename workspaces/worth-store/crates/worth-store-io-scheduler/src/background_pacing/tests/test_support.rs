use worth_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};
use worth_store_security::admitted_store_internal_security_scope_for_io_qos_test;

use super::policy_receipts::{background_policy_receipt, foreground_policy_receipt};

use crate::foreground_reservation::{
    admit_foreground_reservation, admit_foreground_reservation_capacity,
    ForegroundArbitrationDeclaration, ForegroundIoLaneKind, ForegroundLaneDeclaration,
    ForegroundLatencyEnvelope, ForegroundReservationAdmissionRequest,
    ForegroundReservationCapacityAdmissionRequest, ForegroundReservationReceipt,
    ForegroundResourceBudget,
};
use crate::{
    admit_backend_capability_for_scheduler_claim, admit_background_capacity,
    admit_secure_io_scope_for_scheduler, admit_security_scope_for_scheduler,
    BackgroundCapacityAdmissionRequest, BackgroundIdleCapacityLeaseRequest,
    BackgroundIoPressureShape, BackgroundResourceBudget, BandwidthToken, CacheResidencyHint,
    FlushPermit, IoSchedulerBackendCapabilityAdmission, IoSchedulerBackendCapabilityRequirement,
    IoSchedulerSecurityScopeAdmission, QueueSlot, ReadAheadWindow, SecureIoOperation,
    SecureIoPostureRequirement, SecureIoPreservationRequest, WorkerPermit, WriteBackWindow,
};

pub(super) struct World {
    foreground: ForegroundReservationReceipt,
    backend: IoSchedulerBackendCapabilityAdmission,
    security: IoSchedulerSecurityScopeAdmission,
}

impl World {
    pub(super) fn new() -> Self {
        Self::new_for(
            IoSchedulerBackendCapabilityRequirement::DirectIo,
            point_read_lane(),
            ForegroundIoLaneKind::PointRead,
        )
    }

    pub(super) fn commit_wal() -> Self {
        Self::new_for(
            IoSchedulerBackendCapabilityRequirement::Fsync,
            commit_wal_lane(),
            ForegroundIoLaneKind::CommitCriticalWalWrite,
        )
    }

    fn new_for(
        requirement: IoSchedulerBackendCapabilityRequirement,
        lane: ForegroundLaneDeclaration,
        lane_kind: ForegroundIoLaneKind,
    ) -> Self {
        let security = security_scope();
        let backend = backend_admission(requirement);
        let arbitration = ForegroundArbitrationDeclaration::for_lane(lane_kind);
        let capacity = admit_foreground_reservation_capacity(
            ForegroundReservationCapacityAdmissionRequest::new(
                lane,
                crate::foreground_reservation::ForegroundReservationCapacityBasis::new(
                    &backend, &security,
                ),
                arbitration,
                lane.requested_budget(),
                foreground_capacity_budget(),
                foreground_policy_receipt(lane.requested_budget(), lane.requested_budget()),
            ),
        )
        .expect("foreground capacity should admit");
        let foreground = admit_foreground_reservation(ForegroundReservationAdmissionRequest::new(
            lane,
            &backend,
            &security,
            arbitration,
            &capacity,
        ))
        .into_result()
        .expect("foreground reservation should admit");
        Self {
            foreground,
            backend,
            security,
        }
    }

    pub(super) fn request(
        &self,
        pressure: BackgroundIoPressureShape,
    ) -> BackgroundIdleCapacityLeaseRequest {
        self.request_with(
            pressure,
            pressure.requested_budget(),
            pressure.requested_budget(),
            BackgroundResourceBudget::new(),
        )
    }

    pub(super) fn request_with(
        &self,
        pressure: BackgroundIoPressureShape,
        idle_available: BackgroundResourceBudget,
        policy_admitted: BackgroundResourceBudget,
        debt_limit: BackgroundResourceBudget,
    ) -> BackgroundIdleCapacityLeaseRequest {
        let capacity = admit_background_capacity(self.capacity_request_with(
            pressure,
            idle_available,
            policy_admitted,
            debt_limit,
        ))
        .expect("background capacity should admit");
        BackgroundIdleCapacityLeaseRequest::new(capacity)
    }

    pub(super) fn capacity_request_with(
        &self,
        pressure: BackgroundIoPressureShape,
        idle_available: BackgroundResourceBudget,
        policy_admitted: BackgroundResourceBudget,
        debt_limit: BackgroundResourceBudget,
    ) -> BackgroundCapacityAdmissionRequest<'_> {
        let request = BackgroundCapacityAdmissionRequest::new(
            pressure,
            &self.foreground,
            &self.backend,
            background_policy_receipt(pressure.requested_budget(), policy_admitted),
        )
        .with_idle_available(idle_available)
        .with_policy_admitted(policy_admitted)
        .with_debt_limit(debt_limit);
        if !pressure.secure_scope_required()
            && pressure.backend_requirement()
                != IoSchedulerBackendCapabilityRequirement::SecureFrameIo
        {
            return request;
        }
        request.with_secure_io_scope(self.secure_io_for_pressure(pressure))
    }

    pub(super) fn request_with_current(
        &self,
        pressure: BackgroundIoPressureShape,
        idle_available: BackgroundResourceBudget,
        policy_admitted: BackgroundResourceBudget,
        debt_limit: BackgroundResourceBudget,
    ) -> BackgroundIdleCapacityLeaseRequest {
        self.request_with(pressure, idle_available, policy_admitted, debt_limit)
    }

    pub(super) const fn foreground(&self) -> &ForegroundReservationReceipt {
        &self.foreground
    }

    pub(super) const fn backend(&self) -> &IoSchedulerBackendCapabilityAdmission {
        &self.backend
    }

    pub(super) fn capacity_denial(
        &self,
        pressure: BackgroundIoPressureShape,
    ) -> crate::BackgroundPacingDenial {
        admit_background_capacity(BackgroundCapacityAdmissionRequest::new(
            pressure,
            &self.foreground,
            &self.backend,
            background_policy_receipt(read_pressure_budget(), read_pressure_budget()),
        ))
        .expect_err("background capacity should deny")
    }

    pub(super) fn capacity_denial_with_secure_io(
        &self,
        pressure: BackgroundIoPressureShape,
        secure_io: crate::SecureIoPreservationReceipt,
    ) -> crate::BackgroundPacingDenial {
        admit_background_capacity(
            BackgroundCapacityAdmissionRequest::new(
                pressure,
                &self.foreground,
                &self.backend,
                background_policy_receipt(pressure.requested_budget(), pressure.requested_budget()),
            )
            .with_secure_io_scope(secure_io),
        )
        .expect_err("background capacity should deny hostile secure-I/O receipt")
    }

    pub(super) fn secure_io_for_pressure(
        &self,
        pressure: BackgroundIoPressureShape,
    ) -> crate::SecureIoPreservationReceipt {
        let operation = match pressure.class() {
            crate::BackgroundIoPressureClass::RepairScan => SecureIoOperation::RepairScan,
            crate::BackgroundIoPressureClass::VerificationPressure => {
                SecureIoOperation::VerificationPressure
            }
            _ => SecureIoOperation::BackgroundLease,
        };
        self.secure_io_for_operation(operation)
    }

    pub(super) fn secure_io_for_operation(
        &self,
        operation: SecureIoOperation,
    ) -> crate::SecureIoPreservationReceipt {
        secure_io_for(operation, &self.security, &self.backend)
    }

    pub(super) fn secure_io_for_backend_requirement(
        &self,
        operation: SecureIoOperation,
        requirement: IoSchedulerBackendCapabilityRequirement,
    ) -> crate::SecureIoPreservationReceipt {
        let backend = backend_admission(requirement);
        secure_io_for(operation, &self.security, &backend)
    }
}

fn commit_wal_lane() -> ForegroundLaneDeclaration {
    ForegroundLaneDeclaration::commit_critical_wal_write()
        .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
            "background-test-commit-wal",
            2,
        ))
        .with_budget(
            read_foreground_budget()
                .with_flush_permits(crate::FlushPermit::new(1).unwrap())
                .with_sync_debt(crate::SyncDebt::units(1).unwrap()),
        )
}

pub(super) fn read_pressure_budget() -> BackgroundResourceBudget {
    BackgroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(2).unwrap())
        .with_bandwidth(BandwidthToken::bytes(8192).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(2).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
}

pub(super) fn background_budget_with_queue_slots(
    queue_slots: QueueSlot,
) -> BackgroundResourceBudget {
    BackgroundResourceBudget::new().with_queue_slots(queue_slots)
}

pub(super) fn background_budget_with_bandwidth(bytes: u64) -> BackgroundResourceBudget {
    BackgroundResourceBudget::new().with_bandwidth(BandwidthToken::bytes(bytes).unwrap())
}

pub(super) fn background_budget_with_worker_permits(workers: u64) -> BackgroundResourceBudget {
    BackgroundResourceBudget::new().with_worker_permits(WorkerPermit::new(workers).unwrap())
}

pub(super) fn background_budget_with_flush_permits(permits: u64) -> BackgroundResourceBudget {
    BackgroundResourceBudget::new().with_flush_permits(FlushPermit::new(permits).unwrap())
}

pub(super) fn background_budget_with_write_back_pages(pages: u64) -> BackgroundResourceBudget {
    BackgroundResourceBudget::new().with_write_back(WriteBackWindow::pages(pages).unwrap())
}

fn point_read_lane() -> ForegroundLaneDeclaration {
    ForegroundLaneDeclaration::point_read()
        .with_latency_envelope(ForegroundLatencyEnvelope::bounded_interference(
            "background-test-point-read",
            2,
        ))
        .with_budget(read_foreground_budget())
}

fn read_foreground_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(1).unwrap())
}

fn foreground_capacity_budget() -> ForegroundResourceBudget {
    ForegroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(8).unwrap())
        .with_bandwidth(BandwidthToken::bytes(1_048_576).unwrap())
        .with_flush_permits(crate::FlushPermit::new(8).unwrap())
        .with_sync_debt(crate::SyncDebt::units(8).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(8).unwrap())
        .with_worker_permits(WorkerPermit::new(8).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(8).unwrap())
}

fn security_scope() -> IoSchedulerSecurityScopeAdmission {
    let security_scope = admitted_store_internal_security_scope_for_io_qos_test();
    admit_security_scope_for_scheduler(&security_scope)
        .expect("test security scope should admit for scheduler use")
}

fn secure_io_for(
    operation: SecureIoOperation,
    security: &IoSchedulerSecurityScopeAdmission,
    backend: &IoSchedulerBackendCapabilityAdmission,
) -> crate::SecureIoPreservationReceipt {
    admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(operation, security, backend)
            .require_posture(SecureIoPostureRequirement::ScopePreserving),
    )
    .expect("background secure-I/O scope should admit")
}

fn backend_admission(
    requirement: IoSchedulerBackendCapabilityRequirement,
) -> IoSchedulerBackendCapabilityAdmission {
    let request = BackendCapabilityAdmissionRequest::new(
        BackendTargetProfile::PosixFileFsyncDirSync,
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::all_supported(),
        BackendMediaAssumptionSet::platform_file_defaults()
            .with_direct_io_alignment()
            .with_sector_atomicity()
            .with_page_cache_policy()
            .with_mmap_coherence()
            .with_async_ordering()
            .with_secure_frame_io()
            .with_flush_ordering()
            .with_fdatasync_durability(),
        BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
    );
    let witness = PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(request)
        .expect("backend should admit");
    admit_backend_capability_for_scheduler_claim(&witness, requirement)
        .expect("scheduler backend should admit")
}
