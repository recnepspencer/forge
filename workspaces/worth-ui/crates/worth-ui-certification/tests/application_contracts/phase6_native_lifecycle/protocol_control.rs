use super::causal_mutation::CausalEventMutation;
use super::mutation_receipt::{emit, MutationReceiptCase, MutationTrace};
use super::oracle::{expected, Event, State};
use super::protocol_world::{UiNativeLifecycleEvent, UiNativeLifecycleState};

#[test]
fn oracle_substitution_mutation_is_rejected() {
    let expected_successor = expected(State::Presented, Event::BeginSuccessor);
    let result = CausalEventMutation {
        initial: UiNativeLifecycleState::Presented,
        schedule: &[UiNativeLifecycleEvent::BeginSuccessor],
        action_index: 0,
        replacement: UiNativeLifecycleEvent::Pointer,
    }
    .run();
    let baseline = result.baseline_at_divergence();
    assert_eq!(
        baseline.next_action.map(|action| format!("{action:?}")),
        expected_successor
            .next_action
            .map(|action| format!("{action:?}"))
    );

    let mutant = result.mutant_at_divergence();
    assert_ne!(baseline, mutant);
    let mutant_expected = expected(State::Presented, Event::Pointer);
    assert_ne!(expected_successor, mutant_expected);
    emit(MutationReceiptCase {
        requirement: "P6-PROTOCOL-WORLD-01",
        case: "oracle-substitution",
        baseline: MutationTrace {
            posture: "production schedule selects the independent successor transition",
            state: &format!("{:?}:{:?}", baseline.state, baseline.next_action),
        },
        mutant: MutationTrace {
            posture: "oracle substitution selects an input retention transition",
            state: &format!("{:?}:{:?}", mutant.state, mutant.effect),
        },
        denial: "independent oracle transition must match production",
        first_divergence: "BeginSuccessor versus Pointer at Presented",
    });
}
