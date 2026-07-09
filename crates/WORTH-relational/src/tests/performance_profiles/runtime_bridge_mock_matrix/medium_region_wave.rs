use super::*;

pub(super) fn certify_geometry_commit_bridge_medium_region_wave(suite: &'static str) {
    for (case, development_profile) in [
        (
            "geometry_commit_bridge_wave_medium_region_operational",
            false,
        ),
        (
            "geometry_commit_bridge_wave_medium_region_development",
            true,
        ),
    ] {
        let samples = capture_perf_samples(suite, case, || {
            let mut relational =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
            relational
                .config
                .diagnostics
                .profile
                .detailed_traces_enabled = development_profile;
            relational
                .config
                .diagnostics
                .profile
                .max_entries_per_artifact = if development_profile { 256 } else { 0 };

            let entities = seed_bridge_region_world(&mut relational, "bridge-medium", 24, 4);
            let updated = entities[10];
            let seeds = Arc::from([entities[8], entities[10], entities[12], entities[14]]);
            let mut bridge_runtime = build_mock_bridge_runtime(development_profile, entities.len());

            let relational_commit_started_at = Instant::now();
            let update = update_entity(&mut relational, updated, "bridge-medium-updated");
            let relational_commit_micros = relational_commit_started_at.elapsed().as_micros();

            let snapshot = relational.visibility_authority().snapshot();
            let traversal_packet = PlannedQueryPacket {
                label: "bridge-medium-traversal".to_string(),
                context_id: relational
                    .read_truth()
                    .query_plan_context(&snapshot)
                    .expect("bridge medium query plan context"),
                scope: QueryScope::ConnectivityTraversal {
                    seeds,
                    relation_kind_scope: Some(Arc::from([KindId(2)])),
                    max_depth: Some(3),
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalTraversalOrder,
                access_contract: QueryAccessContract::AuthoritativeStorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(92_101),
                target_count_hint: 4,
            };
            let relational_query_started_at = Instant::now();
            let traversal = relational
                .read_truth()
                .execute_query_plan(
                    relational
                        .read_truth()
                        .plan_query_packet(&snapshot, traversal_packet)
                        .expect("bridge medium traversal plan"),
                )
                .expect("bridge medium traversal outcome");
            let relational_query_micros = relational_query_started_at.elapsed().as_micros();

            let affected_sources = traversal
                .result
                .entities
                .len()
                .min(bridge_runtime.source_versions.len())
                .max(4);
            let bridge_before = bridge_runtime.observe();
            let bridge_started_at = Instant::now();
            bridge_runtime.apply_changes(affected_sources);
            let bridge_micros = bridge_started_at.elapsed().as_micros();
            let bridge_after = bridge_runtime.observe();

            PerfMeasurement {
                elapsed_micros: relational_commit_micros + relational_query_micros + bridge_micros,
                metrics: perf_metrics!({
                    "resident_entities": entities.len(),
                    "relational_changed_records": update.changed_records.len(),
                    "relational_result_entities": traversal.result.entities.len(),
                    "affected_bridge_sources": affected_sources,
                    "bridge_nodes_evaluated": bridge_after.evaluation.nodes_evaluated
                        - bridge_before.evaluation.nodes_evaluated,
                    "bridge_nodes_recomputed": bridge_after.evaluation.nodes_recomputed
                        - bridge_before.evaluation.nodes_recomputed,
                    "bridge_tasks_scheduled": bridge_after.planner.tasks_scheduled
                        - bridge_before.planner.tasks_scheduled,
                    "bridge_tasks_pruned": bridge_after.planner.tasks_pruned_before_execution
                        - bridge_before.planner.tasks_pruned_before_execution,
                    "bridge_history_entries": bridge_runtime.recent_history_len(),
                    "phase_timing": {
                        "relational_commit_micros": relational_commit_micros,
                        "relational_query_micros": relational_query_micros,
                        "bridge_micros": bridge_micros,
                    },
                }),
            }
        });
        emit_metric_summaries(
            suite,
            case,
            &samples,
            &[
                (
                    "relational_commit_micros",
                    &["phase_timing", "relational_commit_micros"],
                ),
                (
                    "relational_query_micros",
                    &["phase_timing", "relational_query_micros"],
                ),
                ("bridge_micros", &["phase_timing", "bridge_micros"]),
                ("resident_entities", &["resident_entities"]),
                ("affected_bridge_sources", &["affected_bridge_sources"]),
                ("bridge_nodes_recomputed", &["bridge_nodes_recomputed"]),
                ("bridge_tasks_scheduled", &["bridge_tasks_scheduled"]),
            ],
        );
        assert_budget(
            &samples,
            "medium bridge region certification should scale recompute with the affected region instead of the whole resident world",
            |metrics| {
                let affected = metrics["affected_bridge_sources"].as_u64().unwrap_or(0);
                let resident = metrics["resident_entities"].as_u64().unwrap_or(0);
                metrics["relational_changed_records"].as_u64() == Some(1)
                    && metrics["relational_result_entities"].as_u64().unwrap_or(0) >= 8
                    && affected >= 8
                    && affected < resident
                    && metrics["bridge_nodes_recomputed"].as_u64().unwrap_or(0) >= affected
                    && metrics["bridge_nodes_recomputed"].as_u64().unwrap_or(0) <= affected * 4
                    && metrics["bridge_tasks_scheduled"].as_u64().unwrap_or(0) >= affected
                    && metrics["bridge_tasks_scheduled"].as_u64().unwrap_or(0) <= affected * 3
            },
        );
    }
}
