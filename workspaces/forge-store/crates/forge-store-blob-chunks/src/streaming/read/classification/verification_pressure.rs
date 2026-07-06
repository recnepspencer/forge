use forge_store_budgets::CounterEvidenceStrength;
use forge_store_io_scheduler::{
    BackgroundIoPressureClass, BackgroundPacingCounterSnapshot, BackgroundPacingOutcome,
};

use crate::{BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial};

pub(crate) fn classify_verification_pressure(
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
    BlobStreamingReadCounterSnapshot::start(CounterEvidenceStrength::Exact)
        .record_background_pressure(counters)
}