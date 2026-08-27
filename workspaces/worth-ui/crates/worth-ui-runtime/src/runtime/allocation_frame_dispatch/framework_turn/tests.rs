use crate::graph::UiGraphNodeIdentity;
use crate::runtime::tests::source_ingress_test_support::{empty_artifact, framework_from_artifact};
use crate::runtime::WorthUiTransientInteractionState;

mod query_observation_tests;

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
        assert_eq!(
            execution.planning_counters(),
            super::UiFrameworkTransitionPlanningCounters::default()
        );
        let _boundary = execution.activation_boundary();
    }
}

#[test]
fn denied_narrowing_releases_the_next_framework_turn() {
    let mut runtime = framework_from_artifact(empty_artifact());
    let frame_epoch_before = runtime.frame_epoch();
    let source_order_before = runtime.allocation_source_order_ledger.clone();
    let truth_revision_before = runtime.allocation_receipt_ledger.truth_revision();
    let durable_state_before = runtime.allocation_receipt_ledger.durable_semantic_state();
    let portal_binding_before = runtime
        .allocation_invalidation_index
        .borrow()
        .portal_binding_identity_digest();
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

    assert_eq!(runtime.frame_epoch(), frame_epoch_before);
    assert_eq!(runtime.allocation_source_order_ledger, source_order_before);
    assert_eq!(
        runtime.allocation_receipt_ledger.truth_revision(),
        truth_revision_before
    );
    assert_eq!(
        runtime.allocation_receipt_ledger.durable_semantic_state(),
        durable_state_before
    );
    assert_eq!(
        runtime
            .allocation_invalidation_index
            .borrow()
            .portal_binding_identity_digest(),
        portal_binding_before
    );

    let second_completion = runtime.execute_framework_turn(|_| {});
    assert!(second_completion.into_execution().is_ok());
}

#[test]
fn callback_unwind_still_closes_and_pumps_the_turn() {
    let mut runtime = framework_from_artifact(empty_artifact());
    let before = runtime.allocation_frame_dispatcher_counters();
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
    let after = runtime.allocation_frame_dispatcher_counters();
    assert_eq!(after.frame_count(), before.frame_count() + 1);
    assert_eq!(
        after.canonical_drain_count(),
        before.canonical_drain_count() + 1
    );
}

#[test]
fn operation_live_publication_preserves_prior_truth_across_callback_unwind() {
    let mut fixture = worth_ui_query_binding::certification::WorthUiOperationLiveTestFixture::new(
        "framework-turn-operation-live",
    );
    let reference = fixture.reference().clone();
    let binding = fixture.binding_plan().prepare_downstream_state();
    let resource = fixture.open_resource();
    let mut runtime = framework_from_artifact(empty_artifact());
    runtime.install_query_binding_state_for_test(binding);

    drop(runtime.execute_framework_turn(|turn| {
        turn.query_projection(|source| {
            source
                .admit_operation_live(resource)
                .expect("live resource belongs to the installed binding");
        });
    }));

    fixture.update_measurement();
    drop(runtime.execute_framework_turn(|turn| {
        turn.query_projection(|source| {
            let outcome = source
                .refresh_operation_live(fixture.refresh_request())
                .expect("first exact patch stages");
            assert!(matches!(
                outcome,
                worth_ui_query_binding::WorthUiOperationLiveSourceRefreshOutcome::Staged(_)
            ));
        });
    }));
    let first = runtime.operation_live_change_observation_for_test(&reference);
    assert_eq!(first.staged_change_count(), 0);
    assert_eq!(first.admitted_change_count(), 1);

    fixture.update_measurement();
    let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = runtime.execute_framework_turn(|turn| {
            turn.query_projection(|source| {
                source
                    .refresh_operation_live(fixture.refresh_request())
                    .expect("second exact patch stages");
            });
            panic!("interrupt collection-change publication");
        });
    }));
    assert!(unwind.is_err());
    let interrupted = runtime.operation_live_change_observation_for_test(&reference);
    assert_eq!(interrupted.staged_change_count(), 1);
    assert_eq!(interrupted.admitted_change_count(), 1);

    drop(runtime.execute_framework_turn(|_| {}));
    let retried = runtime.operation_live_change_observation_for_test(&reference);
    assert_eq!(retried.staged_change_count(), 0);
    assert_eq!(retried.admitted_change_count(), 2);

    let retirement = runtime
        .shutdown()
        .unwrap_or_else(|recovery| panic!("shutdown blocked: {:?}", recovery.blocker()))
        .into_operation_live_retirement();
    let closed = fixture.close_retirement(retirement);
    assert!(matches!(
        closed,
        worth_ui_query_binding::WorthUiOperationLiveRetirementCloseOutcome::Closed(_)
    ));
}

#[test]
fn denied_framework_transition_does_not_publish_operation_live_change() {
    let mut fixture = worth_ui_query_binding::certification::WorthUiOperationLiveTestFixture::new(
        "denied-turn-operation-live",
    );
    let reference = fixture.reference().clone();
    let binding = fixture.binding_plan().prepare_downstream_state();
    let resource = fixture.open_resource();
    let mut runtime = framework_from_artifact(empty_artifact());
    runtime.install_query_binding_state_for_test(binding);
    drop(runtime.execute_framework_turn(|turn| {
        turn.query_projection(|source| {
            source.admit_operation_live(resource).unwrap();
        });
    }));

    fixture.update_measurement();
    let completion = runtime.execute_framework_turn(|turn| {
        turn.query_projection(|source| {
            source
                .refresh_operation_live(fixture.refresh_request())
                .expect("exact change stages before unrelated denial");
        });
        turn.interaction(|source| {
            source
                .admit_and_submit(
                    UiGraphNodeIdentity::new(9_001),
                    WorthUiTransientInteractionState::Hover,
                )
                .expect("unsupported posture enters stream classification");
        });
    });
    assert!(matches!(
        completion,
        super::WorthUiFrameworkTurnCompletion::AllocationFrameResolutionDenied { .. }
    ));
    let denied = runtime.operation_live_change_observation_for_test(&reference);
    assert_eq!(denied.staged_change_count(), 1);
    assert_eq!(denied.admitted_change_count(), 0);

    drop(runtime.execute_framework_turn(|_| {}));
    let retried = runtime.operation_live_change_observation_for_test(&reference);
    assert_eq!(retried.staged_change_count(), 0);
    assert_eq!(retried.admitted_change_count(), 1);
    let retirement = runtime
        .shutdown()
        .unwrap_or_else(|recovery| panic!("shutdown blocked: {:?}", recovery.blocker()))
        .into_operation_live_retirement();
    assert!(matches!(
        fixture.close_retirement(retirement),
        worth_ui_query_binding::WorthUiOperationLiveRetirementCloseOutcome::Closed(_)
    ));
}

#[test]
fn policy_family_executors_cannot_reach_framework_clock_or_whole_runtime() {
    let executors = [
        include_str!("policy_execution/ordinary.rs"),
        include_str!("policy_execution/viewport.rs"),
        include_str!("policy_execution/resize_preview.rs"),
        include_str!("policy_execution/durable_resize.rs"),
        include_str!("policy_execution/drag_resize.rs"),
    ];
    for source in executors {
        assert!(!source.contains("WorthUiRuntime"));
        assert!(!source.contains("allocation_frame_scheduler"));
        assert!(!source.contains("run_turn("));
        assert!(!source.contains("select_replan_neighborhoods"));
        assert!(!source.contains("narrow_resolved_frame"));
    }
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
