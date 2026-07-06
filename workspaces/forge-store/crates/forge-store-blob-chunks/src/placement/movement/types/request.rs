use forge_store_io_scheduler::foreground_reservation::{
    ForegroundReservationReceipt, ReservationViolatedWithCause,
};

use crate::{AdmittedBlobPlacement, LifecycleReceipt};

use super::{BlobPlacementMovementColdOutcome, BlobPlacementMovementFreshness, BlobPlacementMovementReadHold};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPlacementMovementForegroundReservation {
    Admitted(ForegroundReservationReceipt),
    Violated(ReservationViolatedWithCause),
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobPlacementMovementRequest {
    lifecycle: LifecycleReceipt,
    source: AdmittedBlobPlacement,
    target: AdmittedBlobPlacement,
    read_hold: Option<BlobPlacementMovementReadHold>,
    foreground_reservation: BlobPlacementMovementForegroundReservation,
    cold_outcome: BlobPlacementMovementColdOutcome,
    freshness: BlobPlacementMovementFreshness,
}

impl BlobPlacementMovementRequest {
    pub const fn new(
        lifecycle: LifecycleReceipt,
        source: AdmittedBlobPlacement,
        target: AdmittedBlobPlacement,
        read_hold: BlobPlacementMovementReadHold,
        foreground_reservation: BlobPlacementMovementForegroundReservation,
        cold_outcome: BlobPlacementMovementColdOutcome,
        freshness: BlobPlacementMovementFreshness,
    ) -> Self {
        Self {
            lifecycle,
            source,
            target,
            read_hold: Some(read_hold),
            foreground_reservation,
            cold_outcome,
            freshness,
        }
    }

    pub const fn without_movement_read_hold(
        lifecycle: LifecycleReceipt,
        source: AdmittedBlobPlacement,
        target: AdmittedBlobPlacement,
        foreground_reservation: BlobPlacementMovementForegroundReservation,
        cold_outcome: BlobPlacementMovementColdOutcome,
        freshness: BlobPlacementMovementFreshness,
    ) -> Self {
        Self {
            lifecycle,
            source,
            target,
            read_hold: None,
            foreground_reservation,
            cold_outcome,
            freshness,
        }
    }

    pub(crate) const fn lifecycle(&self) -> &LifecycleReceipt {
        &self.lifecycle
    }

    pub(crate) const fn source(&self) -> &AdmittedBlobPlacement {
        &self.source
    }

    pub(crate) const fn target(&self) -> &AdmittedBlobPlacement {
        &self.target
    }

    pub(crate) const fn read_hold(&self) -> Option<BlobPlacementMovementReadHold> {
        self.read_hold
    }

    pub(crate) const fn foreground_reservation(
        &self,
    ) -> BlobPlacementMovementForegroundReservation {
        self.foreground_reservation
    }

    pub(crate) const fn cold_outcome(&self) -> BlobPlacementMovementColdOutcome {
        self.cold_outcome
    }

    pub(crate) const fn freshness(&self) -> BlobPlacementMovementFreshness {
        self.freshness
    }
}

impl From<Result<ForegroundReservationReceipt, ReservationViolatedWithCause>>
    for BlobPlacementMovementForegroundReservation
{
    fn from(value: Result<ForegroundReservationReceipt, ReservationViolatedWithCause>) -> Self {
        match value {
            Ok(receipt) => Self::Admitted(receipt),
            Err(violation) => Self::Violated(violation),
        }
    }
}

impl From<ForegroundReservationReceipt> for BlobPlacementMovementForegroundReservation {
    fn from(receipt: ForegroundReservationReceipt) -> Self {
        Self::Admitted(receipt)
    }
}