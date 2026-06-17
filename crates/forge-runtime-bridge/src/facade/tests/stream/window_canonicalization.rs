use crate::facade::tests::{canonical_envelope, runtime};
use crate::policy::BridgeRuntimePolicy;

#[test]
fn runtime_lowers_identical_envelope_windows_canonically() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = crate::stream::ChangeStreamDeclaration::new(
        crate::stream::StreamConsumerShape::RoutingConsumer,
        crate::stream::StreamResumeMode::FromCheckpointOnly,
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow,
        crate::stream::StreamCoalescingIntent::Prefer(
            crate::stream::StreamCoalescingFamily::RoutingWindowCoalescing,
        ),
        crate::stream::StreamReplayMode::Enabled,
        crate::stream::StreamDeliveryIntent::RouteInvalidations,
        crate::stream::StreamDiagnosticsPolicyClass::Standard,
    );
    let protocol = runtime
        .validate_change_stream_declaration(declaration)
        .expect("stream declarations should validate");
    let contract = runtime
        .resolve_change_stream_consumer_contract(&protocol)
        .expect("stream contract should resolve");
    let left = runtime
        .plan_change_stream_window(
            &contract,
            vec![
                canonical_envelope(
                    crate::truth_identity_fixtures::truth_branch_fixture("main"),
                    crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                    crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                    crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                ),
                canonical_envelope(
                    crate::truth_identity_fixtures::truth_branch_fixture("main"),
                    crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
                    crate::truth_identity_fixtures::truth_patch_fixture("patch-b"),
                    crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                ),
            ],
        )
        .expect("window should plan");
    let right = runtime
        .plan_change_stream_window(
            &contract,
            vec![
                canonical_envelope(
                    crate::truth_identity_fixtures::truth_branch_fixture("main"),
                    crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                    crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                    crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                ),
                canonical_envelope(
                    crate::truth_identity_fixtures::truth_branch_fixture("main"),
                    crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
                    crate::truth_identity_fixtures::truth_patch_fixture("patch-b"),
                    crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                ),
            ],
        )
        .expect("identical stream material should plan identically");

    assert_eq!(
        left.stream_window_identity(),
        right.stream_window_identity()
    );
    assert_eq!(left.member_set_digest(), right.member_set_digest());
    assert_eq!(left.digest(), right.digest());
}

#[test]
fn runtime_rejects_coalesced_windows_across_snapshot_boundaries() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = crate::stream::ChangeStreamDeclaration::new(
        crate::stream::StreamConsumerShape::RoutingConsumer,
        crate::stream::StreamResumeMode::FromCheckpointOnly,
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow,
        crate::stream::StreamCoalescingIntent::Prefer(
            crate::stream::StreamCoalescingFamily::RoutingWindowCoalescing,
        ),
        crate::stream::StreamReplayMode::Enabled,
        crate::stream::StreamDeliveryIntent::RouteInvalidations,
        crate::stream::StreamDiagnosticsPolicyClass::Standard,
    );
    let protocol = runtime
        .validate_change_stream_declaration(declaration)
        .expect("stream declarations should validate");
    let contract = runtime
        .resolve_change_stream_consumer_contract(&protocol)
        .expect("stream contract should resolve");

    let error = runtime
        .plan_change_stream_window(
            &contract,
            vec![
                canonical_envelope(
                    crate::truth_identity_fixtures::truth_branch_fixture("main"),
                    crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                    crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                    crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                ),
                canonical_envelope(
                    crate::truth_identity_fixtures::truth_branch_fixture("main"),
                    crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
                    crate::truth_identity_fixtures::truth_patch_fixture("patch-b"),
                    crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-b"),
                ),
            ],
        )
        .expect_err("coalesced windows must not cross snapshot boundaries");

    assert_eq!(
        error.kind(),
        crate::error::BridgeStreamErrorKind::IllegalCoalescingBoundary
    );
}

#[test]
fn runtime_classifies_stable_no_pressure_backpressure_record() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = crate::stream::ChangeStreamDeclaration::new(
        crate::stream::StreamConsumerShape::RoutingConsumer,
        crate::stream::StreamResumeMode::FromCheckpointOnly,
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow,
        crate::stream::StreamCoalescingIntent::None,
        crate::stream::StreamReplayMode::Enabled,
        crate::stream::StreamDeliveryIntent::RouteInvalidations,
        crate::stream::StreamDiagnosticsPolicyClass::Minimal,
    );
    let protocol = runtime
        .validate_change_stream_declaration(declaration)
        .expect("stream declarations should validate");
    let contract = runtime
        .resolve_change_stream_consumer_contract(&protocol)
        .expect("stream contract should resolve");
    let window = runtime
        .plan_change_stream_window(
            &contract,
            vec![canonical_envelope(
                crate::truth_identity_fixtures::truth_branch_fixture("main"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            )],
        )
        .expect("single-member window should plan");

    let left = runtime.classify_stream_backpressure(&window);
    let right = runtime.classify_stream_backpressure(&window);

    assert_eq!(left, right);
    assert_eq!(
        left.consumer_contract_identity(),
        contract.consumer_contract_identity()
    );
    assert_eq!(
        left.stream_window_identity(),
        window.stream_window_identity()
    );
}
