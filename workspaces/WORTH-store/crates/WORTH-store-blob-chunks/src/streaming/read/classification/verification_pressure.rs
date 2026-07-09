use worth_store_budgets::CounterEvidenceStrength;
use worth_store_io_scheduler::{
    BackgroundIoPressureClass, BackgroundPacingCapability, BackgroundPacingCounterSnapshot,
    BackgroundPacingOutcome,
};

use crate::{BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial};

pub(crate) fn classify_verification_pressure(
    outcome: BackgroundPacingOutcome,
) -> Result<BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial> {
    if outcome.class() != BackgroundIoPressureClass::VerificationPressure {
        return Err(BlobStreamingReadDenial::VerificationPressureClassMismatch {
            actual: outcome.class(),
        });
    }
    match outcome {
        BackgroundPacingOutcome::AdmittedWithDebt(_) => {
            seal_verification_pressure_counters(outcome)
        }
        BackgroundPacingOutcome::Throttled(outcome) => {
            if outcome.admitted_budget().is_empty() {
                let counters = denial_counters(outcome.counters());
                Err(
                    BlobStreamingReadDenial::VerificationPressureThrottledWithoutAdmittedCapacity {
                        counters,
                    },
                )
            } else {
                seal_verification_pressure_counters(BackgroundPacingOutcome::Throttled(outcome))
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

fn seal_verification_pressure_counters(
    outcome: BackgroundPacingOutcome,
) -> Result<BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial> {
    let capability =
        BackgroundPacingCapability::from_admitted_outcome(outcome).map_err(|denial| {
            BlobStreamingReadDenial::VerificationPressureDenied {
                denial,
                counters: BlobStreamingReadCounterSnapshot::start(CounterEvidenceStrength::Exact),
            }
        })?;
    Ok(project_blob_pressure_counters(capability.counters()))
}

fn project_blob_pressure_counters(
    counters: BackgroundPacingCounterSnapshot,
) -> BlobStreamingReadCounterSnapshot {
    BlobStreamingReadCounterSnapshot::start(CounterEvidenceStrength::Exact)
        .record_background_pressure(counters)
}

fn denial_counters(counters: BackgroundPacingCounterSnapshot) -> BlobStreamingReadCounterSnapshot {
    BlobStreamingReadCounterSnapshot::start(CounterEvidenceStrength::Exact)
        .record_background_pressure(counters)
}
