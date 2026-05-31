use super::*;

pub(super) fn certify_geometry_commit_bridge_mixed_locality_wave(suite: &'static str) {
    let mixed_locality_samples = capture_perf_samples(
        suite,
        "geometry_commit_bridge_wave_mixed_locality_operational",
        || {
            let mut relational =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
            let entities = seed_bridge_region_world(&mut relational, "bridge-mixed", 20, 5);
            let updated = entities[9];
            let query_targets = [
                "bridge-mixed-node-2",
                "bridge-mixed-node-7",
                "bridge-mixed-node-11",
                "bridge-mixed-node-16",
            ];
            let traversal_seeds = Arc::from([entities[7], entities[9]]);
            let mut bridge_runtime = build_mock_bridge_runtime(false, entities.len());

            let relational_commit_started_at = Instant::now();
            let update = update_entity(&mut relational, updated, "bridge-mixed-updated");
            let relational_commit_micros = relational_commit_started_at.elapsed().as_micros();

            let snapshot = relational.visibility_authority().snapshot();
            let traversal_packet = PlannedQueryPacket {
                label: "bridge-mixed-traversal".to_string(),
                context_id: relational
                    .read_truth()
                    .query_plan_context(&snapshot)
                    .expect("bridge mixed query plan context"),
                scope: QueryScope::ConnectivityTraversal {
                    seeds: traversal_seeds,
                    relation_kind_scope: Some(Arc::from([KindId(2)])),
                    max_depth: Some(2),
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalTraversalOrder,
                access_contract: QueryAccessContract::AuthoritativeStorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(92_201),
                target_count_hint: 2,
            };
            let relational_query_started_at = Instant::now();
            let traversal = relational
                .read_truth()
                .execute_query_plan(
                    relational
                        .read_truth()
                        .plan_query_packet(&snapshot, traversal_packet)
                        .expect("bridge mixed traversal plan"),
                )
                .expect("bridge mixed traversal outcome");
            let explicit_hits = query_targets
                .iter()
                .map(|name| {
                    relational
                        .read_truth()
                        .execute_query_plan(
                            relational
                                .read_truth()
                                .plan_query_packet(
                                    &snapshot,
                                    entity_name_index_packet(
                                        &relational,
                                        &snapshot,
                                        "bridge-mixed-explicit",
                                        name,
                                    ),
                                )
                                .expect("bridge mixed explicit plan"),
                        )
                        .expect("bridge mixed explicit outcome")
                        .result
                        .entities
                        .len()
                })
                .sum::<usize>();
            let relational_query_micros = relational_query_started_at.elapsed().as_micros();

            let affected_sources = (traversal.result.entities.len() + explicit_hits)
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
                    "traversal_result_entities": traversal.result.entities.len(),
                    "explicit_result_entities": explicit_hits,
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
        },
    );
    emit_metric_summaries(
        suite,
        "geometry_commit_bridge_wave_mixed_locality_operational",
        &mixed_locality_samples,
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
            ("traversal_result_entities", &["traversal_result_entities"]),
            ("explicit_result_entities", &["explicit_result_entities"]),
            ("affected_bridge_sources", &["affected_bridge_sources"]),
            ("bridge_tasks_scheduled", &["bridge_tasks_scheduled"]),
        ],
    );
    assert_budget(
        &mixed_locality_samples,
        "mixed locality bridge certification should keep explicit and traversal reads additive without exploding downstream recompute",
        |metrics| {
            let traversal = metrics["traversal_result_entities"].as_u64().unwrap_or(0);
            let explicit = metrics["explicit_result_entities"].as_u64().unwrap_or(0);
            let affected = metrics["affected_bridge_sources"].as_u64().unwrap_or(0);
            metrics["relational_changed_records"].as_u64() == Some(1)
                && traversal >= 4
                && explicit >= 4
                && affected >= explicit
                && metrics["bridge_tasks_scheduled"].as_u64().unwrap_or(0) >= affected
                && metrics["bridge_nodes_recomputed"].as_u64().unwrap_or(0) >= affected
        },
    );
}
