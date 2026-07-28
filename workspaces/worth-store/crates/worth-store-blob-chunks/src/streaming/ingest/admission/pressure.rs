use worth_store_io_scheduler::{
    admit_background_pacing,
    foreground_reservation::{
        ForegroundIoLaneKind, ForegroundReservationReceipt, ForegroundReservationState,
    },
    BackgroundCapacityAdmission, BackgroundIdleCapacityLeaseRequest, BackgroundIoPressureClass,
    BackgroundPacingAdmissionBasis, BackgroundPacingOutcome,
};

use crate::BlobStreamingIngestDenial;

#[derive(Debug, PartialEq, Eq)]
pub struct BlobStreamingPressureAdmission {
    basis: BackgroundPacingAdmissionBasis,
    outcome: BackgroundPacingOutcome,
}

impl BlobStreamingPressureAdmission {
    pub fn from_io_qos_background_capacity(
        capacity: BackgroundCapacityAdmission,
        foreground_pressure_events: u64,
        late_yield: bool,
    ) -> Result<Self, BlobStreamingIngestDenial> {
        let basis = capacity.basis();
        if basis.class() != BackgroundIoPressureClass::IngestPressure {
            return Err(BlobStreamingIngestDenial::BackgroundPressureClassMismatch {
                actual: basis.class(),
            });
        }
        match basis.foreground_lane() {
            ForegroundIoLaneKind::CommitCriticalWalWrite
            | ForegroundIoLaneKind::OrdinaryPageWrite => {
                let mut request = BackgroundIdleCapacityLeaseRequest::new(capacity)
                    .with_foreground_pressure_events(foreground_pressure_events);
                if late_yield {
                    request = request.with_late_yield();
                }
                Ok(Self {
                    basis,
                    outcome: admit_background_pacing(request),
                })
            }
            lane => Err(BlobStreamingIngestDenial::ForegroundReservationLaneMismatch { lane }),
        }
    }

    pub fn reject_unbound_foreground_reservation(
        foreground: ForegroundReservationReceipt,
    ) -> BlobStreamingIngestDenial {
        if foreground.state() != ForegroundReservationState::ReservationAdmitted {
            BlobStreamingIngestDenial::ForegroundReservationNotAdmitted {
                lane: foreground.lane(),
            }
        } else {
            BlobStreamingIngestDenial::ForegroundReservationLaneMismatch {
                lane: foreground.lane(),
            }
        }
    }

    pub const fn basis(&self) -> BackgroundPacingAdmissionBasis {
        self.basis
    }

    pub fn into_outcome(self) -> BackgroundPacingOutcome {
        self.outcome
    }
}
