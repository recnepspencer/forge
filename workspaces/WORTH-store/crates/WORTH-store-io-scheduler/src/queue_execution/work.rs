use worth_store_buffer_pool::BufferPoolQueueExecutionDeclaration;
use worth_store_contracts::{S6QueueProducerKind, S6QueueProducerResourceShape};
use worth_store_security::{StoreAuthenticityRequirement, StoreKeyScope, StoreTenantScope};
use worth_store_wal::WalQueueExecutionDeclaration;

use crate::foreground_reservation::{ForegroundIoLaneKind, ForegroundReservationReceipt};
use crate::{
    BackgroundIdleCapacityLease, BackgroundIoPressureClass, BackgroundResourceBudget,
    BandwidthToken, CacheResidencyHint, DirtyPageBudget, FlushPermit,
    IoSchedulerBackendCapabilityRequirement, QueueSlot, ReadAheadWindow, ReclaimPermit,
    SecureIoPreservationReceipt, SyncDebt, WorkerPermit, WriteBackWindow,
};

use super::{
    QueueExecutionAdmissionDenial, QueueGroupingBasis, QueueRecoveryOrdering, QueueWritebackPolicy,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6QueueDurabilityClass {
    ReadOnly,
    BufferedWrite,
    WalCommit,
    PlatformDurable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueWorkClass {
    Foreground(ForegroundIoLaneKind),
    Background(BackgroundIoPressureClass),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueWorkDeclaration {
    class: QueueWorkClass,
    backend_requirement: IoSchedulerBackendCapabilityRequirement,
    security_scope_identity: worth_store_security::StoreSecurityScopeIdentity,
    durability_class: S6QueueDurabilityClass,
    requested_budget: BackgroundResourceBudget,
    foreground_reservation: Option<ForegroundReservationReceipt>,
    grouping_basis: Option<QueueGroupingBasis>,
    secure_io: Option<SecureIoPreservationReceipt>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct QueueProducerExecutionDeclaration {
    kind: S6QueueProducerKind,
    resource_shape: S6QueueProducerResourceShape,
    flush_epoch: u64,
    tenant_scope: StoreTenantScope,
    key_scope: StoreKeyScope,
    authenticity_requirement: StoreAuthenticityRequirement,
}

impl QueueWorkDeclaration {
    pub const fn foreground(
        reservation: ForegroundReservationReceipt,
        durability_class: S6QueueDurabilityClass,
        requested_budget: BackgroundResourceBudget,
    ) -> Self {
        Self {
            class: QueueWorkClass::Foreground(reservation.lane()),
            backend_requirement: reservation.backend_requirement(),
            security_scope_identity: reservation.security_scope_identity(),
            durability_class,
            requested_budget,
            foreground_reservation: Some(reservation),
            grouping_basis: None,
            secure_io: None,
        }
    }

    pub const fn foreground_wal_commit(
        reservation: ForegroundReservationReceipt,
        requested_budget: BackgroundResourceBudget,
    ) -> Self {
        Self::foreground(
            reservation,
            S6QueueDurabilityClass::WalCommit,
            requested_budget,
        )
    }

    pub const fn background(lease: BackgroundIdleCapacityLease) -> Self {
        Self {
            class: QueueWorkClass::Background(lease.class()),
            backend_requirement: lease.basis().backend_requirement(),
            security_scope_identity: lease.basis().security_scope_identity(),
            durability_class: durability_for_background(lease.class()),
            requested_budget: lease.admitted_budget(),
            foreground_reservation: None,
            grouping_basis: None,
            secure_io: lease.secure_io(),
        }
    }

    pub(crate) const fn with_grouping_basis(mut self, grouping_basis: QueueGroupingBasis) -> Self {
        self.grouping_basis = Some(grouping_basis);
        self
    }

    pub const fn with_secure_io_scope(mut self, secure_io: SecureIoPreservationReceipt) -> Self {
        self.secure_io = Some(secure_io);
        self
    }

    pub const fn class(self) -> QueueWorkClass {
        self.class
    }

    pub const fn backend_requirement(self) -> IoSchedulerBackendCapabilityRequirement {
        self.backend_requirement
    }

    pub const fn security_scope_identity(self) -> worth_store_security::StoreSecurityScopeIdentity {
        self.security_scope_identity
    }

    pub const fn durability_class(self) -> S6QueueDurabilityClass {
        self.durability_class
    }

    pub const fn requested_budget(self) -> BackgroundResourceBudget {
        self.requested_budget
    }

    pub const fn foreground_reservation(self) -> Option<ForegroundReservationReceipt> {
        self.foreground_reservation
    }

    pub const fn grouping_basis(self) -> Option<QueueGroupingBasis> {
        self.grouping_basis
    }

    pub const fn secure_io(self) -> Option<SecureIoPreservationReceipt> {
        self.secure_io
    }
}

impl QueueProducerExecutionDeclaration {
    const fn new(
        kind: S6QueueProducerKind,
        resource_shape: S6QueueProducerResourceShape,
        flush_epoch: u64,
        tenant_scope: StoreTenantScope,
        key_scope: StoreKeyScope,
        authenticity_requirement: StoreAuthenticityRequirement,
    ) -> Self {
        Self {
            kind,
            resource_shape,
            flush_epoch,
            tenant_scope,
            key_scope,
            authenticity_requirement,
        }
    }

    pub fn lower_foreground(
        self,
        reservation: ForegroundReservationReceipt,
    ) -> Result<QueueWorkDeclaration, QueueExecutionAdmissionDenial> {
        let durability = durability_for_producer(self.kind);
        let work = QueueWorkDeclaration::foreground(
            reservation.execution_ready(),
            durability,
            budget_from_shape(self.resource_shape)?,
        );
        Ok(work.with_grouping_basis(QueueGroupingBasis::new(
            reservation.security_scope_identity(),
            self.tenant_scope,
            self.key_scope,
            self.authenticity_requirement,
            durability,
            self.flush_epoch,
            QueueWorkClass::Foreground(reservation.lane()),
            recovery_ordering_for_producer(self.kind),
            writeback_policy_for_producer(self.kind),
        )))
    }
}

pub fn lower_wal_queue_declaration(
    declaration: WalQueueExecutionDeclaration,
    reservation: ForegroundReservationReceipt,
) -> Result<QueueWorkDeclaration, QueueExecutionAdmissionDenial> {
    let grouping_scope = declaration.grouping_scope();
    QueueProducerExecutionDeclaration::new(
        declaration.producer_kind(),
        declaration.resource_shape(),
        declaration.flush_epoch(),
        grouping_scope.tenant_scope(),
        grouping_scope.key_scope(),
        grouping_scope.authenticity_requirement(),
    )
    .lower_foreground(reservation)
}

pub fn lower_buffer_pool_queue_declaration(
    declaration: BufferPoolQueueExecutionDeclaration,
    reservation: ForegroundReservationReceipt,
) -> Result<QueueWorkDeclaration, QueueExecutionAdmissionDenial> {
    let security_identity = reservation.security_scope_identity();
    QueueProducerExecutionDeclaration::new(
        declaration.producer_kind(),
        declaration.resource_shape(),
        declaration.flush_epoch(),
        security_identity.tenant_scope(),
        security_identity.key_scope(),
        security_identity.authenticity_requirement(),
    )
    .lower_foreground(reservation)
}

pub fn lower_background_queue_lease(lease: BackgroundIdleCapacityLease) -> QueueWorkDeclaration {
    let basis = lease.basis();
    let security_identity = basis.security_scope_identity();
    let work = QueueWorkDeclaration::background(lease);
    work.with_grouping_basis(QueueGroupingBasis::new(
        security_identity,
        security_identity.tenant_scope(),
        security_identity.key_scope(),
        security_identity.authenticity_requirement(),
        durability_for_background(lease.class()),
        background_flush_epoch(lease.class()),
        QueueWorkClass::Background(lease.class()),
        recovery_ordering_for_background(lease.class()),
        writeback_policy_for_background(lease.class()),
    ))
}

const fn durability_for_background(class: BackgroundIoPressureClass) -> S6QueueDurabilityClass {
    match class {
        BackgroundIoPressureClass::CompactionRewrite
        | BackgroundIoPressureClass::CheckpointFlush
        | BackgroundIoPressureClass::IngestPressure
        | BackgroundIoPressureClass::MigrationPressure => S6QueueDurabilityClass::BufferedWrite,
        BackgroundIoPressureClass::ScrubScan
        | BackgroundIoPressureClass::ReplicationPrepRead
        | BackgroundIoPressureClass::BackupPrepRead
        | BackgroundIoPressureClass::RepairScan
        | BackgroundIoPressureClass::VerificationPressure => S6QueueDurabilityClass::ReadOnly,
    }
}

const fn recovery_ordering_for_background(
    class: BackgroundIoPressureClass,
) -> QueueRecoveryOrdering {
    match class {
        BackgroundIoPressureClass::CheckpointFlush => QueueRecoveryOrdering::WalBeforeData,
        BackgroundIoPressureClass::RepairScan | BackgroundIoPressureClass::VerificationPressure => {
            QueueRecoveryOrdering::RecoveryReadOnly
        }
        BackgroundIoPressureClass::CompactionRewrite
        | BackgroundIoPressureClass::ScrubScan
        | BackgroundIoPressureClass::ReplicationPrepRead
        | BackgroundIoPressureClass::IngestPressure
        | BackgroundIoPressureClass::MigrationPressure
        | BackgroundIoPressureClass::BackupPrepRead => QueueRecoveryOrdering::NotRecoveryCritical,
    }
}

const fn writeback_policy_for_background(class: BackgroundIoPressureClass) -> QueueWritebackPolicy {
    match class {
        BackgroundIoPressureClass::CompactionRewrite
        | BackgroundIoPressureClass::CheckpointFlush
        | BackgroundIoPressureClass::IngestPressure
        | BackgroundIoPressureClass::MigrationPressure => {
            QueueWritebackPolicy::DeferredWithinFlushEpoch
        }
        BackgroundIoPressureClass::ScrubScan
        | BackgroundIoPressureClass::ReplicationPrepRead
        | BackgroundIoPressureClass::BackupPrepRead
        | BackgroundIoPressureClass::RepairScan
        | BackgroundIoPressureClass::VerificationPressure => QueueWritebackPolicy::None,
    }
}

const fn background_flush_epoch(class: BackgroundIoPressureClass) -> u64 {
    match class {
        BackgroundIoPressureClass::CheckpointFlush => 1,
        BackgroundIoPressureClass::CompactionRewrite
        | BackgroundIoPressureClass::ScrubScan
        | BackgroundIoPressureClass::ReplicationPrepRead
        | BackgroundIoPressureClass::IngestPressure
        | BackgroundIoPressureClass::MigrationPressure
        | BackgroundIoPressureClass::BackupPrepRead
        | BackgroundIoPressureClass::RepairScan
        | BackgroundIoPressureClass::VerificationPressure => 0,
    }
}

const fn durability_for_producer(kind: S6QueueProducerKind) -> S6QueueDurabilityClass {
    match kind {
        S6QueueProducerKind::WalCommitRecord | S6QueueProducerKind::WalCheckpointRecord => {
            S6QueueDurabilityClass::WalCommit
        }
        S6QueueProducerKind::BufferPoolReadAhead => S6QueueDurabilityClass::ReadOnly,
        S6QueueProducerKind::BufferPoolWriteBack => S6QueueDurabilityClass::BufferedWrite,
    }
}

const fn recovery_ordering_for_producer(kind: S6QueueProducerKind) -> QueueRecoveryOrdering {
    match kind {
        S6QueueProducerKind::WalCommitRecord | S6QueueProducerKind::WalCheckpointRecord => {
            QueueRecoveryOrdering::WalBeforeData
        }
        S6QueueProducerKind::BufferPoolReadAhead | S6QueueProducerKind::BufferPoolWriteBack => {
            QueueRecoveryOrdering::NotRecoveryCritical
        }
    }
}

const fn writeback_policy_for_producer(kind: S6QueueProducerKind) -> QueueWritebackPolicy {
    match kind {
        S6QueueProducerKind::WalCommitRecord | S6QueueProducerKind::WalCheckpointRecord => {
            QueueWritebackPolicy::Immediate
        }
        S6QueueProducerKind::BufferPoolReadAhead => QueueWritebackPolicy::None,
        S6QueueProducerKind::BufferPoolWriteBack => QueueWritebackPolicy::DeferredWithinFlushEpoch,
    }
}

fn budget_from_shape(
    shape: S6QueueProducerResourceShape,
) -> Result<BackgroundResourceBudget, QueueExecutionAdmissionDenial> {
    let mut budget = BackgroundResourceBudget::new();
    if shape.queue_slots() > 0 {
        budget = budget.with_queue_slots(
            QueueSlot::new(shape.queue_slots())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.bandwidth_tokens() > 0 {
        budget = budget.with_bandwidth(
            BandwidthToken::bytes(shape.bandwidth_tokens())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.flush_permits() > 0 {
        budget = budget.with_flush_permits(
            FlushPermit::new(shape.flush_permits())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.sync_debt() > 0 {
        budget = budget.with_sync_debt(
            SyncDebt::units(shape.sync_debt())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.read_ahead_windows() > 0 {
        budget = budget.with_read_ahead(
            ReadAheadWindow::pages(shape.read_ahead_windows())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.write_back_windows() > 0 {
        budget = budget.with_write_back(
            WriteBackWindow::pages(shape.write_back_windows())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.dirty_page_budget() > 0 {
        budget = budget.with_dirty_pages(
            DirtyPageBudget::pages(shape.dirty_page_budget())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.worker_permits() > 0 {
        budget = budget.with_worker_permits(
            WorkerPermit::new(shape.worker_permits())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.cache_residency_hints() > 0 {
        budget = budget.with_cache_residency(
            CacheResidencyHint::frames(shape.cache_residency_hints())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    if shape.reclaim_permits() > 0 {
        budget = budget.with_reclaim_permits(
            ReclaimPermit::new(shape.reclaim_permits())
                .map_err(QueueExecutionAdmissionDenial::ResourceUnit)?,
        );
    }
    Ok(budget)
}
