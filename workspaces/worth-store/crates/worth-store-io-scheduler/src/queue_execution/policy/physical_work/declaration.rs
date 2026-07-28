use worth_store_contracts::QueueProducerResourceShape;

use crate::foreground_reservation::ForegroundReservationReceipt;
use crate::SecureIoPreservationReceipt;

use super::super::{
    QueueDurabilityClass, QueueLocalityIdentity, QueueRecoveryOrdering, QueueWritebackPolicy,
};

#[derive(Debug)]
pub struct PhysicalForegroundWorkDeclaration {
    pub(super) reservation: ForegroundReservationReceipt,
    pub(super) locality: QueueLocalityIdentity,
    pub(super) resources: QueueProducerResourceShape,
    pub(super) durability: QueueDurabilityClass,
    pub(super) flush_epoch: u64,
    pub(super) recovery: QueueRecoveryOrdering,
    pub(super) writeback: QueueWritebackPolicy,
    pub(super) secure_io: Option<SecureIoPreservationReceipt>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PhysicalForegroundOperationPosture {
    Read,
    BufferedWrite,
    DurableWrite,
}

#[derive(Debug)]
struct PhysicalForegroundWorkInputs {
    reservation: ForegroundReservationReceipt,
    locality: QueueLocalityIdentity,
    resources: QueueProducerResourceShape,
    flush_epoch: u64,
}

impl PhysicalForegroundWorkDeclaration {
    pub fn read(
        reservation: ForegroundReservationReceipt,
        locality: QueueLocalityIdentity,
        resources: QueueProducerResourceShape,
        flush_epoch: u64,
    ) -> Self {
        Self::from_inputs(
            PhysicalForegroundWorkInputs::new(reservation, locality, resources, flush_epoch),
            PhysicalForegroundOperationPosture::Read,
        )
    }

    pub fn buffered_write(
        reservation: ForegroundReservationReceipt,
        locality: QueueLocalityIdentity,
        resources: QueueProducerResourceShape,
        flush_epoch: u64,
    ) -> Self {
        Self::from_inputs(
            PhysicalForegroundWorkInputs::new(reservation, locality, resources, flush_epoch),
            PhysicalForegroundOperationPosture::BufferedWrite,
        )
    }

    pub fn durable_write(
        reservation: ForegroundReservationReceipt,
        locality: QueueLocalityIdentity,
        resources: QueueProducerResourceShape,
        flush_epoch: u64,
    ) -> Self {
        Self::from_inputs(
            PhysicalForegroundWorkInputs::new(reservation, locality, resources, flush_epoch),
            PhysicalForegroundOperationPosture::DurableWrite,
        )
    }

    pub const fn with_secure_io_scope(mut self, secure_io: SecureIoPreservationReceipt) -> Self {
        self.secure_io = Some(secure_io);
        self
    }

    fn from_inputs(
        inputs: PhysicalForegroundWorkInputs,
        posture: PhysicalForegroundOperationPosture,
    ) -> Self {
        Self {
            reservation: inputs.reservation,
            locality: inputs.locality,
            resources: inputs.resources,
            durability: posture.durability(),
            flush_epoch: inputs.flush_epoch,
            recovery: QueueRecoveryOrdering::NotRecoveryCritical,
            writeback: posture.writeback(),
            secure_io: None,
        }
    }
}

impl PhysicalForegroundWorkInputs {
    fn new(
        reservation: ForegroundReservationReceipt,
        locality: QueueLocalityIdentity,
        resources: QueueProducerResourceShape,
        flush_epoch: u64,
    ) -> Self {
        Self {
            reservation,
            locality,
            resources,
            flush_epoch,
        }
    }
}

impl PhysicalForegroundOperationPosture {
    const fn durability(self) -> QueueDurabilityClass {
        match self {
            Self::Read => QueueDurabilityClass::ReadOnly,
            Self::BufferedWrite => QueueDurabilityClass::BufferedWrite,
            Self::DurableWrite => QueueDurabilityClass::PlatformDurable,
        }
    }

    const fn writeback(self) -> QueueWritebackPolicy {
        match self {
            Self::Read => QueueWritebackPolicy::None,
            Self::BufferedWrite => QueueWritebackPolicy::DeferredWithinFlushEpoch,
            Self::DurableWrite => QueueWritebackPolicy::Immediate,
        }
    }
}
