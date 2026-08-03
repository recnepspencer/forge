use worth_store_budgets::CounterEvidenceStrength;
use worth_store_io_scheduler::{
    BackgroundIdleCapacityLease, BackgroundIoPressureClass, BackgroundPacingCounterSnapshot,
    BackgroundPacingOutcome,
};

use crate::{BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct AdmittedBlobVerificationPressure {
    _lease: BackgroundIdleCapacityLease,
    counters: BlobStreamingReadCounterSnapshot,
}

impl AdmittedBlobVerificationPressure {
    pub(crate) const fn counters(&self) -> BlobStreamingReadCounterSnapshot {
        self.counters
    }
}

pub(crate) fn classify_verification_pressure(
    outcome: BackgroundPacingOutcome,
) -> Result<AdmittedBlobVerificationPressure, BlobStreamingReadDenial> {
    if outcome.class() != BackgroundIoPressureClass::VerificationPressure {
        return Err(BlobStreamingReadDenial::VerificationPressureClassMismatch {
            actual: outcome.class(),
        });
    }
    match outcome {
        BackgroundPacingOutcome::AdmittedWithDebt(admitted) => {
            let counters = admitted.counters();
            seal_verification_pressure(admitted.into_lease(), counters)
        }
        BackgroundPacingOutcome::Throttled(throttled) => {
            let counters = throttled.counters();
            let Some(lease) = throttled.into_lease() else {
                return Err(
                    BlobStreamingReadDenial::VerificationPressureThrottledWithoutAdmittedCapacity {
                        counters: denial_counters(counters),
                    },
                );
            };
            seal_verification_pressure(lease, counters)
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
        BackgroundPacingOutcome::Violation(outcome) => {
            let counters = denial_counters(outcome.counters());
            Err(BlobStreamingReadDenial::VerificationPressureViolation { counters })
        }
    }
}

fn seal_verification_pressure(
    lease: BackgroundIdleCapacityLease,
    counters: BackgroundPacingCounterSnapshot,
) -> Result<AdmittedBlobVerificationPressure, BlobStreamingReadDenial> {
    if lease.class() != BackgroundIoPressureClass::VerificationPressure {
        return Err(BlobStreamingReadDenial::VerificationPressureClassMismatch {
            actual: lease.class(),
        });
    }
    Ok(AdmittedBlobVerificationPressure {
        _lease: lease,
        counters: project_blob_pressure_counters(counters),
    })
}

fn project_blob_pressure_counters(
    counters: BackgroundPacingCounterSnapshot,
) -> BlobStreamingReadCounterSnapshot {
    BlobStreamingReadCounterSnapshot::start(CounterEvidenceStrength::Exact)
        .record_background_pressure(counters)
}

fn denial_counters(counters: BackgroundPacingCounterSnapshot) -> BlobStreamingReadCounterSnapshot {
    project_blob_pressure_counters(counters)
}
