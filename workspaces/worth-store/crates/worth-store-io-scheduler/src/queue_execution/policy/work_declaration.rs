use worth_store_buffer_pool::{
    BufferPoolReadQueueExecutionDeclaration, BufferPoolWritebackQueueExecutionDeclaration,
};

use crate::foreground_reservation::{ForegroundIoLaneKind, ForegroundReservationReceipt};
use crate::{
    BackgroundIdleCapacityLease, BackgroundIoPressureClass, BackgroundResourceBudget,
    IoSchedulerBackendCapabilityRequirement, SecureIoPreservationReceipt,
};

use super::QueueGroupingBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueDurabilityClass {
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
enum BufferPoolQueueExecutionEvidence {
    Read(BufferPoolReadQueueExecutionDeclaration),
    Writeback(BufferPoolWritebackQueueExecutionDeclaration),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueueWorkDeclaration {
    class: QueueWorkClass,
    backend_requirement: IoSchedulerBackendCapabilityRequirement,
    security_scope_identity: worth_store_security::StoreSecurityScopeIdentity,
    durability_class: QueueDurabilityClass,
    requested_budget: BackgroundResourceBudget,
    foreground_reservation: Option<ForegroundReservationReceipt>,
    grouping_basis: Option<QueueGroupingBasis>,
    secure_io: Option<SecureIoPreservationReceipt>,
    buffer_pool: Option<BufferPoolQueueExecutionEvidence>,
}

impl QueueWorkDeclaration {
    pub const fn foreground(
        reservation: ForegroundReservationReceipt,
        durability_class: QueueDurabilityClass,
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
            buffer_pool: None,
        }
    }

    pub const fn foreground_wal_commit(
        reservation: ForegroundReservationReceipt,
        requested_budget: BackgroundResourceBudget,
    ) -> Self {
        Self::foreground(
            reservation,
            QueueDurabilityClass::WalCommit,
            requested_budget,
        )
    }

    pub(super) const fn foreground_buffer_pool_read(
        reservation: ForegroundReservationReceipt,
        requested_budget: BackgroundResourceBudget,
        declaration: BufferPoolReadQueueExecutionDeclaration,
    ) -> Self {
        let mut work = Self::foreground(
            reservation,
            QueueDurabilityClass::ReadOnly,
            requested_budget,
        );
        work.buffer_pool = Some(BufferPoolQueueExecutionEvidence::Read(declaration));
        work
    }

    pub(super) const fn foreground_buffer_pool_writeback(
        reservation: ForegroundReservationReceipt,
        durability: QueueDurabilityClass,
        requested_budget: BackgroundResourceBudget,
        declaration: BufferPoolWritebackQueueExecutionDeclaration,
    ) -> Self {
        let mut work = Self::foreground(reservation, durability, requested_budget);
        work.buffer_pool = Some(BufferPoolQueueExecutionEvidence::Writeback(declaration));
        work
    }

    pub const fn background(lease: BackgroundIdleCapacityLease) -> Self {
        Self {
            class: QueueWorkClass::Background(lease.class()),
            backend_requirement: lease.basis().backend_requirement(),
            security_scope_identity: lease.basis().security_scope_identity(),
            durability_class: super::background_lease::durability_for_background(lease.class()),
            requested_budget: lease.admitted_budget(),
            foreground_reservation: None,
            grouping_basis: None,
            secure_io: lease.secure_io(),
            buffer_pool: None,
        }
    }

    pub(crate) fn with_grouping_basis(mut self, grouping_basis: QueueGroupingBasis) -> Self {
        self.grouping_basis = Some(grouping_basis);
        self
    }

    pub const fn with_secure_io_scope(mut self, secure_io: SecureIoPreservationReceipt) -> Self {
        self.secure_io = Some(secure_io);
        self
    }

    pub const fn class(&self) -> QueueWorkClass {
        self.class
    }

    pub const fn backend_requirement(&self) -> IoSchedulerBackendCapabilityRequirement {
        self.backend_requirement
    }

    pub const fn security_scope_identity(
        &self,
    ) -> worth_store_security::StoreSecurityScopeIdentity {
        self.security_scope_identity
    }

    pub const fn durability_class(&self) -> QueueDurabilityClass {
        self.durability_class
    }

    pub const fn requested_budget(&self) -> BackgroundResourceBudget {
        self.requested_budget
    }

    pub const fn foreground_reservation(&self) -> Option<ForegroundReservationReceipt> {
        self.foreground_reservation
    }

    pub fn grouping_basis(&self) -> Option<&QueueGroupingBasis> {
        self.grouping_basis.as_ref()
    }

    pub const fn secure_io(&self) -> Option<SecureIoPreservationReceipt> {
        self.secure_io
    }

    pub const fn buffer_pool_read_declaration(
        &self,
    ) -> Option<BufferPoolReadQueueExecutionDeclaration> {
        match self.buffer_pool {
            Some(BufferPoolQueueExecutionEvidence::Read(declaration)) => Some(declaration),
            Some(BufferPoolQueueExecutionEvidence::Writeback(_)) | None => None,
        }
    }

    pub const fn buffer_pool_writeback_declaration(
        &self,
    ) -> Option<BufferPoolWritebackQueueExecutionDeclaration> {
        match self.buffer_pool {
            Some(BufferPoolQueueExecutionEvidence::Writeback(declaration)) => Some(declaration),
            Some(BufferPoolQueueExecutionEvidence::Read(_)) | None => None,
        }
    }
}
