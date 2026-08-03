use worth_store_io_scheduler::BackgroundPacingOutcome;

use crate::{BlobStreamingIngestCounterSnapshot, BlobStreamingIngestDenial};

use super::super::admission::{
    pressure::BlobStreamingPressureAdmission, BlobStreamingIngestExecutionLease,
};

pub(crate) fn classify_pressure_outcome(
    pressure: BlobStreamingPressureAdmission,
) -> Result<
    (
        BlobStreamingIngestExecutionLease,
        BlobStreamingIngestCounterSnapshot,
    ),
    BlobStreamingIngestDenial,
> {
    let counters = BlobStreamingIngestCounterSnapshot::start();
    match pressure.into_outcome() {
        BackgroundPacingOutcome::Yield(yielded) => {
            Err(BlobStreamingIngestDenial::BackgroundPressureYielded {
                counters: counters.record_background_pressure(yielded.counters()),
            })
        }
        BackgroundPacingOutcome::Throttled(throttled) => {
            let counters = counters.record_background_pressure(throttled.counters());
            let Some(lease) = throttled.into_lease() else {
                return Err(
                    BlobStreamingIngestDenial::BackgroundPressureThrottledWithoutAdmittedCapacity {
                        counters,
                    },
                );
            };
            Ok((
                BlobStreamingIngestExecutionLease::from_scheduler_lease(lease)?,
                counters,
            ))
        }
        BackgroundPacingOutcome::AdmittedWithDebt(admitted) => {
            let counters = counters.record_background_pressure(admitted.counters());
            Ok((
                BlobStreamingIngestExecutionLease::from_scheduler_lease(admitted.into_lease())?,
                counters,
            ))
        }
        BackgroundPacingOutcome::Deferred(deferred) => {
            Err(BlobStreamingIngestDenial::BackgroundPressureDeferred {
                counters: counters.record_background_pressure(deferred.counters()),
            })
        }
        BackgroundPacingOutcome::Denied(denied) => {
            Err(BlobStreamingIngestDenial::BackgroundPressureDenied {
                source: denied.denial(),
                counters: counters.record_background_pressure(denied.counters()),
            })
        }
        BackgroundPacingOutcome::Violation(violation) => {
            Err(BlobStreamingIngestDenial::BackgroundPressureViolation {
                counters: counters.record_background_pressure(violation.counters()),
            })
        }
    }
}
