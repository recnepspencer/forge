use forge_store_io_scheduler::foreground_reservation::{
    ForegroundIoLaneKind, ForegroundReservationAdmissionOutcome, ForegroundReservationReceipt,
    ForegroundReservationState,
};
use forge_store_io_scheduler::{
    BackgroundIoPressureClass, BackgroundPacingCounterSnapshot, BackgroundPacingOutcome,
};
use forge_store_physical_isolation::{PhysicalReadExecutionDenial, StablePhysicalReadReceipt};

use crate::{BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobStreamingReadAdmission {
    stable_read: StablePhysicalReadReceipt,
    foreground: ForegroundReservationReceipt,
    pressure_counters: BackgroundPacingCounterSnapshot,
}

impl BlobStreamingReadAdmission {
    pub fn from_stable_physical_read(
        stable_read: StablePhysicalReadReceipt,
        foreground: ForegroundReservationReceipt,
        pressure: BackgroundPacingOutcome,
    ) -> Result<Self, BlobStreamingReadDenial> {
        let pressure_counters = admit_verification_pressure(pressure)?;
        if foreground.state() != ForegroundReservationState::ReservationAdmitted {
            return Err(BlobStreamingReadDenial::ForegroundReservationNotAdmitted {
                lane: foreground.lane(),
                state: foreground.state(),
            });
        }
        match foreground.lane() {
            ForegroundIoLaneKind::PointRead
            | ForegroundIoLaneKind::RangeRead
            | ForegroundIoLaneKind::InternalForegroundRead
            | ForegroundIoLaneKind::CommitCriticalWalWrite
            | ForegroundIoLaneKind::OrdinaryPageWrite => Ok(Self {
                stable_read,
                foreground,
                pressure_counters,
            }),
            lane => Err(BlobStreamingReadDenial::ForegroundReservationLaneMismatch { lane }),
        }
    }

    pub fn from_foreground_outcome(
        stable_read: StablePhysicalReadReceipt,
        outcome: ForegroundReservationAdmissionOutcome,
        pressure: BackgroundPacingOutcome,
    ) -> Result<Self, BlobStreamingReadDenial> {
        match outcome {
            ForegroundReservationAdmissionOutcome::Admitted(foreground) => {
                Self::from_stable_physical_read(stable_read, foreground, pressure)
            }
            ForegroundReservationAdmissionOutcome::Held(held) => {
                Err(BlobStreamingReadDenial::ForegroundReservationNotAdmitted {
                    lane: held.lane(),
                    state: held.state(),
                })
            }
            ForegroundReservationAdmissionOutcome::Denied(denied) => {
                Err(BlobStreamingReadDenial::ForegroundReservationAdmissionDenied(denied.denial()))
            }
            ForegroundReservationAdmissionOutcome::StaleRebindRequired(stale) => {
                Err(BlobStreamingReadDenial::ForegroundReservationNotAdmitted {
                    lane: stale.lane(),
                    state: stale.state(),
                })
            }
        }
    }

    pub fn reject_physical_read_denial(
        denial: PhysicalReadExecutionDenial,
    ) -> BlobStreamingReadDenial {
        BlobStreamingReadDenial::StablePhysicalReadDenied(denial)
    }

    pub(crate) const fn seed_counters(
        self,
        counters: BlobStreamingReadCounterSnapshot,
    ) -> BlobStreamingReadCounterSnapshot {
        let foreground = self.foreground.counters();
        counters
            .record_stable_read(self.stable_read.counters())
            .record_background_pressure(self.pressure_counters)
            .record_foreground_scheduler_waits(
                foreground.stable_read_wait_count() + foreground.stable_read_retry_count(),
            )
    }

    pub(crate) const fn stable_read(self) -> StablePhysicalReadReceipt {
        self.stable_read
    }

    pub const fn foreground(self) -> ForegroundReservationReceipt {
        self.foreground
    }

    pub const fn pressure_counters(self) -> BackgroundPacingCounterSnapshot {
        self.pressure_counters
    }
}

fn admit_verification_pressure(
    outcome: BackgroundPacingOutcome,
) -> Result<BackgroundPacingCounterSnapshot, BlobStreamingReadDenial> {
    if outcome.class() != BackgroundIoPressureClass::VerificationPressure {
        return Err(BlobStreamingReadDenial::VerificationPressureClassMismatch {
            actual: outcome.class(),
        });
    }
    match outcome {
        BackgroundPacingOutcome::AdmittedWithDebt(outcome) => Ok(outcome.counters()),
        BackgroundPacingOutcome::Throttled(outcome) => {
            if outcome.admitted_budget().is_empty() {
                let counters = denial_counters(outcome.counters());
                Err(
                    BlobStreamingReadDenial::VerificationPressureThrottledWithoutAdmittedCapacity {
                        counters,
                    },
                )
            } else {
                Ok(outcome.counters())
            }
        }
        BackgroundPacingOutcome::Yield(outcome) => {
            let counters = denial_counters(outcome.counters());
            Err(BlobStreamingReadDenial::VerificationPressureYielded { counters })
        }
        BackgroundPacingOutcome::Deferred(outcome) => {
            let counters = denial_counters(outcome.counters());
            Err(BlobStreamingReadDenial::VerificationPressureDeferred { counters })
        }
        BackgroundPacingOutcome::Denied(outcome) => {
            let counters = denial_counters(outcome.counters());
            Err(BlobStreamingReadDenial::VerificationPressureDenied {
                denial: outcome.denial(),
                counters,
            })
        }
        BackgroundPacingOutcome::StaleRebindRequired(outcome) => {
            let counters = denial_counters(outcome.counters()).record_pressure_stale_denial();
            Err(
                BlobStreamingReadDenial::VerificationPressureStaleRebindRequired {
                    kind: outcome.kind(),
                    counters,
                },
            )
        }
        BackgroundPacingOutcome::Violation(outcome) => {
            let counters = denial_counters(outcome.counters());
            Err(BlobStreamingReadDenial::VerificationPressureViolation { counters })
        }
    }
}

fn denial_counters(counters: BackgroundPacingCounterSnapshot) -> BlobStreamingReadCounterSnapshot {
    BlobStreamingReadCounterSnapshot::start(forge_store_budgets::CounterEvidenceStrength::Exact)
        .record_background_pressure(counters)
}
