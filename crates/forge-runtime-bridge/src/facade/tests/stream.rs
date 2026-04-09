#[test]
fn runtime_validates_and_resolves_routing_stream_contract() {
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
        .validate_change_stream_declaration(declaration.clone())
        .expect("stream declarations should validate");
    let contract = runtime
        .resolve_change_stream_consumer_contract(&protocol)
        .expect("routing declaration should resolve into a routing contract");

    assert_eq!(protocol.declaration(), &declaration);
    assert_eq!(
        contract.consumer_shape(),
        crate::stream::StreamConsumerShape::RoutingConsumer
    );
    assert_eq!(
        contract.admitted_coalescing_family(),
        crate::stream::StreamCoalescingFamily::RoutingWindowCoalescing
    );
    assert_eq!(
        contract.admitted_checkpoint_mode(),
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow
    );
}

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
                canonical_envelope("main", "commit-a", "patch-a", "snapshot-a"),
                canonical_envelope("main", "commit-b", "patch-b", "snapshot-a"),
            ],
        )
        .expect("window should plan");
    let right = runtime
        .plan_change_stream_window(
            &contract,
            vec![
                canonical_envelope("main", "commit-a", "patch-a", "snapshot-a"),
                canonical_envelope("main", "commit-b", "patch-b", "snapshot-a"),
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
                canonical_envelope("main", "commit-a", "patch-a", "snapshot-a"),
                canonical_envelope("main", "commit-b", "patch-b", "snapshot-b"),
            ],
        )
        .expect_err("coalesced windows must not cross snapshot boundaries");

    assert_eq!(
        error.kind(),
        crate::error::BridgeStreamErrorKind::IllegalCoalescingBoundary
    );
}

#[test]
fn runtime_publishes_checkpoint_from_window() {
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
    let window = runtime
        .plan_change_stream_window(
            &contract,
            vec![
                canonical_envelope("main", "commit-a", "patch-a", "snapshot-a"),
                canonical_envelope("main", "commit-b", "patch-b", "snapshot-a"),
            ],
        )
        .expect("window should plan");

    let checkpoint = runtime.publish_consumer_checkpoint(
        &contract,
        &window,
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );

    assert_eq!(
        checkpoint.consumer_contract_identity(),
        contract.consumer_contract_identity()
    );
    assert_eq!(
        checkpoint.stream_protocol_identity(),
        contract.stream_protocol_identity()
    );
    assert_eq!(checkpoint.checkpoint_member_count(), 2);
    assert_eq!(
        checkpoint.checkpoint_frontier_kind(),
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier
    );
    assert_eq!(
        checkpoint.contiguous_acknowledged_through_position(),
        window.last_stream_position().stream_position_identity()
    );
}

#[test]
fn runtime_checkpoint_member_count_tracks_cumulative_frontier_width() {
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
    let first_window = runtime
        .plan_change_stream_window(
            &contract,
            vec![canonical_envelope(
                "main",
                "commit-a",
                "patch-a",
                "snapshot-a",
            )],
        )
        .expect("first window should plan");
    let first_checkpoint = runtime.publish_consumer_checkpoint(
        &contract,
        &first_window,
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );
    let _first_replay = runtime
        .canonicalize_stream_replay_record(&contract, &first_window, &first_checkpoint)
        .expect("first replay record should canonicalize");
    let resumed = runtime
        .resume_stream_window_from_checkpoint(
            &contract,
            vec![canonical_envelope(
                "main",
                "commit-b",
                "patch-b",
                "snapshot-a",
            )],
            first_checkpoint.checkpoint_token_identity(),
        )
        .expect("resume should succeed");
    let second_checkpoint = runtime.publish_consumer_checkpoint(
        &contract,
        resumed.resumed_window(),
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );

    assert_eq!(first_checkpoint.checkpoint_member_count(), 1);
    assert_eq!(second_checkpoint.checkpoint_member_count(), 2);
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
                "main",
                "commit-a",
                "patch-a",
                "snapshot-a",
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

#[test]
fn runtime_canonicalizes_stream_replay_record_from_matching_checkpoint() {
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
    let window = runtime
        .plan_change_stream_window(
            &contract,
            vec![
                canonical_envelope("main", "commit-a", "patch-a", "snapshot-a"),
                canonical_envelope("main", "commit-b", "patch-b", "snapshot-a"),
            ],
        )
        .expect("window should plan");
    let checkpoint = runtime.publish_consumer_checkpoint(
        &contract,
        &window,
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );

    runtime
        .validate_consumer_checkpoint(&contract, &window, &checkpoint)
        .expect("matching checkpoints should validate");
    let replay = runtime
        .canonicalize_stream_replay_record(&contract, &window, &checkpoint)
        .expect("matching stream facts should canonicalize a replay record");

    assert_eq!(
        replay.consumer_contract_identity(),
        contract.consumer_contract_identity()
    );
    assert_eq!(
        replay.stream_window_identity(),
        window.stream_window_identity()
    );
    assert_eq!(
        replay.checkpoint_token_identity(),
        checkpoint.checkpoint_token_identity()
    );
    assert!(replay
        .digest()
        .starts_with("canonical-stream-replay-record:sha256:"));
}

#[test]
fn runtime_rejects_checkpoint_reuse_across_different_contracts() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let routing_declaration = crate::stream::ChangeStreamDeclaration::new(
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
    let replay_declaration = crate::stream::ChangeStreamDeclaration::new(
        crate::stream::StreamConsumerShape::ReplayAuditConsumer,
        crate::stream::StreamResumeMode::FromCheckpointOnly,
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow,
        crate::stream::StreamCoalescingIntent::Prefer(
            crate::stream::StreamCoalescingFamily::ReplayAuditWindowCoalescing,
        ),
        crate::stream::StreamReplayMode::Enabled,
        crate::stream::StreamDeliveryIntent::ReplayAudit,
        crate::stream::StreamDiagnosticsPolicyClass::Standard,
    );
    let routing_protocol = runtime
        .validate_change_stream_declaration(routing_declaration)
        .expect("routing declaration should validate");
    let routing_contract = runtime
        .resolve_change_stream_consumer_contract(&routing_protocol)
        .expect("routing contract should resolve");
    let replay_protocol = runtime
        .validate_change_stream_declaration(replay_declaration)
        .expect("replay declaration should validate");
    let replay_contract = runtime
        .resolve_change_stream_consumer_contract(&replay_protocol)
        .expect("replay contract should resolve");
    let window = runtime
        .plan_change_stream_window(
            &routing_contract,
            vec![canonical_envelope(
                "main",
                "commit-a",
                "patch-a",
                "snapshot-a",
            )],
        )
        .expect("window should plan");
    let checkpoint = runtime.publish_consumer_checkpoint(
        &routing_contract,
        &window,
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );

    let error = runtime
        .validate_consumer_checkpoint(&replay_contract, &window, &checkpoint)
        .expect_err("checkpoints should not be reusable across different contracts");

    assert_eq!(
        error.kind(),
        crate::error::BridgeStreamErrorKind::CheckpointContractMismatch
    );
}

#[test]
fn runtime_retains_stream_checkpoint_and_replay_records() {
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
    let window = runtime
        .plan_change_stream_window(
            &contract,
            vec![canonical_envelope(
                "main",
                "commit-a",
                "patch-a",
                "snapshot-a",
            )],
        )
        .expect("window should plan");
    let checkpoint = runtime.publish_consumer_checkpoint(
        &contract,
        &window,
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );
    let replay = runtime
        .canonicalize_stream_replay_record(&contract, &window, &checkpoint)
        .expect("stream replay record should canonicalize");

    assert_eq!(
        runtime
            .diagnostics()
            .stream_checkpoint_for_identity(checkpoint.checkpoint_token_identity())
            .expect("checkpoint should be retained")
            .checkpoint_token_identity(),
        checkpoint.checkpoint_token_identity()
    );
    assert_eq!(
        runtime
            .diagnostics()
            .stream_replay_record_for_identity(replay.replay_record_identity().as_str())
            .expect("replay record should be retained")
            .replay_record_identity(),
        replay.replay_record_identity()
    );
    assert_eq!(
        runtime
            .diagnostics()
            .stream_replay_record_for_checkpoint_identity(checkpoint.checkpoint_token_identity())
            .expect("checkpoint-to-replay lookup should be retained")
            .replay_record_identity(),
        replay.replay_record_identity()
    );
}

#[test]
fn runtime_resume_rejects_truncated_checkpoint_identity() {
    let runtime = runtime(
        BridgeRuntimePolicy::development()
            .with_route_record_limit(1)
            .with_failure_record_limit(1),
    );
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
    let first_window = runtime
        .plan_change_stream_window(
            &contract,
            vec![canonical_envelope(
                "main",
                "commit-a",
                "patch-a",
                "snapshot-a",
            )],
        )
        .expect("first window should plan");
    let first_checkpoint = runtime.publish_consumer_checkpoint(
        &contract,
        &first_window,
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );
    let second_window = runtime
        .plan_change_stream_window(
            &contract,
            vec![canonical_envelope(
                "main",
                "commit-b",
                "patch-b",
                "snapshot-a",
            )],
        )
        .expect("second window should plan");
    let _second_checkpoint = runtime.publish_consumer_checkpoint(
        &contract,
        &second_window,
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );

    let error = runtime
        .resume_stream_window_from_checkpoint(
            &contract,
            vec![canonical_envelope(
                "main",
                "commit-b",
                "patch-b",
                "snapshot-a",
            )],
            first_checkpoint.checkpoint_token_identity(),
        )
        .expect_err("evicted checkpoints should be treated as truncated");

    assert_eq!(
        error.kind(),
        crate::error::BridgeStreamErrorKind::CheckpointTruncated
    );
}

#[test]
fn runtime_resume_reuses_retained_checkpoint_and_replay_truth() {
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
    let first_window = runtime
        .plan_change_stream_window(
            &contract,
            vec![canonical_envelope(
                "main",
                "commit-a",
                "patch-a",
                "snapshot-a",
            )],
        )
        .expect("first window should plan");
    let checkpoint = runtime.publish_consumer_checkpoint(
        &contract,
        &first_window,
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    );
    let replay = runtime
        .canonicalize_stream_replay_record(&contract, &first_window, &checkpoint)
        .expect("replay record should canonicalize");

    let resumed = runtime
        .resume_stream_window_from_checkpoint(
            &contract,
            vec![canonical_envelope(
                "main",
                "commit-b",
                "patch-b",
                "snapshot-a",
            )],
            checkpoint.checkpoint_token_identity(),
        )
        .expect("retained checkpoint should resume cleanly");

    assert_eq!(
        resumed.checkpoint().checkpoint_token_identity(),
        checkpoint.checkpoint_token_identity()
    );
    assert_eq!(
        resumed.replay_record().replay_record_identity(),
        replay.replay_record_identity()
    );
    assert_eq!(resumed.resumed_window().members().len(), 1);
    assert_eq!(
        resumed
            .resumed_window()
            .first_stream_position()
            .ordinal_position(),
        checkpoint.checkpoint_member_count()
    );
}

#[test]
fn runtime_delivers_routing_stream_window_through_admitted_contract() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = crate::stream::ChangeStreamDeclaration::new(
        crate::stream::StreamConsumerShape::RoutingConsumer,
        crate::stream::StreamResumeMode::FromCheckpointOnly,
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow,
        crate::stream::StreamCoalescingIntent::None,
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
    let window = runtime
        .plan_change_stream_window(
            &contract,
            vec![
                canonical_envelope("main", "commit-a", "patch-a", "snapshot-a"),
                canonical_envelope("main", "commit-b", "patch-b", "snapshot-a"),
            ],
        )
        .expect("window should plan");

    let result = runtime
        .deliver_change_stream_window(&contract, &window)
        .expect("routing-consumer windows should deliver");

    assert!(window.lowered_change_set().is_some());
    assert_eq!(
        window
            .lowered_change_set()
            .and_then(|lowered| lowered.planned_routes())
            .map(|routes| routes.len()),
        Some(2)
    );
    assert_eq!(
        result.summary().stream_window_identity(),
        window.stream_window_identity()
    );
    assert_eq!(result.summary().delivered_member_count(), 2);
    assert_eq!(result.summary().delivered_route_count(), 2);
    assert_eq!(result.route_results().len(), 2);
}

#[test]
fn runtime_rejects_delivery_for_non_routing_consumer_shape() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = crate::stream::ChangeStreamDeclaration::new(
        crate::stream::StreamConsumerShape::ReplayAuditConsumer,
        crate::stream::StreamResumeMode::FromCheckpointOnly,
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow,
        crate::stream::StreamCoalescingIntent::Prefer(
            crate::stream::StreamCoalescingFamily::ReplayAuditWindowCoalescing,
        ),
        crate::stream::StreamReplayMode::Enabled,
        crate::stream::StreamDeliveryIntent::ReplayAudit,
        crate::stream::StreamDiagnosticsPolicyClass::Standard,
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
                "main",
                "commit-a",
                "patch-a",
                "snapshot-a",
            )],
        )
        .expect("window should plan");

    let error = runtime
        .deliver_change_stream_window(&contract, &window)
        .expect_err("non-routing consumer delivery should be rejected explicitly");

    assert_eq!(
        error.kind(),
        crate::error::BridgeStreamErrorKind::UnsupportedConsumerShape
    );
}

#[test]
fn runtime_delivers_replay_audit_stream_window_and_retains_protocol_truth() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = crate::stream::ChangeStreamDeclaration::new(
        crate::stream::StreamConsumerShape::ReplayAuditConsumer,
        crate::stream::StreamResumeMode::FromCheckpointOnly,
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow,
        crate::stream::StreamCoalescingIntent::Prefer(
            crate::stream::StreamCoalescingFamily::ReplayAuditWindowCoalescing,
        ),
        crate::stream::StreamReplayMode::Enabled,
        crate::stream::StreamDeliveryIntent::ReplayAudit,
        crate::stream::StreamDiagnosticsPolicyClass::Standard,
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
            vec![
                canonical_envelope("main", "commit-a", "patch-a", "snapshot-a"),
                canonical_envelope("main", "commit-b", "patch-b", "snapshot-a"),
            ],
        )
        .expect("window should plan");

    let result = runtime
        .deliver_replay_audit_stream_window(&contract, &window)
        .expect("replay-audit windows should deliver");

    assert!(window.lowered_change_set().is_some());
    assert_eq!(
        result.summary().stream_window_identity(),
        window.stream_window_identity()
    );
    assert_eq!(result.summary().audited_member_count(), 2);
    assert_eq!(
        runtime
            .diagnostics()
            .stream_checkpoint_for_identity(result.checkpoint().checkpoint_token_identity())
            .expect("audit checkpoint should be retained")
            .checkpoint_token_identity(),
        result.checkpoint().checkpoint_token_identity()
    );
    assert_eq!(
        runtime
            .diagnostics()
            .stream_replay_record_for_identity(
                result.replay_record().replay_record_identity().as_str()
            )
            .expect("audit replay record should be retained")
            .replay_record_identity(),
        result.replay_record().replay_record_identity()
    );
}

#[test]
fn runtime_explains_last_stream_checkpoint_and_replay_record() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = crate::stream::ChangeStreamDeclaration::new(
        crate::stream::StreamConsumerShape::ReplayAuditConsumer,
        crate::stream::StreamResumeMode::FromCheckpointOnly,
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow,
        crate::stream::StreamCoalescingIntent::Prefer(
            crate::stream::StreamCoalescingFamily::ReplayAuditWindowCoalescing,
        ),
        crate::stream::StreamReplayMode::Enabled,
        crate::stream::StreamDeliveryIntent::ReplayAudit,
        crate::stream::StreamDiagnosticsPolicyClass::Standard,
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
                "main",
                "commit-a",
                "patch-a",
                "snapshot-a",
            )],
        )
        .expect("window should plan");
    let result = runtime
        .deliver_replay_audit_stream_window(&contract, &window)
        .expect("replay-audit windows should deliver");

    let checkpoint_explanation = runtime
        .diagnostics()
        .explain_last_stream_checkpoint()
        .expect("checkpoint explanation should be available");
    let replay_explanation = runtime
        .diagnostics()
        .explain_last_stream_replay_record()
        .expect("replay explanation should be available");

    assert_eq!(
        checkpoint_explanation.checkpoint_token_identity(),
        result.checkpoint().checkpoint_token_identity()
    );
    assert_eq!(
        replay_explanation.replay_record_identity(),
        result.replay_record().replay_record_identity().as_str()
    );
    assert_eq!(
        replay_explanation.checkpoint_token_identity(),
        result.checkpoint().checkpoint_token_identity()
    );
}

#[test]
fn runtime_rejects_replay_disabled_checkpoint_resume_contract_during_resolution() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = crate::stream::ChangeStreamDeclaration::new(
        crate::stream::StreamConsumerShape::RoutingConsumer,
        crate::stream::StreamResumeMode::FromCheckpointOnly,
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow,
        crate::stream::StreamCoalescingIntent::None,
        crate::stream::StreamReplayMode::Disabled,
        crate::stream::StreamDeliveryIntent::RouteInvalidations,
        crate::stream::StreamDiagnosticsPolicyClass::Standard,
    );
    let protocol = runtime
        .validate_change_stream_declaration(declaration)
        .expect("stream declarations should validate");

    let error = runtime
        .resolve_change_stream_consumer_contract(&protocol)
        .expect_err(
        "replay-disabled checkpoint resume contracts must be rejected during contract resolution",
    );

    assert_eq!(
        error.kind(),
        crate::error::BridgeStreamErrorKind::UnsupportedResumeMode
    );
}

#[test]
fn runtime_rejects_checkpoint_resume_contract_when_replay_mode_is_disabled() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = crate::stream::ChangeStreamDeclaration::new(
        crate::stream::StreamConsumerShape::RoutingConsumer,
        crate::stream::StreamResumeMode::FromCheckpointOnly,
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow,
        crate::stream::StreamCoalescingIntent::None,
        crate::stream::StreamReplayMode::Disabled,
        crate::stream::StreamDeliveryIntent::RouteInvalidations,
        crate::stream::StreamDiagnosticsPolicyClass::Standard,
    );
    let protocol = runtime
        .validate_change_stream_declaration(declaration)
        .expect("stream declarations should validate");

    let error = runtime
        .resolve_change_stream_consumer_contract(&protocol)
        .expect_err(
            "checkpoint resume without replay records must be rejected during contract resolution",
        );

    assert_eq!(
        error.kind(),
        crate::error::BridgeStreamErrorKind::UnsupportedResumeMode
    );
}
use super::*;
