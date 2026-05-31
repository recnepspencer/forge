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
mod commit_delta_matrix;
mod durability_append_matrix;
mod geometry_artifact_decomposition_matrix;
mod geometry_kernel_matrix;
mod index_parity_matrix;
mod inspection_budget_matrix;
mod invariant_materialization_matrix;
mod invariant_support;
mod measurement_support;
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
mod snapshot_materialization_matrix;

use bridge_runtime_support::*;
use invariant_support::*;
use measurement_support::*;
use rocketship_bulk_intents::*;
use rocketship_layout::*;
use rocketship_pseudorealistic::*;

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_chip_simulator_matrix() {
    let suite = "chip_simulator_matrix";

    let fanout_compile_samples = capture_perf_samples(suite, "dense_fanout_compile_wave", || {
        let mut runtime =
            runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
        let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
        let source = create_entity_in_partition(&mut runtime, "net-driver", PartitionId(7));
        let targets = (0..24)
            .map(|index| {
                let partition_id = match index % 4 {
                    0 => PartitionId(11),
                    1 => PartitionId(13),
                    2 => PartitionId(17),
                    _ => PartitionId(19),
                };
                create_entity_in_partition(&mut runtime, &format!("net-sink-{index}"), partition_id)
            })
            .collect::<Vec<_>>();

        runtime.performance_access().reset_counters();
        let commit_started_at = Instant::now();
        let commit_outcome = {
            let mut txn = runtime.begin_transaction(TransactionOptions::default());
            let mut batch = WorkerIntentBatch::new("chip-fanout-wave");
            for (index, target) in targets.iter().enumerate() {
                batch = batch.push(MutationIntent::Create(CreateIntent::Relation(
                    crate::transactions::data::RelationSpec {
                        partition_id: PartitionId(29),
                        kind_id: KindId(2),
                        client_key: crate::symbols::data::ClientKey::raw(format!(
                            "chip-fanout-{index}"
                        )),
                        source: crate::transactions::data::EntityReference::Existing(source),
                        target: crate::transactions::data::EntityReference::Existing(*target),
                        fields: crate::transactions::data::AspectFieldPatch::default(),
                    },
                )));
            }
            txn.push_batch(batch);
            txn.commit().expect("chip fanout relation burst commit")
        };
        let commit_micros = commit_started_at.elapsed().as_micros();
        let commit = runtime
            .history()
            .latest_commit()
            .expect("chip fanout commit")
            .clone();

        let compile_started_at = Instant::now();
        let artifact = runtime
            .compiled_artifacts_authority()
            .compile_execution_artifact(
                commit.commit_id,
                vec![
                    PartitionId(7),
                    PartitionId(11),
                    PartitionId(13),
                    PartitionId(17),
                    PartitionId(19),
                    PartitionId(29),
                ],
            )
            .expect("chip fanout compiled artifact");
        let compile_micros = compile_started_at.elapsed().as_micros();

        let adjacency_started_at = Instant::now();
        let outgoing_relations = runtime
            .storage_access()
            .outgoing_relations_for_entity(source, commit.version_id);
        let adjacency_micros = adjacency_started_at.elapsed().as_micros();

        let counters = runtime.performance_access().counters();
        let (diagnostic_artifact_count, detailed_trace_entries) =
            fresh_diagnostics_metrics(&runtime, diagnostics_start);

        PerfMeasurement {
            elapsed_micros: commit_micros + compile_micros + adjacency_micros,
            metrics: perf_metrics!({
                "commit_micros": commit_micros,
                "compile_micros": compile_micros,
                "adjacency_micros": adjacency_micros,
                "changed_records": commit_outcome.changed_records.len(),
                "dense_patch_record_count": dense_patch_record_count(&runtime),
                "outgoing_relation_count": outgoing_relations.len(),
                "diagnostic_artifact_count": diagnostic_artifact_count,
                "detailed_trace_entries": detailed_trace_entries,
                "profile_boundary": profile_boundary_metrics(
                    &runtime,
                    RelationalRuntimeProfile::ChipSimulation,
                ),
                "adjacency_backend": format!("{:?}", runtime.config().storage.adjacency_policy.backend),
                "compiled_artifact_authority_status": format!(
                    "{:?}",
                    runtime
                        .compiled_artifacts()
                        .compiled_artifact_authority_status(artifact.artifact_id)
                ),
                "counters": counters,
            }),
        }
    });
    emit_metric_summaries(
        suite,
        "dense_fanout_compile_wave",
        &fanout_compile_samples,
        &[
            ("commit_micros", &["commit_micros"]),
            ("compile_micros", &["compile_micros"]),
            ("adjacency_micros", &["adjacency_micros"]),
            ("changed_records", &["changed_records"]),
            ("dense_patch_record_count", &["dense_patch_record_count"]),
            ("outgoing_relation_count", &["outgoing_relation_count"]),
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
    assert!(fanout_compile_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &fanout_compile_samples,
        "chip fanout compile wave should preserve compressed adjacency truth and dense patch shape",
        |metrics| {
            metrics["changed_records"].as_u64() == Some(24)
                && metrics["dense_patch_record_count"].as_u64() == Some(24)
                && metrics["outgoing_relation_count"].as_u64() == Some(24)
                && metrics["adjacency_backend"].as_str()
                    == Some(&format!(
                        "{:?}",
                        AdjacencyBackend::CompressedFanoutAdjacency
                    ))
                && metrics["compiled_artifact_authority_status"].as_str()
                    == Some(&format!(
                        "{:?}",
                        CompiledArtifactAuthorityStatus::Authoritative
                    ))
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(1)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(1)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(1)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "relation_slots_touched_by_commit") == 24
        },
    );

    let rich_fanout_compile_samples = capture_perf_samples(
        suite,
        "dense_fanout_compile_wave_rich_diagnostics",
        || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::ChipRichCertification,
            );
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let source =
                create_entity_in_partition(&mut runtime, "rich-net-driver", PartitionId(7));
            let targets = (0..24)
                .map(|index| {
                    let partition_id = match index % 4 {
                        0 => PartitionId(11),
                        1 => PartitionId(13),
                        2 => PartitionId(17),
                        _ => PartitionId(19),
                    };
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("rich-net-sink-{index}"),
                        partition_id,
                    )
                })
                .collect::<Vec<_>>();

            runtime.performance_access().reset_counters();
            let commit_started_at = Instant::now();
            let commit_outcome = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                let mut batch = WorkerIntentBatch::new("chip-fanout-wave-rich");
                for (index, target) in targets.iter().enumerate() {
                    batch = batch.push(MutationIntent::Create(CreateIntent::Relation(
                        crate::transactions::data::RelationSpec {
                            partition_id: PartitionId(29),
                            kind_id: KindId(2),
                            client_key: crate::symbols::data::ClientKey::raw(format!(
                                "chip-fanout-rich-{index}"
                            )),
                            source: crate::transactions::data::EntityReference::Existing(source),
                            target: crate::transactions::data::EntityReference::Existing(*target),
                            fields: crate::transactions::data::AspectFieldPatch::default(),
                        },
                    )));
                }
                txn.push_batch(batch);
                txn.commit()
                    .expect("chip fanout relation burst commit with rich diagnostics")
            };
            let commit_micros = commit_started_at.elapsed().as_micros();
            let commit = runtime
                .history()
                .latest_commit()
                .expect("chip fanout rich commit")
                .clone();

            let compile_started_at = Instant::now();
            let artifact = runtime
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(13),
                        PartitionId(17),
                        PartitionId(19),
                        PartitionId(29),
                    ],
                )
                .expect("chip fanout rich compiled artifact");
            let compile_micros = compile_started_at.elapsed().as_micros();

            let adjacency_started_at = Instant::now();
            let outgoing_relations = runtime
                .storage_access()
                .outgoing_relations_for_entity(source, commit.version_id);
            let adjacency_micros = adjacency_started_at.elapsed().as_micros();

            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            PerfMeasurement {
                elapsed_micros: commit_micros + compile_micros + adjacency_micros,
                metrics: perf_metrics!({
                    "commit_micros": commit_micros,
                    "compile_micros": compile_micros,
                    "adjacency_micros": adjacency_micros,
                    "changed_records": commit_outcome.changed_records.len(),
                    "dense_patch_record_count": dense_patch_record_count(&runtime),
                    "outgoing_relation_count": outgoing_relations.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "profile_boundary": profile_boundary_metrics(
                        &runtime,
                        RelationalRuntimeProfile::ChipSimulation,
                    ),
                    "adjacency_backend": format!("{:?}", runtime.config().storage.adjacency_policy.backend),
                    "compiled_artifact_authority_status": format!(
                        "{:?}",
                        runtime
                            .compiled_artifacts()
                            .compiled_artifact_authority_status(artifact.artifact_id)
                    ),
                    "counters": counters,
                }),
            }
        },
    );
    emit_metric_summaries(
        suite,
        "dense_fanout_compile_wave_rich_diagnostics",
        &rich_fanout_compile_samples,
        &[
            ("commit_micros", &["commit_micros"]),
            ("compile_micros", &["compile_micros"]),
            ("adjacency_micros", &["adjacency_micros"]),
            ("changed_records", &["changed_records"]),
            ("dense_patch_record_count", &["dense_patch_record_count"]),
            ("outgoing_relation_count", &["outgoing_relation_count"]),
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
    assert!(rich_fanout_compile_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &rich_fanout_compile_samples,
        "chip rich fanout compile wave should preserve the same dense truth while surfacing trace cost",
        |metrics| {
            metrics["changed_records"].as_u64() == Some(24)
                && metrics["dense_patch_record_count"].as_u64() == Some(24)
                && metrics["outgoing_relation_count"].as_u64() == Some(24)
                && metrics["adjacency_backend"].as_str()
                    == Some(&format!("{:?}", AdjacencyBackend::CompressedFanoutAdjacency))
                && metrics["compiled_artifact_authority_status"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactAuthorityStatus::Authoritative))
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64().unwrap_or(0) >= 1
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(1)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(1)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "relation_slots_touched_by_commit") == 24
        },
    );

    let checkpoint_recover_compile_samples = capture_perf_samples(
        suite,
        "checkpoint_window_recover_compile_round_trip",
        || {
            let mut runtime = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::ChipSimulation,
            );
            let source =
                create_entity_in_partition(&mut runtime, "persisted-driver", PartitionId(7));
            let targets = (0..12)
                .map(|index| {
                    let partition_id = match index % 3 {
                        0 => PartitionId(11),
                        1 => PartitionId(13),
                        _ => PartitionId(17),
                    };
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("persisted-sink-{index}"),
                        partition_id,
                    )
                })
                .collect::<Vec<_>>();
            for (index, target) in targets.iter().enumerate() {
                create_relation_in_partition(
                    &mut runtime,
                    source,
                    *target,
                    &format!("persisted-edge-{index}"),
                    PartitionId(29),
                );
            }

            let checkpoint_started_at = Instant::now();
            runtime
                .durability_authority()
                .checkpoint()
                .expect("chip checkpoint window");
            let checkpoint_micros = checkpoint_started_at.elapsed().as_micros();

            let plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let mut recovered = persisted_runtime_with_test_schema_profile(
                RelationalRuntimeProfile::ChipSimulation,
            );
            let recover_started_at = Instant::now();
            recovered
                .durability_authority()
                .recover(plan)
                .expect("chip checkpoint recovery");
            let recover_micros = recover_started_at.elapsed().as_micros();

            let recovered_commit = recovered
                .history()
                .latest_commit()
                .expect("recovered chip commit")
                .clone();
            let compile_started_at = Instant::now();
            let artifact = recovered
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    recovered_commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(13),
                        PartitionId(17),
                        PartitionId(29),
                    ],
                )
                .expect("recovered chip compiled artifact");
            let compile_micros = compile_started_at.elapsed().as_micros();

            let adjacency_started_at = Instant::now();
            let outgoing_relations = recovered
                .storage_access()
                .outgoing_relations_for_entity(source, recovered_commit.version_id);
            let adjacency_micros = adjacency_started_at.elapsed().as_micros();

            PerfMeasurement {
                elapsed_micros: checkpoint_micros
                    + recover_micros
                    + compile_micros
                    + adjacency_micros,
                metrics: perf_metrics!({
                    "checkpoint_micros": checkpoint_micros,
                    "recover_micros": recover_micros,
                    "compile_micros": compile_micros,
                    "adjacency_micros": adjacency_micros,
                    "recovered_segment_count": recovered
                        .durability()
                        .durable_log()
                        .len(),
                    "outgoing_relation_count": outgoing_relations.len(),
                    "compiled_artifact_authority_status": format!(
                        "{:?}",
                        recovered
                            .compiled_artifacts()
                            .compiled_artifact_authority_status(artifact.artifact_id)
                    ),
                    "counters": recovered.performance_access().counters(),
                }),
            }
        },
    );
    emit_metric_summaries(
        suite,
        "checkpoint_window_recover_compile_round_trip",
        &checkpoint_recover_compile_samples,
        &[
            ("checkpoint_micros", &["checkpoint_micros"]),
            ("recover_micros", &["recover_micros"]),
            ("compile_micros", &["compile_micros"]),
            ("adjacency_micros", &["adjacency_micros"]),
            ("recovered_segment_count", &["recovered_segment_count"]),
            ("outgoing_relation_count", &["outgoing_relation_count"]),
        ],
    );
    assert!(checkpoint_recover_compile_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &checkpoint_recover_compile_samples,
        "chip checkpoint window recovery should preserve compile-ready fanout truth after recovery",
        |metrics| {
            metrics["checkpoint_micros"].as_u64().unwrap_or(0) > 0
                && metrics["recover_micros"].as_u64().unwrap_or(0) > 0
                && metrics["outgoing_relation_count"].as_u64() == Some(12)
                && metrics["compiled_artifact_authority_status"].as_str()
                    == Some(&format!(
                        "{:?}",
                        CompiledArtifactAuthorityStatus::Authoritative
                    ))
                && counter_u64(metrics, "full_state_clones") == 0
        },
    );

    let branch_rollback_compile_samples =
        capture_perf_samples(suite, "branch_rollback_compile_step_window", || {
            let feature_branch = BranchId("feature".to_string());
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let source =
                create_entity_in_partition(&mut runtime, "rollback-driver", PartitionId(7));
            let stable_targets = (0..8)
                .map(|index| {
                    let partition_id = match index % 4 {
                        0 => PartitionId(11),
                        1 => PartitionId(13),
                        2 => PartitionId(17),
                        _ => PartitionId(19),
                    };
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("rollback-stable-sink-{index}"),
                        partition_id,
                    )
                })
                .collect::<Vec<_>>();
            let transient_targets = (0..8)
                .map(|index| {
                    let partition_id = match index % 4 {
                        0 => PartitionId(23),
                        1 => PartitionId(31),
                        2 => PartitionId(37),
                        _ => PartitionId(41),
                    };
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("rollback-transient-sink-{index}"),
                        partition_id,
                    )
                })
                .collect::<Vec<_>>();
            create_branch_from_main(&mut runtime, "feature");
            for (index, target) in stable_targets.iter().enumerate() {
                create_relation_in_partition_on_branch(
                    &mut runtime,
                    source,
                    *target,
                    &format!("rollback-stable-edge-{index}"),
                    "stable",
                    PartitionId(29),
                    feature_branch.clone(),
                );
            }

            runtime.performance_access().reset_counters();
            let mut txn = runtime.begin_transaction(TransactionOptions {
                target_branch: Some(feature_branch.clone()),
                ..TransactionOptions::default()
            });
            let savepoint = txn.create_savepoint();
            let mut transient_batch = WorkerIntentBatch::new("chip-transient-fanout");
            for (index, target) in transient_targets.iter().enumerate() {
                transient_batch = transient_batch.push(MutationIntent::Create(
                    CreateIntent::Relation(crate::transactions::data::RelationSpec {
                        partition_id: PartitionId(43),
                        kind_id: KindId(2),
                        client_key: crate::symbols::data::ClientKey::raw(format!(
                            "rollback-transient-edge-{index}"
                        )),
                        source: crate::transactions::data::EntityReference::Existing(source),
                        target: crate::transactions::data::EntityReference::Existing(*target),
                        fields: crate::transactions::data::AspectFieldPatch::default(),
                    }),
                ));
            }
            txn.push_batch(transient_batch);

            let rollback_started_at = Instant::now();
            let rollback = txn
                .rollback_to_savepoint(savepoint)
                .expect("chip savepoint rollback");
            let rollback_micros = rollback_started_at.elapsed().as_micros();

            txn.push_batch(
                WorkerIntentBatch::new("chip-committed-step").push(
                    MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                        UpdateEntityFieldsIntent {
                            entity_id: source,
                            fields: crate::tests::support::aspect_field_patch_from_values([
                                (
                                    crate::tests::support::aspect_key("name"),
                                    crate::tests::support::field_key("name"),
                                    crate::tests::support::string_aspect_value("rollback-driver"),
                                ),
                                (
                                    crate::tests::support::aspect_key("step"),
                                    crate::tests::support::field_key("step"),
                                    crate::tests::support::u64_aspect_value(1),
                                ),
                                (
                                    crate::tests::support::aspect_key("branch"),
                                    crate::tests::support::field_key("branch"),
                                    crate::tests::support::string_aspect_value("feature"),
                                ),
                            ]),
                        },
                    ))
                    .into(),
                ),
            );
            let commit_started_at = Instant::now();
            let commit_outcome = txn.commit().expect("chip branch step commit");
            let commit_micros = commit_started_at.elapsed().as_micros();

            let feature_commit = runtime
                .history()
                .branch_head(&feature_branch)
                .expect("feature branch head")
                .clone();
            let compile_started_at = Instant::now();
            let artifact = runtime
                .compiled_artifacts_authority()
                .compile_execution_artifact(
                    feature_commit.commit_id,
                    vec![
                        PartitionId(7),
                        PartitionId(11),
                        PartitionId(13),
                        PartitionId(17),
                        PartitionId(19),
                        PartitionId(29),
                    ],
                )
                .expect("feature branch compiled artifact");
            let compile_micros = compile_started_at.elapsed().as_micros();

            let adjacency_started_at = Instant::now();
            let outgoing_relations = runtime
                .storage_access()
                .outgoing_relations_for_entity(source, feature_commit.version_id);
            let adjacency_micros = adjacency_started_at.elapsed().as_micros();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            PerfMeasurement {
                elapsed_micros: rollback_micros + commit_micros + compile_micros + adjacency_micros,
                metrics: perf_metrics!({
                    "rollback_micros": rollback_micros,
                    "commit_micros": commit_micros,
                    "compile_micros": compile_micros,
                    "adjacency_micros": adjacency_micros,
                    "rollback_effect_count": rollback.effect_count(),
                    "rollback_discarded_creations": rollback.summary.discarded_creation_count(),
                    "rollback_restored_records": rollback.summary.restored_record_count(),
                    "committed_changed_records": commit_outcome.changed_records.len(),
                    "outgoing_relation_count": outgoing_relations.len(),
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "compiled_artifact_authority_status": format!(
                        "{:?}",
                        runtime
                            .compiled_artifacts()
                            .compiled_artifact_authority_status(artifact.artifact_id)
                    ),
                    "counters": runtime.performance_access().counters(),
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "branch_rollback_compile_step_window",
        &branch_rollback_compile_samples,
        &[
            ("rollback_micros", &["rollback_micros"]),
            ("commit_micros", &["commit_micros"]),
            ("compile_micros", &["compile_micros"]),
            ("adjacency_micros", &["adjacency_micros"]),
            ("rollback_effect_count", &["rollback_effect_count"]),
            (
                "rollback_discarded_creations",
                &["rollback_discarded_creations"],
            ),
            ("committed_changed_records", &["committed_changed_records"]),
            ("outgoing_relation_count", &["outgoing_relation_count"]),
        ],
    );
    assert!(branch_rollback_compile_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &branch_rollback_compile_samples,
        "chip branch rollback compile windows should discard abandoned fanout work and keep feature truth narrow",
        |metrics| {
            metrics["rollback_effect_count"].as_u64() == Some(8)
                && metrics["rollback_discarded_creations"].as_u64() == Some(8)
                && metrics["rollback_restored_records"].as_u64() == Some(0)
                && metrics["committed_changed_records"].as_u64() == Some(1)
                && metrics["outgoing_relation_count"].as_u64() == Some(8)
                && metrics["compiled_artifact_authority_status"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactAuthorityStatus::Authoritative))
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && counter_u64(metrics, "full_state_clones") == 0
        },
    );

    let flat_step_batch_samples = capture_perf_samples(
        suite,
        "flat_entity_step_batch_compile_window",
        || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
            runtime.config.diagnostics.profile.detailed_traces_enabled = false;
            runtime.config.diagnostics.profile.max_entries_per_artifact = 0;
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();

            let _source =
                create_entity_in_partition(&mut runtime, "chip-batch-driver", PartitionId(7));
            let mut partition_targets = BTreeMap::new();
            for partition_offset in 0..8u32 {
                let partition_id = PartitionId(11 + partition_offset * 2);
                let targets = (0..8)
                    .map(|index| {
                        create_entity_in_partition(
                            &mut runtime,
                            &format!("chip-batch-sink-{}-{index}", partition_id.0),
                            partition_id,
                        )
                    })
                    .collect::<Vec<_>>();
                partition_targets.insert(partition_id, targets);
            }
            let compile_partitions = std::iter::once(PartitionId(7))
                .chain(partition_targets.keys().copied())
                .collect::<Vec<_>>();

            runtime.performance_access().reset_counters();
            let update_started_at = Instant::now();
            let update = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                let mut batch = WorkerIntentBatch::new("chip-flat-entity-step-batch");
                for (partition_id, targets) in &partition_targets {
                    for (index, entity) in targets.iter().enumerate().take(4) {
                        batch = batch.push(MutationIntent::Entity(
                            EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                                entity_id: *entity,
                                fields: crate::tests::support::aspect_field_patch_from_values([
                                    (
                                        crate::tests::support::aspect_key("partition"),
                                        crate::tests::support::field_key("partition"),
                                        crate::tests::support::u64_aspect_value(
                                            partition_id.0 as u64,
                                        ),
                                    ),
                                    (
                                        crate::tests::support::aspect_key("lane"),
                                        crate::tests::support::field_key("lane"),
                                        crate::tests::support::string_aspect_value("global-step"),
                                    ),
                                    (
                                        crate::tests::support::aspect_key("step"),
                                        crate::tests::support::field_key("step"),
                                        crate::tests::support::usize_aspect_value(index),
                                    ),
                                ]),
                            }),
                        ));
                    }
                }
                txn.push_batch(batch);
                txn.commit().expect("chip flat entity step batch commit")
            };
            let update_micros = update_started_at.elapsed().as_micros();
            let phase_timing = update.execution.phase_timing.clone();
            let commit = runtime
                .history()
                .latest_commit()
                .expect("chip flat batch latest commit")
                .clone();

            let compile_started_at = Instant::now();
            let artifact = runtime
                .compiled_artifacts_authority()
                .compile_execution_artifact(commit.commit_id, compile_partitions)
                .expect("chip flat batch compiled artifact");
            let compile_micros = compile_started_at.elapsed().as_micros();

            let sample_targets = partition_targets
                .values()
                .flat_map(|targets| targets.iter().take(1).copied())
                .map(RecordRef::Entity)
                .collect::<Vec<_>>();
            let snapshot = runtime.visibility_authority().snapshot();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "chip-flat-batch-explicit",
                sample_targets,
            );
            let explicit_started_at = Instant::now();
            let explicit = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("planned chip flat batch explicit query"),
                )
                .expect("chip flat batch explicit outcome");
            let explicit_query_micros = explicit_started_at.elapsed().as_micros();
            assert!(runtime.visibility_authority().release_snapshot(&snapshot));

            let counters = runtime.performance_access().counters();
            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);

            measurement_with_elapsed(
                update_micros + compile_micros + explicit_query_micros,
                || {
                    perf_metrics!({
                        "batch_target_count": 32,
                        "batch_partition_count": partition_targets.len(),
                        "update_micros": update_micros,
                        "compile_micros": compile_micros,
                        "explicit_query_micros": explicit_query_micros,
                        "hot_changed_records": update.changed_records.len(),
                        "explicit_result_entities": explicit.result.entities.len(),
                        "diagnostic_artifact_count": diagnostic_artifact_count,
                        "detailed_trace_entries": detailed_trace_entries,
                        "compiled_artifact_authority_status": format!(
                            "{:?}",
                            runtime
                                .compiled_artifacts()
                                .compiled_artifact_authority_status(artifact.artifact_id)
                        ),
                        "phase_timing": {
                            "draft_preparation_micros": phase_timing.draft_preparation_micros,
                            "draft_working_state_clone_micros": phase_timing.draft_working_state_clone_micros,
                            "publication_storage_commit_micros": phase_timing.publication_storage_commit_micros,
                        },
                        "counters": counters,
                    })
                },
            )
        },
    );
    emit_metric_summaries(
        suite,
        "flat_entity_step_batch_compile_window",
        &flat_step_batch_samples,
        &[
            ("batch_target_count", &["batch_target_count"]),
            ("batch_partition_count", &["batch_partition_count"]),
            ("update_micros", &["update_micros"]),
            ("compile_micros", &["compile_micros"]),
            ("explicit_query_micros", &["explicit_query_micros"]),
            (
                "draft_preparation_micros",
                &["phase_timing", "draft_preparation_micros"],
            ),
            (
                "draft_working_state_clone_micros",
                &["phase_timing", "draft_working_state_clone_micros"],
            ),
            (
                "publication_storage_commit_micros",
                &["phase_timing", "publication_storage_commit_micros"],
            ),
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
    assert!(flat_step_batch_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &flat_step_batch_samples,
        "chip flat entity step batches should stay on the widened sparse AoSoA path while remaining compile-ready",
        |metrics| {
            metrics["batch_target_count"].as_u64() == Some(32)
                && metrics["batch_partition_count"].as_u64() == Some(8)
                && metrics["hot_changed_records"].as_u64() == Some(32)
                && metrics["explicit_result_entities"].as_u64() == Some(8)
                && metrics["compiled_artifact_authority_status"].as_str()
                    == Some(&format!("{:?}", CompiledArtifactAuthorityStatus::Authoritative))
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 1
                && metrics["detailed_trace_entries"].as_u64() == Some(0)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "entity_slots_touched_by_commit") == 32
                && counter_u64(metrics, "partitions_touched_by_commit") >= 8
                && counter_u64(metrics, "aosoa_entity_chunk_slots_materialized") == 32
                && counter_u64(metrics, "aosoa_entity_chunks_published") >= 8
                && counter_u64(metrics, "aosoa_publish_soa_merge_count") == 0
        },
    );

    let event_wave_churn_samples =
        capture_perf_samples(suite, "event_wave_compile_churn_window", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
            runtime.config.diagnostics.profile.detailed_traces_enabled = false;
            runtime.config.diagnostics.profile.max_entries_per_artifact = 0;
            let source = create_entity_in_partition(&mut runtime, "event-driver", PartitionId(7));
            let sinks = (0..16)
                .map(|index| {
                    let partition_id = match index % 4 {
                        0 => PartitionId(11),
                        1 => PartitionId(13),
                        2 => PartitionId(17),
                        _ => PartitionId(19),
                    };
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("event-sink-{index}"),
                        partition_id,
                    )
                })
                .collect::<Vec<_>>();
            for (index, sink) in sinks.iter().enumerate() {
                create_relation_in_partition(
                    &mut runtime,
                    source,
                    *sink,
                    &format!("event-link-{index}"),
                    PartitionId(29),
                );
            }

            const ITERATIONS: usize = 24;
            let mut total_update_micros = 0u128;
            let mut total_compile_micros = 0u128;
            let mut total_adjacency_micros = 0u128;
            let mut max_compile_micros = 0u128;
            let mut max_outgoing_relation_count = 0usize;

            runtime.performance_access().reset_counters();
            for step in 0..ITERATIONS {
                let update_started_at = Instant::now();
                let _ = update_entity(&mut runtime, source, &format!("event-driver-step-{step}"));
                total_update_micros += update_started_at.elapsed().as_micros();

                let commit = runtime
                    .history()
                    .latest_commit()
                    .expect("chip event-wave commit")
                    .clone();
                let compile_started_at = Instant::now();
                let artifact = runtime
                    .compiled_artifacts_authority()
                    .compile_execution_artifact(
                        commit.commit_id,
                        vec![
                            PartitionId(7),
                            PartitionId(11),
                            PartitionId(13),
                            PartitionId(17),
                            PartitionId(19),
                            PartitionId(29),
                        ],
                    )
                    .expect("chip event-wave compiled artifact");
                let compile_micros = compile_started_at.elapsed().as_micros();
                total_compile_micros += compile_micros;
                max_compile_micros = max_compile_micros.max(compile_micros);

                let adjacency_started_at = Instant::now();
                let outgoing_relations = runtime
                    .storage_access()
                    .outgoing_relations_for_entity(source, commit.version_id);
                total_adjacency_micros += adjacency_started_at.elapsed().as_micros();
                max_outgoing_relation_count =
                    max_outgoing_relation_count.max(outgoing_relations.len());
                assert_eq!(
                    runtime
                        .compiled_artifacts()
                        .compiled_artifact_authority_status(artifact.artifact_id),
                    CompiledArtifactAuthorityStatus::Authoritative
                );
            }

            PerfMeasurement {
                elapsed_micros: total_update_micros + total_compile_micros + total_adjacency_micros,
                metrics: perf_metrics!({
                    "iterations": ITERATIONS,
                    "average_update_micros": total_update_micros / ITERATIONS as u128,
                    "average_compile_micros": total_compile_micros / ITERATIONS as u128,
                    "average_adjacency_micros": total_adjacency_micros / ITERATIONS as u128,
                    "max_compile_micros": max_compile_micros,
                    "max_outgoing_relation_count": max_outgoing_relation_count,
                    "counters": runtime.performance_access().counters(),
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "event_wave_compile_churn_window",
        &event_wave_churn_samples,
        &[
            ("average_update_micros", &["average_update_micros"]),
            ("average_compile_micros", &["average_compile_micros"]),
            ("average_adjacency_micros", &["average_adjacency_micros"]),
            ("max_compile_micros", &["max_compile_micros"]),
            (
                "max_outgoing_relation_count",
                &["max_outgoing_relation_count"],
            ),
        ],
    );
    assert!(event_wave_churn_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &event_wave_churn_samples,
        "chip event-wave churn should keep repeated compile windows supported and bounded under sustained stepping",
        |metrics| {
            metrics["iterations"].as_u64() == Some(24)
                && metrics["max_outgoing_relation_count"].as_u64() == Some(16)
                && metrics["max_compile_micros"].as_u64().unwrap_or(0) > 0
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "bulk_mutation_entity_target_count") == 24
        },
    );

    let event_wave_rich_diagnostics_samples =
        capture_perf_samples(suite, "event_wave_compile_churn_rich_diagnostics", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
            let diagnostics_start = runtime.publication().diagnostic_artifacts().len();
            let source =
                create_entity_in_partition(&mut runtime, "event-driver-rich", PartitionId(7));
            let sinks = (0..16)
                .map(|index| {
                    let partition_id = match index % 4 {
                        0 => PartitionId(11),
                        1 => PartitionId(13),
                        2 => PartitionId(17),
                        _ => PartitionId(19),
                    };
                    create_entity_in_partition(
                        &mut runtime,
                        &format!("event-sink-rich-{index}"),
                        partition_id,
                    )
                })
                .collect::<Vec<_>>();
            for (index, sink) in sinks.iter().enumerate() {
                create_relation_in_partition(
                    &mut runtime,
                    source,
                    *sink,
                    &format!("event-link-rich-{index}"),
                    PartitionId(29),
                );
            }

            const ITERATIONS: usize = 16;
            let mut total_update_micros = 0u128;
            let mut total_compile_micros = 0u128;
            let mut total_adjacency_micros = 0u128;
            let mut max_compile_micros = 0u128;
            let mut max_outgoing_relation_count = 0usize;

            runtime.performance_access().reset_counters();
            for step in 0..ITERATIONS {
                let update_started_at = Instant::now();
                let _ = update_entity(
                    &mut runtime,
                    source,
                    &format!("event-driver-rich-step-{step}"),
                );
                total_update_micros += update_started_at.elapsed().as_micros();

                let commit = runtime
                    .history()
                    .latest_commit()
                    .expect("chip event-wave rich commit")
                    .clone();
                let compile_started_at = Instant::now();
                let artifact = runtime
                    .compiled_artifacts_authority()
                    .compile_execution_artifact(
                        commit.commit_id,
                        vec![
                            PartitionId(7),
                            PartitionId(11),
                            PartitionId(13),
                            PartitionId(17),
                            PartitionId(19),
                            PartitionId(29),
                        ],
                    )
                    .expect("chip event-wave rich compiled artifact");
                let compile_micros = compile_started_at.elapsed().as_micros();
                total_compile_micros += compile_micros;
                max_compile_micros = max_compile_micros.max(compile_micros);

                let adjacency_started_at = Instant::now();
                let outgoing_relations = runtime
                    .storage_access()
                    .outgoing_relations_for_entity(source, commit.version_id);
                total_adjacency_micros += adjacency_started_at.elapsed().as_micros();
                max_outgoing_relation_count =
                    max_outgoing_relation_count.max(outgoing_relations.len());
                assert_eq!(
                    runtime
                        .compiled_artifacts()
                        .compiled_artifact_authority_status(artifact.artifact_id),
                    CompiledArtifactAuthorityStatus::Authoritative
                );
            }

            let (diagnostic_artifact_count, detailed_trace_entries) =
                fresh_diagnostics_metrics(&runtime, diagnostics_start);
            PerfMeasurement {
                elapsed_micros: total_update_micros + total_compile_micros + total_adjacency_micros,
                metrics: perf_metrics!({
                    "iterations": ITERATIONS,
                    "average_update_micros": total_update_micros / ITERATIONS as u128,
                    "average_compile_micros": total_compile_micros / ITERATIONS as u128,
                    "average_adjacency_micros": total_adjacency_micros / ITERATIONS as u128,
                    "max_compile_micros": max_compile_micros,
                    "max_outgoing_relation_count": max_outgoing_relation_count,
                    "diagnostic_artifact_count": diagnostic_artifact_count,
                    "detailed_trace_entries": detailed_trace_entries,
                    "counters": runtime.performance_access().counters(),
                }),
            }
        });
    emit_metric_summaries(
        suite,
        "event_wave_compile_churn_rich_diagnostics",
        &event_wave_rich_diagnostics_samples,
        &[
            ("average_update_micros", &["average_update_micros"]),
            ("average_compile_micros", &["average_compile_micros"]),
            ("average_adjacency_micros", &["average_adjacency_micros"]),
            ("max_compile_micros", &["max_compile_micros"]),
            (
                "max_outgoing_relation_count",
                &["max_outgoing_relation_count"],
            ),
            ("diagnostic_artifact_count", &["diagnostic_artifact_count"]),
            ("detailed_trace_entries", &["detailed_trace_entries"]),
        ],
    );
    assert!(event_wave_rich_diagnostics_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &event_wave_rich_diagnostics_samples,
        "chip event-wave rich diagnostics should keep compile windows supported while surfacing diagnostic cost clearly",
        |metrics| {
            metrics["iterations"].as_u64() == Some(16)
                && metrics["max_outgoing_relation_count"].as_u64() == Some(16)
                && metrics["max_compile_micros"].as_u64().unwrap_or(0) > 0
                && metrics["diagnostic_artifact_count"].as_u64().unwrap_or(0) >= 16
                && metrics["detailed_trace_entries"].as_u64().is_some()
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "bulk_mutation_entity_target_count") == 16
        },
    );
}

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
fn perf_workflow_matrix() {
    let suite = "workflow_matrix";

    let trade_correction_samples =
        capture_perf_samples(suite, "trade_correction_analysis_round_trip", || {
            let mut runtime = persisted_runtime_with_test_schema();
            let account =
                create_entity_in_partition(&mut runtime, "portfolio-account", PartitionId(10));
            create_branch_from_main(&mut runtime, "analysis");

            runtime.performance_access().reset_counters();
            let analysis_commit_started_at = Instant::now();
            let analysis_commit = {
                let mut txn = runtime.begin_transaction(TransactionOptions {
                    target_branch: Some(BranchId("analysis".to_string())),
                    ..TransactionOptions::default()
                });
                txn.push_batch(
                    WorkerIntentBatch::new("correct-trade").push(
                        MutationIntent::Create(CreateIntent::Entity(
                            crate::transactions::data::EntitySpec {
                                partition_id: PartitionId(10),
                                kind_id: KindId(1),
                                client_key: crate::symbols::data::ClientKey::raw(
                                    "analysis-trade-correction".to_string(),
                                ),
                                fields: crate::tests::support::string_aspect_field_patch([
                                    (
                                        crate::tests::support::aspect_key("entity_type"),
                                        crate::tests::support::field_key("entity_type"),
                                        "trade",
                                    ),
                                    (
                                        crate::tests::support::aspect_key("case"),
                                        crate::tests::support::field_key("case"),
                                        "trade-correction",
                                    ),
                                    (
                                        crate::tests::support::aspect_key("status"),
                                        crate::tests::support::field_key("status"),
                                        "corrected",
                                    ),
                                    (
                                        crate::tests::support::aspect_key("account"),
                                        crate::tests::support::field_key("account"),
                                        "portfolio-account",
                                    ),
                                ]),
                            },
                        ))
                        .into(),
                    ),
                );
                txn.push_batch(
                    WorkerIntentBatch::new("refresh-risk").push(
                        MutationIntent::Create(CreateIntent::Entity(
                            crate::transactions::data::EntitySpec {
                                partition_id: PartitionId(30),
                                kind_id: KindId(1),
                                client_key: crate::symbols::data::ClientKey::raw(
                                    "analysis-risk-refresh".to_string(),
                                ),
                                fields: crate::tests::support::string_aspect_field_patch([
                                    (
                                        crate::tests::support::aspect_key("entity_type"),
                                        crate::tests::support::field_key("entity_type"),
                                        "risk_view",
                                    ),
                                    (
                                        crate::tests::support::aspect_key("case"),
                                        crate::tests::support::field_key("case"),
                                        "trade-correction",
                                    ),
                                    (
                                        crate::tests::support::aspect_key("status"),
                                        crate::tests::support::field_key("status"),
                                        "refreshed",
                                    ),
                                    (
                                        crate::tests::support::aspect_key("severity"),
                                        crate::tests::support::field_key("severity"),
                                        "medium",
                                    ),
                                ]),
                            },
                        ))
                        .into(),
                    ),
                );
                txn.push_batch(
                    WorkerIntentBatch::new("emit-audit")
                        .push(MutationIntent::Create(CreateIntent::Entity(
                            crate::transactions::data::EntitySpec {
                                partition_id: PartitionId(40),
                                kind_id: KindId(1),
                                client_key: crate::symbols::data::ClientKey::raw(
                                    "analysis-audit-record".to_string(),
                                ),
                                fields: crate::tests::support::string_aspect_field_patch([
                                    (
                                        crate::tests::support::aspect_key("entity_type"),
                                        crate::tests::support::field_key("entity_type"),
                                        "audit_record",
                                    ),
                                    (
                                        crate::tests::support::aspect_key("case"),
                                        crate::tests::support::field_key("case"),
                                        "trade-correction",
                                    ),
                                    (
                                        crate::tests::support::aspect_key("event"),
                                        crate::tests::support::field_key("event"),
                                        "analysis-reviewed",
                                    ),
                                ]),
                            },
                        )))
                        .into(),
                );
                txn.commit().expect("analysis branch correction commit")
            };
            let analysis_commit_micros = analysis_commit_started_at.elapsed().as_micros();
            let analysis_entities = changed_entities(&analysis_commit);
            let trade = analysis_entities[0];
            let risk_view = analysis_entities[1];
            let audit_record = analysis_entities[2];

            let prepared = runtime
                .prepare_merge_execution(MergeExecutionRequest {
                    target_branch: BranchId("main".to_string()),
                    source_branch: BranchId("analysis".to_string()),
                    merge_intent: MergeIntent::ReconcileIntoTarget,
                })
                .expect("prepared analysis merge");
            let merge_started_at = Instant::now();
            let merge_outcome = runtime
                .execute_prepared_merge(prepared)
                .expect("analysis merge execution");
            let merge_execute_micros = merge_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "trade-correction-round-trip",
                vec![
                    RecordRef::Entity(account),
                    RecordRef::Entity(trade),
                    RecordRef::Entity(risk_view),
                    RecordRef::Entity(audit_record),
                ],
            );
            let query_started_at = Instant::now();
            let query_outcome = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, packet)
                        .expect("planned workflow query"),
                )
                .expect("workflow query outcome");
            let query_round_trip_micros = query_started_at.elapsed().as_micros();

            let elapsed_micros =
                analysis_commit_micros + merge_execute_micros + query_round_trip_micros;
            let counters = runtime.performance_access().counters();

            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "analysis_changed_records": analysis_commit.changed_records.len(),
                    "merged_changed_records": merge_outcome.commit.changed_records.len(),
                    "query_entities": query_outcome.result.entities.len(),
                    "query_relations": query_outcome.result.relations.len(),
                    "profile_boundary": profile_boundary_metrics(
                        &runtime,
                        RelationalRuntimeProfile::CertificationCore,
                    ),
                    "phase_timing": {
                        "analysis_commit_micros": analysis_commit_micros,
                        "merge_execute_micros": merge_execute_micros,
                        "query_round_trip_micros": query_round_trip_micros,
                    },
                    "counters": counters,
                })
            })
        });
    emit_metric_summaries(
        suite,
        "trade_correction_analysis_round_trip",
        &trade_correction_samples,
        &[
            (
                "analysis_commit_micros",
                &["phase_timing", "analysis_commit_micros"],
            ),
            (
                "merge_execute_micros",
                &["phase_timing", "merge_execute_micros"],
            ),
            (
                "query_round_trip_micros",
                &["phase_timing", "query_round_trip_micros"],
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
    assert!(trade_correction_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &trade_correction_samples,
        "workflow round trips should stay branch-local, merge one analysis patch, and query a narrow case surface",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "merge_execution_attempts") == 1
                && counter_u64(metrics, "partitions_touched_by_commit") <= 3
                && counter_u64(metrics, "query_packet_count") <= 3
                && metrics["analysis_changed_records"].as_u64() == Some(3)
                && metrics["merged_changed_records"].as_u64() == Some(3)
                && metrics["query_entities"].as_u64() == Some(4)
                && metrics["query_relations"].as_u64() == Some(0)
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(1)
        },
    );

    let fintech_intraday_risk_samples =
        capture_perf_samples(suite, "fintech_intraday_risk_branch_round_trip", || {
            let mut world = setup_intraday_risk_perf_world();
            let baseline_observability = perf_capture_baseline_observability(&world);
            let analysis = perf_open_analysis_branch(&mut world);

            world.runtime.performance_access().reset_counters();
            let stress_started_at = Instant::now();
            let stress_commit = perf_stress_intraday_risk(&mut world, analysis);
            let stress_commit_micros = stress_started_at.elapsed().as_micros();

            let query_started_at = Instant::now();
            let probe = perf_capture_intraday_risk_probe(&world);
            let query_probe_micros = query_started_at.elapsed().as_micros();
            let elapsed_micros = stress_commit_micros + query_probe_micros;
            let counters = world.runtime.performance_access().counters();
            let post_observability = perf_capture_post_mutation_observability(&world);

            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "changed_records": stress_commit.changed_records.len(),
                    "query_entities": probe.entity_count,
                    "query_relations": probe.relation_count,
                    "open_breach_count": probe.open_breach_count,
                    "diagnostic_artifact_delta": post_observability
                        .diagnostics_artifact_count
                        .saturating_sub(baseline_observability.diagnostics_artifact_count),
                    "latest_patch_present": post_observability.latest_patch_present,
                    "profile_boundary": profile_boundary_metrics(
                        &world.runtime,
                        RelationalRuntimeProfile::AiWorkflow,
                    ),
                    "phase_timing": {
                        "stress_commit_micros": stress_commit_micros,
                        "query_probe_micros": query_probe_micros,
                    },
                    "shape_metrics": {
                        "packet_count": counters.query_packet_count,
                        "scope_unit_count": counters.query_scope_unit_count,
                    },
                    "counters": counters,
                })
            })
        });
    emit_metric_summaries(
        suite,
        "fintech_intraday_risk_branch_round_trip",
        &fintech_intraday_risk_samples,
        &[
            (
                "stress_commit_micros",
                &["phase_timing", "stress_commit_micros"],
            ),
            (
                "query_probe_micros",
                &["phase_timing", "query_probe_micros"],
            ),
            ("packet_count", &["shape_metrics", "packet_count"]),
            ("scope_unit_count", &["shape_metrics", "scope_unit_count"]),
            ("diagnostic_artifact_delta", &["diagnostic_artifact_delta"]),
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
    assert!(fintech_intraday_risk_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &fintech_intraday_risk_samples,
        "fintech intraday risk should expose one open breach without widening beyond the case probe",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && metric_u64(metrics, "changed_records") == 4
                && metric_u64(metrics, "query_entities") == 4
                && metric_u64(metrics, "query_relations") == 0
                && metric_u64(metrics, "open_breach_count") == 1
                && metric_u64(metrics, "diagnostic_artifact_delta") >= 1
                && metrics["latest_patch_present"].as_bool() == Some(true)
                && counter_u64(metrics, "query_packet_count") <= 4
                && counter_u64(metrics, "query_scope_unit_count") <= 4
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(3)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(3)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(1)
        },
    );

    let fintech_trade_correction_samples =
        capture_perf_samples(suite, "fintech_trade_correction_audit_round_trip", || {
            let mut world = setup_trade_correction_perf_world();
            let baseline_observability = perf_capture_baseline_observability(&world);
            let analysis = perf_open_analysis_branch(&mut world);

            world.runtime.performance_access().reset_counters();
            let correction_started_at = Instant::now();
            let correction_commit = perf_correct_trade_correction(&mut world, analysis.clone());
            let correction_commit_micros = correction_started_at.elapsed().as_micros();

            let audit_started_at = Instant::now();
            let audit_commit = perf_emit_trade_correction_audit(&mut world, analysis);
            let audit_commit_micros = audit_started_at.elapsed().as_micros();

            let query_started_at = Instant::now();
            let probe = perf_capture_trade_correction_probe(&world);
            let query_probe_micros = query_started_at.elapsed().as_micros();
            let elapsed_micros =
                correction_commit_micros + audit_commit_micros + query_probe_micros;
            let counters = world.runtime.performance_access().counters();
            let post_observability = perf_capture_post_mutation_observability(&world);

            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "correction_records": correction_commit.changed_records.len(),
                    "audit_records": audit_commit.changed_records.len(),
                    "query_entities": probe.entity_count,
                    "query_relations": probe.relation_count,
                    "corrected_trade_count": probe.corrected_trade_count,
                    "audit_record_count": probe.audit_record_count,
                    "diagnostic_artifact_delta": post_observability
                        .diagnostics_artifact_count
                        .saturating_sub(baseline_observability.diagnostics_artifact_count),
                    "profile_boundary": profile_boundary_metrics(
                        &world.runtime,
                        RelationalRuntimeProfile::AiWorkflow,
                    ),
                    "phase_timing": {
                        "correction_commit_micros": correction_commit_micros,
                        "audit_commit_micros": audit_commit_micros,
                        "query_probe_micros": query_probe_micros,
                    },
                    "shape_metrics": {
                        "packet_count": counters.query_packet_count,
                        "scope_unit_count": counters.query_scope_unit_count,
                    },
                    "counters": counters,
                })
            })
        });
    emit_metric_summaries(
        suite,
        "fintech_trade_correction_audit_round_trip",
        &fintech_trade_correction_samples,
        &[
            (
                "correction_commit_micros",
                &["phase_timing", "correction_commit_micros"],
            ),
            (
                "audit_commit_micros",
                &["phase_timing", "audit_commit_micros"],
            ),
            (
                "query_probe_micros",
                &["phase_timing", "query_probe_micros"],
            ),
            ("packet_count", &["shape_metrics", "packet_count"]),
            ("scope_unit_count", &["shape_metrics", "scope_unit_count"]),
            ("diagnostic_artifact_delta", &["diagnostic_artifact_delta"]),
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
    assert!(fintech_trade_correction_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &fintech_trade_correction_samples,
        "fintech trade correction should surface one corrected trade and one audit record without broadening the case probe",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && metric_u64(metrics, "correction_records") == 1
                && metric_u64(metrics, "audit_records") == 1
                && metric_u64(metrics, "query_entities") == 3
                && metric_u64(metrics, "query_relations") == 0
                && metric_u64(metrics, "corrected_trade_count") == 1
                && metric_u64(metrics, "audit_record_count") == 1
                && metric_u64(metrics, "diagnostic_artifact_delta") >= 2
                && counter_u64(metrics, "query_packet_count") <= 3
                && counter_u64(metrics, "query_scope_unit_count") <= 3
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(3)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(3)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(1)
        },
    );

    let replay_recovery_samples = capture_perf_samples(
        suite,
        "persisted_recovery_replay_round_trip",
        || {
            let mut runtime = persisted_runtime_with_test_schema();
            let source_created = create_entity_outcome(&mut runtime, "recovery-source");
            let target_created = create_entity_outcome(&mut runtime, "recovery-target");
            let source = changed_entities(&source_created)[0];
            let target = changed_entities(&target_created)[0];
            let source_lineage = runtime
                .lineage_access()
                .for_record(source)
                .expect("source lineage")
                .lineage_id;
            let target_lineage = runtime
                .lineage_access()
                .for_record(target)
                .expect("target lineage")
                .lineage_id;

            let checkpoint_started_at = Instant::now();
            runtime
                .durability_authority()
                .checkpoint()
                .expect("workflow checkpoint");
            let checkpoint_micros = checkpoint_started_at.elapsed().as_micros();

            let post_checkpoint_commit_started_at = Instant::now();
            let candidate = runtime.lineage_authority().record_correspondence_candidate(
                BranchId("main".to_string()),
                vec![source_lineage],
                vec![target_lineage],
                "workflow-recovery-lineage",
            );
            let promotion = runtime
                .lineage_authority()
                .promote_correspondence(candidate.candidate_id, target_created.commit.clone())
                .expect("promote workflow correspondence");
            let post_checkpoint_commit_micros =
                post_checkpoint_commit_started_at.elapsed().as_micros();
            let recovery_plan = runtime.durability().recovery_plan(
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            );
            let tail_commit_id = promotion.promoted_commit_id().expect("promoted commit id");

            let mut recovered = persisted_runtime_with_test_schema();
            recovered.performance_access().reset_counters();
            let recover_started_at = Instant::now();
            let recovery_outcome = recovered
                .durability_authority()
                .recover(recovery_plan)
                .expect("workflow recovery");
            let recover_micros = recover_started_at.elapsed().as_micros();

            let replay_started_at = Instant::now();
            let replay_outcome =
                recovered
                    .replay_authority()
                    .replay_commit(RelationalReplayRequest {
                        commit_id: tail_commit_id,
                        branch_id: BranchId("main".to_string()),
                        execution_mode: ReplayExecutionMode::SerialDeterministic,
                        verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
                    });
            let replay_commit_micros = replay_started_at.elapsed().as_micros();

            let recovered_snapshot = recovered.visibility_authority().snapshot();
            let recovered_packet = explicit_query_packet(
                &recovered,
                &recovered_snapshot,
                "recovery-round-trip-query",
                vec![RecordRef::Entity(source), RecordRef::Entity(target)],
            );
            let query_started_at = Instant::now();
            let query_outcome = recovered
                .read_truth()
                .execute_query_plan(
                    recovered
                        .read_truth()
                        .plan_query_packet(&recovered_snapshot, recovered_packet)
                        .expect("planned recovered workflow query"),
                )
                .expect("recovered workflow query");
            let post_recovery_query_micros = query_started_at.elapsed().as_micros();

            let elapsed_micros = checkpoint_micros
                + post_checkpoint_commit_micros
                + recover_micros
                + replay_commit_micros
                + post_recovery_query_micros;
            let counters = recovered.performance_access().counters();

            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "checkpoint_commit_count": recovery_outcome.coverage.checkpoint_commits,
                    "tail_commit_count": recovery_outcome.coverage.replayed_tail_commits,
                    "recovered_commits": recovery_outcome.recovered_commits,
                    "selected_checkpoint": recovery_outcome.cursor.checkpoint_id.is_some(),
                    "replay_failure": replay_outcome.failure.as_ref().map(|failure| format!("{failure:?}")),
                    "replay_mismatch_count": replay_outcome.mismatches.len(),
                    "query_entities": query_outcome.result.entities.len(),
                    "query_relations": query_outcome.result.relations.len(),
                    "profile_boundary": profile_boundary_metrics(
                        &recovered,
                        RelationalRuntimeProfile::CertificationCore,
                    ),
                    "phase_timing": {
                        "checkpoint_micros": checkpoint_micros,
                        "post_checkpoint_commit_micros": post_checkpoint_commit_micros,
                        "recover_micros": recover_micros,
                        "replay_commit_micros": replay_commit_micros,
                        "post_recovery_query_micros": post_recovery_query_micros,
                    },
                    "counters": counters,
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "persisted_recovery_replay_round_trip",
        &replay_recovery_samples,
        &[
            ("checkpoint_micros", &["phase_timing", "checkpoint_micros"]),
            (
                "post_checkpoint_commit_micros",
                &["phase_timing", "post_checkpoint_commit_micros"],
            ),
            ("recover_micros", &["phase_timing", "recover_micros"]),
            (
                "replay_commit_micros",
                &["phase_timing", "replay_commit_micros"],
            ),
            (
                "post_recovery_query_micros",
                &["phase_timing", "post_recovery_query_micros"],
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
    assert!(replay_recovery_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &replay_recovery_samples,
        "persisted recovery round trips should select a checkpoint, replay cleanly, and query the recovered tail surface",
        |metrics| {
            metrics["selected_checkpoint"].as_bool() == Some(true)
                && metrics["replay_failure"].is_null()
                && metrics["replay_mismatch_count"].as_u64() == Some(0)
                && metrics["checkpoint_commit_count"].as_u64().unwrap_or(0) >= 1
                && metrics["tail_commit_count"].as_u64().unwrap_or(0) >= 1
                && counter_u64(metrics, "replay_lineage_authority_lookup_requests") == 1
                && counter_u64(metrics, "query_packet_count") <= 3
                && metrics["query_entities"].as_u64() == Some(2)
                && metrics["query_relations"].as_u64() == Some(0)
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(1)
        },
    );

    let retention_samples =
        capture_perf_samples(suite, "retention_release_reclaim_round_trip", || {
            let mut runtime = runtime_with_test_schema();
            let survivor =
                create_entity_in_partition(&mut runtime, "retention-survivor", PartitionId(10));
            let deleted_created = create_entity_outcome(&mut runtime, "retention-deleted");
            let created_snapshot = runtime.visibility_authority().snapshot();
            let deleted_entity = changed_entities(&deleted_created)[0];
            let deleted_commit = delete_entity(&mut runtime, deleted_entity);
            let deleted_snapshot = runtime.visibility_authority().snapshot();

            assert!(runtime
                .visibility_authority()
                .release_snapshot(&created_snapshot));
            assert!(runtime
                .visibility_authority()
                .release_snapshot(&deleted_snapshot));

            runtime.performance_access().reset_counters();
            let inspect_started_at = Instant::now();
            let inspect_plan = runtime.retention().inspect_plan();
            let inspect_plan_micros = inspect_started_at.elapsed().as_micros();
            let reclaim_started_at = Instant::now();
            let reclaim_pass = runtime.retention().run_pass();
            let run_pass_micros = reclaim_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "retention-reclaim-round-trip",
                vec![
                    RecordRef::Entity(survivor),
                    RecordRef::Entity(deleted_entity),
                ],
            );
            let query_started_at = Instant::now();
            let query_outcome = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, packet)
                        .expect("planned retention workflow query"),
                )
                .expect("retention workflow query");
            let post_reclaim_query_micros = query_started_at.elapsed().as_micros();

            let elapsed_micros = inspect_plan_micros + run_pass_micros + post_reclaim_query_micros;
            let counters = runtime.performance_access().counters();

            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "deleted_commit_records": deleted_commit.changed_records.len(),
                    "active_snapshot_count": inspect_plan.active_snapshot_count,
                    "reclaimable_entities": inspect_plan.reclaimable_entities,
                    "entity_reclaimable": reclaim_pass.entity_reclaimable,
                    "entity_reclaimed": reclaim_pass.entity_reclaimed,
                    "query_entities": query_outcome.result.entities.len(),
                    "query_relations": query_outcome.result.relations.len(),
                    "profile_boundary": profile_boundary_metrics(
                        &runtime,
                        RelationalRuntimeProfile::CertificationCore,
                    ),
                    "phase_timing": {
                        "inspect_plan_micros": inspect_plan_micros,
                        "run_pass_micros": run_pass_micros,
                        "post_reclaim_query_micros": post_reclaim_query_micros,
                    },
                    "counters": counters,
                })
            })
        });
    emit_metric_summaries(
        suite,
        "retention_release_reclaim_round_trip",
        &retention_samples,
        &[
            (
                "inspect_plan_micros",
                &["phase_timing", "inspect_plan_micros"],
            ),
            ("run_pass_micros", &["phase_timing", "run_pass_micros"]),
            (
                "post_reclaim_query_micros",
                &["phase_timing", "post_reclaim_query_micros"],
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
    assert!(retention_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &retention_samples,
        "retention release round trips should expose reclaimability and keep the survivor queryable without clone-heavy reclaim work",
        |metrics| {
            counter_u64(metrics, "full_state_clones") == 0
                && metrics["deleted_commit_records"].as_u64() == Some(1)
                && metrics["active_snapshot_count"].as_u64() == Some(0)
                && metrics["reclaimable_entities"].as_u64().unwrap_or(0) >= 1
                && metrics["entity_reclaimable"].as_u64().unwrap_or(0) >= 1
                && metrics["entity_reclaimed"].as_u64().unwrap_or(0)
                    <= metrics["entity_reclaimable"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "query_packet_count") <= 2
                && metrics["query_entities"].as_u64() == Some(1)
                && metrics["query_relations"].as_u64() == Some(0)
                && metrics["profile_boundary"]["execution_lane_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["diagnostics_boundary_code"].as_u64() == Some(2)
                && metrics["profile_boundary"]["matches_defaults"].as_u64() == Some(1)
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

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_runtime_bridge_mock_matrix() {
    let suite = "runtime_bridge_mock_matrix";

    for (case, development_profile) in [
        ("geometry_commit_bridge_wave_operational", false),
        ("geometry_commit_bridge_wave_development", true),
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

            let source = create_entity_outcome(&mut relational, "merged-geometry-source");
            let middle = create_entity_outcome(&mut relational, "merged-geometry-middle");
            let target = create_entity_outcome(&mut relational, "merged-geometry-target");
            let source_entity = changed_entities(&source)[0];
            let middle_entity = changed_entities(&middle)[0];
            let target_entity = changed_entities(&target)[0];
            create_relation_outcome(
                &mut relational,
                source_entity,
                middle_entity,
                "merged-geometry-link-a",
            );
            create_relation_outcome(
                &mut relational,
                middle_entity,
                target_entity,
                "merged-geometry-link-b",
            );

            let mut bridge_runtime = build_mock_bridge_runtime(development_profile, 4);

            let relational_commit_started_at = Instant::now();
            let update = update_entity(
                &mut relational,
                middle_entity,
                "merged-geometry-middle-updated",
            );
            let relational_commit_micros = relational_commit_started_at.elapsed().as_micros();

            let snapshot = relational.visibility_authority().snapshot();
            let traversal_packet = PlannedQueryPacket {
                label: "merged-relational-signal-traversal".to_string(),
                context_id: relational
                    .read_truth()
                    .query_plan_context(&snapshot)
                    .expect("merged query plan context"),
                scope: QueryScope::ConnectivityTraversal {
                    seeds: Arc::from([source_entity, middle_entity]),
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
            let relational_query_started_at = Instant::now();
            let traversal = relational
                .read_truth()
                .execute_query_plan(
                    relational
                        .read_truth()
                        .plan_query_packet(&snapshot, traversal_packet)
                        .expect("merged traversal plan"),
                )
                .expect("merged traversal outcome");
            let relational_query_micros = relational_query_started_at.elapsed().as_micros();

            let affected_sources = traversal
                .result
                .entities
                .len()
                .min(bridge_runtime.source_versions.len())
                .max(1);
            let bridge_before = bridge_runtime.observe();
            let bridge_started_at = Instant::now();
            bridge_runtime.apply_changes(affected_sources);
            let bridge_micros = bridge_started_at.elapsed().as_micros();
            let bridge_after = bridge_runtime.observe();
            let bridge_history_entries = bridge_runtime.recent_history_len();

            PerfMeasurement {
                elapsed_micros: relational_commit_micros + relational_query_micros + bridge_micros,
                metrics: perf_metrics!({
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
                    "bridge_suppressed_downstream": bridge_after.evaluation.suppressed_downstream_propagations
                        - bridge_before.evaluation.suppressed_downstream_propagations,
                    "bridge_history_entries": bridge_history_entries,
                    "bridge_has_latest_flow": bridge_runtime.latest_flow_diagnostics().is_some(),
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
                ("affected_bridge_sources", &["affected_bridge_sources"]),
                ("bridge_nodes_evaluated", &["bridge_nodes_evaluated"]),
                ("bridge_nodes_recomputed", &["bridge_nodes_recomputed"]),
                ("bridge_tasks_scheduled", &["bridge_tasks_scheduled"]),
                ("bridge_history_entries", &["bridge_history_entries"]),
            ],
        );
        assert_budget(
            &samples,
            "mocked bridge certification should keep truth updates narrow while surfacing downstream invalidation and recomputation work without crossing the crate boundary",
            |metrics| {
                let affected = metrics["affected_bridge_sources"].as_u64().unwrap_or(0);
                metrics["relational_changed_records"].as_u64() == Some(1)
                    && metrics["relational_result_entities"].as_u64().unwrap_or(0) >= 2
                    && affected >= 1
                    && metrics["bridge_nodes_recomputed"].as_u64().unwrap_or(0) >= affected
                    && metrics["bridge_tasks_scheduled"].as_u64().unwrap_or(0) >= affected
                    && metrics["bridge_history_entries"].as_u64().unwrap_or(0) >= 1
                    && metrics["bridge_has_latest_flow"].as_bool() == Some(true)
            },
        );
    }

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

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_game_engine_matrix() {
    let suite = "game_engine_matrix";

    let local_scene_wave_samples =
        capture_perf_samples(suite, "local_scene_graph_propagation_wave", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore);
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            let seeded = seed_game_engine_frame_world(&mut runtime, "scene-local", 8, 24);
            let updated = seeded.frame_targets[3];
            let explicit_targets = seeded
                .explicit_targets
                .iter()
                .take(12)
                .map(|entity| RecordRef::Entity(*entity))
                .collect::<Vec<_>>();
            let traversal_seeds = Arc::from([
                seeded.propagation_seeds[1],
                seeded.propagation_seeds[2],
                seeded.propagation_seeds[3],
                seeded.propagation_seeds[4],
            ]);
            let mut bridge_runtime = build_mock_bridge_runtime(false, 32);

            runtime.performance_access().reset_counters();
            let update_started_at = Instant::now();
            let update = update_entity(&mut runtime, updated, "scene-local-updated");
            let update_micros = update_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let propagation_packet = PlannedQueryPacket {
                label: "scene-local-propagation".to_string(),
                context_id: runtime
                    .read_truth()
                    .query_plan_context(&snapshot)
                    .expect("scene local query context"),
                scope: QueryScope::ConnectivityTraversal {
                    seeds: traversal_seeds,
                    relation_kind_scope: Some(Arc::from([KindId(2)])),
                    max_depth: Some(3),
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalTraversalOrder,
                access_contract: QueryAccessContract::AuthoritativeStorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(93_001),
                target_count_hint: 4,
            };
            let propagation_started_at = Instant::now();
            let propagation = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, propagation_packet)
                        .expect("scene local propagation plan"),
                )
                .expect("scene local propagation outcome");
            let propagation_micros = propagation_started_at.elapsed().as_micros();

            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "scene-local-explicit",
                explicit_targets,
            );
            let explicit_started_at = Instant::now();
            let explicit = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("scene local explicit plan"),
                )
                .expect("scene local explicit outcome");
            let explicit_micros = explicit_started_at.elapsed().as_micros();
            assert!(runtime.visibility_authority().release_snapshot(&snapshot));

            let affected_sources = (propagation.result.entities.len()
                + explicit.result.entities.len())
            .min(bridge_runtime.source_versions.len())
            .max(4);
            let bridge_before = bridge_runtime.observe();
            let bridge_started_at = Instant::now();
            bridge_runtime.apply_changes(affected_sources);
            let bridge_micros = bridge_started_at.elapsed().as_micros();
            let bridge_after = bridge_runtime.observe();

            measurement_with_elapsed(
                update_micros + propagation_micros + explicit_micros + bridge_micros,
                || {
                    perf_metrics!({
                        "region_count": seeded.region_count,
                        "resident_entities": seeded.entities.len(),
                        "resident_relations": seeded.relation_count,
                        "changed_records": update.changed_records.len(),
                        "update_micros": update_micros,
                        "propagation_micros": propagation_micros,
                        "explicit_query_micros": explicit_micros,
                        "bridge_micros": bridge_micros,
                        "propagation_result_entities": propagation.result.entities.len(),
                        "explicit_result_entities": explicit.result.entities.len(),
                        "affected_bridge_sources": affected_sources,
                        "bridge_nodes_recomputed": bridge_after.evaluation.nodes_recomputed
                            - bridge_before.evaluation.nodes_recomputed,
                        "bridge_tasks_scheduled": bridge_after.planner.tasks_scheduled
                            - bridge_before.planner.tasks_scheduled,
                        "counters": runtime.performance_access().counters(),
                    })
                },
            )
        });
    emit_metric_summaries(
        suite,
        "local_scene_graph_propagation_wave",
        &local_scene_wave_samples,
        &[
            ("update_micros", &["update_micros"]),
            ("propagation_micros", &["propagation_micros"]),
            ("explicit_query_micros", &["explicit_query_micros"]),
            ("bridge_micros", &["bridge_micros"]),
            (
                "propagation_result_entities",
                &["propagation_result_entities"],
            ),
            ("explicit_result_entities", &["explicit_result_entities"]),
            ("affected_bridge_sources", &["affected_bridge_sources"]),
            ("bridge_tasks_scheduled", &["bridge_tasks_scheduled"]),
        ],
    );
    assert_budget(
        &local_scene_wave_samples,
        "game-engine local scene waves should keep frame-local propagation and derived work region-bounded",
        |metrics| {
            let affected = metrics["affected_bridge_sources"].as_u64().unwrap_or(0);
            metrics["region_count"].as_u64() == Some(8)
                && metrics["resident_entities"].as_u64() == Some(192)
                && metrics["changed_records"].as_u64() == Some(1)
                && metrics["propagation_result_entities"].as_u64().unwrap_or(0) >= 8
                && metrics["explicit_result_entities"].as_u64().unwrap_or(0) == 12
                && affected >= 8
                && affected <= 32
                && metrics["bridge_tasks_scheduled"].as_u64().unwrap_or(0) >= affected
                && counter_u64(metrics, "full_state_clones") == 0
        },
    );

    let flat_batch_wave_samples =
        capture_perf_samples(suite, "flat_entity_batch_region_wave", || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore);
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            let seeded = seed_game_engine_frame_world(&mut runtime, "scene-batch", 8, 24);

            let mut partition_targets = BTreeMap::new();
            for entity in &seeded.entities {
                let targets = partition_targets
                    .entry(entity.partition_id)
                    .or_insert_with(Vec::new);
                if targets.len() < 8 {
                    targets.push(*entity);
                }
                if partition_targets.len() >= 4
                    && partition_targets.values().all(|targets| targets.len() >= 6)
                {
                    break;
                }
            }
            let batch_targets = partition_targets
                .values()
                .flat_map(|targets| targets.iter().take(6).copied())
                .collect::<Vec<_>>();
            assert!(
                batch_targets.len() >= 24,
                "batch wave should gather a multi-partition entity batch"
            );

            runtime.performance_access().reset_counters();
            let update_started_at = Instant::now();
            let update = {
                let mut txn = runtime.begin_transaction(TransactionOptions::default());
                let mut batch = WorkerIntentBatch::new("scene-batch-flat-entity-wave");
                for (index, entity) in batch_targets.iter().enumerate() {
                    batch = batch.push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                        UpdateEntityFieldsIntent {
                            entity_id: *entity,
                            fields: crate::tests::support::aspect_field_patch_from_values([
                                (
                                    crate::tests::support::aspect_key("name"),
                                    crate::tests::support::field_key("name"),
                                    crate::tests::support::string_aspect_value(&format!(
                                        "scene-batch-updated-{index}"
                                    )),
                                ),
                                (
                                    crate::tests::support::aspect_key("phase"),
                                    crate::tests::support::field_key("phase"),
                                    crate::tests::support::string_aspect_value("batch-wave"),
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
                txn.commit().expect("scene batch flat entity wave commit")
            };
            let update_micros = update_started_at.elapsed().as_micros();

            let snapshot = runtime.visibility_authority().snapshot();
            let explicit_targets = batch_targets
                .iter()
                .take(12)
                .map(|entity| RecordRef::Entity(*entity))
                .collect::<Vec<_>>();
            let explicit_packet = explicit_query_packet(
                &runtime,
                &snapshot,
                "scene-batch-explicit",
                explicit_targets,
            );
            let explicit_started_at = Instant::now();
            let explicit = runtime
                .read_truth()
                .execute_query_plan(
                    runtime
                        .read_truth()
                        .plan_query_packet(&snapshot, explicit_packet)
                        .expect("scene batch explicit plan"),
                )
                .expect("scene batch explicit outcome");
            let explicit_micros = explicit_started_at.elapsed().as_micros();
            assert!(runtime.visibility_authority().release_snapshot(&snapshot));

            measurement_with_elapsed(update_micros + explicit_micros, || {
                perf_metrics!({
                    "region_count": seeded.region_count,
                    "resident_entities": seeded.entities.len(),
                    "resident_relations": seeded.relation_count,
                    "batch_target_count": batch_targets.len(),
                    "batch_partition_count": partition_targets.len(),
                    "changed_records": update.changed_records.len(),
                    "update_micros": update_micros,
                    "explicit_query_micros": explicit_micros,
                    "explicit_result_entities": explicit.result.entities.len(),
                    "counters": runtime.performance_access().counters(),
                })
            })
        });
    emit_metric_summaries(
        suite,
        "flat_entity_batch_region_wave",
        &flat_batch_wave_samples,
        &[
            ("update_micros", &["update_micros"]),
            ("explicit_query_micros", &["explicit_query_micros"]),
            ("batch_target_count", &["batch_target_count"]),
            ("batch_partition_count", &["batch_partition_count"]),
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
    assert_budget(
        &flat_batch_wave_samples,
        "game-engine flat entity batches should stay on the sparse AoSoA path across a few touched partitions",
        |metrics| {
            let batch_target_count = metrics["batch_target_count"].as_u64().unwrap_or(0);
            let batch_partition_count = metrics["batch_partition_count"].as_u64().unwrap_or(0);
            metrics["region_count"].as_u64() == Some(8)
                && batch_target_count >= 24
                && batch_partition_count >= 4
                && metrics["changed_records"].as_u64() == Some(batch_target_count)
                && counter_u64(metrics, "entity_slots_touched_by_commit") == batch_target_count
                && counter_u64(metrics, "partitions_touched_by_commit") >= batch_partition_count
                && counter_u64(metrics, "aosoa_entity_chunk_slots_materialized")
                    == batch_target_count
                && counter_u64(metrics, "aosoa_entity_chunks_published") >= batch_partition_count
                && counter_u64(metrics, "aosoa_publish_soa_merge_count") == 0
                && counter_u64(metrics, "full_state_clones") == 0
        },
    );

    let mixed_frame_churn_samples = capture_perf_samples(
        suite,
        "mixed_read_write_frame_churn_window",
        || {
            let mut runtime =
                runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore);
            apply_perf_diagnostics_policy(
                &mut runtime,
                PerfDiagnosticsPolicy::GeometryOperationalHotPath,
            );
            let seeded = seed_game_engine_frame_world(&mut runtime, "scene-frame", 8, 24);
            let mut bridge_runtime = build_mock_bridge_runtime(false, 48);

            const ITERATIONS: usize = 48;
            const WINDOW: usize = 12;
            let mut cycle_samples = Vec::with_capacity(ITERATIONS);
            let mut total_update_micros = 0u128;
            let mut total_propagation_micros = 0u128;
            let mut total_explicit_query_micros = 0u128;
            let mut total_bridge_micros = 0u128;
            let mut max_packets_per_iteration = 0usize;
            let mut max_scope_units_per_iteration = 0usize;
            let mut max_bridge_tasks_scheduled = 0u64;
            let mut previous_packets = 0usize;
            let mut previous_scope_units = 0usize;

            runtime.performance_access().reset_counters();
            for frame in 0..ITERATIONS {
                let actor = seeded.frame_targets[frame % seeded.frame_targets.len()];
                let update_started_at = Instant::now();
                let _ = update_entity(&mut runtime, actor, &format!("scene-frame-step-{frame}"));
                let update_micros = update_started_at.elapsed().as_micros();
                total_update_micros += update_micros;

                let snapshot = runtime.visibility_authority().snapshot();
                let propagation_packet = PlannedQueryPacket {
                    label: "scene-frame-propagation".to_string(),
                    context_id: runtime
                        .read_truth()
                        .query_plan_context(&snapshot)
                        .expect("scene frame query context"),
                    scope: QueryScope::ConnectivityTraversal {
                        seeds: Arc::from([
                            seeded.propagation_seeds[frame % seeded.propagation_seeds.len()],
                            seeded.propagation_seeds[(frame + 1) % seeded.propagation_seeds.len()],
                        ]),
                        relation_kind_scope: Some(Arc::from([KindId(2)])),
                        max_depth: Some(2),
                    },
                    locality: QueryLocalityClass::CrossPartitionTraversal,
                    ordering: QueryOrderingContract::CanonicalTraversalOrder,
                    access_contract: QueryAccessContract::AuthoritativeStorageOnly,
                    execution_shape: QueryExecutionShape::BulkPacketized,
                    reduction: ReductionDiscipline::DeterministicMerge,
                    plan_key: DeterministicQueryPlanKey(93_101),
                    target_count_hint: 2,
                };
                let propagation_started_at = Instant::now();
                let propagation = runtime
                    .read_truth()
                    .execute_query_plan(
                        runtime
                            .read_truth()
                            .plan_query_packet(&snapshot, propagation_packet)
                            .expect("scene frame propagation plan"),
                    )
                    .expect("scene frame propagation outcome");
                let propagation_micros = propagation_started_at.elapsed().as_micros();
                total_propagation_micros += propagation_micros;

                let explicit_targets = seeded
                    .explicit_targets
                    .iter()
                    .cycle()
                    .skip(frame)
                    .take(8)
                    .map(|entity| RecordRef::Entity(*entity))
                    .collect::<Vec<_>>();
                let explicit_packet = explicit_query_packet(
                    &runtime,
                    &snapshot,
                    "scene-frame-explicit",
                    explicit_targets,
                );
                let explicit_started_at = Instant::now();
                let explicit = runtime
                    .read_truth()
                    .execute_query_plan(
                        runtime
                            .read_truth()
                            .plan_query_packet(&snapshot, explicit_packet)
                            .expect("scene frame explicit plan"),
                    )
                    .expect("scene frame explicit outcome");
                let explicit_micros = explicit_started_at.elapsed().as_micros();
                total_explicit_query_micros += explicit_micros;
                assert!(runtime.visibility_authority().release_snapshot(&snapshot));

                let affected_sources = (propagation.result.entities.len()
                    + explicit.result.entities.len())
                .min(bridge_runtime.source_versions.len())
                .max(4);
                let bridge_before = bridge_runtime.observe();
                let bridge_started_at = Instant::now();
                bridge_runtime.apply_changes(affected_sources);
                let bridge_micros = bridge_started_at.elapsed().as_micros();
                total_bridge_micros += bridge_micros;
                let bridge_after = bridge_runtime.observe();
                max_bridge_tasks_scheduled = max_bridge_tasks_scheduled.max(
                    bridge_after.planner.tasks_scheduled - bridge_before.planner.tasks_scheduled,
                );

                cycle_samples
                    .push(update_micros + propagation_micros + explicit_micros + bridge_micros);
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
            let elapsed_micros = total_update_micros
                + total_propagation_micros
                + total_explicit_query_micros
                + total_bridge_micros;
            measurement_with_elapsed(elapsed_micros, || {
                perf_metrics!({
                    "iterations": ITERATIONS,
                    "region_count": seeded.region_count,
                    "resident_entities": seeded.entities.len(),
                    "resident_relations": seeded.relation_count,
                    "average_update_micros": total_update_micros / ITERATIONS as u128,
                    "average_propagation_micros": total_propagation_micros / ITERATIONS as u128,
                    "average_explicit_query_micros": total_explicit_query_micros / ITERATIONS as u128,
                    "average_bridge_micros": total_bridge_micros / ITERATIONS as u128,
                    "first_window_average_cycle_micros": first_window_average_cycle_micros,
                    "last_window_average_cycle_micros": last_window_average_cycle_micros,
                    "max_packets_per_iteration": max_packets_per_iteration,
                    "max_scope_units_per_iteration": max_scope_units_per_iteration,
                    "max_bridge_tasks_scheduled": max_bridge_tasks_scheduled,
                    "counters": runtime.performance_access().counters(),
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "mixed_read_write_frame_churn_window",
        &mixed_frame_churn_samples,
        &[
            ("iterations", &["iterations"]),
            ("average_update_micros", &["average_update_micros"]),
            (
                "average_propagation_micros",
                &["average_propagation_micros"],
            ),
            (
                "average_explicit_query_micros",
                &["average_explicit_query_micros"],
            ),
            ("average_bridge_micros", &["average_bridge_micros"]),
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
            (
                "max_bridge_tasks_scheduled",
                &["max_bridge_tasks_scheduled"],
            ),
        ],
    );
    assert_budget(
        &mixed_frame_churn_samples,
        "game-engine frame churn should keep repeated mixed read/write cycles local and stable across a bounded frame window",
        |metrics| {
            let first_window = metrics["first_window_average_cycle_micros"]
                .as_u64()
                .unwrap_or(0);
            let last_window = metrics["last_window_average_cycle_micros"]
                .as_u64()
                .unwrap_or(0);
            metrics["iterations"].as_u64() == Some(48)
                && metrics["region_count"].as_u64() == Some(8)
                && metrics["resident_entities"].as_u64() == Some(192)
                && metrics["max_packets_per_iteration"].as_u64().unwrap_or(0) <= 16
                && metrics["max_scope_units_per_iteration"].as_u64().unwrap_or(0) <= 16
                && metrics["max_bridge_tasks_scheduled"].as_u64().unwrap_or(0) <= 64
                && last_window <= first_window.saturating_mul(2).max(1)
                && counter_u64(metrics, "full_state_clones") == 0
                && counter_u64(metrics, "bulk_mutation_batch_count") == 48
        },
    );
}

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_merge_lineage_matrix() {
    let suite = "merge_lineage_matrix";

    let merge_planning_samples =
        capture_perf_samples(suite, "merge_planning_divergent_update", || {
            let mut runtime = persisted_runtime_with_test_schema();
            let shared = create_entity(&mut runtime, "shared");
            create_branch_from_main(&mut runtime, "feature");
            let _ = update_entity(&mut runtime, shared, "main-value");
            let _ = update_entity_on_branch(
                &mut runtime,
                shared,
                "feature-value",
                BranchId("feature".to_string()),
            );

            runtime.performance_access().reset_counters();
            let started_at = Instant::now();
            let artifact = runtime
                .merge()
                .inspect_planning_scope(crate::merge::data::MergePlanningRequest::new(
                    BranchId("main".to_string()),
                    BranchId("feature".to_string()),
                    MergeIntent::ReconcileIntoTarget,
                ))
                .expect("merge planning artifact");
            let elapsed_micros = started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: perf_metrics!({
                    "candidate_count": artifact.identity_discovery.candidate_count,
                    "classified_records": artifact.conflict_classification.classified_record_count,
                    "resolved_records": artifact.policy_resolution.resolved_record_count,
                    "decision_count": artifact.decision_log.decisions.len(),
                    "counters": counters,
                }),
            }
        });
    assert!(merge_planning_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &merge_planning_samples,
        "merge planning should stay request-shaped and artifact-accounted",
        |metrics| {
            counter_u64(metrics, "merge_planning_requests") == 1
                && counter_u64(metrics, "merge_identity_candidates_discovered")
                    == metrics["candidate_count"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "merge_conflict_records_classified")
                    == metrics["classified_records"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "merge_decision_log_width")
                    == metrics["decision_count"].as_u64().unwrap_or(0)
        },
    );

    let merge_execution_samples = capture_perf_samples(
        suite,
        "merge_execution_feature_adoption",
        || {
            let mut runtime = persisted_runtime_with_test_schema();
            create_entity(&mut runtime, "main-anchor");
            create_branch_from_main(&mut runtime, "feature");
            let mut txn = runtime.begin_transaction(TransactionOptions {
                target_branch: Some(BranchId("feature".to_string())),
                ..TransactionOptions::default()
            });
            txn.push_batch(
                WorkerIntentBatch::new("create-feature-only").push(
                    MutationIntent::Create(CreateIntent::Entity(
                        crate::transactions::data::EntitySpec {
                            partition_id: PartitionId::main(),
                            kind_id: KindId(1),
                            client_key: crate::symbols::data::ClientKey::raw("feature-only"),
                            fields: crate::tests::support::single_string_aspect_field_patch(
                                crate::tests::support::aspect_key("name"),
                                crate::tests::support::field_key("name"),
                                "feature-only",
                            ),
                        },
                    ))
                    .into(),
                ),
            );
            let _feature_only = changed_entities(&txn.commit().expect("feature create"))[0];

            let prepared = runtime
                .prepare_merge_execution(MergeExecutionRequest {
                    target_branch: BranchId("main".to_string()),
                    source_branch: BranchId("feature".to_string()),
                    merge_intent: MergeIntent::ReconcileIntoTarget,
                })
                .expect("prepared merge");

            runtime.performance_access().reset_counters();
            let started_at = Instant::now();
            let outcome = runtime
                .execute_prepared_merge(prepared)
                .expect("execute merge");
            let elapsed_micros = started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: perf_metrics!({
                    "executed_record_count": outcome.structural_summary.executed_record_count,
                    "emitted_mutation_intent_count": outcome.structural_summary.emitted_mutation_intent_count,
                    "adopted_source_record_count": outcome.structural_summary.adopted_source_record_count,
                    "changed_entities": changed_entities(&outcome.commit).len(),
                    "counters": counters,
                }),
            }
        },
    );
    assert!(merge_execution_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &merge_execution_samples,
        "merge execution should admit and emit exactly the scoped merge work",
        |metrics| {
            counter_u64(metrics, "merge_execution_attempts") == 1
                && counter_u64(metrics, "merge_execution_records_admitted")
                    == metrics["executed_record_count"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "merge_execution_mutation_intents_emitted")
                    == metrics["emitted_mutation_intent_count"]
                        .as_u64()
                        .unwrap_or(0)
                && metrics["adopted_source_record_count"].as_u64() == Some(1)
                && metrics["changed_entities"].as_u64() == Some(1)
        },
    );

    let merge_execution_zero_diag_samples = capture_perf_samples(
        suite,
        "merge_execution_feature_adoption_zero_diagnostics_budget",
        || {
            let mut runtime = persisted_runtime_with_test_schema();
            runtime.config.diagnostics.profile.max_entries_per_artifact = 0;
            create_entity(&mut runtime, "main-anchor");
            create_branch_from_main(&mut runtime, "feature");
            let mut txn = runtime.begin_transaction(TransactionOptions {
                target_branch: Some(BranchId("feature".to_string())),
                ..TransactionOptions::default()
            });
            txn.push_batch(
                WorkerIntentBatch::new("create-feature-only").push(
                    MutationIntent::Create(CreateIntent::Entity(
                        crate::transactions::data::EntitySpec {
                            partition_id: PartitionId::main(),
                            kind_id: KindId(1),
                            client_key: crate::symbols::data::ClientKey::raw("feature-only"),
                            fields: crate::tests::support::single_string_aspect_field_patch(
                                crate::tests::support::aspect_key("name"),
                                crate::tests::support::field_key("name"),
                                "feature-only",
                            ),
                        },
                    ))
                    .into(),
                ),
            );
            let _feature_only = changed_entities(&txn.commit().expect("feature create"))[0];

            let prepared = runtime
                .prepare_merge_execution(MergeExecutionRequest {
                    target_branch: BranchId("main".to_string()),
                    source_branch: BranchId("feature".to_string()),
                    merge_intent: MergeIntent::ReconcileIntoTarget,
                })
                .expect("prepared merge");

            runtime.performance_access().reset_counters();
            let started_at = Instant::now();
            let outcome = runtime
                .execute_prepared_merge(prepared)
                .expect("execute merge");
            let elapsed_micros = started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: perf_metrics!({
                    "executed_record_count": outcome.structural_summary.executed_record_count,
                    "emitted_mutation_intent_count": outcome.structural_summary.emitted_mutation_intent_count,
                    "adopted_source_record_count": outcome.structural_summary.adopted_source_record_count,
                    "changed_entities": changed_entities(&outcome.commit).len(),
                    "diagnostic_artifact_entries": runtime
                        .publication()
                        .diagnostics()
                        .artifacts()
                        .iter()
                        .rev()
                        .find(|artifact| {
                            artifact.scope == crate::facade::diagnostics::DiagnosticsScope::History
                                && artifact.kind
                                    == crate::facade::diagnostics::DiagnosticsArtifactKind::DetailedTrace
                        })
                        .map(|artifact| artifact.entries.len())
                        .unwrap_or(0),
                    "counters": counters,
                }),
            }
        },
    );
    assert!(merge_execution_zero_diag_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &merge_execution_zero_diag_samples,
        "merge execution should preserve structural truth even when detailed diagnostics are budget-zero",
        |metrics| {
            counter_u64(metrics, "merge_execution_attempts") == 1
                && counter_u64(metrics, "merge_execution_records_admitted")
                    == metrics["executed_record_count"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "merge_execution_mutation_intents_emitted")
                    == metrics["emitted_mutation_intent_count"].as_u64().unwrap_or(0)
                && metrics["adopted_source_record_count"].as_u64() == Some(1)
                && metrics["changed_entities"].as_u64() == Some(1)
                && metrics["diagnostic_artifact_entries"].as_u64() == Some(0)
        },
    );

    let merge_prepare_execute_split_samples = capture_perf_samples(
        suite,
        "merge_prepare_vs_execute_feature_adoption",
        || {
            let mut runtime = persisted_runtime_with_test_schema();
            create_entity(&mut runtime, "main-anchor");
            create_branch_from_main(&mut runtime, "feature");
            let mut txn = runtime.begin_transaction(TransactionOptions {
                target_branch: Some(BranchId("feature".to_string())),
                ..TransactionOptions::default()
            });
            txn.push_batch(
                WorkerIntentBatch::new("create-feature-only").push(
                    MutationIntent::Create(CreateIntent::Entity(
                        crate::transactions::data::EntitySpec {
                            partition_id: PartitionId::main(),
                            kind_id: KindId(1),
                            client_key: crate::symbols::data::ClientKey::raw("feature-only"),
                            fields: crate::tests::support::single_string_aspect_field_patch(
                                crate::tests::support::aspect_key("name"),
                                crate::tests::support::field_key("name"),
                                "feature-only",
                            ),
                        },
                    ))
                    .into(),
                ),
            );
            let _feature_only = changed_entities(&txn.commit().expect("feature create"))[0];

            runtime.performance_access().reset_counters();
            let prepare_started_at = Instant::now();
            let prepared = runtime
                .prepare_merge_execution(MergeExecutionRequest {
                    target_branch: BranchId("main".to_string()),
                    source_branch: BranchId("feature".to_string()),
                    merge_intent: MergeIntent::ReconcileIntoTarget,
                })
                .expect("prepared merge");
            let prepare_elapsed_micros = prepare_started_at.elapsed().as_micros();

            runtime.performance_access().reset_counters();
            let execute_started_at = Instant::now();
            let outcome = runtime
                .execute_prepared_merge(prepared)
                .expect("execute merge");
            let execute_elapsed_micros = execute_started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros: prepare_elapsed_micros + execute_elapsed_micros,
                metrics: perf_metrics!({
                    "prepare_elapsed_micros": prepare_elapsed_micros,
                    "execute_elapsed_micros": execute_elapsed_micros,
                    "executed_record_count": outcome.structural_summary.executed_record_count,
                    "emitted_mutation_intent_count": outcome.structural_summary.emitted_mutation_intent_count,
                    "changed_entities": changed_entities(&outcome.commit).len(),
                    "counters": counters,
                }),
            }
        },
    );
    assert!(merge_prepare_execute_split_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &merge_prepare_execute_split_samples,
        "merge prepare/execute split should preserve the same single-record structural truth",
        |metrics| {
            metrics["prepare_elapsed_micros"].as_u64().unwrap_or(0) > 0
                && metrics["execute_elapsed_micros"].as_u64().unwrap_or(0) > 0
                && counter_u64(metrics, "merge_execution_attempts") == 1
                && counter_u64(metrics, "merge_execution_records_admitted")
                    == metrics["executed_record_count"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "merge_execution_mutation_intents_emitted")
                    == metrics["emitted_mutation_intent_count"]
                        .as_u64()
                        .unwrap_or(0)
                && metrics["changed_entities"].as_u64() == Some(1)
        },
    );

    let merge_vs_commit_floor_samples = capture_perf_samples(
        suite,
        "merge_execution_vs_persisted_commit_floor",
        || {
            let mut merge_runtime = persisted_runtime_with_test_schema();
            create_entity(&mut merge_runtime, "main-anchor");
            create_branch_from_main(&mut merge_runtime, "feature");
            let mut txn = merge_runtime.begin_transaction(TransactionOptions {
                target_branch: Some(BranchId("feature".to_string())),
                ..TransactionOptions::default()
            });
            txn.push_batch(
                WorkerIntentBatch::new("create-feature-only").push(
                    MutationIntent::Create(CreateIntent::Entity(
                        crate::transactions::data::EntitySpec {
                            partition_id: PartitionId::main(),
                            kind_id: KindId(1),
                            client_key: crate::symbols::data::ClientKey::raw("feature-only"),
                            fields: crate::tests::support::single_string_aspect_field_patch(
                                crate::tests::support::aspect_key("name"),
                                crate::tests::support::field_key("name"),
                                "feature-only",
                            ),
                        },
                    ))
                    .into(),
                ),
            );
            let _feature_only = changed_entities(&txn.commit().expect("feature create"))[0];

            let prepared = merge_runtime
                .prepare_merge_execution(MergeExecutionRequest {
                    target_branch: BranchId("main".to_string()),
                    source_branch: BranchId("feature".to_string()),
                    merge_intent: MergeIntent::ReconcileIntoTarget,
                })
                .expect("prepared merge");

            merge_runtime.performance_access().reset_counters();
            let merge_started_at = Instant::now();
            let merge_outcome = merge_runtime
                .execute_prepared_merge(prepared)
                .expect("execute merge");
            let merge_elapsed_micros = merge_started_at.elapsed().as_micros();
            let merge_counters = merge_runtime.performance_access().counters();

            let mut control_runtime = persisted_runtime_with_test_schema();
            control_runtime.performance_access().reset_counters();
            let control_started_at = Instant::now();
            let control_outcome = {
                let mut txn = control_runtime.begin_transaction(TransactionOptions::default());
                txn.push_batch(batch_create("control-single"));
                txn.commit().expect("control persisted single create")
            };
            let control_elapsed_micros = control_started_at.elapsed().as_micros();
            let control_counters = control_runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros: merge_elapsed_micros,
                metrics: perf_metrics!({
                    "merge_elapsed_micros": merge_elapsed_micros,
                    "control_commit_elapsed_micros": control_elapsed_micros,
                    "merge_over_control_delta_micros": merge_elapsed_micros as i128 - control_elapsed_micros as i128,
                    "merge_control_ratio": merge_elapsed_micros as f64 / control_elapsed_micros.max(1) as f64,
                    "executed_record_count": merge_outcome.structural_summary.executed_record_count,
                    "emitted_mutation_intent_count": merge_outcome.structural_summary.emitted_mutation_intent_count,
                    "merge_changed_entities": changed_entities(&merge_outcome.commit).len(),
                    "control_changed_records": control_outcome.changed_records.len(),
                    "merge_counters": merge_counters,
                    "control_counters": control_counters,
                }),
            }
        },
    );
    assert!(merge_vs_commit_floor_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &merge_vs_commit_floor_samples,
        "merge execute vs persisted commit floor should preserve single-record structural truth on both paths",
        |metrics| {
            metrics["merge_elapsed_micros"].as_u64().unwrap_or(0) > 0
                && metrics["control_commit_elapsed_micros"].as_u64().unwrap_or(0) > 0
                && metrics["merge_changed_entities"].as_u64() == Some(1)
                && metrics["control_changed_records"].as_u64() == Some(1)
                && metrics["merge_counters"]["merge_execution_attempts"].as_u64() == Some(1)
                && metrics["merge_counters"]["merge_execution_records_admitted"].as_u64()
                    == metrics["executed_record_count"].as_u64()
                && metrics["merge_counters"]["merge_execution_mutation_intents_emitted"].as_u64()
                    == metrics["emitted_mutation_intent_count"].as_u64()
                && metrics["control_counters"]["full_state_clones"].as_u64() == Some(0)
                && metrics["control_counters"]["snapshot_pin_full_rebuilds"].as_u64() == Some(0)
                && metrics["control_counters"]["partitions_touched_by_commit"].as_u64() == Some(1)
        },
    );

    let merge_verify_execute_split_samples = capture_perf_samples(
        suite,
        "merge_verify_vs_execute_feature_adoption",
        || {
            let mut runtime = persisted_runtime_with_test_schema();
            create_entity(&mut runtime, "main-anchor");
            create_branch_from_main(&mut runtime, "feature");
            let mut txn = runtime.begin_transaction(TransactionOptions {
                target_branch: Some(BranchId("feature".to_string())),
                ..TransactionOptions::default()
            });
            txn.push_batch(
                WorkerIntentBatch::new("create-feature-only").push(
                    MutationIntent::Create(CreateIntent::Entity(
                        crate::transactions::data::EntitySpec {
                            partition_id: PartitionId::main(),
                            kind_id: KindId(1),
                            client_key: crate::symbols::data::ClientKey::raw("feature-only"),
                            fields: crate::tests::support::single_string_aspect_field_patch(
                                crate::tests::support::aspect_key("name"),
                                crate::tests::support::field_key("name"),
                                "feature-only",
                            ),
                        },
                    ))
                    .into(),
                ),
            );
            let _feature_only = changed_entities(&txn.commit().expect("feature create"))[0];

            let prepared = runtime
                .prepare_merge_execution(MergeExecutionRequest {
                    target_branch: BranchId("main".to_string()),
                    source_branch: BranchId("feature".to_string()),
                    merge_intent: MergeIntent::ReconcileIntoTarget,
                })
                .expect("prepared merge");

            runtime.performance_access().reset_counters();
            let verify_started_at = Instant::now();
            runtime
                .merge()
                .verify_prepared_merge_execution(&prepared)
                .expect("verify prepared merge");
            let verify_elapsed_micros = verify_started_at.elapsed().as_micros();
            let verify_counters = runtime.performance_access().counters();

            runtime.performance_access().reset_counters();
            let execute_started_at = Instant::now();
            let outcome = runtime
                .execute_prepared_merge(prepared)
                .expect("execute merge");
            let execute_elapsed_micros = execute_started_at.elapsed().as_micros();
            let execute_counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros: verify_elapsed_micros + execute_elapsed_micros,
                metrics: perf_metrics!({
                    "verify_elapsed_micros": verify_elapsed_micros,
                    "execute_elapsed_micros": execute_elapsed_micros,
                    "executed_record_count": outcome.structural_summary.executed_record_count,
                    "emitted_mutation_intent_count": outcome.structural_summary.emitted_mutation_intent_count,
                    "changed_entities": changed_entities(&outcome.commit).len(),
                    "verify_counters": verify_counters,
                    "execute_counters": execute_counters,
                }),
            }
        },
    );
    assert!(merge_verify_execute_split_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &merge_verify_execute_split_samples,
        "merge verify/execute split should show certified verification and preserve single-record execute truth",
        |metrics| {
            metrics["verify_elapsed_micros"].as_u64().unwrap_or(0) > 0
                && metrics["execute_elapsed_micros"].as_u64().unwrap_or(0) > 0
                && metrics["changed_entities"].as_u64() == Some(1)
                && metrics["verify_counters"]["merge_execution_verification_requests"].as_u64()
                    == Some(1)
                && metrics["verify_counters"]["merge_execution_branch_head_checks"].as_u64()
                    == Some(2)
                && metrics["verify_counters"]["merge_execution_merge_base_checks"].as_u64()
                    == Some(1)
                && metrics["verify_counters"]["merge_execution_compiled_plan_digest_checks"]
                    .as_u64()
                    == Some(1)
                && metrics["execute_counters"]["merge_execution_attempts"].as_u64() == Some(1)
                && metrics["execute_counters"]["merge_execution_records_admitted"].as_u64()
                    == metrics["executed_record_count"].as_u64()
                && metrics["execute_counters"]["merge_execution_mutation_intents_emitted"].as_u64()
                    == metrics["emitted_mutation_intent_count"].as_u64()
        },
    );

    let merge_phase_timing_samples = capture_perf_samples(
        suite,
        "merge_execute_phase_timing_feature_adoption",
        || {
            let mut runtime = persisted_runtime_with_test_schema();
            create_entity(&mut runtime, "main-anchor");
            create_branch_from_main(&mut runtime, "feature");
            let mut txn = runtime.begin_transaction(TransactionOptions {
                target_branch: Some(BranchId("feature".to_string())),
                ..TransactionOptions::default()
            });
            txn.push_batch(
                WorkerIntentBatch::new("create-feature-only").push(
                    MutationIntent::Create(CreateIntent::Entity(
                        crate::transactions::data::EntitySpec {
                            partition_id: PartitionId::main(),
                            kind_id: KindId(1),
                            client_key: crate::symbols::data::ClientKey::raw("feature-only"),
                            fields: crate::tests::support::single_string_aspect_field_patch(
                                crate::tests::support::aspect_key("name"),
                                crate::tests::support::field_key("name"),
                                "feature-only",
                            ),
                        },
                    ))
                    .into(),
                ),
            );
            let _feature_only = changed_entities(&txn.commit().expect("feature create"))[0];

            let prepared = runtime
                .prepare_merge_execution(MergeExecutionRequest {
                    target_branch: BranchId("main".to_string()),
                    source_branch: BranchId("feature".to_string()),
                    merge_intent: MergeIntent::ReconcileIntoTarget,
                })
                .expect("prepared merge");

            runtime.performance_access().reset_counters();
            let started_at = Instant::now();
            let outcome = runtime
                .execute_prepared_merge(prepared)
                .expect("execute merge");
            let elapsed_micros = started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();
            let phase_timing = outcome.commit.execution.phase_timing.clone();

            PerfMeasurement {
                elapsed_micros,
                metrics: perf_metrics!({
                    "executed_record_count": outcome.structural_summary.executed_record_count,
                    "changed_entities": changed_entities(&outcome.commit).len(),
                    "phase_timing": {
                        "working_state_preparation_micros": phase_timing.working_state_preparation_micros,
                        "invariant_pre_check_micros": phase_timing.invariant_pre_check_micros,
                        "authoritative_mutation_micros": phase_timing.authoritative_mutation_micros,
                        "history_resolution_micros": phase_timing.history_resolution_micros,
                        "invariant_post_check_micros": phase_timing.invariant_post_check_micros,
                        "artifact_assembly_micros": phase_timing.artifact_assembly_micros,
                        "durable_append_micros": phase_timing.durable_append_micros,
                        "publication_micros": phase_timing.publication_micros
                    },
                    "counters": counters,
                }),
            }
        },
    );
    emit_metric_summaries(
        suite,
        "merge_execute_phase_timing_feature_adoption",
        &merge_phase_timing_samples,
        &[
            (
                "working_state_preparation_micros",
                &["phase_timing", "working_state_preparation_micros"],
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
        ],
    );
    assert!(merge_phase_timing_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &merge_phase_timing_samples,
        "merge execute phase timing should preserve single-record truth and expose nonzero tail-phase timings",
        |metrics| {
            metrics["changed_entities"].as_u64() == Some(1)
                && metrics["executed_record_count"].as_u64() == Some(1)
                && metrics["phase_timing"]["authoritative_mutation_micros"]
                    .as_u64()
                    .unwrap_or(0)
                    > 0
                && metrics["phase_timing"]["artifact_assembly_micros"]
                    .as_u64()
                    .unwrap_or(0)
                    > 0
                && metrics["phase_timing"]["durable_append_micros"]
                    .as_u64()
                    .unwrap_or(0)
                    > 0
                && metrics["phase_timing"]["publication_micros"]
                    .as_u64()
                    .unwrap_or(0)
                    > 0
                && metrics["counters"]["merge_execution_attempts"].as_u64() == Some(1)
        },
    );

    let lineage_divergence_samples =
        capture_perf_samples(suite, "lineage_branch_divergence_breadth", || {
            let mut runtime = runtime_with_test_schema();
            let created = create_entity_outcome(&mut runtime, "main");
            let start_lineage = runtime
                .lineage_access()
                .for_record(changed_entities(&created)[0])
                .expect("start lineage")
                .lineage_id;
            create_branch_from_main(&mut runtime, "feature");
            let _ = create_entity_outcome_on_branch(
                &mut runtime,
                "feature",
                BranchId("feature".to_string()),
            );

            runtime.performance_access().reset_counters();
            let started_at = Instant::now();
            let divergence =
                runtime
                    .lineage_access()
                    .divergence_between_branches(LineageDivergenceRequest {
                        left_branch: BranchId("main".to_string()),
                        right_branch: BranchId("feature".to_string()),
                        traversal_basis: LineageDivergenceTraversalBasis::FullBranchGraphComparison,
                    });
            let resolution =
                runtime
                    .lineage_access()
                    .resolve_historical_lineage(HistoricalResolutionRequest {
                        branch_id: BranchId("main".to_string()),
                        lineage_id: start_lineage,
                        boundedness_basis:
                            HistoricalResolutionBoundednessBasis::BranchScopedLineageSeed,
                    });
            let elapsed_micros = started_at.elapsed().as_micros();
            let counters = runtime.performance_access().counters();

            PerfMeasurement {
                elapsed_micros,
                metrics: perf_metrics!({
                    "left_event_count": divergence.metrics.left_event_count,
                    "right_event_count": divergence.metrics.right_event_count,
                    "left_node_count": divergence.metrics.left_node_count,
                    "right_node_count": divergence.metrics.right_node_count,
                    "resolution_event_scans": resolution.metrics.branch_event_scan_count,
                    "resolution_traversed_events": resolution.metrics.traversed_event_count,
                    "counters": counters,
                }),
            }
        });
    assert!(lineage_divergence_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &lineage_divergence_samples,
        "lineage divergence and branch-scoped resolution should report their true breadths",
        |metrics| {
            counter_u64(metrics, "lineage_branch_divergence_requests") == 1
                && counter_u64(metrics, "lineage_historical_resolution_requests") == 1
                && counter_u64(metrics, "lineage_branch_divergence_event_scans")
                    == metrics["left_event_count"].as_u64().unwrap_or(0)
                        + metrics["right_event_count"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "lineage_branch_divergence_node_scans")
                    == metrics["left_node_count"].as_u64().unwrap_or(0)
                        + metrics["right_node_count"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "lineage_historical_resolution_branch_event_scans")
                    == metrics["resolution_event_scans"].as_u64().unwrap_or(0)
                && counter_u64(metrics, "lineage_historical_resolution_traversed_events")
                    == metrics["resolution_traversed_events"].as_u64().unwrap_or(0)
        },
    );
}
