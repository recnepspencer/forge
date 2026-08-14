use worth_runtime_bridge::facade::{
    BridgeManagedClockObservationOutcome, BridgeManagedTemporalDenialKind,
};

use super::{ErasedClockObservationOutcome, ErasedClockObservationReceipt};

pub(super) fn map_bridge_clock_outcome(
    outcome: Result<
        BridgeManagedClockObservationOutcome,
        worth_runtime_bridge::facade::BridgeManagedTemporalDenial,
    >,
    mut retain: impl FnMut(
        worth_runtime_bridge::facade::BridgeManagedClockAcceptedObservation,
    ) -> ErasedClockObservationReceipt,
) -> ErasedClockObservationOutcome {
    match outcome {
        Ok(BridgeManagedClockObservationOutcome::Accepted(accepted)) => {
            ErasedClockObservationOutcome::Accepted(retain(accepted))
        }
        Ok(BridgeManagedClockObservationOutcome::Duplicate(duplicate)) => {
            ErasedClockObservationOutcome::Duplicate(retain(duplicate))
        }
        Ok(BridgeManagedClockObservationOutcome::Stale) => ErasedClockObservationOutcome::Stale,
        Ok(BridgeManagedClockObservationOutcome::Reordered) => {
            ErasedClockObservationOutcome::Reordered
        }
        Err(denial) if denial.kind() == BridgeManagedTemporalDenialKind::ClosedClockBinding => {
            ErasedClockObservationOutcome::Closed
        }
        Err(denial) => {
            super::authoritative_clock_progression::runtime_rejection(denial.detail().to_string())
        }
    }
}
