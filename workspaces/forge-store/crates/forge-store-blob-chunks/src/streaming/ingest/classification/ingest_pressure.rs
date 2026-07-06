use forge_store_io_scheduler::BackgroundPacingOutcome;

use crate::{BlobStreamingIngestCounterSnapshot, BlobStreamingIngestDenial};

use super::super::admission::pressure::BlobStreamingPressureAdmission;

pub(crate) fn classify_pressure_outcome(
    pressure: BlobStreamingPressureAdmission,
) -> Result<BlobStreamingIngestCounterSnapshot, BlobStreamingIngestDenial> {
    let counters = BlobStreamingIngestCounterSnapshot::start();
    match pressure.outcome() {
        BackgroundPacingOutcome::Yield(yielded) => Ok(counters
            .record_yield()
            .record_scheduler_waits(yielded.counters().foreground_pressure_events())),
        BackgroundPacingOutcome::Throttled(throttled) => Ok(counters
            .record_throttle()
            .record_scheduler_waits(throttled.counters().foreground_pressure_events())),
        BackgroundPacingOutcome::AdmittedWithDebt(admitted) => Ok(counters
            .record_admission()
            .record_scheduler_waits(admitted.counters().foreground_pressure_events())),
        BackgroundPacingOutcome::Deferred(_) => {
            Err(BlobStreamingIngestDenial::BackgroundPressureDeferred)
        }
        BackgroundPacingOutcome::Denied(_) => {
            Err(BlobStreamingIngestDenial::BackgroundPressureDenied)
        }
        BackgroundPacingOutcome::StaleRebindRequired(stale) => {
            Err(BlobStreamingIngestDenial::BackgroundPressureStale { kind: stale.kind() })
        }
        BackgroundPacingOutcome::Violation(_) => {
            Err(BlobStreamingIngestDenial::BackgroundPressureViolation)
        }
    }
}