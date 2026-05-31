use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::Instant;

use super::domains::fintech::{
    perf_capture_baseline_observability, perf_capture_intraday_risk_probe,
    perf_capture_post_mutation_observability, perf_capture_trade_correction_probe,
    perf_correct_trade_correction, perf_emit_trade_correction_audit, perf_open_analysis_branch,
    perf_stress_intraday_risk, setup_intraday_risk_perf_world, setup_trade_correction_perf_world,
};
use super::performance_support::*;
use crate::capabilities::{DurabilityRead, DurabilityWrite};
use crate::facade::config::{AdjacencyBackend, RelationalRuntimeProfile};
use crate::facade::history::BranchId;
use crate::facade::indexes::{
    DerivedIndexBuildRequest, DerivedIndexDefinition, DerivedIndexId, DerivedIndexKind,
};
use crate::facade::inspection::{
    ConnectivityInspectionBudget, ConnectivityInspectionRequest, InspectionRecordClass,
    InspectionScope, KindInspectionRequest, RecentCommitInspectionRequest,
    StructuralIdentityQueryRequest,
};
use crate::facade::lineage::{
    HistoricalResolutionBoundednessBasis, HistoricalResolutionRequest, LineageDivergenceRequest,
    LineageDivergenceTraversalBasis,
};
use crate::facade::merge::{MergeExecutionRequest, MergeIntent};
use crate::facade::query::{
    DeterministicQueryPlanKey, IndexParityMode, PlannedQueryPacket, QueryAccessContract,
    QueryExecutionShape, QueryLocalityClass, QueryOrderingContract, QueryScope,
    ReductionDiscipline,
};
use crate::facade::replay::{RelationalReplayRequest, ReplayExecutionMode, ReplayVerificationMode};
use crate::facade::runtime::{CompiledArtifactAuthorityStatus, EntityRecordProjection};
use crate::facade::symbols::Symbol;
use crate::replay::data::digest_diagnostics_surface;
use crate::tests::support::*;
use crate::validation::data::{
    CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
    CustomInvariantOperationalMetadata, CustomInvariantPreparationError,
    CustomInvariantRegistration, CustomInvariantRule, CustomInvariantRuleId,
    CustomInvariantScopePlanner, CustomInvariantSemanticIdentity, CustomInvariantSemanticVersion,
    CustomInvariantVerdict, InvariantCostClass, InvariantExecutionPoint, InvariantFailureEffect,
    InvariantGroup, InvariantGroupSet,
};

mod artifact_recoverability_matrix;
mod bridge_runtime_support;
mod cad_topology_matrix;
mod chip_simulator_matrix;
mod commit_delta_matrix;
mod durability_append_matrix;
mod game_engine_matrix;
mod geometry_artifact_decomposition_matrix;
mod geometry_kernel_matrix;
mod hot_cold_path_matrix;
mod index_parity_matrix;
mod inspection_budget_matrix;
mod invariant_materialization_matrix;
mod invariant_support;
mod measurement_support;
mod merge_lineage_matrix;
mod mixed_load_matrix;
mod performance_harness_matrix;
mod profile_matrix;
mod query_packet_matrix;
mod recoverability_policy_matrix;
mod replay_recovery_matrix;
mod retention_reclaim_matrix;
mod rocketship_bulk_intents;
mod rocketship_layout;
mod rocketship_pseudorealistic;
mod runtime_bridge_mock_matrix;
mod snapshot_materialization_matrix;
mod sustained_load_matrix;
mod workflow_matrix;

use bridge_runtime_support::*;
use invariant_support::*;
use measurement_support::*;
use rocketship_bulk_intents::*;
use rocketship_layout::*;
use rocketship_pseudorealistic::*;

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_rocketship_scale_matrix() {
    let suite = "rocketship_scale_matrix";
    let node_count = rocketship_node_count();
    let query_target_count = rocketship_query_target_count(node_count);

    let zero_diagnostics_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_zero_diagnostics_narrow_round_trip",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            runtime.config.diagnostics.profile.detailed_traces_enabled = false;
            runtime.config.diagnostics.profile.max_entries_per_artifact = 0;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded = seed_rocketship_world(&mut runtime, node_count);

            runtime.performance_access().reset_counters();
            let target_index = seeded.entities.len() / 2;
            let hot_update_started_at = Instant::now();
            let update = update_entity(
                &mut runtime,
                seeded.entities[target_index],
                "rocket-node-hot-update",
            );
            let hot_update_micros = hot_update_started_at.elapsed().as_micros();
            let hot_phase_timing = update.execution.phase_timing.clone();
            let snapshot = runtime.visibility_authority().snapshot();
            let half_window = query_target_count / 2;
            let window_start = target_index.saturating_sub(half_window);
            let window_end = (window_start + query_target_count).min(seeded.entities.len());
            let targets = seeded.entities[window_start..window_end]
                .iter()
                .copied()
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let packet = explicit_query_packet(&runtime, &snapshot, "rocketship-explicit", targets);

            let hot_query_plan_started_at = Instant::now();
            let planned = runtime
                .read_truth()
                .plan_query_packet(&snapshot, packet)
                .expect("planned rocketship explicit query");
            let hot_query_planning_micros = hot_query_plan_started_at.elapsed().as_micros();
            let hot_query_execution_started_at = Instant::now();
            let outcome = runtime
                .read_truth()
                .execute_query_plan(planned)
                .expect("rocketship explicit query outcome");
            let hot_query_execution_micros = hot_query_execution_started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            let elapsed_micros = seeded.entity_commit_micros
                + seeded.relation_commit_micros
                + hot_update_micros
                + hot_query_planning_micros
                + hot_query_execution_micros;
            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "bootstrap_entity_commit_micros": seeded.entity_commit_micros,
                    "bootstrap_relation_commit_micros": seeded.relation_commit_micros,
                    "bootstrap_relation_phase_timing": {
                        "draft_preparation_micros": seeded.relation_commit_phase_timing.draft_preparation_micros,
                        "draft_bulk_admission_micros": seeded.relation_commit_phase_timing.draft_bulk_admission_micros,
                        "draft_merge_plan_micros": seeded.relation_commit_phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": seeded.relation_commit_phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": seeded.relation_commit_phase_timing.draft_working_state_clone_micros,
                        "invariant_pre_check_micros": seeded.relation_commit_phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": seeded.relation_commit_phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": seeded.relation_commit_phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": seeded.relation_commit_phase_timing.invariant_post_check_micros,
                        "artifact_assembly_micros": seeded.relation_commit_phase_timing.artifact_assembly_micros,
                        "durable_append_micros": seeded.relation_commit_phase_timing.durable_append_micros,
                        "publication_micros": seeded.relation_commit_phase_timing.publication_micros,
                        "publication_storage_commit_micros": seeded.relation_commit_phase_timing.publication_storage_commit_micros,
                    },
                    "hot_update_micros": hot_update_micros,
                    "hot_query_planning_micros": hot_query_planning_micros,
                    "hot_query_execution_micros": hot_query_execution_micros,
                    "phase_timing": {
                        "draft_preparation_micros": hot_phase_timing.draft_preparation_micros,
                        "draft_bulk_admission_micros": hot_phase_timing.draft_bulk_admission_micros,
                        "draft_merge_plan_micros": hot_phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": hot_phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": hot_phase_timing.draft_working_state_clone_micros,
                        "working_state_preparation_micros": hot_phase_timing.working_state_preparation_micros,
                        "invariant_pre_check_micros": hot_phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": hot_phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": hot_phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": hot_phase_timing.invariant_post_check_micros,
                        "artifact_assembly_micros": hot_phase_timing.artifact_assembly_micros,
                        "durable_append_micros": hot_phase_timing.durable_append_micros,
                        "publication_micros": hot_phase_timing.publication_micros,
                        "publication_storage_commit_micros": hot_phase_timing.publication_storage_commit_micros,
                        "publication_index_refresh_micros": hot_phase_timing.publication_index_refresh_micros,
                        "publication_history_publish_micros": hot_phase_timing.publication_history_publish_micros,
                        "publication_visibility_pin_micros": hot_phase_timing.publication_visibility_pin_micros,
                        "publication_bundle_publish_micros": hot_phase_timing.publication_bundle_publish_micros,
                        "publication_post_commit_consumer_micros": hot_phase_timing.publication_post_commit_consumer_micros,
                    },
                    "hot_changed_records": update.changed_records.len(),
                    "query_target_count": window_end - window_start,
                    "query_result_entities": outcome.result.entities.len(),
                    "query_result_relations": outcome.result.relations.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "profile_boundary": profile_boundary_metrics(
                        &runtime,
                        RelationalRuntimeProfile::GeometryKernel,
                    ),
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_zero_diagnostics_narrow_round_trip",
        &zero_diagnostics_samples,
        &[
            ("resident_node_count", &["resident_node_count"]),
            ("resident_relation_count", &["resident_relation_count"]),
            (
                "bootstrap_entity_commit_micros",
                &["bootstrap_entity_commit_micros"],
            ),
            (
                "bootstrap_relation_commit_micros",
                &["bootstrap_relation_commit_micros"],
            ),
            ("hot_update_micros", &["hot_update_micros"]),
            (
                "draft_preparation_micros",
                &["phase_timing", "draft_preparation_micros"],
            ),
            (
                "publication_storage_commit_micros",
                &["phase_timing", "publication_storage_commit_micros"],
            ),
            (
                "publication_index_refresh_micros",
                &["phase_timing", "publication_index_refresh_micros"],
            ),
            (
                "publication_history_publish_micros",
                &["phase_timing", "publication_history_publish_micros"],
            ),
            (
                "publication_visibility_pin_micros",
                &["phase_timing", "publication_visibility_pin_micros"],
            ),
            (
                "publication_bundle_publish_micros",
                &["phase_timing", "publication_bundle_publish_micros"],
            ),
            (
                "publication_post_commit_consumer_micros",
                &["phase_timing", "publication_post_commit_consumer_micros"],
            ),
            ("hot_query_planning_micros", &["hot_query_planning_micros"]),
            (
                "hot_query_execution_micros",
                &["hot_query_execution_micros"],
            ),
            ("query_target_count", &["query_target_count"]),
            ("query_result_entities", &["query_result_entities"]),
            ("diagnostic_artifact_count", &["diagnostic_artifact_count"]),
            ("detailed_trace_entries", &["detailed_trace_entries"]),
            (
                "profile_execution_lane_code",
                &["profile_boundary", "execution_lane_code"],
            ),
            (
                "profile_diagnostics_boundary_code",
                &["profile_boundary", "diagnostics_boundary_code"],
            ),
            (
                "profile_matches_defaults",
                &["profile_boundary", "matches_defaults"],
            ),
        ],
    );
    assert!(zero_diagnostics_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &zero_diagnostics_samples,
        "rocketship zero-diagnostics should preserve a 100k-node resident world while keeping the hot path narrow and clone-free",
        |metrics| {
            let resident_node_count = metrics["resident_node_count"].as_u64().unwrap_or(0) as usize;
            let resident_relation_count =
                metrics["resident_relation_count"].as_u64().unwrap_or(0) as usize;
            let query_target_count = metrics["query_target_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && resident_relation_count >= resident_node_count.saturating_sub(1)
                && metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["query_result_entities"].as_u64() == Some(query_target_count)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == 1
                && counter_u64(metrics, "query_packet_count") <= 8
                && counter_u64(metrics, "query_scope_unit_count") <= query_target_count
        },
    );

    let rich_geometry_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_geometry_profile_narrow_round_trip",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryRichCertification,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded = seed_rocketship_world(&mut runtime, node_count);

            runtime.performance_access().reset_counters();
            let target_index = seeded.entities.len() / 2;
            let hot_update_started_at = Instant::now();
            let update = update_entity(
                &mut runtime,
                seeded.entities[target_index],
                "rocket-node-hot-update-rich",
            );
            let hot_update_micros = hot_update_started_at.elapsed().as_micros();
            let hot_phase_timing = update.execution.phase_timing.clone();
            let snapshot = runtime.visibility_authority().snapshot();
            let half_window = query_target_count / 2;
            let window_start = target_index.saturating_sub(half_window);
            let window_end = (window_start + query_target_count).min(seeded.entities.len());
            let targets = seeded.entities[window_start..window_end]
                .iter()
                .copied()
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let packet =
                explicit_query_packet(&runtime, &snapshot, "rocketship-explicit-rich", targets);

            let hot_query_plan_started_at = Instant::now();
            let planned = runtime
                .read_truth()
                .plan_query_packet(&snapshot, packet)
                .expect("planned rocketship explicit rich query");
            let hot_query_planning_micros = hot_query_plan_started_at.elapsed().as_micros();
            let hot_query_execution_started_at = Instant::now();
            let outcome = runtime
                .read_truth()
                .execute_query_plan(planned)
                .expect("rocketship explicit rich query outcome");
            let hot_query_execution_micros = hot_query_execution_started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            let elapsed_micros = seeded.entity_commit_micros
                + seeded.relation_commit_micros
                + hot_update_micros
                + hot_query_planning_micros
                + hot_query_execution_micros;
            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "bootstrap_entity_commit_micros": seeded.entity_commit_micros,
                    "bootstrap_relation_commit_micros": seeded.relation_commit_micros,
                    "hot_update_micros": hot_update_micros,
                    "hot_query_planning_micros": hot_query_planning_micros,
                    "hot_query_execution_micros": hot_query_execution_micros,
                    "phase_timing": {
                        "draft_preparation_micros": hot_phase_timing.draft_preparation_micros,
                        "draft_bulk_admission_micros": hot_phase_timing.draft_bulk_admission_micros,
                        "draft_merge_plan_micros": hot_phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": hot_phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": hot_phase_timing.draft_working_state_clone_micros,
                        "working_state_preparation_micros": hot_phase_timing.working_state_preparation_micros,
                        "invariant_pre_check_micros": hot_phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": hot_phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": hot_phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": hot_phase_timing.invariant_post_check_micros,
                        "artifact_assembly_micros": hot_phase_timing.artifact_assembly_micros,
                        "durable_append_micros": hot_phase_timing.durable_append_micros,
                        "publication_micros": hot_phase_timing.publication_micros,
                        "publication_storage_commit_micros": hot_phase_timing.publication_storage_commit_micros,
                        "publication_index_refresh_micros": hot_phase_timing.publication_index_refresh_micros,
                        "publication_history_publish_micros": hot_phase_timing.publication_history_publish_micros,
                        "publication_visibility_pin_micros": hot_phase_timing.publication_visibility_pin_micros,
                        "publication_bundle_publish_micros": hot_phase_timing.publication_bundle_publish_micros,
                        "publication_post_commit_consumer_micros": hot_phase_timing.publication_post_commit_consumer_micros,
                    },
                    "hot_changed_records": update.changed_records.len(),
                    "query_target_count": window_end - window_start,
                    "query_result_entities": outcome.result.entities.len(),
                    "query_result_relations": outcome.result.relations.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "profile_boundary": profile_boundary_metrics(
                        &runtime,
                        RelationalRuntimeProfile::GeometryKernel,
                    ),
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_geometry_profile_narrow_round_trip",
        &rich_geometry_samples,
        &[
            ("resident_node_count", &["resident_node_count"]),
            ("resident_relation_count", &["resident_relation_count"]),
            (
                "bootstrap_entity_commit_micros",
                &["bootstrap_entity_commit_micros"],
            ),
            (
                "bootstrap_relation_commit_micros",
                &["bootstrap_relation_commit_micros"],
            ),
            ("hot_update_micros", &["hot_update_micros"]),
            (
                "publication_storage_commit_micros",
                &["phase_timing", "publication_storage_commit_micros"],
            ),
            (
                "publication_index_refresh_micros",
                &["phase_timing", "publication_index_refresh_micros"],
            ),
            (
                "publication_history_publish_micros",
                &["phase_timing", "publication_history_publish_micros"],
            ),
            (
                "publication_visibility_pin_micros",
                &["phase_timing", "publication_visibility_pin_micros"],
            ),
            (
                "publication_bundle_publish_micros",
                &["phase_timing", "publication_bundle_publish_micros"],
            ),
            (
                "publication_post_commit_consumer_micros",
                &["phase_timing", "publication_post_commit_consumer_micros"],
            ),
            ("hot_query_planning_micros", &["hot_query_planning_micros"]),
            (
                "hot_query_execution_micros",
                &["hot_query_execution_micros"],
            ),
            ("query_target_count", &["query_target_count"]),
            ("query_result_entities", &["query_result_entities"]),
            ("diagnostic_artifact_count", &["diagnostic_artifact_count"]),
            ("detailed_trace_entries", &["detailed_trace_entries"]),
            (
                "profile_execution_lane_code",
                &["profile_boundary", "execution_lane_code"],
            ),
            (
                "profile_diagnostics_boundary_code",
                &["profile_boundary", "diagnostics_boundary_code"],
            ),
            (
                "profile_matches_defaults",
                &["profile_boundary", "matches_defaults"],
            ),
        ],
    );
    assert!(rich_geometry_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &rich_geometry_samples,
        "rocketship geometry-profile diagnostics should preserve the same 100k-node hot-path truth while deferring hot detailed traces",
        |metrics| {
            let resident_node_count = metrics["resident_node_count"].as_u64().unwrap_or(0) as usize;
            let resident_relation_count =
                metrics["resident_relation_count"].as_u64().unwrap_or(0) as usize;
            let query_target_count = metrics["query_target_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && resident_relation_count >= resident_node_count.saturating_sub(1)
                && metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["query_result_entities"].as_u64() == Some(query_target_count)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(1)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == 1
                && counter_u64(metrics, "query_packet_count") <= 8
                && counter_u64(metrics, "query_scope_unit_count") <= query_target_count
        },
    );

    let pseudorealistic_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_pseudorealistic_subsystem_round_trip",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded =
                seed_pseudorealistic_rocketship_world(&mut runtime, node_count, query_target_count);

            runtime.performance_access().reset_counters();
            let hot_update_started_at = Instant::now();
            let update = update_entity(
                &mut runtime,
                seeded.hot_update_target,
                "rocket.engine_cluster.hot_patch",
            );
            let hot_update_micros = hot_update_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "rocketship-pseudorealistic-explicit",
                seeded.mixed_query_targets.clone(),
            );
            let explicit_plan_started_at = Instant::now();
            let explicit_plan = runtime
                .read_truth()
                .plan_query_packet(&snapshot, explicit_packet)
                .expect("planned pseudorealistic explicit query");
            let explicit_query_planning_micros = explicit_plan_started_at.elapsed().as_micros();
            let explicit_execution_started_at = Instant::now();
            let explicit_outcome = runtime
                .read_truth()
                .execute_query_plan(explicit_plan)
                .expect("pseudorealistic explicit query outcome");
            let explicit_query_execution_micros =
                explicit_execution_started_at.elapsed().as_micros();

            let traversal_context = runtime
                .read_truth()
                .query_plan_context(&snapshot)
                .expect("pseudorealistic traversal context");
            let traversal_packet = PlannedQueryPacket {
                label: "rocketship-pseudorealistic-traversal".to_string(),
                context_id: traversal_context,
                scope: QueryScope::ConnectivityTraversal {
                    seeds: Arc::from(seeded.traversal_seeds.clone()),
                    relation_kind_scope: Some(Arc::from([KindId(2)])),
                    max_depth: Some(2),
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalTraversalOrder,
                access_contract: QueryAccessContract::AuthoritativeStorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(91_001),
                target_count_hint: seeded.traversal_seeds.len(),
            };
            let traversal_plan_started_at = Instant::now();
            let traversal_plan = runtime
                .read_truth()
                .plan_query_packet(&snapshot, traversal_packet)
                .expect("planned pseudorealistic traversal query");
            let traversal_planning_micros = traversal_plan_started_at.elapsed().as_micros();
            let traversal_execution_started_at = Instant::now();
            let traversal_outcome = runtime
                .read_truth()
                .execute_query_plan(traversal_plan)
                .expect("pseudorealistic traversal outcome");
            let traversal_execution_micros = traversal_execution_started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            let elapsed_micros = seeded.entity_commit_micros
                + seeded.relation_commit_micros
                + hot_update_micros
                + explicit_query_planning_micros
                + explicit_query_execution_micros
                + traversal_planning_micros
                + traversal_execution_micros;
            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "subsystem_count": seeded.subsystem_count,
                    "bootstrap_entity_commit_micros": seeded.entity_commit_micros,
                    "bootstrap_relation_commit_micros": seeded.relation_commit_micros,
                    "bootstrap_relation_phase_timing": {
                        "draft_preparation_micros": seeded.relation_commit_phase_timing.draft_preparation_micros,
                        "draft_bulk_admission_micros": seeded.relation_commit_phase_timing.draft_bulk_admission_micros,
                        "draft_merge_plan_micros": seeded.relation_commit_phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": seeded.relation_commit_phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": seeded.relation_commit_phase_timing.draft_working_state_clone_micros,
                        "invariant_pre_check_micros": seeded.relation_commit_phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": seeded.relation_commit_phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": seeded.relation_commit_phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": seeded.relation_commit_phase_timing.invariant_post_check_micros,
                        "artifact_assembly_micros": seeded.relation_commit_phase_timing.artifact_assembly_micros,
                        "durable_append_micros": seeded.relation_commit_phase_timing.durable_append_micros,
                        "publication_micros": seeded.relation_commit_phase_timing.publication_micros,
                        "publication_storage_commit_micros": seeded.relation_commit_phase_timing.publication_storage_commit_micros,
                    },
                    "hot_update_micros": hot_update_micros,
                    "explicit_query_planning_micros": explicit_query_planning_micros,
                    "explicit_query_execution_micros": explicit_query_execution_micros,
                    "traversal_planning_micros": traversal_planning_micros,
                    "traversal_execution_micros": traversal_execution_micros,
                    "hot_changed_records": update.changed_records.len(),
                    "mixed_query_target_count": seeded.mixed_query_targets.len(),
                    "explicit_query_result_entities": explicit_outcome.result.entities.len(),
                    "traversal_seed_count": seeded.traversal_seeds.len(),
                    "traversal_result_entities": traversal_outcome.result.entities.len(),
                    "traversal_result_relations": traversal_outcome.result.relations.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "profile_boundary": profile_boundary_metrics(
                        &runtime,
                        RelationalRuntimeProfile::GeometryKernel,
                    ),
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_pseudorealistic_subsystem_round_trip",
        &pseudorealistic_samples,
        &[
            ("resident_node_count", &["resident_node_count"]),
            ("resident_relation_count", &["resident_relation_count"]),
            ("subsystem_count", &["subsystem_count"]),
            (
                "bootstrap_entity_commit_micros",
                &["bootstrap_entity_commit_micros"],
            ),
            (
                "bootstrap_relation_commit_micros",
                &["bootstrap_relation_commit_micros"],
            ),
            ("hot_update_micros", &["hot_update_micros"]),
            (
                "explicit_query_planning_micros",
                &["explicit_query_planning_micros"],
            ),
            (
                "explicit_query_execution_micros",
                &["explicit_query_execution_micros"],
            ),
            ("traversal_planning_micros", &["traversal_planning_micros"]),
            (
                "traversal_execution_micros",
                &["traversal_execution_micros"],
            ),
            ("mixed_query_target_count", &["mixed_query_target_count"]),
            (
                "explicit_query_result_entities",
                &["explicit_query_result_entities"],
            ),
            ("traversal_seed_count", &["traversal_seed_count"]),
            ("traversal_result_entities", &["traversal_result_entities"]),
            (
                "traversal_result_relations",
                &["traversal_result_relations"],
            ),
            (
                "profile_execution_lane_code",
                &["profile_boundary", "execution_lane_code"],
            ),
            (
                "profile_diagnostics_boundary_code",
                &["profile_boundary", "diagnostics_boundary_code"],
            ),
            (
                "profile_matches_defaults",
                &["profile_boundary", "matches_defaults"],
            ),
        ],
    );
    assert!(pseudorealistic_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &pseudorealistic_samples,
        "pseudorealistic rocketship should preserve mixed subsystem truth, narrow hot updates, and bounded mixed-locality query work",
        |metrics| {
            let mixed_query_target_count =
                metrics["mixed_query_target_count"].as_u64().unwrap_or(0);
            let traversal_seed_count = metrics["traversal_seed_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0) >= node_count as u64
                && metrics["subsystem_count"].as_u64() == Some(12)
                && metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["explicit_query_result_entities"].as_u64()
                    == Some(mixed_query_target_count)
                && metrics["traversal_result_entities"].as_u64().unwrap_or(0)
                    >= traversal_seed_count
                && metrics["traversal_result_relations"].as_u64().unwrap_or(0) >= 1
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == 1
                && counter_u64(metrics, "query_packet_count")
                    <= mixed_query_target_count + traversal_seed_count
                && counter_u64(metrics, "query_scope_unit_count")
                    <= mixed_query_target_count + traversal_seed_count
        },
    );

    let propagation_wave_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_pseudorealistic_propagation_wave",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded =
                seed_pseudorealistic_rocketship_world(&mut runtime, node_count, query_target_count);

            runtime.performance_access().reset_counters();
            let hot_update_started_at = Instant::now();
            let update = update_entity(
                &mut runtime,
                seeded.hot_update_target,
                "rocket.plumbing_and_feed.propagation_patch",
            );
            let hot_update_micros = hot_update_started_at.elapsed().as_micros();
            let hot_phase_timing = update.execution.phase_timing.clone();

            let snapshot = runtime.visibility_authority().snapshot();
            let context = runtime
                .read_truth()
                .query_plan_context(&snapshot)
                .expect("rocketship propagation context");
            let propagation_seeds = vec![
                seeded.traversal_seeds[0],
                seeded.traversal_seeds[1],
                seeded.traversal_seeds[9],
                seeded.traversal_seeds[10],
            ];
            let propagation_packet = PlannedQueryPacket {
                label: "rocketship-pseudorealistic-propagation".to_string(),
                context_id: context,
                scope: QueryScope::ConnectivityTraversal {
                    seeds: Arc::from(propagation_seeds.clone()),
                    relation_kind_scope: Some(Arc::from([KindId(2)])),
                    max_depth: Some(3),
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalTraversalOrder,
                access_contract: QueryAccessContract::AuthoritativeStorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(91_002),
                target_count_hint: propagation_seeds.len(),
            };
            let propagation_plan_started_at = Instant::now();
            let propagation_plan = runtime
                .read_truth()
                .plan_query_packet(&snapshot, propagation_packet)
                .expect("planned rocketship propagation query");
            let propagation_planning_micros = propagation_plan_started_at.elapsed().as_micros();
            let propagation_execution_started_at = Instant::now();
            let propagation_outcome = runtime
                .read_truth()
                .execute_query_plan(propagation_plan)
                .expect("rocketship propagation outcome");
            let propagation_execution_micros =
                propagation_execution_started_at.elapsed().as_micros();

            let explicit_targets = seeded
                .mixed_query_targets
                .iter()
                .take(12)
                .cloned()
                .collect::<Vec<_>>();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "rocketship-pseudorealistic-propagation-explicit",
                explicit_targets.clone(),
            );
            let explicit_started_at = Instant::now();
            let explicit_outcome = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("planned rocketship propagation explicit query"),
                )
                .expect("rocketship propagation explicit query outcome");
            let explicit_query_micros = explicit_started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            let elapsed_micros = seeded.entity_commit_micros
                + seeded.relation_commit_micros
                + hot_update_micros
                + propagation_planning_micros
                + propagation_execution_micros
                + explicit_query_micros;
            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "subsystem_count": seeded.subsystem_count,
                    "bootstrap_entity_commit_micros": seeded.entity_commit_micros,
                    "bootstrap_relation_commit_micros": seeded.relation_commit_micros,
                    "hot_update_micros": hot_update_micros,
                    "phase_timing": {
                        "draft_preparation_micros": hot_phase_timing.draft_preparation_micros,
                        "draft_bulk_admission_micros": hot_phase_timing.draft_bulk_admission_micros,
                        "draft_merge_plan_micros": hot_phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": hot_phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": hot_phase_timing.draft_working_state_clone_micros,
                        "working_state_preparation_micros": hot_phase_timing.working_state_preparation_micros,
                        "invariant_pre_check_micros": hot_phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": hot_phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": hot_phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": hot_phase_timing.invariant_post_check_micros,
                        "artifact_assembly_micros": hot_phase_timing.artifact_assembly_micros,
                        "durable_append_micros": hot_phase_timing.durable_append_micros,
                        "publication_micros": hot_phase_timing.publication_micros,
                        "publication_storage_commit_micros": hot_phase_timing.publication_storage_commit_micros,
                        "publication_index_refresh_micros": hot_phase_timing.publication_index_refresh_micros,
                        "publication_history_publish_micros": hot_phase_timing.publication_history_publish_micros,
                        "publication_visibility_pin_micros": hot_phase_timing.publication_visibility_pin_micros,
                        "publication_bundle_publish_micros": hot_phase_timing.publication_bundle_publish_micros,
                        "publication_post_commit_consumer_micros": hot_phase_timing.publication_post_commit_consumer_micros,
                    },
                    "propagation_planning_micros": propagation_planning_micros,
                    "propagation_execution_micros": propagation_execution_micros,
                    "explicit_query_micros": explicit_query_micros,
                    "hot_changed_records": update.changed_records.len(),
                    "propagation_seed_count": propagation_seeds.len(),
                    "propagation_result_entities": propagation_outcome.result.entities.len(),
                    "propagation_result_relations": propagation_outcome.result.relations.len(),
                    "explicit_target_count": explicit_targets.len(),
                    "explicit_result_entities": explicit_outcome.result.entities.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_pseudorealistic_propagation_wave",
        &propagation_wave_samples,
        &[
            ("resident_node_count", &["resident_node_count"]),
            ("resident_relation_count", &["resident_relation_count"]),
            ("subsystem_count", &["subsystem_count"]),
            (
                "bootstrap_entity_commit_micros",
                &["bootstrap_entity_commit_micros"],
            ),
            (
                "bootstrap_relation_commit_micros",
                &["bootstrap_relation_commit_micros"],
            ),
            ("hot_update_micros", &["hot_update_micros"]),
            (
                "draft_preparation_micros",
                &["phase_timing", "draft_preparation_micros"],
            ),
            (
                "publication_storage_commit_micros",
                &["phase_timing", "publication_storage_commit_micros"],
            ),
            (
                "publication_index_refresh_micros",
                &["phase_timing", "publication_index_refresh_micros"],
            ),
            (
                "publication_history_publish_micros",
                &["phase_timing", "publication_history_publish_micros"],
            ),
            (
                "publication_visibility_pin_micros",
                &["phase_timing", "publication_visibility_pin_micros"],
            ),
            (
                "publication_bundle_publish_micros",
                &["phase_timing", "publication_bundle_publish_micros"],
            ),
            (
                "publication_post_commit_consumer_micros",
                &["phase_timing", "publication_post_commit_consumer_micros"],
            ),
            (
                "propagation_planning_micros",
                &["propagation_planning_micros"],
            ),
            (
                "propagation_execution_micros",
                &["propagation_execution_micros"],
            ),
            ("explicit_query_micros", &["explicit_query_micros"]),
            ("propagation_seed_count", &["propagation_seed_count"]),
            (
                "propagation_result_entities",
                &["propagation_result_entities"],
            ),
            (
                "propagation_result_relations",
                &["propagation_result_relations"],
            ),
            ("explicit_target_count", &["explicit_target_count"]),
            ("explicit_result_entities", &["explicit_result_entities"]),
        ],
    );
    assert!(propagation_wave_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &propagation_wave_samples,
        "pseudorealistic rocketship propagation waves should stay bounded while spanning multiple subsystem interfaces",
        |metrics| {
            let propagation_seed_count =
                metrics["propagation_seed_count"].as_u64().unwrap_or(0);
            let explicit_target_count =
                metrics["explicit_target_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0) >= node_count as u64
                && metrics["subsystem_count"].as_u64() == Some(12)
                && metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["propagation_result_entities"].as_u64().unwrap_or(0)
                    >= propagation_seed_count
                && metrics["propagation_result_relations"].as_u64().unwrap_or(0) >= 1
                && metrics["explicit_result_entities"].as_u64() == Some(explicit_target_count)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == 1
                && counter_u64(metrics, "query_packet_count") <= 32
                && counter_u64(metrics, "query_scope_unit_count")
                    <= propagation_seed_count + explicit_target_count
        },
    );

    let flat_batch_wave_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_pseudorealistic_flat_entity_batch_wave",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded =
                seed_pseudorealistic_rocketship_world(&mut runtime, node_count, query_target_count);

            let mut partition_targets = BTreeMap::new();
            for entity in &seeded.entities {
                let targets = partition_targets
                    .entry(entity.partition_id)
                    .or_insert_with(Vec::new);
                if targets.len() < 8 {
                    targets.push(*entity);
                }
                if partition_targets.len() >= 8
                    && partition_targets.values().all(|targets| targets.len() >= 8)
                {
                    break;
                }
            }
            let batch_targets = partition_targets
                .values()
                .flat_map(|targets| targets.iter().take(8).copied())
                .collect::<Vec<_>>();
            assert!(
                batch_targets.len() >= 64,
                "rocketship flat batch wave should gather a broad multi-partition entity batch"
            );

            runtime.performance_access().reset_counters();
            let update_started_at = Instant::now();
            let update = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                let mut batch = WorkerIntentBatch::new("rocketship-flat-entity-batch-wave");
                for (index, entity) in batch_targets.iter().enumerate() {
                    batch = batch.push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                        UpdateEntityFieldsIntent {
                            entity_id: *entity,
                            fields: crate::tests::support::aspect_field_patch_from_values([
                                (
                                    crate::tests::support::aspect_key("section"),
                                    crate::tests::support::field_key("section"),
                                    crate::tests::support::string_aspect_value("batch-wave"),
                                ),
                                (
                                    crate::tests::support::aspect_key("tag"),
                                    crate::tests::support::field_key("tag"),
                                    crate::tests::support::string_aspect_value(&format!(
                                        "rocket.batch.{index}"
                                    )),
                                ),
                                (
                                    crate::tests::support::aspect_key("partition"),
                                    crate::tests::support::field_key("partition"),
                                    crate::tests::support::u64_aspect_value(
                                        entity.partition_id.0 as u64,
                                    ),
                                ),
                            ]),
                        },
                    )));
                }
                txn.push_batch(batch);
                txn.commit()
                    .expect("rocketship flat entity batch wave commit")
            };
            let update_micros = update_started_at.elapsed().as_micros();
            let phase_timing = update.execution.phase_timing.clone();

            let snapshot = runtime.visibility_authority().snapshot();
            let explicit_targets = batch_targets
                .iter()
                .take(16)
                .copied()
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "rocketship-flat-entity-batch-explicit",
                explicit_targets,
            );
            let explicit_started_at = Instant::now();
            let explicit = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("planned rocketship flat batch explicit query"),
                )
                .expect("rocketship flat batch explicit outcome");
            let explicit_query_micros = explicit_started_at.elapsed().as_micros();
            assert!(runtime.visibility_authority().release_snapshot(&snapshot));

            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            measurement_with_elapsed(update_micros + explicit_query_micros, || {
                perf_metrics!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "subsystem_count": seeded.subsystem_count,
                    "batch_target_count": batch_targets.len(),
                    "batch_partition_count": partition_targets.len(),
                    "update_micros": update_micros,
                    "explicit_query_micros": explicit_query_micros,
                    "hot_changed_records": update.changed_records.len(),
                    "explicit_result_entities": explicit.result.entities.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "phase_timing": {
                        "draft_preparation_micros": phase_timing.draft_preparation_micros,
                        "draft_intent_normalization_micros": phase_timing.draft_intent_normalization_micros,
                        "draft_merge_plan_micros": phase_timing.draft_merge_plan_micros,
                        "draft_intent_validation_micros": phase_timing.draft_intent_validation_micros,
                        "draft_intent_sort_micros": phase_timing.draft_intent_sort_micros,
                        "draft_conflict_detection_micros": phase_timing.draft_conflict_detection_micros,
                        "draft_structural_summary_micros": phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": phase_timing.draft_working_state_clone_micros,
                        "invariant_pre_check_micros": phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": phase_timing.invariant_post_check_micros,
                        "durable_append_micros": phase_timing.durable_append_micros,
                        "publication_micros": phase_timing.publication_micros,
                        "publication_storage_commit_micros": phase_timing.publication_storage_commit_micros,
                    },
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_pseudorealistic_flat_entity_batch_wave",
        &flat_batch_wave_samples,
        &[
            ("batch_target_count", &["batch_target_count"]),
            ("batch_partition_count", &["batch_partition_count"]),
            ("update_micros", &["update_micros"]),
            (
                "draft_preparation_micros",
                &["phase_timing", "draft_preparation_micros"],
            ),
            (
                "draft_intent_normalization_micros",
                &["phase_timing", "draft_intent_normalization_micros"],
            ),
            (
                "draft_merge_plan_micros",
                &["phase_timing", "draft_merge_plan_micros"],
            ),
            (
                "draft_intent_validation_micros",
                &["phase_timing", "draft_intent_validation_micros"],
            ),
            (
                "draft_intent_sort_micros",
                &["phase_timing", "draft_intent_sort_micros"],
            ),
            (
                "draft_conflict_detection_micros",
                &["phase_timing", "draft_conflict_detection_micros"],
            ),
            (
                "draft_structural_summary_micros",
                &["phase_timing", "draft_structural_summary_micros"],
            ),
            (
                "draft_working_state_clone_micros",
                &["phase_timing", "draft_working_state_clone_micros"],
            ),
            (
                "invariant_pre_check_micros",
                &["phase_timing", "invariant_pre_check_micros"],
            ),
            (
                "authoritative_mutation_micros",
                &["phase_timing", "authoritative_mutation_micros"],
            ),
            (
                "history_resolution_micros",
                &["phase_timing", "history_resolution_micros"],
            ),
            (
                "invariant_post_check_micros",
                &["phase_timing", "invariant_post_check_micros"],
            ),
            (
                "durable_append_micros",
                &["phase_timing", "durable_append_micros"],
            ),
            (
                "publication_storage_commit_micros",
                &["phase_timing", "publication_storage_commit_micros"],
            ),
            ("explicit_query_micros", &["explicit_query_micros"]),
            (
                "aosoa_entity_chunk_slots_materialized",
                &["counters", "aosoa_entity_chunk_slots_materialized"],
            ),
            (
                "aosoa_entity_chunks_published",
                &["counters", "aosoa_entity_chunks_published"],
            ),
        ],
    );
    assert!(flat_batch_wave_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &flat_batch_wave_samples,
        "pseudorealistic rocketship flat entity batches should stay on the widened sparse AoSoA path across multiple touched partitions",
        |metrics| {
            let batch_target_count = metrics["batch_target_count"].as_u64().unwrap_or(0);
            let batch_partition_count = metrics["batch_partition_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0) >= node_count as u64
                && metrics["subsystem_count"].as_u64() == Some(12)
                && batch_target_count >= 64
                && batch_partition_count >= 8
                && metrics["hot_changed_records"].as_u64() == Some(batch_target_count)
                && metrics["explicit_result_entities"].as_u64() == Some(16)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == batch_target_count
                && counter_u64(metrics, "partitions_touched_by_commit") >= batch_partition_count
                && counter_u64(metrics, "aosoa_entity_chunk_slots_materialized")
                    == batch_target_count
                && counter_u64(metrics, "aosoa_entity_chunks_published") >= batch_partition_count
                && counter_u64(metrics, "aosoa_publish_soa_merge_count") == 0
        },
    );

    let varied_flat_batch_wave_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_pseudorealistic_varied_locality_batch_wave",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded =
                seed_pseudorealistic_rocketship_world(&mut runtime, node_count, query_target_count);

            let mut selected_partitions = Vec::new();
            for entity in seeded.entities.iter().step_by(257) {
                if !selected_partitions.contains(&entity.partition_id) {
                    selected_partitions.push(entity.partition_id);
                }
                if selected_partitions.len() >= 8 {
                    break;
                }
            }
            assert!(
                selected_partitions.len() >= 8,
                "rocketship varied batch wave should discover a bounded multi-partition spread"
            );

            let mut partition_targets = BTreeMap::new();
            for entity in seeded.entities.iter() {
                if !selected_partitions.contains(&entity.partition_id) {
                    continue;
                }
                let targets = partition_targets
                    .entry(entity.partition_id)
                    .or_insert_with(Vec::new);
                if targets.len() < 8 {
                    targets.push(*entity);
                }
                if partition_targets.len() == selected_partitions.len()
                    && partition_targets.values().all(|targets| targets.len() >= 8)
                {
                    break;
                }
            }
            let batch_targets = partition_targets
                .values()
                .flat_map(|targets| targets.iter().take(8).copied())
                .collect::<Vec<_>>();
            assert!(
                batch_targets.len() >= 64,
                "rocketship varied batch wave should gather a varied-locality entity batch within a bounded partition spread"
            );

            runtime.performance_access().reset_counters();
            let update_started_at = Instant::now();
            let update = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                let mut batch = WorkerIntentBatch::new("rocketship-varied-locality-batch-wave");
                for (index, entity) in batch_targets.iter().enumerate() {
                    batch = batch.push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                        UpdateEntityFieldsIntent {
                            entity_id: *entity,
                            fields: crate::tests::support::aspect_field_patch_from_values([
                                (
                                    crate::tests::support::aspect_key("section"),
                                    crate::tests::support::field_key("section"),
                                    crate::tests::support::string_aspect_value("varied-batch-wave"),
                                ),
                                (
                                    crate::tests::support::aspect_key("tag"),
                                    crate::tests::support::field_key("tag"),
                                    crate::tests::support::string_aspect_value(&format!(
                                        "rocket.varied.{index}"
                                    )),
                                ),
                                (
                                    crate::tests::support::aspect_key("partition"),
                                    crate::tests::support::field_key("partition"),
                                    crate::tests::support::u64_aspect_value(
                                        entity.partition_id.0 as u64,
                                    ),
                                ),
                            ]),
                        },
                    )));
                }
                txn.push_batch(batch);
                txn.commit()
                    .expect("rocketship varied locality batch wave commit")
            };
            let update_micros = update_started_at.elapsed().as_micros();
            let phase_timing = update.execution.phase_timing.clone();

            let snapshot = runtime.visibility_authority().snapshot();
            let explicit_targets = batch_targets
                .iter()
                .step_by(3)
                .take(16)
                .copied()
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "rocketship-varied-batch-explicit",
                explicit_targets,
            );
            let explicit_started_at = Instant::now();
            let explicit = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("planned rocketship varied batch explicit query"),
                )
                .expect("rocketship varied batch explicit outcome");
            let explicit_query_micros = explicit_started_at.elapsed().as_micros();
            assert!(runtime.visibility_authority().release_snapshot(&snapshot));

            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            measurement_with_elapsed(update_micros + explicit_query_micros, || {
                perf_metrics!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "subsystem_count": seeded.subsystem_count,
                    "batch_target_count": batch_targets.len(),
                    "batch_partition_count": partition_targets.len(),
                    "update_micros": update_micros,
                    "explicit_query_micros": explicit_query_micros,
                    "hot_changed_records": update.changed_records.len(),
                    "explicit_result_entities": explicit.result.entities.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "phase_timing": {
                        "draft_preparation_micros": phase_timing.draft_preparation_micros,
                        "draft_merge_plan_micros": phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": phase_timing.draft_working_state_clone_micros,
                        "invariant_pre_check_micros": phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": phase_timing.invariant_post_check_micros,
                        "durable_append_micros": phase_timing.durable_append_micros,
                        "publication_micros": phase_timing.publication_micros,
                        "publication_storage_commit_micros": phase_timing.publication_storage_commit_micros,
                    },
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_pseudorealistic_varied_locality_batch_wave",
        &varied_flat_batch_wave_samples,
        &[
            ("batch_target_count", &["batch_target_count"]),
            ("batch_partition_count", &["batch_partition_count"]),
            ("update_micros", &["update_micros"]),
            (
                "draft_preparation_micros",
                &["phase_timing", "draft_preparation_micros"],
            ),
            (
                "draft_merge_plan_micros",
                &["phase_timing", "draft_merge_plan_micros"],
            ),
            (
                "draft_structural_summary_micros",
                &["phase_timing", "draft_structural_summary_micros"],
            ),
            (
                "draft_working_state_clone_micros",
                &["phase_timing", "draft_working_state_clone_micros"],
            ),
            (
                "invariant_pre_check_micros",
                &["phase_timing", "invariant_pre_check_micros"],
            ),
            (
                "authoritative_mutation_micros",
                &["phase_timing", "authoritative_mutation_micros"],
            ),
            (
                "history_resolution_micros",
                &["phase_timing", "history_resolution_micros"],
            ),
            (
                "invariant_post_check_micros",
                &["phase_timing", "invariant_post_check_micros"],
            ),
            (
                "durable_append_micros",
                &["phase_timing", "durable_append_micros"],
            ),
            (
                "publication_storage_commit_micros",
                &["phase_timing", "publication_storage_commit_micros"],
            ),
            ("explicit_query_micros", &["explicit_query_micros"]),
            (
                "aosoa_entity_chunk_slots_materialized",
                &["counters", "aosoa_entity_chunk_slots_materialized"],
            ),
            (
                "aosoa_entity_chunks_published",
                &["counters", "aosoa_entity_chunks_published"],
            ),
        ],
    );
    assert!(varied_flat_batch_wave_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &varied_flat_batch_wave_samples,
        "pseudorealistic rocketship varied-locality batches should stay on the widened sparse AoSoA path across broader partition spread",
        |metrics| {
            let batch_target_count = metrics["batch_target_count"].as_u64().unwrap_or(0);
            let batch_partition_count = metrics["batch_partition_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0) >= node_count as u64
                && metrics["subsystem_count"].as_u64() == Some(12)
                && batch_target_count >= 64
                && batch_partition_count == 8
                && metrics["hot_changed_records"].as_u64() == Some(batch_target_count)
                && metrics["explicit_result_entities"].as_u64() == Some(16)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == batch_target_count
                && counter_u64(metrics, "partitions_touched_by_commit") >= batch_partition_count
                && counter_u64(metrics, "aosoa_entity_chunk_slots_materialized")
                    == batch_target_count
                && counter_u64(metrics, "aosoa_entity_chunks_published") >= batch_partition_count
                && counter_u64(metrics, "aosoa_publish_soa_merge_count") == 0
        },
    );

    let larger_flat_batch_wave_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_pseudorealistic_large_flat_entity_batch_wave",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded =
                seed_pseudorealistic_rocketship_world(&mut runtime, node_count, query_target_count);

            let mut partition_targets = BTreeMap::new();
            for entity in &seeded.entities {
                let targets = partition_targets
                    .entry(entity.partition_id)
                    .or_insert_with(Vec::new);
                if targets.len() < 16 {
                    targets.push(*entity);
                }
                if partition_targets.len() >= 8
                    && partition_targets
                        .values()
                        .all(|targets| targets.len() >= 16)
                {
                    break;
                }
            }
            let batch_targets = partition_targets
                .values()
                .flat_map(|targets| targets.iter().take(16).copied())
                .collect::<Vec<_>>();
            assert!(
                batch_targets.len() >= 128,
                "rocketship large flat batch wave should gather a larger bounded multi-partition entity batch"
            );

            runtime.performance_access().reset_counters();
            let update_started_at = Instant::now();
            let update = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                let mut batch = WorkerIntentBatch::new("rocketship-large-flat-entity-batch-wave");
                for (index, entity) in batch_targets.iter().enumerate() {
                    batch = batch.push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                        UpdateEntityFieldsIntent {
                            entity_id: *entity,
                            fields: crate::tests::support::aspect_field_patch_from_values([
                                (
                                    crate::tests::support::aspect_key("section"),
                                    crate::tests::support::field_key("section"),
                                    crate::tests::support::string_aspect_value("large-batch-wave"),
                                ),
                                (
                                    crate::tests::support::aspect_key("tag"),
                                    crate::tests::support::field_key("tag"),
                                    crate::tests::support::string_aspect_value(&format!(
                                        "rocket.large_batch.{index}"
                                    )),
                                ),
                                (
                                    crate::tests::support::aspect_key("partition"),
                                    crate::tests::support::field_key("partition"),
                                    crate::tests::support::u64_aspect_value(
                                        entity.partition_id.0 as u64,
                                    ),
                                ),
                            ]),
                        },
                    )));
                }
                txn.push_batch(batch);
                txn.commit()
                    .expect("rocketship large flat entity batch wave commit")
            };
            let update_micros = update_started_at.elapsed().as_micros();
            let phase_timing = update.execution.phase_timing.clone();

            let snapshot = runtime.visibility_authority().snapshot();
            let explicit_targets = batch_targets
                .iter()
                .step_by(4)
                .take(16)
                .copied()
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "rocketship-large-flat-entity-batch-explicit",
                explicit_targets,
            );
            let explicit_started_at = Instant::now();
            let explicit = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("planned rocketship large flat batch explicit query"),
                )
                .expect("rocketship large flat batch explicit outcome");
            let explicit_query_micros = explicit_started_at.elapsed().as_micros();
            assert!(runtime.visibility_authority().release_snapshot(&snapshot));

            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            measurement_with_elapsed(update_micros + explicit_query_micros, || {
                perf_metrics!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "subsystem_count": seeded.subsystem_count,
                    "batch_target_count": batch_targets.len(),
                    "batch_partition_count": partition_targets.len(),
                    "update_micros": update_micros,
                    "explicit_query_micros": explicit_query_micros,
                    "hot_changed_records": update.changed_records.len(),
                    "explicit_result_entities": explicit.result.entities.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "phase_timing": {
                        "draft_preparation_micros": phase_timing.draft_preparation_micros,
                        "draft_merge_plan_micros": phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": phase_timing.draft_working_state_clone_micros,
                        "invariant_pre_check_micros": phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": phase_timing.invariant_post_check_micros,
                        "durable_append_micros": phase_timing.durable_append_micros,
                        "publication_micros": phase_timing.publication_micros,
                        "publication_storage_commit_micros": phase_timing.publication_storage_commit_micros,
                    },
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_pseudorealistic_large_flat_entity_batch_wave",
        &larger_flat_batch_wave_samples,
        &[
            ("batch_target_count", &["batch_target_count"]),
            ("batch_partition_count", &["batch_partition_count"]),
            ("update_micros", &["update_micros"]),
            (
                "draft_preparation_micros",
                &["phase_timing", "draft_preparation_micros"],
            ),
            (
                "draft_merge_plan_micros",
                &["phase_timing", "draft_merge_plan_micros"],
            ),
            (
                "draft_structural_summary_micros",
                &["phase_timing", "draft_structural_summary_micros"],
            ),
            (
                "draft_working_state_clone_micros",
                &["phase_timing", "draft_working_state_clone_micros"],
            ),
            (
                "invariant_pre_check_micros",
                &["phase_timing", "invariant_pre_check_micros"],
            ),
            (
                "authoritative_mutation_micros",
                &["phase_timing", "authoritative_mutation_micros"],
            ),
            (
                "history_resolution_micros",
                &["phase_timing", "history_resolution_micros"],
            ),
            (
                "invariant_post_check_micros",
                &["phase_timing", "invariant_post_check_micros"],
            ),
            (
                "durable_append_micros",
                &["phase_timing", "durable_append_micros"],
            ),
            (
                "publication_storage_commit_micros",
                &["phase_timing", "publication_storage_commit_micros"],
            ),
            ("explicit_query_micros", &["explicit_query_micros"]),
            (
                "aosoa_entity_chunk_slots_materialized",
                &["counters", "aosoa_entity_chunk_slots_materialized"],
            ),
            (
                "aosoa_entity_chunks_published",
                &["counters", "aosoa_entity_chunks_published"],
            ),
        ],
    );
    assert!(larger_flat_batch_wave_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &larger_flat_batch_wave_samples,
        "pseudorealistic rocketship large flat entity batches should stay on the widened sparse AoSoA path when the bounded batch doubles in width",
        |metrics| {
            let batch_target_count = metrics["batch_target_count"].as_u64().unwrap_or(0);
            let batch_partition_count = metrics["batch_partition_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0) >= node_count as u64
                && metrics["subsystem_count"].as_u64() == Some(12)
                && batch_target_count >= 128
                && batch_partition_count >= 8
                && metrics["hot_changed_records"].as_u64() == Some(batch_target_count)
                && metrics["explicit_result_entities"].as_u64() == Some(16)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == batch_target_count
                && counter_u64(metrics, "partitions_touched_by_commit") >= batch_partition_count
                && counter_u64(metrics, "aosoa_entity_chunk_slots_materialized")
                    == batch_target_count
                && counter_u64(metrics, "aosoa_entity_chunks_published") >= batch_partition_count
                && counter_u64(metrics, "aosoa_publish_soa_merge_count") == 0
        },
    );

    let mixed_entity_relation_batch_wave_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_pseudorealistic_mixed_entity_relation_batch_wave",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded =
                seed_pseudorealistic_rocketship_world(&mut runtime, node_count, query_target_count);

            let mut partition_targets = BTreeMap::new();
            for entity in &seeded.entities {
                let targets = partition_targets
                    .entry(entity.partition_id)
                    .or_insert_with(Vec::new);
                if targets.len() < 8 {
                    targets.push(*entity);
                }
                if partition_targets.len() >= 8
                    && partition_targets.values().all(|targets| targets.len() >= 8)
                {
                    break;
                }
            }
            let batch_targets = partition_targets
                .values()
                .flat_map(|targets| targets.iter().take(8).copied())
                .collect::<Vec<_>>();
            assert!(
                batch_targets.len() >= 64,
                "rocketship mixed entity-plus-relation batch wave should gather a broad multi-partition entity batch"
            );

            let relation_specs = partition_targets
                .values()
                .enumerate()
                .flat_map(|(partition_index, targets)| {
                    targets
                        .windows(2)
                        .take(2)
                        .enumerate()
                        .map(
                            move |(edge_index, pair)| crate::transactions::data::RelationSpec {
                                partition_id: PartitionId(601 + partition_index as u32),
                                kind_id: KindId(2),
                                client_key: crate::symbols::data::ClientKey::raw(format!(
                                    "rocket.batch.local.{}.{}",
                                    partition_index, edge_index
                                )),
                                source: crate::transactions::data::EntityReference::Existing(
                                    pair[0],
                                ),
                                target: crate::transactions::data::EntityReference::Existing(
                                    pair[1],
                                ),
                                fields: crate::transactions::data::AspectFieldPatch::default(),
                            },
                        )
                })
                .collect::<Vec<_>>();
            assert!(
                relation_specs.len() >= 16,
                "rocketship mixed entity-plus-relation batch wave should add a bounded local relation wave"
            );

            runtime.performance_access().reset_counters();
            let update_started_at = Instant::now();
            let update = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                let mut batch =
                    WorkerIntentBatch::new("rocketship-mixed-entity-relation-batch-wave");
                for (index, entity) in batch_targets.iter().enumerate() {
                    batch = batch.push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                        UpdateEntityFieldsIntent {
                            entity_id: *entity,
                            fields: crate::tests::support::aspect_field_patch_from_values([
                                (
                                    crate::tests::support::aspect_key("section"),
                                    crate::tests::support::field_key("section"),
                                    crate::tests::support::string_aspect_value("mixed-batch-wave"),
                                ),
                                (
                                    crate::tests::support::aspect_key("tag"),
                                    crate::tests::support::field_key("tag"),
                                    crate::tests::support::string_aspect_value(&format!(
                                        "rocket.mixed_batch.{index}"
                                    )),
                                ),
                                (
                                    crate::tests::support::aspect_key("partition"),
                                    crate::tests::support::field_key("partition"),
                                    crate::tests::support::u64_aspect_value(
                                        entity.partition_id.0 as u64,
                                    ),
                                ),
                            ]),
                        },
                    )));
                }
                for intent in bulk_relation_create_intents(&relation_specs) {
                    batch = batch.push(intent);
                }
                txn.push_batch(batch);
                txn.commit()
                    .expect("rocketship mixed entity plus relation batch wave commit")
            };
            let update_micros = update_started_at.elapsed().as_micros();
            let phase_timing = update.execution.phase_timing.clone();

            let snapshot = runtime.visibility_authority().snapshot();
            let explicit_targets = batch_targets
                .iter()
                .step_by(3)
                .take(16)
                .copied()
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "rocketship-mixed-batch-explicit",
                explicit_targets,
            );
            let explicit_started_at = Instant::now();
            let explicit = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("planned rocketship mixed batch explicit query"),
                )
                .expect("rocketship mixed batch explicit outcome");
            let explicit_query_micros = explicit_started_at.elapsed().as_micros();
            assert!(runtime.visibility_authority().release_snapshot(&snapshot));

            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            measurement_with_elapsed(update_micros + explicit_query_micros, || {
                perf_metrics!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "subsystem_count": seeded.subsystem_count,
                    "batch_target_count": batch_targets.len(),
                    "batch_partition_count": partition_targets.len(),
                    "created_relation_count": relation_specs.len(),
                    "update_micros": update_micros,
                    "explicit_query_micros": explicit_query_micros,
                    "hot_changed_records": update.changed_records.len(),
                    "explicit_result_entities": explicit.result.entities.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "phase_timing": {
                        "draft_preparation_micros": phase_timing.draft_preparation_micros,
                        "draft_merge_plan_micros": phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": phase_timing.draft_working_state_clone_micros,
                        "invariant_pre_check_micros": phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": phase_timing.invariant_post_check_micros,
                        "durable_append_micros": phase_timing.durable_append_micros,
                        "publication_micros": phase_timing.publication_micros,
                        "publication_storage_commit_micros": phase_timing.publication_storage_commit_micros,
                    },
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_pseudorealistic_mixed_entity_relation_batch_wave",
        &mixed_entity_relation_batch_wave_samples,
        &[
            ("batch_target_count", &["batch_target_count"]),
            ("batch_partition_count", &["batch_partition_count"]),
            ("created_relation_count", &["created_relation_count"]),
            ("update_micros", &["update_micros"]),
            (
                "draft_preparation_micros",
                &["phase_timing", "draft_preparation_micros"],
            ),
            (
                "draft_merge_plan_micros",
                &["phase_timing", "draft_merge_plan_micros"],
            ),
            (
                "draft_structural_summary_micros",
                &["phase_timing", "draft_structural_summary_micros"],
            ),
            (
                "draft_working_state_clone_micros",
                &["phase_timing", "draft_working_state_clone_micros"],
            ),
            (
                "invariant_pre_check_micros",
                &["phase_timing", "invariant_pre_check_micros"],
            ),
            (
                "authoritative_mutation_micros",
                &["phase_timing", "authoritative_mutation_micros"],
            ),
            (
                "history_resolution_micros",
                &["phase_timing", "history_resolution_micros"],
            ),
            (
                "invariant_post_check_micros",
                &["phase_timing", "invariant_post_check_micros"],
            ),
            (
                "durable_append_micros",
                &["phase_timing", "durable_append_micros"],
            ),
            (
                "publication_storage_commit_micros",
                &["phase_timing", "publication_storage_commit_micros"],
            ),
            ("explicit_query_micros", &["explicit_query_micros"]),
            (
                "aosoa_entity_chunk_slots_materialized",
                &["counters", "aosoa_entity_chunk_slots_materialized"],
            ),
            (
                "aosoa_entity_chunks_published",
                &["counters", "aosoa_entity_chunks_published"],
            ),
        ],
    );
    assert!(mixed_entity_relation_batch_wave_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &mixed_entity_relation_batch_wave_samples,
        "pseudorealistic rocketship mixed entity plus relation batches should stay bounded and preserve semantic purity when the commit leaves the pure flat-entity fast path",
        |metrics| {
            let batch_target_count = metrics["batch_target_count"].as_u64().unwrap_or(0);
            let batch_partition_count = metrics["batch_partition_count"].as_u64().unwrap_or(0);
            let created_relation_count = metrics["created_relation_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0) >= node_count as u64
                && metrics["subsystem_count"].as_u64() == Some(12)
                && batch_target_count >= 64
                && batch_partition_count >= 8
                && created_relation_count >= 16
                && metrics["hot_changed_records"].as_u64().unwrap_or(0)
                    >= batch_target_count + created_relation_count
                && metrics["explicit_result_entities"].as_u64() == Some(16)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == batch_target_count
                && counter_u64(metrics, "relation_slots_touched_by_commit")
                    >= created_relation_count
                && counter_u64(metrics, "partitions_touched_by_commit") >= batch_partition_count
        },
    );

    let rich_propagation_wave_samples = capture_perf_samples(
        suite,
        "hundred_k_nodes_geometry_profile_propagation_wave",
        || {
            let mut runtime = runtime_with_test_schema_profile_and_chunks(
                RelationalRuntimeProfile::GeometryKernel,
                ROCKETSHIP_CHUNK_SIZE,
                ROCKETSHIP_CHUNK_SIZE,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryRichCertification,
            );
            runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit = node_count * 2;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let seeded =
                seed_pseudorealistic_rocketship_world(&mut runtime, node_count, query_target_count);

            runtime.performance_access().reset_counters();
            let hot_update_started_at = Instant::now();
            let update = update_entity(
                &mut runtime,
                seeded.hot_update_target,
                "rocket.plumbing_and_feed.propagation_patch.rich",
            );
            let hot_update_micros = hot_update_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let context = runtime
                .read_truth()
                .query_plan_context(&snapshot)
                .expect("rocketship rich propagation context");
            let propagation_seeds = vec![
                seeded.traversal_seeds[0],
                seeded.traversal_seeds[1],
                seeded.traversal_seeds[9],
                seeded.traversal_seeds[10],
            ];
            let propagation_packet = PlannedQueryPacket {
                label: "rocketship-pseudorealistic-propagation-rich".to_string(),
                context_id: context,
                scope: QueryScope::ConnectivityTraversal {
                    seeds: Arc::from(propagation_seeds.clone()),
                    relation_kind_scope: Some(Arc::from([KindId(2)])),
                    max_depth: Some(3),
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalTraversalOrder,
                access_contract: QueryAccessContract::AuthoritativeStorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(91_003),
                target_count_hint: propagation_seeds.len(),
            };
            let propagation_plan_started_at = Instant::now();
            let propagation_plan = runtime
                .read_truth()
                .plan_query_packet(&snapshot, propagation_packet)
                .expect("planned rocketship rich propagation query");
            let propagation_planning_micros = propagation_plan_started_at.elapsed().as_micros();
            let propagation_execution_started_at = Instant::now();
            let propagation_outcome = runtime
                .read_truth()
                .execute_query_plan(propagation_plan)
                .expect("rocketship rich propagation outcome");
            let propagation_execution_micros =
                propagation_execution_started_at.elapsed().as_micros();

            let explicit_targets = seeded
                .mixed_query_targets
                .iter()
                .take(12)
                .cloned()
                .collect::<Vec<_>>();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "rocketship-pseudorealistic-propagation-explicit-rich",
                explicit_targets.clone(),
            );
            let explicit_started_at = Instant::now();
            let explicit_outcome = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("planned rocketship rich propagation explicit query"),
                )
                .expect("rocketship rich propagation explicit query outcome");
            let explicit_query_micros = explicit_started_at.elapsed().as_micros();
            let hot_phase_timing = update.execution.phase_timing.clone();
            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            let elapsed_micros = seeded.entity_commit_micros
                + seeded.relation_commit_micros
                + hot_update_micros
                + propagation_planning_micros
                + propagation_execution_micros
                + explicit_query_micros;
            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "subsystem_count": seeded.subsystem_count,
                    "bootstrap_entity_commit_micros": seeded.entity_commit_micros,
                    "bootstrap_relation_commit_micros": seeded.relation_commit_micros,
                    "hot_update_micros": hot_update_micros,
                    "phase_timing": {
                        "draft_preparation_micros": hot_phase_timing.draft_preparation_micros,
                        "draft_bulk_admission_micros": hot_phase_timing.draft_bulk_admission_micros,
                        "draft_merge_plan_micros": hot_phase_timing.draft_merge_plan_micros,
                        "draft_structural_summary_micros": hot_phase_timing.draft_structural_summary_micros,
                        "draft_working_state_clone_micros": hot_phase_timing.draft_working_state_clone_micros,
                        "working_state_preparation_micros": hot_phase_timing.working_state_preparation_micros,
                        "invariant_pre_check_micros": hot_phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": hot_phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": hot_phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": hot_phase_timing.invariant_post_check_micros,
                        "artifact_assembly_micros": hot_phase_timing.artifact_assembly_micros,
                        "durable_append_micros": hot_phase_timing.durable_append_micros,
                        "publication_micros": hot_phase_timing.publication_micros,
                    },
                    "propagation_planning_micros": propagation_planning_micros,
                    "propagation_execution_micros": propagation_execution_micros,
                    "explicit_query_micros": explicit_query_micros,
                    "hot_changed_records": update.changed_records.len(),
                    "propagation_seed_count": propagation_seeds.len(),
                    "propagation_result_entities": propagation_outcome.result.entities.len(),
                    "propagation_result_relations": propagation_outcome.result.relations.len(),
                    "explicit_target_count": explicit_targets.len(),
                    "explicit_result_entities": explicit_outcome.result.entities.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "hundred_k_nodes_geometry_profile_propagation_wave",
        &rich_propagation_wave_samples,
        &[
            ("resident_node_count", &["resident_node_count"]),
            ("resident_relation_count", &["resident_relation_count"]),
            ("subsystem_count", &["subsystem_count"]),
            (
                "bootstrap_entity_commit_micros",
                &["bootstrap_entity_commit_micros"],
            ),
            (
                "bootstrap_relation_commit_micros",
                &["bootstrap_relation_commit_micros"],
            ),
            ("hot_update_micros", &["hot_update_micros"]),
            (
                "draft_preparation_micros",
                &["phase_timing", "draft_preparation_micros"],
            ),
            (
                "propagation_planning_micros",
                &["propagation_planning_micros"],
            ),
            (
                "propagation_execution_micros",
                &["propagation_execution_micros"],
            ),
            ("explicit_query_micros", &["explicit_query_micros"]),
            ("propagation_seed_count", &["propagation_seed_count"]),
            (
                "propagation_result_entities",
                &["propagation_result_entities"],
            ),
            (
                "propagation_result_relations",
                &["propagation_result_relations"],
            ),
            ("explicit_target_count", &["explicit_target_count"]),
            ("explicit_result_entities", &["explicit_result_entities"]),
            ("diagnostic_artifact_count", &["diagnostic_artifact_count"]),
            ("detailed_trace_entries", &["detailed_trace_entries"]),
        ],
    );
    assert!(rich_propagation_wave_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &rich_propagation_wave_samples,
        "rocketship geometry-profile propagation waves should preserve bounded mixed-locality execution while deferring hot detailed traces",
        |metrics| {
            let propagation_seed_count =
                metrics["propagation_seed_count"].as_u64().unwrap_or(0);
            let explicit_target_count =
                metrics["explicit_target_count"].as_u64().unwrap_or(0);
            metrics["resident_node_count"].as_u64() == Some(node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0) >= node_count as u64
                && metrics["subsystem_count"].as_u64() == Some(12)
                && metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["propagation_result_entities"].as_u64().unwrap_or(0)
                    >= propagation_seed_count
                && metrics["propagation_result_relations"].as_u64().unwrap_or(0) >= 1
                && metrics["explicit_result_entities"].as_u64() == Some(explicit_target_count)
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == 1
                && counter_u64(metrics, "query_packet_count") <= 32
                && counter_u64(metrics, "query_scope_unit_count")
                    <= propagation_seed_count + explicit_target_count
        },
    );
}
