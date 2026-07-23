use worth_store::physical_runtime::{
    LifecycleGeneration, ObservationError, RuntimeIdentity, RuntimeObservation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleAction {
    Close,
    Abort,
    Panic,
    UnexpectedDrop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OracleLifecyclePhase {
    Admitted,
    Closed,
    Aborted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OracleObservationOutcome {
    Closed,
    Stale,
}

struct LifecycleTransition {
    source: OracleLifecyclePhase,
    action: LifecycleAction,
    terminal: OracleLifecyclePhase,
    observation: OracleObservationOutcome,
}

const LIFECYCLE_TRANSITIONS: [LifecycleTransition; 4] = [
    LifecycleTransition {
        source: OracleLifecyclePhase::Admitted,
        action: LifecycleAction::Close,
        terminal: OracleLifecyclePhase::Closed,
        observation: OracleObservationOutcome::Closed,
    },
    LifecycleTransition {
        source: OracleLifecyclePhase::Admitted,
        action: LifecycleAction::Abort,
        terminal: OracleLifecyclePhase::Aborted,
        observation: OracleObservationOutcome::Stale,
    },
    LifecycleTransition {
        source: OracleLifecyclePhase::Admitted,
        action: LifecycleAction::UnexpectedDrop,
        terminal: OracleLifecyclePhase::Aborted,
        observation: OracleObservationOutcome::Stale,
    },
    LifecycleTransition {
        source: OracleLifecyclePhase::Admitted,
        action: LifecycleAction::Panic,
        terminal: OracleLifecyclePhase::Aborted,
        observation: OracleObservationOutcome::Stale,
    },
];

pub fn assert_terminal_observation(
    action: LifecycleAction,
    outcome: Result<RuntimeObservation, ObservationError>,
    runtime_identity: RuntimeIdentity,
    admitted_generation: LifecycleGeneration,
) {
    let transition = LIFECYCLE_TRANSITIONS
        .iter()
        .find(|transition| {
            transition.source == OracleLifecyclePhase::Admitted && transition.action == action
        })
        .expect("the independent C.3 lifecycle table must cover every exercised action");
    let expected_terminal_generation = admitted_generation.get() + 2;

    match (transition.terminal, transition.observation, outcome) {
        (
            OracleLifecyclePhase::Closed,
            OracleObservationOutcome::Closed,
            Err(ObservationError::Closed {
                runtime_identity: observed_identity,
                closed_generation,
            }),
        ) => {
            assert_eq!(observed_identity, runtime_identity);
            assert_eq!(closed_generation.get(), expected_terminal_generation);
        }
        (
            OracleLifecyclePhase::Aborted,
            OracleObservationOutcome::Stale,
            Err(ObservationError::Stale {
                runtime_identity: observed_identity,
                observed_generation,
                current_generation,
            }),
        ) => {
            assert_eq!(observed_identity, runtime_identity);
            assert_eq!(observed_generation, admitted_generation);
            assert_eq!(current_generation.get(), expected_terminal_generation);
        }
        (terminal, observation, outcome) => {
            panic!("expected {terminal:?}/{observation:?} after {action:?}, received {outcome:?}")
        }
    }
}
