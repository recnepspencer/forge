use super::outcomes::{SuccessfulTransitionOutcome, TransitionOutcome};

pub fn compose_transition_outcome<S, Next, D, De, St, R, F>(
    outcome: TransitionOutcome<S, D, De, St, R, F>,
    next: impl FnOnce(S) -> TransitionOutcome<Next, D, De, St, R, F>,
) -> TransitionOutcome<Next, D, De, St, R, F> {
    match outcome {
        TransitionOutcome::Success(value) => next(value),
        TransitionOutcome::Denied(value) => TransitionOutcome::Denied(value),
        TransitionOutcome::Deferred(value) => TransitionOutcome::Deferred(value),
        TransitionOutcome::Stale(value) => TransitionOutcome::Stale(value),
        TransitionOutcome::RebindRequired(value) => TransitionOutcome::RebindRequired(value),
        TransitionOutcome::Failed(value) => TransitionOutcome::Failed(value),
    }
}

pub fn compose_success_transition<S, Next, D, De, St, R, F>(
    outcome: TransitionOutcome<S, D, De, St, R, F>,
    next: impl FnOnce(S) -> SuccessfulTransitionOutcome<Next>,
) -> TransitionOutcome<Next, D, De, St, R, F> {
    compose_transition_outcome(outcome, |value| next(value).into())
}

#[cfg(test)]
mod tests {
    use super::{compose_success_transition, compose_transition_outcome};
    use crate::transition::{SuccessfulTransitionOutcome, TransitionOutcome};

    #[test]
    fn composition_short_circuits_denial_without_running_later_steps() {
        let denied = TransitionOutcome::<u64, &'static str>::denied("denied");
        let composed =
            compose_success_transition(denied, |value| SuccessfulTransitionOutcome::new(value + 1));

        assert!(matches!(composed, TransitionOutcome::Denied("denied")));
    }

    #[test]
    fn composition_runs_later_step_only_after_success() {
        let admitted = TransitionOutcome::<u64, &'static str>::success(7);
        let composed =
            compose_transition_outcome(admitted, |value| TransitionOutcome::success(value + 3));

        assert!(matches!(composed, TransitionOutcome::Success(10)));
    }
}
