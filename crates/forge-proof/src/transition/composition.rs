use crate::composition::JoinInputs2;

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

pub fn compose_join_transition_outcome<L, Rv, Next, D, De, St, Rb, F>(
    left: TransitionOutcome<L, D, De, St, Rb, F>,
    right: impl FnOnce() -> TransitionOutcome<Rv, D, De, St, Rb, F>,
    next: impl FnOnce(JoinInputs2<L, Rv>) -> TransitionOutcome<Next, D, De, St, Rb, F>,
) -> TransitionOutcome<Next, D, De, St, Rb, F> {
    match left {
        TransitionOutcome::Success(left_value) => match right() {
            TransitionOutcome::Success(right_value) => {
                next(JoinInputs2::new(left_value, right_value))
            }
            TransitionOutcome::Denied(value) => TransitionOutcome::Denied(value),
            TransitionOutcome::Deferred(value) => TransitionOutcome::Deferred(value),
            TransitionOutcome::Stale(value) => TransitionOutcome::Stale(value),
            TransitionOutcome::RebindRequired(value) => TransitionOutcome::RebindRequired(value),
            TransitionOutcome::Failed(value) => TransitionOutcome::Failed(value),
        },
        TransitionOutcome::Denied(value) => TransitionOutcome::Denied(value),
        TransitionOutcome::Deferred(value) => TransitionOutcome::Deferred(value),
        TransitionOutcome::Stale(value) => TransitionOutcome::Stale(value),
        TransitionOutcome::RebindRequired(value) => TransitionOutcome::RebindRequired(value),
        TransitionOutcome::Failed(value) => TransitionOutcome::Failed(value),
    }
}

pub fn compose_join_success_transition<L, Rv, Next, D, De, St, Rb, F>(
    left: TransitionOutcome<L, D, De, St, Rb, F>,
    right: impl FnOnce() -> TransitionOutcome<Rv, D, De, St, Rb, F>,
    next: impl FnOnce(JoinInputs2<L, Rv>) -> SuccessfulTransitionOutcome<Next>,
) -> TransitionOutcome<Next, D, De, St, Rb, F> {
    compose_join_transition_outcome(left, right, |inputs| next(inputs).into())
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use crate::composition::JoinInputs2;

    use super::{
        compose_join_success_transition, compose_join_transition_outcome,
        compose_success_transition, compose_transition_outcome,
    };
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

    #[test]
    fn join_composition_does_not_evaluate_right_lane_after_left_denial() {
        let right_ran = Cell::new(false);
        let denied = TransitionOutcome::<u64, &'static str>::denied("denied");

        let composed = compose_join_success_transition(
            denied,
            || {
                right_ran.set(true);
                TransitionOutcome::success(9_u64)
            },
            |inputs| SuccessfulTransitionOutcome::new(inputs.left() + inputs.right()),
        );

        assert!(matches!(composed, TransitionOutcome::Denied("denied")));
        assert!(!right_ran.get());
    }

    #[test]
    fn join_composition_short_circuits_before_next_after_right_denial() {
        let next_ran = Cell::new(false);
        let left = TransitionOutcome::<u64, &'static str>::success(4);

        let composed = compose_join_transition_outcome(
            left,
            || TransitionOutcome::<u64, &'static str>::denied("right denied"),
            |_inputs: JoinInputs2<u64, u64>| {
                next_ran.set(true);
                TransitionOutcome::success(11_u64)
            },
        );

        assert!(matches!(
            composed,
            TransitionOutcome::Denied("right denied")
        ));
        assert!(!next_ran.get());
    }

    #[test]
    fn join_composition_runs_next_after_both_successes() {
        let left = TransitionOutcome::<u64, &'static str>::success(4);

        let composed = compose_join_success_transition(
            left,
            || TransitionOutcome::success(8_u64),
            |inputs| SuccessfulTransitionOutcome::new(inputs.left() + inputs.right()),
        );

        assert!(matches!(composed, TransitionOutcome::Success(12)));
    }
}
