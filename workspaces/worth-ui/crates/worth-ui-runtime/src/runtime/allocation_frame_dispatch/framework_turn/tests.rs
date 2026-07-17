use crate::graph::UiGraphNodeIdentity;
use crate::runtime::tests::source_ingress_test_support::{empty_artifact, framework_from_artifact};
use crate::runtime::WorthUiTransientInteractionState;

#[test]
fn unknown_graph_targets_replay_to_the_same_typed_denial() {
    let mut first_runtime = framework_from_artifact(empty_artifact());
    let first_completion = first_runtime.execute_framework_turn(|turn| {
        turn.resize_preview(|source| {
            source
                .admit_and_submit(preview_sample(51))
                .expect("first interaction admits");
        });
    });

    let mut second_runtime = framework_from_artifact(empty_artifact());
    let second_completion = second_runtime.execute_framework_turn(|turn| {
        turn.resize_preview(|source| {
            source
                .admit_and_submit(preview_sample(52))
                .expect("second interaction admits");
        });
    });
    let denial = |completion| match completion {
        super::WorthUiFrameworkTurnCompletion::AllocationInvalidationNarrowingDenied {
            rejection,
        } => rejection.denial(),
        _ => panic!("unknown graph target must deny"),
    };
    assert_eq!(denial(first_completion), denial(second_completion));
}

#[test]
fn empty_framework_invocation_returns_typed_empty_outcome() {
    let mut runtime = framework_from_artifact(empty_artifact());
    {
        let completion = runtime.execute_framework_turn(|_| {});
        let execution = completion.into_execution().expect("empty turn executes");
        let _boundary = execution.activation_boundary();
    }
    assert!(runtime.pending_allocation_frame_handoff.is_none());
}

#[test]
fn denied_narrowing_releases_the_next_framework_turn() {
    let mut runtime = framework_from_artifact(empty_artifact());
    {
        let first_completion = runtime.execute_framework_turn(|turn| {
            turn.resize_preview(|source| {
                source
                    .admit_and_submit(preview_sample(42))
                    .expect("interaction admits");
            });
        });
        assert!(matches!(
            first_completion,
            super::WorthUiFrameworkTurnCompletion::AllocationInvalidationNarrowingDenied { .. }
        ));
    }
    assert!(runtime.pending_narrowed_allocation_frame.is_none());

    let second_completion = runtime.execute_framework_turn(|_| {});
    assert!(second_completion.into_execution().is_ok());
}

#[test]
fn callback_unwind_still_closes_and_pumps_the_turn() {
    let mut runtime = framework_from_artifact(empty_artifact());
    let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = runtime.execute_framework_turn(|turn| {
            turn.resize_preview(|source| {
                source
                    .admit_and_submit(preview_sample(43))
                    .expect("interaction admits");
            });
            panic!("source callback failed");
        });
    }));

    assert!(unwind.is_err());
    assert!(runtime.pending_narrowed_allocation_frame.is_none());
}

fn preview_sample(identity: u64) -> crate::runtime::UiResizePreviewSample {
    crate::runtime::UiResizePreviewSample::new(
        UiGraphNodeIdentity::new(identity),
        crate::runtime::UiResizeLogicalExtent::try_from_logical_pixels(320.0).unwrap(),
    )
}

#[test]
fn rejected_frame_returns_exact_disposition_and_releases_next_turn() {
    let mut runtime = framework_from_artifact(empty_artifact());
    let completion = runtime.execute_framework_turn(|turn| {
        turn.interaction(|source| {
            source
                .admit_and_submit(
                    UiGraphNodeIdentity::new(44),
                    WorthUiTransientInteractionState::Hover,
                )
                .expect("interaction source admits before stream classification");
        });
    });
    let super::WorthUiFrameworkTurnCompletion::AllocationFrameResolutionDenied { rejection } =
        completion
    else {
        panic!("unsupported stream posture must return a typed rejection");
    };
    assert_eq!(rejection.ingress_keys().len(), 1);
    assert_eq!(
        rejection.denial(),
        crate::runtime::UiAllocationFrameResolutionDenial::UnsupportedSourcePosture
    );
    let crate::evidence::UiAllocationStreamPolicyEvidenceOutcome::Denied(evidence) =
        rejection.evidence()
    else {
        panic!("rejected handoff must carry denial evidence");
    };
    assert_eq!(evidence.denial(), rejection.denial());
    assert_eq!(evidence.ingress().len(), 1);
    assert_eq!(
        evidence.duplicate_witness().canonical_ingress_keys(),
        rejection.ingress_keys()
    );
    assert_eq!(
        evidence.payload_counters().vector_capacity_reservations(),
        5
    );
    assert_eq!(evidence.payload_counters().boxed_slice_conversions(), 3);
    assert_eq!(
        evidence.payload_counters().denial_source_posture_copies(),
        1
    );
    assert!(runtime.pending_allocation_frame_handoff.is_none());

    let next = runtime.execute_framework_turn(|_| {});
    assert!(next.into_execution().is_ok());
}

#[test]
fn numeric_budget_denial_preflights_before_rich_composition() {
    let mut runtime = framework_from_artifact(empty_artifact());
    let completion = runtime.execute_framework_turn(|turn| {
        turn.interaction(|source| {
            for index in 0..17 {
                source
                    .admit_and_submit(
                        UiGraphNodeIdentity::new(100 + index),
                        WorthUiTransientInteractionState::TextInput,
                    )
                    .expect("text interaction admits");
            }
        });
    });
    let super::WorthUiFrameworkTurnCompletion::AllocationFrameResolutionDenied { rejection } =
        completion
    else {
        panic!("over-budget frame must deny");
    };
    assert!(matches!(
        rejection.denial(),
        crate::runtime::UiAllocationFrameResolutionDenial::Policy(
            crate::runtime::UiAllocationStreamCompositionDenial::InputBudgetExceeded {
                admitted: 17,
                allowed: 16,
            }
        )
    ));
    let crate::evidence::UiAllocationStreamPolicyEvidenceOutcome::Denied(evidence) =
        rejection.evidence()
    else {
        panic!("budget denial owns evidence");
    };
    assert_eq!(
        evidence.payload_counters().vector_capacity_reservations(),
        6
    );
    assert_eq!(evidence.payload_counters().boxed_slice_conversions(), 3);
    assert_eq!(
        evidence.payload_counters().denial_source_posture_copies(),
        17
    );
    assert_eq!(evidence.payload_counters().pair_contract_evaluations(), 0);
    assert_eq!(evidence.payload_counters().pair_policy_joins(), 0);
    assert_eq!(evidence.payload_counters().n_way_policy_joins(), 0);
    assert_eq!(evidence.payload_counters().branch_policy_joins(), 0);
}
