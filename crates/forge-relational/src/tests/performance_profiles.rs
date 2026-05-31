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

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_sustained_load_matrix() {
    let suite = "sustained_load_matrix";

    let commit_query_churn_samples =
        capture_perf_samples(suite, "commit_query_churn_stability", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore);
            const ITERATIONS: usize = 128;
            let mut total_commit_micros = 0u128;
            let mut total_query_micros = 0u128;
            let mut max_query_packets_per_iteration = 0usize;
            let mut max_query_scope_units_per_iteration = 0usize;
            let mut previous_scope_units = 0usize;

            runtime.performance_access().reset_counters();
            for index in 0..ITERATIONS {
                let commit_started_at = Instant::now();
                let outcome = create_entity_outcome(&mut runtime, &format!("sustained-{index}"));
                total_commit_micros += commit_started_at.elapsed().as_micros();

                let entity = changed_entities(&outcome)[0];
                let snapshot = runtime.visibility_authority().snapshot();
                let packet = explicit_query_packet(
                    &runtime,
                    &snapshot,
                    "sustained-explicit-target",
                    vec![RecordRef::Entity(entity)],
                );
                let query_started_at = Instant::now();
                let query_outcome = runtime
                    .read_truth()
                    .execute_query_plan(
                        runtime
                            .read_truth()
                            .plan_query_packet(&snapshot, packet)
                            .expect("planned sustained explicit query"),
                    )
                    .expect("sustained explicit query outcome");
                total_query_micros += query_started_at.elapsed().as_micros();
                max_query_packets_per_iteration =
                    max_query_packets_per_iteration.max(query_outcome.complexity.packet_count);
                let scope_units = runtime
                    .performance_access()
                    .counters()
                    .query_scope_unit_count;
                max_query_scope_units_per_iteration = max_query_scope_units_per_iteration
                    .max(scope_units.saturating_sub(previous_scope_units));
                previous_scope_units = scope_units;
            }

            let latest_version = runtime
                .history()
                .latest_commit()
                .expect("latest sustained commit")
                .version_id;
            let final_entity_count = runtime
                .read_truth()
                .project_version(latest_version)
                .all_authoritative_entity_records()
                .len();
            let counters = runtime.performance_access().counters();

            let elapsed_micros = total_commit_micros + total_query_micros;
            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "iterations": ITERATIONS,
                    "average_commit_micros": total_commit_micros / ITERATIONS as u128,
                    "average_query_micros": total_query_micros / ITERATIONS as u128,
                    "max_query_packets_per_iteration": max_query_packets_per_iteration,
                    "max_query_scope_units_per_iteration": max_query_scope_units_per_iteration,
                    "final_entity_count": final_entity_count,
                    "counters": counters,
                })
            })
        });
    emit_metric_summaries(
        suite,
        "commit_query_churn_stability",
        &commit_query_churn_samples,
        &[
            ("average_commit_micros", &["average_commit_micros"]),
            ("average_query_micros", &["average_query_micros"]),
            (
                "max_query_packets_per_iteration",
                &["max_query_packets_per_iteration"],
            ),
            (
                "max_query_scope_units_per_iteration",
                &["max_query_scope_units_per_iteration"],
            ),
            ("final_entity_count", &["final_entity_count"]),
        ],
    );
    assert!(commit_query_churn_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &commit_query_churn_samples,
        "sustained commit/query churn should stay clone-free and packet-stable across long iteration windows",
        |metrics| {
            metrics["iterations"].as_u64() == Some(128)
                && metrics["final_entity_count"].as_u64() == Some(128)
                && metrics["max_query_packets_per_iteration"].as_u64() == Some(1)
                && metrics["max_query_scope_units_per_iteration"].as_u64() == Some(1)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "query_packet_count") == 128
                && counter_u64(metrics, "query_scope_unit_count") == 128
        },
    );

    let replay_window_drift_samples =
        capture_perf_samples(suite, "replay_window_drift_stability", || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::CertificationCore,
            );
            const HISTORY_DEPTH: usize = 48;
            const REPLAY_WINDOW: usize = 32;
            let mut commit_ids = Vec::with_capacity(HISTORY_DEPTH);
            for index in 0..HISTORY_DEPTH {
                let outcome =
                    create_entity_outcome(&mut runtime, &format!("replay-window-{index}"));
                commit_ids.push(outcome.commit.commit_id);
            }

            runtime.performance_access().reset_counters();
            let mut total_replay_micros = 0u128;
            let mut max_replay_micros = 0u128;
            let mut total_compared_surface_count = 0usize;
            let mut total_reconstructed_commit_closure = 0usize;
            let mut total_mismatch_count = 0usize;
            let mut replayed_commit_count = 0usize;

            for commit_id in commit_ids.iter().rev().take(REPLAY_WINDOW) {
                let replay_started_at = Instant::now();
                let outcome = runtime
                    .replay_authority()
                    .replay_commit(RelationalReplayRequest {
                        commit_id: *commit_id,
                        branch_id: BranchId("main".to_string()),
                        execution_mode: ReplayExecutionMode::SerialDeterministic,
                        verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                    });
                let replay_micros = replay_started_at.elapsed().as_micros();
                assert!(
                    outcome.failure.is_none(),
                    "replay window drift sample should not fail: {:?}",
                    outcome.failure
                );
                total_replay_micros += replay_micros;
                max_replay_micros = max_replay_micros.max(replay_micros);
                total_compared_surface_count += outcome.compared_surfaces.len();
                total_reconstructed_commit_closure += outcome.reconstructed_commit_closure.len();
                total_mismatch_count += outcome.mismatches.len();
                replayed_commit_count += 1;
            }

            measurement_with_elapsed(total_replay_micros, || {
                perf_metrics!({
                    "history_depth": HISTORY_DEPTH,
                    "replay_window": REPLAY_WINDOW,
                    "average_replay_micros": total_replay_micros / REPLAY_WINDOW as u128,
                    "max_replay_micros": max_replay_micros,
                    "replayed_commit_count": replayed_commit_count,
                    "total_compared_surface_count": total_compared_surface_count,
                    "total_reconstructed_commit_closure": total_reconstructed_commit_closure,
                    "total_mismatch_count": total_mismatch_count,
                    "counters": runtime.performance_access().counters(),
                })
            })
        });
    emit_metric_summaries(
        suite,
        "replay_window_drift_stability",
        &replay_window_drift_samples,
        &[
            ("average_replay_micros", &["average_replay_micros"]),
            ("max_replay_micros", &["max_replay_micros"]),
            ("replayed_commit_count", &["replayed_commit_count"]),
            (
                "total_compared_surface_count",
                &["total_compared_surface_count"],
            ),
            (
                "total_reconstructed_commit_closure",
                &["total_reconstructed_commit_closure"],
            ),
        ],
    );
    assert!(replay_window_drift_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &replay_window_drift_samples,
        "replay drift windows should stay mismatch-free while replaying a bounded recent history slice",
        |metrics| {
            metrics["history_depth"].as_u64() == Some(48)
                && metrics["replay_window"].as_u64() == Some(32)
                && metrics["replayed_commit_count"].as_u64() == Some(32)
                && metrics["total_mismatch_count"].as_u64() == Some(0)
                && metrics["total_compared_surface_count"].as_u64().unwrap_or(0) >= 32
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "replay_lineage_authority_lookup_requests") == 32
        },
    );

    let retention_pass_drift_samples =
        capture_perf_samples(suite, "retention_pass_drift_stability", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore);
            const ITERATIONS: usize = 48;
            let mut total_inspect_micros = 0u128;
            let mut total_run_pass_micros = 0u128;
            let mut total_entity_reclaimable = 0usize;
            let mut total_entity_reclaimed = 0usize;
            let mut max_reclaimable_entities = 0usize;

            runtime.performance_access().reset_counters();
            for index in 0..ITERATIONS {
                let created =
                    create_entity_outcome(&mut runtime, &format!("retention-drift-{index}"));
                let entity = changed_entities(&created)[0];
                let deleted = delete_entity(&mut runtime, entity);
                assert!(runtime
                    .visibility_authority()
                    .release_snapshot(&created.snapshot));
                assert!(runtime
                    .visibility_authority()
                    .release_snapshot(&deleted.snapshot));

                let inspect_started_at = Instant::now();
                let plan = runtime.retention().inspect_plan();
                total_inspect_micros += inspect_started_at.elapsed().as_micros();
                max_reclaimable_entities = max_reclaimable_entities.max(plan.reclaimable_entities);

                let run_pass_started_at = Instant::now();
                let pass = runtime.retention().run_pass();
                total_run_pass_micros += run_pass_started_at.elapsed().as_micros();
                total_entity_reclaimable += pass.entity_reclaimable;
                total_entity_reclaimed += pass.entity_reclaimed;
            }

            let trailing_plan = runtime.retention().inspect_plan();
            let elapsed_micros = total_inspect_micros + total_run_pass_micros;
            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "iterations": ITERATIONS,
                    "average_inspect_micros": total_inspect_micros / ITERATIONS as u128,
                    "average_run_pass_micros": total_run_pass_micros / ITERATIONS as u128,
                    "total_entity_reclaimable": total_entity_reclaimable,
                    "total_entity_reclaimed": total_entity_reclaimed,
                    "max_reclaimable_entities": max_reclaimable_entities,
                    "trailing_reclaimable_entities": trailing_plan.reclaimable_entities,
                    "counters": runtime.performance_access().counters(),
                })
            })
        });
    emit_metric_summaries(
        suite,
        "retention_pass_drift_stability",
        &retention_pass_drift_samples,
        &[
            ("average_inspect_micros", &["average_inspect_micros"]),
            ("average_run_pass_micros", &["average_run_pass_micros"]),
            ("total_entity_reclaimable", &["total_entity_reclaimable"]),
            ("total_entity_reclaimed", &["total_entity_reclaimed"]),
            ("max_reclaimable_entities", &["max_reclaimable_entities"]),
        ],
    );
    assert!(retention_pass_drift_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &retention_pass_drift_samples,
        "retention drift windows should surface reclaimable released deletions without rebuild-heavy retention behavior",
        |metrics| {
            metrics["iterations"].as_u64() == Some(48)
                && metrics["total_entity_reclaimable"].as_u64().unwrap_or(0) >= 48
                && metrics["total_entity_reclaimed"].as_u64() == Some(0)
                && metrics["trailing_reclaimable_entities"].as_u64() == Some(48)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "snapshot_pin_full_rebuilds") == 0
        },
    );

    let mixed_topology_churn_samples = capture_perf_samples(
        suite,
        "mixed_topology_query_churn_stability",
        || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
            let entities = (0..24)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("mixed-topology-{index}"),
                        PartitionId((index % 6) as u32 + 1),
                    )
                })
                .collect::<Vec<_>>();
            for index in 0..24 {
                create_relation_in_partition(
                    &mut runtime,
                    entities[index],
                    entities[(index + 1) % 24],
                    &format!("mixed-ring-{index}"),
                    PartitionId(40 + (index % 4) as u32),
                );
                if index % 4 == 0 {
                    create_relation_in_partition(
                        &mut runtime,
                        entities[index],
                        entities[(index + 6) % 24],
                        &format!("mixed-brace-{index}"),
                        PartitionId(50 + (index % 3) as u32),
                    );
                }
            }

            const ITERATIONS: usize = 48;
            let mut total_update_micros = 0u128;
            let mut total_explicit_query_micros = 0u128;
            let mut total_traversal_micros = 0u128;
            let mut max_packets_per_iteration = 0usize;
            let mut max_scope_units_per_iteration = 0usize;
            let mut previous_packets = 0usize;
            let mut previous_scope_units = 0usize;

            runtime.performance_access().reset_counters();
            for index in 0..ITERATIONS {
                let hot_entity = entities[(index * 3) % entities.len()];
                let update_started_at = Instant::now();
                let _ = update_entity(
                    &mut runtime,
                    hot_entity,
                    &format!("mixed-topology-hot-{index}"),
                );
                total_update_micros += update_started_at.elapsed().as_micros();

                let snapshot = runtime.visibility_authority().snapshot();
                let explicit_targets = vec![
                    RecordRef::Entity(entities[(index * 3) % entities.len()]),
                    RecordRef::Entity(entities[(index * 3 + 1) % entities.len()]),
                    RecordRef::Entity(entities[(index * 3 + 6) % entities.len()]),
                    RecordRef::Entity(entities[(index * 3 + 12) % entities.len()]),
                ];
                let explicit_packet = explicit_query_packet(
                    &runtime,
                    &snapshot,
                    "mixed-topology-explicit",
                    explicit_targets,
                );
                let explicit_started_at = Instant::now();
                let _ = runtime
                    .read_truth()
                    .execute_query_plan(
                        runtime
                            .read_truth()
                            .plan_query_packet(&snapshot, explicit_packet)
                            .expect("planned mixed topology explicit query"),
                    )
                    .expect("mixed topology explicit query outcome");
                total_explicit_query_micros += explicit_started_at.elapsed().as_micros();

                let context = runtime
                    .read_truth()
                    .query_plan_context(&snapshot)
                    .expect("mixed topology query plan context");
                let traversal_packet = PlannedQueryPacket {
                    label: "mixed-topology-traversal".to_string(),
                    context_id: context,
                    scope: QueryScope::ConnectivityTraversal {
                        seeds: Arc::from([
                            entities[(index * 3) % entities.len()],
                            entities[(index * 3 + 6) % entities.len()],
                        ]),
                        relation_kind_scope: Some(Arc::from([KindId(2)])),
                        max_depth: Some(2),
                    },
                    locality: QueryLocalityClass::CrossPartitionTraversal,
                    ordering: QueryOrderingContract::CanonicalTraversalOrder,
                    access_contract: QueryAccessContract::AuthoritativeStorageOnly,
                    execution_shape: QueryExecutionShape::BulkPacketized,
                    reduction: ReductionDiscipline::DeterministicMerge,
                    plan_key: DeterministicQueryPlanKey(92_001),
                    target_count_hint: 2,
                };
                let traversal_started_at = Instant::now();
                let _ = runtime
                    .read_truth()
                    .execute_query_plan(
                        runtime
                            .read_truth()
                            .plan_query_packet(&snapshot, traversal_packet)
                            .expect("planned mixed topology traversal query"),
                    )
                    .expect("mixed topology traversal outcome");
                total_traversal_micros += traversal_started_at.elapsed().as_micros();

                let counters = runtime.performance_access().counters();
                max_packets_per_iteration = max_packets_per_iteration
                    .max(counters.query_packet_count.saturating_sub(previous_packets));
                max_scope_units_per_iteration = max_scope_units_per_iteration.max(
                    counters
                        .query_scope_unit_count
                        .saturating_sub(previous_scope_units),
                );
                previous_packets = counters.query_packet_count;
                previous_scope_units = counters.query_scope_unit_count;
            }

            let elapsed_micros =
                total_update_micros + total_explicit_query_micros + total_traversal_micros;
            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "iterations": ITERATIONS,
                    "average_update_micros": total_update_micros / ITERATIONS as u128,
                    "average_explicit_query_micros": total_explicit_query_micros / ITERATIONS as u128,
                    "average_traversal_micros": total_traversal_micros / ITERATIONS as u128,
                    "max_packets_per_iteration": max_packets_per_iteration,
                    "max_scope_units_per_iteration": max_scope_units_per_iteration,
                    "counters": runtime.performance_access().counters(),
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "mixed_topology_query_churn_stability",
        &mixed_topology_churn_samples,
        &[
            ("average_update_micros", &["average_update_micros"]),
            (
                "average_explicit_query_micros",
                &["average_explicit_query_micros"],
            ),
            ("average_traversal_micros", &["average_traversal_micros"]),
            ("max_packets_per_iteration", &["max_packets_per_iteration"]),
            (
                "max_scope_units_per_iteration",
                &["max_scope_units_per_iteration"],
            ),
        ],
    );
    assert!(mixed_topology_churn_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &mixed_topology_churn_samples,
        "mixed sustained topology churn should keep packet and scope growth bounded across repeated update plus read waves",
        |metrics| {
            metrics["iterations"].as_u64() == Some(48)
                && metrics["max_packets_per_iteration"].as_u64().unwrap_or(0) <= 8
                && metrics["max_scope_units_per_iteration"].as_u64().unwrap_or(0) <= 8
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "query_packet_count") >= 96
                && counter_u64(metrics, "query_scope_unit_count") >= 96
        },
    );

    let rocketship_endurance_node_count = rocketship_node_count();
    let rocketship_hot_update_endurance_samples =
        capture_perf_samples(suite, "rocketship_hot_update_endurance", || {
            let query_target_count = rocketship_query_target_count(rocketship_endurance_node_count);
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
                .max_patch_records_per_commit = rocketship_endurance_node_count * 2;
            let seeded = seed_pseudorealistic_rocketship_world(
                &mut runtime,
                rocketship_endurance_node_count,
                query_target_count,
            );

            const ITERATIONS: usize = 256;
            const WINDOW: usize = 32;
            let mut update_samples = Vec::with_capacity(ITERATIONS);
            let mut total_update_micros = 0u128;
            let mut max_update_micros = 0u128;
            let mut max_query_micros = 0u128;

            runtime.performance_access().reset_counters();
            for index in 0..ITERATIONS {
                let target = seeded.traversal_seeds[index % seeded.traversal_seeds.len()];
                let update_started_at = Instant::now();
                let _ = update_entity(
                    &mut runtime,
                    target,
                    &format!("rocket.endurance.hot-loop.{index}"),
                );
                let update_micros = update_started_at.elapsed().as_micros();
                update_samples.push(update_micros);
                total_update_micros += update_micros;
                max_update_micros = max_update_micros.max(update_micros);

                if index % 16 == 0 {
                    let snapshot = runtime.visibility_authority().snapshot();
                    let explicit_targets = seeded
                        .mixed_query_targets
                        .iter()
                        .skip(index % seeded.mixed_query_targets.len())
                        .take(8)
                        .cloned()
                        .collect::<Vec<_>>();
                    let packet = explicit_query_packet(
                        &runtime,
                        &snapshot,
                        "rocketship-endurance-explicit",
                        explicit_targets,
                    );
                    let query_started_at = Instant::now();
                    let _ = runtime
                        .read_truth()
                        .execute_query_plan(
                            runtime
                                .read_truth()
                                .plan_query_packet(&snapshot, packet)
                                .expect("planned rocketship endurance explicit query"),
                        )
                        .expect("rocketship endurance explicit query outcome");
                    max_query_micros = max_query_micros.max(query_started_at.elapsed().as_micros());
                    assert!(runtime.visibility_authority().release_snapshot(&snapshot));
                }
            }

            let first_window_average_update_micros =
                update_samples.iter().take(WINDOW).copied().sum::<u128>() / WINDOW as u128;
            let last_window_average_update_micros = update_samples
                .iter()
                .rev()
                .take(WINDOW)
                .copied()
                .sum::<u128>()
                / WINDOW as u128;
            measurement_with_elapsed(total_update_micros, || {
                perf_metrics!({
                    "iterations": ITERATIONS,
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "average_update_micros": total_update_micros / ITERATIONS as u128,
                    "max_update_micros": max_update_micros,
                    "first_window_average_update_micros": first_window_average_update_micros,
                    "last_window_average_update_micros": last_window_average_update_micros,
                    "max_explicit_query_micros": max_query_micros,
                    "counters": runtime.performance_access().counters(),
                })
            })
        });
    emit_metric_summaries(
        suite,
        "rocketship_hot_update_endurance",
        &rocketship_hot_update_endurance_samples,
        &[
            ("iterations", &["iterations"]),
            ("resident_node_count", &["resident_node_count"]),
            ("resident_relation_count", &["resident_relation_count"]),
            ("average_update_micros", &["average_update_micros"]),
            ("max_update_micros", &["max_update_micros"]),
            (
                "first_window_average_update_micros",
                &["first_window_average_update_micros"],
            ),
            (
                "last_window_average_update_micros",
                &["last_window_average_update_micros"],
            ),
            ("max_explicit_query_micros", &["max_explicit_query_micros"]),
        ],
    );
    assert_budget(
        &rocketship_hot_update_endurance_samples,
        "rocketship hot endurance should stay region-local and resist drift across long update windows",
        |metrics| {
            let first_window = metrics["first_window_average_update_micros"]
                .as_u64()
                .unwrap_or(0);
            let last_window = metrics["last_window_average_update_micros"]
                .as_u64()
                .unwrap_or(0);
            metrics["iterations"].as_u64() == Some(256)
                && metrics["resident_node_count"]
                    .as_u64()
                    == Some(rocketship_endurance_node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0)
                    >= rocketship_endurance_node_count as u64
                && last_window <= first_window.saturating_mul(2).max(1)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "bulk_mutation_batch_count") == 256
                && counter_u64(metrics, "partitions_cloned") <= 256
        },
    );

    let rocketship_propagation_endurance_samples = capture_perf_samples(
        suite,
        "rocketship_propagation_endurance",
        || {
            let query_target_count = rocketship_query_target_count(rocketship_endurance_node_count);
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
                .max_patch_records_per_commit = rocketship_endurance_node_count * 2;
            let seeded = seed_pseudorealistic_rocketship_world(
                &mut runtime,
                rocketship_endurance_node_count,
                query_target_count,
            );

            const ITERATIONS: usize = 96;
            const WINDOW: usize = 16;
            let mut cycle_samples = Vec::with_capacity(ITERATIONS);
            let mut total_update_micros = 0u128;
            let mut total_propagation_micros = 0u128;
            let mut total_explicit_query_micros = 0u128;
            let mut max_packets_per_iteration = 0usize;
            let mut max_scope_units_per_iteration = 0usize;
            let mut previous_packets = 0usize;
            let mut previous_scope_units = 0usize;

            runtime.performance_access().reset_counters();
            for index in 0..ITERATIONS {
                let target = seeded.traversal_seeds[index % seeded.traversal_seeds.len()];
                let update_started_at = Instant::now();
                let _ = update_entity(
                    &mut runtime,
                    target,
                    &format!("rocket.endurance.propagation.{index}"),
                );
                let update_micros = update_started_at.elapsed().as_micros();
                total_update_micros += update_micros;

                let snapshot = runtime.visibility_authority().snapshot();
                let context = runtime
                    .read_truth()
                    .query_plan_context(&snapshot)
                    .expect("rocketship endurance propagation context");
                let propagation_seeds = vec![
                    seeded.traversal_seeds[index % seeded.traversal_seeds.len()],
                    seeded.traversal_seeds[(index + 1) % seeded.traversal_seeds.len()],
                    seeded.traversal_seeds[(index + 9) % seeded.traversal_seeds.len()],
                    seeded.traversal_seeds[(index + 10) % seeded.traversal_seeds.len()],
                ];
                let propagation_packet = PlannedQueryPacket {
                    label: "rocketship-endurance-propagation".to_string(),
                    context_id: context,
                    scope: QueryScope::ConnectivityTraversal {
                        seeds: Arc::from(propagation_seeds),
                        relation_kind_scope: Some(Arc::from([KindId(2)])),
                        max_depth: Some(3),
                    },
                    locality: QueryLocalityClass::CrossPartitionTraversal,
                    ordering: QueryOrderingContract::CanonicalTraversalOrder,
                    access_contract: QueryAccessContract::AuthoritativeStorageOnly,
                    execution_shape: QueryExecutionShape::BulkPacketized,
                    reduction: ReductionDiscipline::DeterministicMerge,
                    plan_key: DeterministicQueryPlanKey(92_250),
                    target_count_hint: 4,
                };
                let propagation_started_at = Instant::now();
                let _ = runtime
                    .read_truth()
                    .execute_query_plan(
                        runtime
                            .read_truth()
                            .plan_query_packet(&snapshot, propagation_packet)
                            .expect("planned rocketship endurance propagation query"),
                    )
                    .expect("rocketship endurance propagation outcome");
                let propagation_micros = propagation_started_at.elapsed().as_micros();
                total_propagation_micros += propagation_micros;

                let explicit_targets = seeded
                    .mixed_query_targets
                    .iter()
                    .cycle()
                    .skip(index)
                    .take(12)
                    .cloned()
                    .collect::<Vec<_>>();
                let explicit_packet = explicit_query_packet(
                    &runtime,
                    &snapshot,
                    "rocketship-endurance-explicit-broad",
                    explicit_targets,
                );
                let explicit_started_at = Instant::now();
                let _ = runtime
                    .read_truth()
                    .execute_query_plan(
                        runtime
                            .read_truth()
                            .plan_query_packet(&snapshot, explicit_packet)
                            .expect("planned rocketship endurance explicit broad query"),
                    )
                    .expect("rocketship endurance explicit broad outcome");
                let explicit_query_micros = explicit_started_at.elapsed().as_micros();
                total_explicit_query_micros += explicit_query_micros;
                assert!(runtime.visibility_authority().release_snapshot(&snapshot));

                let cycle_micros = update_micros + propagation_micros + explicit_query_micros;
                cycle_samples.push(cycle_micros);

                let counters = runtime.performance_access().counters();
                max_packets_per_iteration = max_packets_per_iteration
                    .max(counters.query_packet_count.saturating_sub(previous_packets));
                max_scope_units_per_iteration = max_scope_units_per_iteration.max(
                    counters
                        .query_scope_unit_count
                        .saturating_sub(previous_scope_units),
                );
                previous_packets = counters.query_packet_count;
                previous_scope_units = counters.query_scope_unit_count;
            }

            let first_window_average_cycle_micros =
                cycle_samples.iter().take(WINDOW).copied().sum::<u128>() / WINDOW as u128;
            let last_window_average_cycle_micros = cycle_samples
                .iter()
                .rev()
                .take(WINDOW)
                .copied()
                .sum::<u128>()
                / WINDOW as u128;
            let elapsed_micros =
                total_update_micros + total_propagation_micros + total_explicit_query_micros;
            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "iterations": ITERATIONS,
                    "resident_node_count": seeded.entities.len(),
                    "resident_relation_count": seeded.relation_count,
                    "average_update_micros": total_update_micros / ITERATIONS as u128,
                    "average_propagation_micros": total_propagation_micros / ITERATIONS as u128,
                    "average_explicit_query_micros": total_explicit_query_micros / ITERATIONS as u128,
                    "first_window_average_cycle_micros": first_window_average_cycle_micros,
                    "last_window_average_cycle_micros": last_window_average_cycle_micros,
                    "max_packets_per_iteration": max_packets_per_iteration,
                    "max_scope_units_per_iteration": max_scope_units_per_iteration,
                    "counters": runtime.performance_access().counters(),
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "rocketship_propagation_endurance",
        &rocketship_propagation_endurance_samples,
        &[
            ("iterations", &["iterations"]),
            ("resident_node_count", &["resident_node_count"]),
            ("resident_relation_count", &["resident_relation_count"]),
            ("average_update_micros", &["average_update_micros"]),
            (
                "average_propagation_micros",
                &["average_propagation_micros"],
            ),
            (
                "average_explicit_query_micros",
                &["average_explicit_query_micros"],
            ),
            (
                "first_window_average_cycle_micros",
                &["first_window_average_cycle_micros"],
            ),
            (
                "last_window_average_cycle_micros",
                &["last_window_average_cycle_micros"],
            ),
            ("max_packets_per_iteration", &["max_packets_per_iteration"]),
            (
                "max_scope_units_per_iteration",
                &["max_scope_units_per_iteration"],
            ),
        ],
    );
    assert_budget(
        &rocketship_propagation_endurance_samples,
        "rocketship propagation endurance should keep broad-wave cycles bounded across extended 100k-node operation",
        |metrics| {
            let first_window = metrics["first_window_average_cycle_micros"]
                .as_u64()
                .unwrap_or(0);
            let last_window = metrics["last_window_average_cycle_micros"]
                .as_u64()
                .unwrap_or(0);
            metrics["iterations"].as_u64() == Some(96)
                && metrics["resident_node_count"]
                    .as_u64()
                    == Some(rocketship_endurance_node_count as u64)
                && metrics["resident_relation_count"].as_u64().unwrap_or(0)
                    >= rocketship_endurance_node_count as u64
                && last_window <= first_window.saturating_mul(2).max(1)
                && metrics["max_packets_per_iteration"].as_u64().unwrap_or(0) <= 24
                && metrics["max_scope_units_per_iteration"].as_u64().unwrap_or(0) <= 24
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "bulk_mutation_batch_count") == 96
        },
    );

    let chip_global_step_endurance_samples =
        capture_perf_samples(suite, "chip_global_step_endurance", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::ChipOperationalHotPath,
            );

            let drivers = (0..8)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("chip-global-driver-{index}"),
                        PartitionId(930 + index as u32),
                    )
                })
                .collect::<Vec<_>>();
            let sinks = (0..64)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("chip-global-sink-{index}"),
                        PartitionId(950 + (index % 8) as u32),
                    )
                })
                .collect::<Vec<_>>();

            for (index, driver) in drivers.iter().enumerate() {
                for fanout in 0..8 {
                    let sink = sinks[index * 8 + fanout];
                    create_relation_in_partition(
                        &mut runtime,
                        *driver,
                        sink,
                        &format!("chip-global-fanout-{index}-{fanout}"),
                        PartitionId(980 + index as u32),
                    );
                }
            }
            for index in 0..(sinks.len() - 1) {
                create_relation_in_partition(
                    &mut runtime,
                    sinks[index],
                    sinks[index + 1],
                    &format!("chip-global-chain-{index}"),
                    PartitionId(990 + (index % 4) as u32),
                );
            }

            const ITERATIONS: usize = 128;
            const WINDOW: usize = 32;
            let mut cycle_samples = Vec::with_capacity(ITERATIONS);
            let mut total_update_micros = 0u128;
            let mut total_compile_micros = 0u128;
            let mut total_adjacency_micros = 0u128;
            let mut max_compile_micros = 0u128;
            let mut max_outgoing_relation_count = 0usize;

            runtime.performance_access().reset_counters();
            for step in 0..ITERATIONS {
                let driver = drivers[step % drivers.len()];
                let update_started_at = Instant::now();
                let _ = update_entity(
                    &mut runtime,
                    driver,
                    &format!("chip-global-driver-step-{step}"),
                );
                let update_micros = update_started_at.elapsed().as_micros();
                total_update_micros += update_micros;

                let commit = runtime
                    .history()
                    .latest_commit()
                    .expect("chip global step commit")
                    .clone();
                let compile_started_at = Instant::now();
                let artifact = runtime
                    .compiled_artifacts_authority()
                    .compile_execution_artifact(
                        commit.commit_id,
                        vec![
                            PartitionId(930),
                            PartitionId(931),
                            PartitionId(932),
                            PartitionId(933),
                            PartitionId(934),
                            PartitionId(935),
                            PartitionId(936),
                            PartitionId(937),
                            PartitionId(950),
                            PartitionId(951),
                            PartitionId(952),
                            PartitionId(953),
                            PartitionId(954),
                            PartitionId(955),
                            PartitionId(956),
                            PartitionId(957),
                        ],
                    )
                    .expect("chip global step compiled artifact");
                let compile_micros = compile_started_at.elapsed().as_micros();
                total_compile_micros += compile_micros;
                max_compile_micros = max_compile_micros.max(compile_micros);

                let adjacency_started_at = Instant::now();
                let outgoing_relations = runtime
                    .storage_access()
                    .outgoing_relations_for_entity(driver, commit.version_id);
                let adjacency_micros = adjacency_started_at.elapsed().as_micros();
                total_adjacency_micros += adjacency_micros;
                max_outgoing_relation_count =
                    max_outgoing_relation_count.max(outgoing_relations.len());
                assert_eq!(
                    runtime
                        .compiled_artifacts()
                        .compiled_artifact_authority_status(artifact.artifact_id),
                    CompiledArtifactAuthorityStatus::Authoritative
                );

                cycle_samples.push(update_micros + compile_micros + adjacency_micros);
            }

            let first_window_average_cycle_micros =
                cycle_samples.iter().take(WINDOW).copied().sum::<u128>() / WINDOW as u128;
            let last_window_average_cycle_micros = cycle_samples
                .iter()
                .rev()
                .take(WINDOW)
                .copied()
                .sum::<u128>()
                / WINDOW as u128;
            let elapsed_micros =
                total_update_micros + total_compile_micros + total_adjacency_micros;
            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "iterations": ITERATIONS,
                    "driver_count": drivers.len(),
                    "sink_count": sinks.len(),
                    "average_update_micros": total_update_micros / ITERATIONS as u128,
                    "average_compile_micros": total_compile_micros / ITERATIONS as u128,
                    "average_adjacency_micros": total_adjacency_micros / ITERATIONS as u128,
                    "first_window_average_cycle_micros": first_window_average_cycle_micros,
                    "last_window_average_cycle_micros": last_window_average_cycle_micros,
                    "max_compile_micros": max_compile_micros,
                    "max_outgoing_relation_count": max_outgoing_relation_count,
                    "counters": runtime.performance_access().counters(),
                })
            })
        });
    emit_metric_summaries(
        suite,
        "chip_global_step_endurance",
        &chip_global_step_endurance_samples,
        &[
            ("iterations", &["iterations"]),
            ("average_update_micros", &["average_update_micros"]),
            ("average_compile_micros", &["average_compile_micros"]),
            ("average_adjacency_micros", &["average_adjacency_micros"]),
            (
                "first_window_average_cycle_micros",
                &["first_window_average_cycle_micros"],
            ),
            (
                "last_window_average_cycle_micros",
                &["last_window_average_cycle_micros"],
            ),
            ("max_compile_micros", &["max_compile_micros"]),
            (
                "max_outgoing_relation_count",
                &["max_outgoing_relation_count"],
            ),
        ],
    );
    assert_budget(
        &chip_global_step_endurance_samples,
        "chip global step endurance should keep repeated denser fanout stepping proportional across a longer sustained window",
        |metrics| {
            let first_window = metrics["first_window_average_cycle_micros"]
                .as_u64()
                .unwrap_or(0);
            let last_window = metrics["last_window_average_cycle_micros"]
                .as_u64()
                .unwrap_or(0);
            metrics["iterations"].as_u64() == Some(128)
                && metrics["driver_count"].as_u64() == Some(8)
                && metrics["sink_count"].as_u64() == Some(64)
                && metrics["max_outgoing_relation_count"].as_u64().unwrap_or(0) >= 8
                && last_window <= first_window.saturating_mul(2).max(1)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "bulk_mutation_entity_target_count") == 128
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_hot_cold_path_matrix() {
    let suite = "hot_cold_path_matrix";

    let geometry_hot_vs_replay_samples = capture_perf_samples(
        suite,
        "geometry_hot_commit_vs_replay_reconstruction",
        || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::GeometryKernel,
            );
            runtime.config.diagnostics.profile.detailed_traces_enabled = false;
            runtime.config.diagnostics.profile.max_entries_per_artifact = 0;

            let source = create_entity_outcome(&mut runtime, "hot-cold-geometry-source");
            let middle = create_entity_outcome(&mut runtime, "hot-cold-geometry-middle");
            let target = create_entity_outcome(&mut runtime, "hot-cold-geometry-target");
            let source_entity = changed_entities(&source)[0];
            let middle_entity = changed_entities(&middle)[0];
            let target_entity = changed_entities(&target)[0];
            create_relation_outcome(
                &mut runtime,
                source_entity,
                middle_entity,
                "hot-cold-geometry-link-a",
            );
            create_relation_outcome(
                &mut runtime,
                middle_entity,
                target_entity,
                "hot-cold-geometry-link-b",
            );

            runtime.performance_access().reset_counters();
            let hot_commit_started_at = Instant::now();
            let hot_commit = update_entity(
                &mut runtime,
                middle_entity,
                "hot-cold-geometry-middle-updated",
            );
            let hot_commit_micros = hot_commit_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let hot_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "hot-cold-geometry-hot-query",
                vec![
                    RecordRef::Entity(source_entity),
                    RecordRef::Entity(middle_entity),
                ],
            );
            let hot_query_started_at = Instant::now();
            let hot_query = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, hot_packet)
                        .expect("planned hot geometry query"),
                )
                .expect("hot geometry query outcome");
            let hot_query_micros = hot_query_started_at.elapsed().as_micros();

            let checkpoint_started_at = Instant::now();
            runtime
                .durability_authority()
                .checkpoint()
                .expect("geometry hot/cold checkpoint");
            let checkpoint_micros = checkpoint_started_at.elapsed().as_micros();

            let plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let mut recovered = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::GeometryKernel,
            );
            recovered.performance_access().reset_counters();
            let recover_started_at = Instant::now();
            recovered
                .durability_authority()
                .recover(plan)
                .expect("geometry hot/cold recovery");
            let recover_micros = recover_started_at.elapsed().as_micros();

            let replay_started_at = Instant::now();
            let replay = recovered
                .replay_authority()
                .replay_commit(RelationalReplayRequest {
                    commit_id: hot_commit.commit.commit_id,
                    branch_id: BranchId("main".to_string()),
                    execution_mode: ReplayExecutionMode::SerialDeterministic,
                    verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                });
            let replay_commit_micros = replay_started_at.elapsed().as_micros();

            let recovered_snapshot = recovered.visibility_authority().snapshot();
            let cold_packet = explicit_query_packet(
                &recovered,
                &recovered_snapshot,
                "hot-cold-geometry-cold-query",
                vec![
                    RecordRef::Entity(source_entity),
                    RecordRef::Entity(middle_entity),
                ],
            );
            let cold_query_started_at = Instant::now();
            let cold_query = recovered
                .read_truth()
                .execute_query_plan(
                    recovered
                        .read_truth()
                        .plan_query_packet(&recovered_snapshot, cold_packet)
                        .expect("planned cold geometry query"),
                )
                .expect("cold geometry query outcome");
            let cold_query_micros = cold_query_started_at.elapsed().as_micros();

            let elapsed_micros = hot_commit_micros
                + hot_query_micros
                + checkpoint_micros
                + recover_micros
                + replay_commit_micros
                + cold_query_micros;
            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "hot_changed_records": hot_commit.changed_records.len(),
                    "hot_result_entities": hot_query.result.entities.len(),
                    "cold_result_entities": cold_query.result.entities.len(),
                    "replay_mismatch_count": replay.mismatches.len(),
                    "replay_failure": replay.failure.as_ref().map(|failure| format!("{failure:?}")),
                    "phase_timing": {
                        "hot_commit_micros": hot_commit_micros,
                        "hot_query_micros": hot_query_micros,
                        "checkpoint_micros": checkpoint_micros,
                        "recover_micros": recover_micros,
                        "replay_commit_micros": replay_commit_micros,
                        "cold_query_micros": cold_query_micros,
                    },
                    "hot_counters": runtime.performance_access().counters(),
                    "cold_counters": recovered.performance_access().counters(),
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "geometry_hot_commit_vs_replay_reconstruction",
        &geometry_hot_vs_replay_samples,
        &[
            ("hot_commit_micros", &["phase_timing", "hot_commit_micros"]),
            ("hot_query_micros", &["phase_timing", "hot_query_micros"]),
            ("checkpoint_micros", &["phase_timing", "checkpoint_micros"]),
            ("recover_micros", &["phase_timing", "recover_micros"]),
            (
                "replay_commit_micros",
                &["phase_timing", "replay_commit_micros"],
            ),
            ("cold_query_micros", &["phase_timing", "cold_query_micros"]),
        ],
    );
    assert!(geometry_hot_vs_replay_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &geometry_hot_vs_replay_samples,
        "geometry hot/cold certification should keep hot updates narrow while proving truth is replay-recoverable on the cold path",
        |metrics| {
            metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["hot_result_entities"].as_u64() == Some(2)
                && metrics["cold_result_entities"].as_u64() == Some(2)
                && metrics["replay_failure"].is_null()
                && metrics["replay_mismatch_count"].as_u64() == Some(0)
                && metrics["phase_timing"]["hot_commit_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["recover_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["replay_commit_micros"].as_u64().unwrap_or(0) > 0
                && metrics["hot_counters"]["full_state_clones"].as_u64() == Some(0)
                && metrics["cold_counters"]["full_state_clones"].as_u64() == Some(0)
        },
    );

    let chip_hot_vs_recovery_samples = capture_perf_samples(
        suite,
        "chip_hot_compile_vs_recovery_compile",
        || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::ChipSimulation,
            );
            runtime.config.diagnostics.profile.detailed_traces_enabled = false;
            runtime.config.diagnostics.profile.max_entries_per_artifact = 0;

            let source =
                create_entity_in_partition(&mut runtime, "chip-hot-cold-source", PartitionId(7));
            let sinks = (0..8)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("chip-hot-cold-sink-{index}"),
                        if index % 2 == 0 {
                            PartitionId(11)
                        } else {
                            PartitionId(12)
                        },
                    )
                })
                .collect::<Vec<_>>();
            for (index, sink) in sinks.iter().enumerate() {
                create_relation_in_partition(
                    &mut runtime,
                    source,
                    *sink,
                    &format!("chip-hot-cold-link-{index}"),
                    PartitionId(19),
                );
            }

            runtime.performance_access().reset_counters();
            let hot_commit_started_at = Instant::now();
            let hot_commit = update_entity(&mut runtime, source, "chip-hot-cold-updated");
            let hot_commit_micros = hot_commit_started_at.elapsed().as_micros();
            let latest_commit = runtime
                .history()
                .latest_commit()
                .expect("chip hot/cold latest commit")
                .clone();
            let hot_compile_started_at = Instant::now();
            let hot_artifact = runtime
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    latest_commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(12),
                        PartitionId(19),
                    ],
                )
                .expect("hot chip compiled artifact");
            let hot_compile_micros = hot_compile_started_at.elapsed().as_micros();

            let checkpoint_started_at = Instant::now();
            runtime
                .durability_authority()
                .checkpoint()
                .expect("chip hot/cold checkpoint");
            let checkpoint_micros = checkpoint_started_at.elapsed().as_micros();

            let plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let mut recovered = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::ChipSimulation,
            );
            recovered.performance_access().reset_counters();
            let recover_started_at = Instant::now();
            recovered
                .durability_authority()
                .recover(plan)
                .expect("chip hot/cold recovery");
            let recover_micros = recover_started_at.elapsed().as_micros();

            let replay_started_at = Instant::now();
            let replay = recovered
                .replay_authority()
                .replay_commit(RelationalReplayRequest {
                    commit_id: hot_commit.commit.commit_id,
                    branch_id: BranchId("main".to_string()),
                    execution_mode: ReplayExecutionMode::SerialDeterministic,
                    verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                });
            let replay_commit_micros = replay_started_at.elapsed().as_micros();

            let recovered_commit = recovered
                .history()
                .latest_commit()
                .expect("recovered chip latest commit")
                .clone();
            let cold_compile_started_at = Instant::now();
            let cold_artifact = recovered
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    recovered_commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(12),
                        PartitionId(19),
                    ],
                )
                .expect("cold chip compiled artifact");
            let cold_compile_micros = cold_compile_started_at.elapsed().as_micros();

            let elapsed_micros = hot_commit_micros
                + hot_compile_micros
                + checkpoint_micros
                + recover_micros
                + replay_commit_micros
                + cold_compile_micros;
            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "hot_changed_records": hot_commit.changed_records.len(),
                    "replay_mismatch_count": replay.mismatches.len(),
                    "replay_failure": replay.failure.as_ref().map(|failure| format!("{failure:?}")),
                    "hot_authority_status": format!(
                        "{:?}",
                        runtime.compiled_artifacts().compiled_artifact_authority_status(hot_artifact.artifact_id)
                    ),
                    "cold_authority_status": format!(
                        "{:?}",
                        recovered.compiled_artifacts().compiled_artifact_authority_status(cold_artifact.artifact_id)
                    ),
                    "phase_timing": {
                        "hot_commit_micros": hot_commit_micros,
                        "hot_compile_micros": hot_compile_micros,
                        "checkpoint_micros": checkpoint_micros,
                        "recover_micros": recover_micros,
                        "replay_commit_micros": replay_commit_micros,
                        "cold_compile_micros": cold_compile_micros,
                    },
                    "hot_counters": runtime.performance_access().counters(),
                    "cold_counters": recovered.performance_access().counters(),
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "chip_hot_compile_vs_recovery_compile",
        &chip_hot_vs_recovery_samples,
        &[
            ("hot_commit_micros", &["phase_timing", "hot_commit_micros"]),
            (
                "hot_compile_micros",
                &["phase_timing", "hot_compile_micros"],
            ),
            ("checkpoint_micros", &["phase_timing", "checkpoint_micros"]),
            ("recover_micros", &["phase_timing", "recover_micros"]),
            (
                "replay_commit_micros",
                &["phase_timing", "replay_commit_micros"],
            ),
            (
                "cold_compile_micros",
                &["phase_timing", "cold_compile_micros"],
            ),
        ],
    );
    assert!(chip_hot_vs_recovery_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &chip_hot_vs_recovery_samples,
        "chip hot/cold certification should keep compile-ready stepping narrow while preserving recovery-compile equivalence on the cold path",
        |metrics| {
            metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["replay_failure"].is_null()
                && metrics["replay_mismatch_count"].as_u64() == Some(0)
                && metrics["hot_authority_status"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactAuthorityStatus::Authoritative))
                && metrics["cold_authority_status"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactAuthorityStatus::Authoritative))
                && metrics["phase_timing"]["hot_compile_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["recover_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["cold_compile_micros"].as_u64().unwrap_or(0) > 0
                && metrics["hot_counters"]["full_state_clones"].as_u64() == Some(0)
                && metrics["cold_counters"]["full_state_clones"].as_u64() == Some(0)
        },
    );

    let geometry_rich_publication_samples = capture_perf_samples(
        suite,
        "geometry_rich_publication_hot_vs_replay_truth",
        || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::GeometryKernel,
            );
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();

            let source = create_entity_outcome(&mut runtime, "hot-cold-geometry-rich-source");
            let middle = create_entity_outcome(&mut runtime, "hot-cold-geometry-rich-middle");
            let target = create_entity_outcome(&mut runtime, "hot-cold-geometry-rich-target");
            let source_entity = changed_entities(&source)[0];
            let middle_entity = changed_entities(&middle)[0];
            let target_entity = changed_entities(&target)[0];
            create_relation_outcome(
                &mut runtime,
                source_entity,
                middle_entity,
                "hot-cold-geometry-rich-link-a",
            );
            create_relation_outcome(
                &mut runtime,
                middle_entity,
                target_entity,
                "hot-cold-geometry-rich-link-b",
            );

            runtime.performance_access().reset_counters();
            let hot_commit_started_at = Instant::now();
            let hot_commit = update_entity(
                &mut runtime,
                middle_entity,
                "hot-cold-geometry-rich-middle-updated",
            );
            let hot_commit_micros = hot_commit_started_at.elapsed().as_micros();
            let hot_phase_timing = hot_commit.execution.phase_timing.clone();

            let snapshot = runtime.visibility_authority().snapshot();
            let hot_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "hot-cold-geometry-rich-hot-query",
                vec![
                    RecordRef::Entity(source_entity),
                    RecordRef::Entity(middle_entity),
                ],
            );
            let hot_query_started_at = Instant::now();
            let hot_query = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, hot_packet)
                        .expect("planned hot rich geometry query"),
                )
                .expect("hot rich geometry query outcome");
            let hot_query_micros = hot_query_started_at.elapsed().as_micros();
            let (hot_diagnostic_artifact_count, hot_detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            let checkpoint_started_at = Instant::now();
            runtime
                .durability_authority()
                .checkpoint()
                .expect("geometry rich hot/cold checkpoint");
            let checkpoint_micros = checkpoint_started_at.elapsed().as_micros();

            let plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let mut recovered = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::GeometryKernel,
            );
            recovered.performance_access().reset_counters();
            let recover_started_at = Instant::now();
            recovered
                .durability_authority()
                .recover(plan)
                .expect("geometry rich hot/cold recovery");
            let recover_micros = recover_started_at.elapsed().as_micros();

            let replay_started_at = Instant::now();
            let replay = recovered
                .replay_authority()
                .replay_commit(RelationalReplayRequest {
                    commit_id: hot_commit.commit.commit_id,
                    branch_id: BranchId("main".to_string()),
                    execution_mode: ReplayExecutionMode::SerialDeterministic,
                    verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                });
            let replay_commit_micros = replay_started_at.elapsed().as_micros();

            let recovered_snapshot = recovered.visibility_authority().snapshot();
            let cold_packet = explicit_query_packet(
                &recovered,
                &recovered_snapshot,
                "hot-cold-geometry-rich-cold-query",
                vec![
                    RecordRef::Entity(source_entity),
                    RecordRef::Entity(middle_entity),
                ],
            );
            let cold_query_started_at = Instant::now();
            let cold_query = recovered
                .read_truth()
                .execute_query_plan(
                    recovered
                        .read_truth()
                        .plan_query_packet(&recovered_snapshot, cold_packet)
                        .expect("planned cold rich geometry query"),
                )
                .expect("cold rich geometry query outcome");
            let cold_query_micros = cold_query_started_at.elapsed().as_micros();

            let elapsed_micros = hot_commit_micros
                + hot_query_micros
                + checkpoint_micros
                + recover_micros
                + replay_commit_micros
                + cold_query_micros;
            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "hot_changed_records": hot_commit.changed_records.len(),
                    "hot_result_entities": hot_query.result.entities.len(),
                    "cold_result_entities": cold_query.result.entities.len(),
                    "hot_diagnostic_artifact_count": hot_diagnostic_artifact_count,
                    "hot_detailed_trace_entries": hot_detailed_trace_entries,
                    "replay_mismatch_count": replay.mismatches.len(),
                    "replay_failure": replay.failure.as_ref().map(|failure| format!("{failure:?}")),
                    "phase_timing": {
                        "hot_commit_micros": hot_commit_micros,
                        "hot_query_micros": hot_query_micros,
                        "artifact_assembly_micros": hot_phase_timing.artifact_assembly_micros,
                        "durable_append_micros": hot_phase_timing.durable_append_micros,
                        "publication_micros": hot_phase_timing.publication_micros,
                        "checkpoint_micros": checkpoint_micros,
                        "recover_micros": recover_micros,
                        "replay_commit_micros": replay_commit_micros,
                        "cold_query_micros": cold_query_micros,
                    },
                    "hot_counters": runtime.performance_access().counters(),
                    "cold_counters": recovered.performance_access().counters(),
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "geometry_rich_publication_hot_vs_replay_truth",
        &geometry_rich_publication_samples,
        &[
            ("hot_commit_micros", &["phase_timing", "hot_commit_micros"]),
            ("hot_query_micros", &["phase_timing", "hot_query_micros"]),
            (
                "artifact_assembly_micros",
                &["phase_timing", "artifact_assembly_micros"],
            ),
            (
                "durable_append_micros",
                &["phase_timing", "durable_append_micros"],
            ),
            (
                "publication_micros",
                &["phase_timing", "publication_micros"],
            ),
            ("checkpoint_micros", &["phase_timing", "checkpoint_micros"]),
            ("recover_micros", &["phase_timing", "recover_micros"]),
            (
                "replay_commit_micros",
                &["phase_timing", "replay_commit_micros"],
            ),
            ("cold_query_micros", &["phase_timing", "cold_query_micros"]),
            (
                "hot_diagnostic_artifact_count",
                &["hot_diagnostic_artifact_count"],
            ),
            (
                "hot_detailed_trace_entries",
                &["hot_detailed_trace_entries"],
            ),
        ],
    );
    assert!(geometry_rich_publication_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &geometry_rich_publication_samples,
        "geometry rich hot/cold certification should isolate summaries on the hot side while proving replay-recoverable truth on the cold side",
        |metrics| {
            metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["hot_result_entities"].as_u64() == Some(2)
                && metrics["cold_result_entities"].as_u64() == Some(2)
                && metrics["hot_diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["hot_detailed_trace_entries"].as_u64() == Some(0)
                && metrics["replay_failure"].is_null()
                && metrics["replay_mismatch_count"].as_u64() == Some(0)
                && metrics["phase_timing"]["artifact_assembly_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["publication_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["recover_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["replay_commit_micros"].as_u64().unwrap_or(0) > 0
                && metrics["hot_counters"]["full_state_clones"].as_u64() == Some(0)
                && metrics["cold_counters"]["full_state_clones"].as_u64() == Some(0)
        },
    );

    let chip_rich_compile_samples = capture_perf_samples(
        suite,
        "chip_rich_compile_hot_vs_recovery_compile",
        || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::ChipSimulation,
            );
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::ChipRichCertification,
            );
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();

            let source = create_entity_in_partition(
                &mut runtime,
                "chip-rich-hot-cold-source",
                PartitionId(7),
            );
            let sinks = (0..8)
                .map(|index| {
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("chip-rich-hot-cold-sink-{index}"),
                        if index % 2 == 0 {
                            PartitionId(11)
                        } else {
                            PartitionId(12)
                        },
                    )
                })
                .collect::<Vec<_>>();
            for (index, sink) in sinks.iter().enumerate() {
                create_relation_in_partition(
                    &mut runtime,
                    source,
                    *sink,
                    &format!("chip-rich-hot-cold-link-{index}"),
                    PartitionId(19),
                );
            }

            runtime.performance_access().reset_counters();
            let hot_commit_started_at = Instant::now();
            let hot_commit = update_entity(&mut runtime, source, "chip-rich-hot-cold-updated");
            let hot_commit_micros = hot_commit_started_at.elapsed().as_micros();
            let latest_commit = runtime
                .history()
                .latest_commit()
                .expect("chip rich hot/cold latest commit")
                .clone();
            let hot_compile_started_at = Instant::now();
            let hot_artifact = runtime
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    latest_commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(12),
                        PartitionId(19),
                    ],
                )
                .expect("hot rich chip compiled artifact");
            let hot_compile_micros = hot_compile_started_at.elapsed().as_micros();
            let (hot_diagnostic_artifact_count, hot_detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            let checkpoint_started_at = Instant::now();
            runtime
                .durability_authority()
                .checkpoint()
                .expect("chip rich hot/cold checkpoint");
            let checkpoint_micros = checkpoint_started_at.elapsed().as_micros();

            let plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let mut recovered = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::ChipSimulation,
            );
            apply_perf_diagnostics_policy(
                &mut recovered,
                PerfDiagnosticsPolicy::ChipRichCertification,
            );
            recovered.performance_access().reset_counters();
            let recover_started_at = Instant::now();
            recovered
                .durability_authority()
                .recover(plan)
                .expect("chip rich hot/cold recovery");
            let recover_micros = recover_started_at.elapsed().as_micros();

            let replay_started_at = Instant::now();
            let replay = recovered
                .replay_authority()
                .replay_commit(RelationalReplayRequest {
                    commit_id: hot_commit.commit.commit_id,
                    branch_id: BranchId("main".to_string()),
                    execution_mode: ReplayExecutionMode::SerialDeterministic,
                    verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                });
            let replay_commit_micros = replay_started_at.elapsed().as_micros();

            let recovered_commit = recovered
                .history()
                .latest_commit()
                .expect("recovered rich chip latest commit")
                .clone();
            let cold_compile_started_at = Instant::now();
            let cold_artifact = recovered
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    recovered_commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(12),
                        PartitionId(19),
                    ],
                )
                .expect("cold rich chip compiled artifact");
            let cold_compile_micros = cold_compile_started_at.elapsed().as_micros();

            let elapsed_micros = hot_commit_micros
                + hot_compile_micros
                + checkpoint_micros
                + recover_micros
                + replay_commit_micros
                + cold_compile_micros;
            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "hot_changed_records": hot_commit.changed_records.len(),
                    "hot_diagnostic_artifact_count": hot_diagnostic_artifact_count,
                    "hot_detailed_trace_entries": hot_detailed_trace_entries,
                    "replay_mismatch_count": replay.mismatches.len(),
                    "replay_failure": replay.failure.as_ref().map(|failure| format!("{failure:?}")),
                    "hot_authority_status": format!(
                        "{:?}",
                        runtime.compiled_artifacts().compiled_artifact_authority_status(hot_artifact.artifact_id)
                    ),
                    "cold_authority_status": format!(
                        "{:?}",
                        recovered.compiled_artifacts().compiled_artifact_authority_status(cold_artifact.artifact_id)
                    ),
                    "phase_timing": {
                        "hot_commit_micros": hot_commit_micros,
                        "hot_compile_micros": hot_compile_micros,
                        "checkpoint_micros": checkpoint_micros,
                        "recover_micros": recover_micros,
                        "replay_commit_micros": replay_commit_micros,
                        "cold_compile_micros": cold_compile_micros,
                    },
                    "hot_counters": runtime.performance_access().counters(),
                    "cold_counters": recovered.performance_access().counters(),
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "chip_rich_compile_hot_vs_recovery_compile",
        &chip_rich_compile_samples,
        &[
            ("hot_commit_micros", &["phase_timing", "hot_commit_micros"]),
            (
                "hot_compile_micros",
                &["phase_timing", "hot_compile_micros"],
            ),
            ("checkpoint_micros", &["phase_timing", "checkpoint_micros"]),
            ("recover_micros", &["phase_timing", "recover_micros"]),
            (
                "replay_commit_micros",
                &["phase_timing", "replay_commit_micros"],
            ),
            (
                "cold_compile_micros",
                &["phase_timing", "cold_compile_micros"],
            ),
            (
                "hot_diagnostic_artifact_count",
                &["hot_diagnostic_artifact_count"],
            ),
            (
                "hot_detailed_trace_entries",
                &["hot_detailed_trace_entries"],
            ),
        ],
    );
    assert!(chip_rich_compile_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &chip_rich_compile_samples,
        "chip rich hot/cold certification should isolate compile and diagnostics on the hot side while preserving recovery-compile equivalence",
        |metrics| {
            metrics["hot_changed_records"].as_u64() == Some(1)
                && metrics["hot_diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["hot_detailed_trace_entries"].as_u64().unwrap_or(0) >= 1
                && metrics["replay_failure"].is_null()
                && metrics["replay_mismatch_count"].as_u64() == Some(0)
                && metrics["hot_authority_status"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactAuthorityStatus::Authoritative))
                && metrics["cold_authority_status"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactAuthorityStatus::Authoritative))
                && metrics["phase_timing"]["hot_compile_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["recover_micros"].as_u64().unwrap_or(0) > 0
                && metrics["phase_timing"]["cold_compile_micros"].as_u64().unwrap_or(0) > 0
                && metrics["hot_counters"]["full_state_clones"].as_u64() == Some(0)
                && metrics["cold_counters"]["full_state_clones"].as_u64() == Some(0)
        },
    );
}
