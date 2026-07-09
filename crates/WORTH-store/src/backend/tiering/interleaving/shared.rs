use crate::backend::records::{StoreState, TierRecallCompletionState};
use crate::tiering::{PlacementRaceOutcome, TierInterleavingObservation};

pub(crate) fn observation_for_artifacts(
    state: &StoreState,
    artifact_keys: Vec<String>,
) -> TierInterleavingObservation {
    let mut observed = Vec::new();
    let mut race_outcome = PlacementRaceOutcome::NoRace;

    for artifact_key in artifact_keys {
        let artifact_outcome = race_outcome_for_artifact(state, &artifact_key);
        if artifact_outcome != PlacementRaceOutcome::NoRace {
            observed.push(artifact_key);
            if severity(artifact_outcome) > severity(race_outcome) {
                race_outcome = artifact_outcome;
            }
        }
    }

    TierInterleavingObservation::new(race_outcome, observed)
}

fn race_outcome_for_artifact(state: &StoreState, artifact_key: &str) -> PlacementRaceOutcome {
    if state.tier_recall_records.values().any(|record| {
        record.artifact_key == artifact_key
            && record.completion_state == TierRecallCompletionState::InFlight
    }) {
        return PlacementRaceOutcome::RecallObserved;
    }

    match state.tier_transfer_records.get(artifact_key) {
        Some(record) if record.cutover_completed => PlacementRaceOutcome::CutoverObserved,
        Some(record) if record.transferred_replica_locator.is_some() => {
            PlacementRaceOutcome::TransferObserved
        }
        Some(_) => PlacementRaceOutcome::MovePrepareObserved,
        None => PlacementRaceOutcome::NoRace,
    }
}

fn severity(outcome: PlacementRaceOutcome) -> u8 {
    match outcome {
        PlacementRaceOutcome::NoRace => 0,
        PlacementRaceOutcome::MovePrepareObserved => 1,
        PlacementRaceOutcome::TransferObserved => 2,
        PlacementRaceOutcome::CutoverObserved => 3,
        PlacementRaceOutcome::RecallObserved => 4,
    }
}

pub(crate) fn record_interleaving_observation(
    counters: &crate::evidence::StoreCounters,
    observation: &crate::TierInterleavingObservation,
    continuation: bool,
    parity_preserved: bool,
) {
    if observation.race_outcome() == PlacementRaceOutcome::NoRace {
        return;
    }

    if continuation {
        counters.record_tier_interleaved_continuations(1);
    } else {
        counters.record_tier_interleaved_reads(1);
    }
    if observation.race_outcome() == PlacementRaceOutcome::RecallObserved {
        counters.record_tier_interleaving_recalls(1);
    }
    if !parity_preserved {
        counters.record_tier_interleaving_parity_failures(1);
    }
}
