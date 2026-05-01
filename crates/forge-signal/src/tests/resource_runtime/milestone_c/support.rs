use super::*;

pub(super) struct ResourceMilestoneCPolicyFixture {
    pub(super) freeze_report: ResourcePolicyRegistryFreezeReport,
    pub(super) denied_retry_report: ResourceRetryScheduleReport,
    pub(super) heartbeat_denial_report: ResourceTimeoutHeartbeatExtensionReport,
    pub(super) retention_report: ResourceLifecycleRetentionCompactionReport,
    pub(super) diagnostics_denial: ResourceDiagnosticsExpansionDenial,
    pub(super) compatible_restore: ResourcePolicyRestoreCompatibilityProof,
    pub(super) incompatible_restore: DeniedResourcePolicyRestoreCompatibility,
    pub(super) missing_restore: DeniedResourcePolicyRestoreCompatibility,
    pub(super) bundle: ResourceMilestoneCPolicyCertificationBundle,
}

pub(super) fn resource_milestone_c_policy_fixture() -> ResourceMilestoneCPolicyFixture {
    let freeze_report = FrozenResourcePolicyRegistry::built_in()
        .freeze_report()
        .clone();

    let mut retry_graph = SignalGraph::new();
    let retry_first = retry_graph.node().build();
    let retry_second = retry_graph.node().build();
    let mut retry_runtime = TestRuntime::build(retry_graph);
    retry_runtime
        .declare_resource_node(retry_budgeted_timeout_resource_declaration(
            retry_first,
            3,
            7,
            ResourceRetryBudgetScope::Runtime,
            1,
        ))
        .expect("first retry declaration should lower");
    retry_runtime
        .declare_resource_node(retry_budgeted_timeout_resource_declaration(
            retry_second,
            3,
            7,
            ResourceRetryBudgetScope::Runtime,
            1,
        ))
        .expect("second retry declaration should lower");
    let _scheduled_retry = schedule_timed_out_retry(&mut retry_runtime, retry_first);
    let denied_retry_report = schedule_timed_out_retry(&mut retry_runtime, retry_second);

    let mut timeout_graph = SignalGraph::new();
    let timeout_node = timeout_graph.node().build();
    let mut timeout_runtime = TestRuntime::build(timeout_graph);
    timeout_runtime
        .declare_resource_node(heartbeat_extension_timeout_resource_declaration(
            timeout_node,
            5,
            2,
        ))
        .expect("timeout declaration should lower");
    let timeout_admitted = timeout_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            timeout_node,
        )))
        .expect("timeout request should admit")
        .admitted_request();
    let timeout_wake = timeout_runtime
        .in_flight_resource_request(timeout_admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should attach");
    timeout_runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .expect("clock should reach timeout");
    let ready_timeout = timeout_runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should become ready");
    let timeout_report = timeout_runtime
        .admit_resource_timeout(timeout_admitted.handle(), ready_timeout)
        .expect("timeout admission should succeed");
    let heartbeat_denial_report = timeout_runtime
        .extend_resource_timeout_heartbeat(timeout_admitted.handle())
        .expect("terminal heartbeat extension should still report denial");

    let mut cancellation_graph = SignalGraph::new();
    let cancel_node = cancellation_graph.node().build();
    let overlap_node = cancellation_graph.node().build();
    let coalesce_node = cancellation_graph.node().build();
    let mut cancellation_runtime = TestRuntime::build(cancellation_graph);
    cancellation_runtime
        .declare_resource_node(resource_declaration(cancel_node))
        .expect("cancellation declaration should lower");
    cancellation_runtime
        .declare_resource_node(overlap_cancelled_host_work_resource_declaration(
            overlap_node,
        ))
        .expect("overlap declaration should lower");
    cancellation_runtime
        .declare_resource_node(intent_equivalent_coalescing_resource_declaration(
            coalesce_node,
        ))
        .expect("coalescing declaration should lower");
    let cancelled_request = cancellation_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            cancel_node,
        )))
        .expect("cancel request should admit")
        .admitted_request();
    let cancellation_report = cancellation_runtime
        .cancel_resource_request(
            cancelled_request.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("cancellation should admit");
    let _first_overlap = cancellation_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            overlap_node,
        )))
        .expect("first overlap request should admit");
    let second_overlap = cancellation_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            overlap_node,
        )))
        .expect("second overlap request should admit");
    let overlap_admission = second_overlap
        .supersession_record()
        .and_then(|record| record.overlap_admission().cloned())
        .expect("overlap policy should retain overlap admission evidence");
    let _first_coalesced = cancellation_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            coalesce_node,
        )))
        .expect("first coalescing request should admit");
    let second_coalesced = cancellation_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            coalesce_node,
        )))
        .expect("second coalescing request should coalesce");
    let intent_coalescing = second_coalesced
        .intent_equivalence_coalescing()
        .expect("coalescing policy should retain lineage evidence");

    let mut revalidation_graph = SignalGraph::new();
    let revalidation_node = revalidation_graph.node().build();
    let mut revalidation_runtime = TestRuntime::build(revalidation_graph);
    revalidation_runtime
        .declare_resource_node(resource_declaration(revalidation_node))
        .expect("revalidation declaration should lower");
    let revalidation_report = revalidation_runtime
        .revalidate_resource_node(ResourceRevalidationIntent::new(ResourceNodeId::from_node(
            revalidation_node,
        )))
        .expect("explicit revalidation should admit");

    let mut observation_graph = SignalGraph::new();
    let observation_node = observation_graph.node().build();
    let mut observation_runtime = TestRuntime::build(observation_graph);
    observation_runtime
        .declare_resource_node(resource_declaration(observation_node))
        .expect("observation declaration should lower");
    let observation_request = observation_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            observation_node,
        )))
        .expect("observation request should admit")
        .admitted_request();
    let observation_completion = observation_runtime
        .admit_resource_completion(raw_completion(
            &observation_runtime,
            observation_node,
            observation_request.handle(),
            observation_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("observation completion should admit");
    let calls = Arc::new(Mutex::new(Vec::<ResourceObservationRecord>::new()));
    observation_runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [observation_node],
        Box::new(ResourceObservationListener {
            calls: Arc::clone(&calls),
        }),
    );
    let mut ctx = ();
    observation_runtime
        .transaction(&mut ctx, |tx| {
            let staged = tx.stage_admitted_resource_completion(observation_completion)?;
            tx.commit_staged_resource_completion(staged.staged_effect())?;
            Ok(())
        })
        .expect("observation completion should commit");
    let observation_report = observation_runtime
        .latest_resource_observation_batch_report()
        .expect("observation batch report should materialize");

    let mut replay_graph = SignalGraph::new();
    let first_replay_node = replay_graph.node().build();
    let second_replay_node = replay_graph.node().build();
    let mut replay_runtime = TestRuntime::build(replay_graph);
    replay_runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(
            first_replay_node,
        ))
        .expect("first replay declaration should lower");
    replay_runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(
            second_replay_node,
        ))
        .expect("second replay declaration should lower");
    let first_replay = replay_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            first_replay_node,
        )))
        .expect("first replay request should admit")
        .admitted_request();
    let second_replay = replay_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            second_replay_node,
        )))
        .expect("second replay request should admit")
        .admitted_request();
    replay_runtime
        .cancel_resource_request(
            first_replay.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("first replay cancellation should admit");
    replay_runtime
        .cancel_resource_request(
            second_replay.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("second replay cancellation should admit");
    let retention_report =
        replay_runtime.compact_resource_lifecycle_history_with_retained_limit(2, 1);
    let replay_availability = replay_runtime
        .resource_replay_availability(&resource_declaration(first_replay_node))
        .expect("default replay availability should classify");
    let diagnostics_denial = replay_runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::retained_summary_only(),
        )
        .expect_err("retained-summary-only diagnostics budget should deny cold reconstruction");

    let mut retention_restore_graph = SignalGraph::new();
    let retention_restore_node = retention_restore_graph.node().build();
    let retention_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Retention,
        "signal.resource.retention.terminal-summaries-only",
    );
    let mut retention_restore_runtime = TestRuntime::builder(retention_restore_graph)
        .with_kernel_defaults()
        .resource_policy_registry(retention_registry)
        .build();
    retention_restore_runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(
            retention_restore_node,
        ))
        .expect("historical retention declaration should lower");
    let compatible_restore = retention_restore_runtime
        .admit_resource_policy_restore_compatibility(&terminal_summaries_only_resource_declaration(
            retention_restore_node,
        ))
        .expect("compatible retention drift should classify")
        .expect("compatible retention drift should admit");

    let mut incompatible_restore_graph = SignalGraph::new();
    let incompatible_restore_node = incompatible_restore_graph.node().build();
    let historical_incompatible_timeout =
        timeout_resource_declaration(incompatible_restore_node, 3);
    let historical_incompatible_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &historical_incompatible_timeout,
        &FrozenResourcePolicyRegistry::built_in(),
    )
    .expect("historical timeout declaration should validate");
    let historical_incompatible_frozen =
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(
            &historical_incompatible_validated,
            &FrozenResourcePolicyRegistry::built_in(),
        )
        .expect("historical timeout declaration should freeze");
    let historical_incompatible_lowered =
        LoweredResourcePolicyBundle::from_frozen_descriptors(&historical_incompatible_frozen);
    let incompatible_registrations = built_in_policy_registrations()
        .into_iter()
        .map(|registration| {
            if matches!(
                (registration.kind(), registration.semantic_name().as_str()),
                (
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.fixed-timeout"
                )
            ) {
                ResourcePolicyRegistration::new(
                    registration.id(),
                    registration.kind(),
                    registration.semantic_name().clone(),
                    ResourcePolicyVersion::new(2, 0),
                    registration.cost_contract(),
                    ResourcePolicyCompatibilityPosture::IncompatibleVersion,
                )
            } else {
                registration
            }
        })
        .collect();
    let incompatible_registry = FrozenResourcePolicyRegistry::new(incompatible_registrations)
        .expect("incompatible registry should freeze");
    let current_incompatible_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &resource_declaration(incompatible_restore_node),
        &incompatible_registry,
    )
    .expect("current declaration should validate against the incompatible registry");
    let incompatible_report =
        ResourcePolicyCompatibilityReport::classify_against_validated_declaration(
            ResourceDescriptorId::new(127),
            ResourceNodeId::from_node(incompatible_restore_node),
            &historical_incompatible_lowered,
            &current_incompatible_validated,
            &incompatible_registry,
        )
        .expect("incompatible-version compatibility classification should succeed");
    let current_incompatible_frozen =
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(
            &current_incompatible_validated,
            &incompatible_registry,
        )
        .expect("current declaration should freeze against the incompatible registry");
    let incompatible_replay_plan = ResourceReplayDecisionPlan::lower(
        current_incompatible_validated.declaration().replay_policy(),
        current_incompatible_frozen.replay(),
    )
    .expect("default replay policy should lower for incompatible-version denial");
    let incompatible_restore = DeniedResourcePolicyRestoreCompatibility::from_compatibility(
        incompatible_report,
        &incompatible_replay_plan,
    );

    let mut missing_restore_graph = SignalGraph::new();
    let missing_restore_node = missing_restore_graph.node().build();
    let missing_registry = FrozenResourcePolicyRegistry::new(
        built_in_policy_registrations()
            .into_iter()
            .filter(|registration| {
                !matches!(
                    (registration.kind(), registration.semantic_name().as_str()),
                    (
                        ResourcePolicyKind::Timeout,
                        "signal.resource.timeout.fixed-timeout"
                    )
                )
            })
            .collect(),
    )
    .expect("missing registry should still freeze");
    let historical_missing_timeout = timeout_resource_declaration(missing_restore_node, 3);
    let historical_missing_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &historical_missing_timeout,
        &FrozenResourcePolicyRegistry::built_in(),
    )
    .expect("historical timeout declaration should validate against the built-in registry");
    let historical_missing_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &historical_missing_validated,
        &FrozenResourcePolicyRegistry::built_in(),
    )
    .expect("historical timeout declaration should freeze against the built-in registry");
    let historical_missing_lowered =
        LoweredResourcePolicyBundle::from_frozen_descriptors(&historical_missing_frozen);
    let current_missing_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &resource_declaration(missing_restore_node),
        &missing_registry,
    )
    .expect("current declaration should validate against the reduced registry");
    let missing_report = ResourcePolicyCompatibilityReport::classify_against_validated_declaration(
        ResourceDescriptorId::new(177),
        ResourceNodeId::from_node(missing_restore_node),
        &historical_missing_lowered,
        &current_missing_validated,
        &missing_registry,
    )
    .expect("missing-descriptor compatibility classification should succeed");
    let current_missing_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &current_missing_validated,
        &missing_registry,
    )
    .expect("current declaration should freeze against the reduced registry");
    let missing_replay_plan = ResourceReplayDecisionPlan::lower(
        current_missing_validated.declaration().replay_policy(),
        current_missing_frozen.replay(),
    )
    .expect("default replay policy should lower for missing-descriptor denial");
    let missing_restore = DeniedResourcePolicyRestoreCompatibility::from_compatibility(
        missing_report,
        &missing_replay_plan,
    );

    let bundle = resource_milestone_c_policy_certification_builder()
        .with_async_resource_policy_family_certification(&freeze_report)
        .expect("policy family certification should accept freeze evidence")
        .with_async_retry_budget_and_backoff_certification(&denied_retry_report)
        .expect("retry family certification should accept retry evidence")
        .with_async_timeout_deadline_certification(&timeout_report, &heartbeat_denial_report)
        .expect("timeout family certification should accept timeout evidence")
        .with_async_cancellation_supersession_policy_certification(
            &cancellation_report,
            &overlap_admission,
            &intent_coalescing,
        )
        .expect("cancellation/supersession family certification should accept evidence")
        .with_async_revalidation_freshness_certification(&revalidation_report)
        .expect("revalidation family certification should accept evidence")
        .with_async_observation_output_continuity_certification(&observation_report)
        .expect("observation family certification should accept evidence")
        .with_async_retention_replay_policy_certification(&retention_report, &replay_availability)
        .expect("retention/replay family certification should accept evidence")
        .build()
        .expect("complete milestone C policy certification bundle should pass");

    ResourceMilestoneCPolicyFixture {
        freeze_report,
        denied_retry_report,
        heartbeat_denial_report,
        retention_report,
        diagnostics_denial,
        compatible_restore,
        incompatible_restore,
        missing_restore,
        bundle,
    }
}
