use std::collections::BTreeMap;

use worth_signal::facade::{CompletionDenialClass, RawCompletionEnvelope};

use super::PhysicalSignalGraph;
use crate::physical_runtime::PhysicalSignalSettlementOutcome;

impl PhysicalSignalGraph {
    pub(super) fn record_settlement(
        &mut self,
        envelope: RawCompletionEnvelope,
    ) -> PhysicalSignalSettlementOutcome {
        let retained_envelope = envelope.clone();
        let report = self.runtime.admit_resource_completion(envelope);
        let Some(admitted) = report.admitted_completion() else {
            if report
                .denied_completion()
                .is_some_and(|denied| denial_proves_terminal(denied.class()))
            {
                self.release_envelope(&retained_envelope);
                return PhysicalSignalSettlementOutcome::ReconciledFromPhysicalTruth;
            }
            return PhysicalSignalSettlementOutcome::DerivedStateUnavailable;
        };
        let Ok(staged) = self.runtime.stage_admitted_resource_completion(admitted) else {
            return PhysicalSignalSettlementOutcome::DerivedStateUnavailable;
        };
        let outcome = match self
            .runtime
            .commit_staged_resource_completion(staged.staged_effect())
        {
            Ok(_) => PhysicalSignalSettlementOutcome::Committed,
            Err(_) => PhysicalSignalSettlementOutcome::DerivedStateUnavailable,
        };
        if outcome != PhysicalSignalSettlementOutcome::DerivedStateUnavailable {
            self.release_envelope(&retained_envelope);
        }
        outcome
    }

    pub(super) fn record_settlement_batch(
        &mut self,
        envelopes: Vec<RawCompletionEnvelope>,
    ) -> Box<[PhysicalSignalSettlementOutcome]> {
        let report = self
            .runtime
            .admit_resource_completion_batch(envelopes.iter().cloned());
        let (admitted, denied) = report.into_parts();
        let mut outcomes = BTreeMap::new();
        for denied in denied {
            outcomes.insert(
                (
                    denied.request_id(),
                    denied.generation(),
                    denied.branch_epoch(),
                    denied.attempt(),
                ),
                if denial_proves_terminal(denied.class()) {
                    PhysicalSignalSettlementOutcome::ReconciledFromPhysicalTruth
                } else {
                    PhysicalSignalSettlementOutcome::DerivedStateUnavailable
                },
            );
        }
        for completion in admitted {
            let key = (
                completion.handle().request_id(),
                completion.handle().generation(),
                completion.handle().branch_epoch(),
                completion.attempt(),
            );
            let outcome = self
                .runtime
                .stage_admitted_resource_completion(completion)
                .ok()
                .and_then(|staged| {
                    self.runtime
                        .commit_staged_resource_completion(staged.staged_effect())
                        .ok()
                })
                .map_or(
                    PhysicalSignalSettlementOutcome::DerivedStateUnavailable,
                    |_| PhysicalSignalSettlementOutcome::Committed,
                );
            outcomes.insert(key, outcome);
        }
        let outcomes = envelopes
            .iter()
            .map(|envelope| {
                outcomes
                    .get(&(
                        envelope.request_id(),
                        envelope.generation(),
                        envelope.branch_epoch(),
                        envelope.attempt(),
                    ))
                    .copied()
                    .unwrap_or(PhysicalSignalSettlementOutcome::DerivedStateUnavailable)
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        for (envelope, outcome) in envelopes.iter().zip(outcomes.iter()) {
            if *outcome != PhysicalSignalSettlementOutcome::DerivedStateUnavailable {
                self.release_envelope(envelope);
            }
        }
        outcomes
    }
}

const fn denial_proves_terminal(class: CompletionDenialClass) -> bool {
    matches!(
        class,
        CompletionDenialClass::Superseded
            | CompletionDenialClass::Duplicate
            | CompletionDenialClass::Retired
            | CompletionDenialClass::RetainedHistoryUnavailable
            | CompletionDenialClass::Cancelled
            | CompletionDenialClass::Rejected
            | CompletionDenialClass::TimedOut
    )
}

#[cfg(test)]
mod tests {
    use super::{denial_proves_terminal, CompletionDenialClass};

    #[test]
    fn only_terminal_signal_denials_may_release_physical_locality() {
        for class in [
            CompletionDenialClass::Superseded,
            CompletionDenialClass::Duplicate,
            CompletionDenialClass::Retired,
            CompletionDenialClass::RetainedHistoryUnavailable,
            CompletionDenialClass::Cancelled,
            CompletionDenialClass::Rejected,
            CompletionDenialClass::TimedOut,
        ] {
            assert!(denial_proves_terminal(class), "{class:?}");
        }
        for class in [
            CompletionDenialClass::Stale,
            CompletionDenialClass::Malformed,
            CompletionDenialClass::Partial,
            CompletionDenialClass::Contradictory,
            CompletionDenialClass::UnknownRequest,
            CompletionDenialClass::Impossible,
        ] {
            assert!(!denial_proves_terminal(class), "{class:?}");
        }
    }
}
