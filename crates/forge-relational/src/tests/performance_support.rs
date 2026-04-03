use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
pub(super) struct PerfMeasurement {
    pub(super) elapsed_micros: u128,
    pub(super) metrics: Value,
}

#[derive(Debug, Serialize)]
pub(super) struct PerfSampleRecord<'a> {
    pub(super) suite: &'a str,
    pub(super) case: &'a str,
    pub(super) sample: usize,
    pub(super) elapsed_micros: u128,
    pub(super) metrics: &'a Value,
}

#[derive(Debug, Serialize)]
pub(super) struct PerfSummaryRecord<'a> {
    pub(super) suite: &'a str,
    pub(super) case: &'a str,
    pub(super) samples: usize,
    pub(super) mean_elapsed_micros: f64,
    pub(super) median_elapsed_micros: u128,
    pub(super) min_elapsed_micros: u128,
    pub(super) max_elapsed_micros: u128,
}

#[derive(Debug, Serialize)]
pub(super) struct PerfMetricSummaryRecord<'a> {
    pub(super) suite: &'a str,
    pub(super) case: &'a str,
    pub(super) metric: &'a str,
    pub(super) samples: usize,
    pub(super) mean: f64,
    pub(super) median: u128,
    pub(super) min: u128,
    pub(super) max: u128,
}

#[derive(Debug, Clone, Copy)]
struct PerfCaseContract {
    elapsed_median_tolerance: f64,
    elapsed_max_tolerance: f64,
    metric_median_tolerance: f64,
    enforce_elapsed_max: bool,
    zero_baseline_metric_floor: u128,
}

impl PerfCaseContract {
    const fn standard() -> Self {
        Self {
            elapsed_median_tolerance: 1.20,
            elapsed_max_tolerance: 1.35,
            metric_median_tolerance: 1.25,
            enforce_elapsed_max: true,
            zero_baseline_metric_floor: 4,
        }
    }

    const fn noisy() -> Self {
        Self {
            elapsed_median_tolerance: 1.50,
            elapsed_max_tolerance: 1.75,
            metric_median_tolerance: 1.75,
            enforce_elapsed_max: true,
            zero_baseline_metric_floor: 8,
        }
    }

    const fn bursty() -> Self {
        Self {
            elapsed_median_tolerance: 1.75,
            elapsed_max_tolerance: 2.10,
            metric_median_tolerance: 1.90,
            enforce_elapsed_max: true,
            zero_baseline_metric_floor: 12,
        }
    }

    const fn tiny() -> Self {
        Self {
            elapsed_median_tolerance: 2.25,
            elapsed_max_tolerance: 2.50,
            metric_median_tolerance: 2.00,
            enforce_elapsed_max: true,
            zero_baseline_metric_floor: 16,
        }
    }

    const fn micro() -> Self {
        Self {
            elapsed_median_tolerance: 3.00,
            elapsed_max_tolerance: 3.50,
            metric_median_tolerance: 3.00,
            enforce_elapsed_max: true,
            zero_baseline_metric_floor: 16,
        }
    }

    const fn io_bursty() -> Self {
        Self {
            elapsed_median_tolerance: 1.90,
            elapsed_max_tolerance: 2.50,
            metric_median_tolerance: 2.00,
            enforce_elapsed_max: false,
            zero_baseline_metric_floor: 32,
        }
    }

    const fn volatile() -> Self {
        Self {
            elapsed_median_tolerance: 2.10,
            elapsed_max_tolerance: 2.50,
            metric_median_tolerance: 2.25,
            enforce_elapsed_max: false,
            zero_baseline_metric_floor: 32,
        }
    }

    const fn extreme_volatile() -> Self {
        Self {
            elapsed_median_tolerance: 2.50,
            elapsed_max_tolerance: 3.00,
            metric_median_tolerance: 3.00,
            enforce_elapsed_max: false,
            zero_baseline_metric_floor: 32,
        }
    }
}

#[derive(Debug, Deserialize)]
struct PerfBaselineElapsedRow {
    suite: String,
    case: String,
    median_elapsed_micros: u128,
    max_elapsed_micros: u128,
}

#[derive(Debug, Deserialize)]
struct PerfBaselineMetricRow {
    suite: String,
    case: String,
    metric: String,
    median: u128,
}

static PERF_BASELINE_ELAPSED_ROWS: OnceLock<BTreeMap<(String, String), PerfBaselineElapsedRow>> =
    OnceLock::new();
static PERF_BASELINE_METRIC_ROWS: OnceLock<
    BTreeMap<(String, String, String), PerfBaselineMetricRow>,
> = OnceLock::new();

pub(super) fn perf_samples() -> usize {
    std::env::var("FORGE_RELATIONAL_PERF_SAMPLES")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(3)
}

fn perf_baseline_path() -> PathBuf {
    std::env::var("RELATIONAL_PERF_BASELINE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("..")
                .join("_docs")
                .join("engineering")
                .join("forge_relational_performance_baseline.jsonl")
        })
}

fn perf_baseline_rows() -> Vec<Value> {
    let path = perf_baseline_path();
    fs::read_to_string(&path)
        .unwrap_or_else(|error| {
            panic!(
                "failed to read relational perf baseline {}: {error}",
                path.display()
            )
        })
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<Value>(line).unwrap_or_else(|error| {
                panic!(
                    "failed to deserialize relational perf baseline row from {}: {error}",
                    path.display()
                )
            })
        })
        .collect()
}

fn perf_baseline_elapsed_rows() -> &'static BTreeMap<(String, String), PerfBaselineElapsedRow> {
    PERF_BASELINE_ELAPSED_ROWS.get_or_init(|| {
        perf_baseline_rows()
            .into_iter()
            .filter(|row| row.get("metric").is_none())
            .map(|row| {
                let parsed = serde_json::from_value::<PerfBaselineElapsedRow>(row)
                    .expect("elapsed baseline row should deserialize");
                ((parsed.suite.clone(), parsed.case.clone()), parsed)
            })
            .collect()
    })
}

fn perf_baseline_metric_rows() -> &'static BTreeMap<(String, String, String), PerfBaselineMetricRow> {
    PERF_BASELINE_METRIC_ROWS.get_or_init(|| {
        perf_baseline_rows()
            .into_iter()
            .filter(|row| row.get("metric").is_some())
            .map(|row| {
                let parsed = serde_json::from_value::<PerfBaselineMetricRow>(row)
                    .expect("metric baseline row should deserialize");
                (
                    (
                        parsed.suite.clone(),
                        parsed.case.clone(),
                        parsed.metric.clone(),
                    ),
                    parsed,
                )
            })
            .collect()
    })
}

fn perf_case_contract(suite: &str, case: &str) -> PerfCaseContract {
    match (suite, case) {
        ("commit_delta_matrix", "persisted_single_entity_create")
        | (
            "artifact_recoverability_matrix",
            "geometry_diagnostics_summary_vs_trace_recoverability",
        )
        | ("artifact_recoverability_matrix", "chip_compiled_artifact_recoverability")
        | ("cad_topology_matrix", "assembly_interface_bridge_wave")
        | ("chip_simulator_matrix", "checkpoint_window_recover_compile_round_trip")
        | ("durability_append_matrix", "append_canonical_envelope_fresh_store")
        | ("durability_append_matrix", "append_canonical_envelope_existing_segment")
        | ("index_parity_matrix", "persisted_recovery_generation_parity")
        | ("invariant_materialization_matrix", "custom_structural_surface_commit_wave")
        | ("hot_cold_path_matrix", "geometry_hot_commit_vs_replay_reconstruction")
        | ("hot_cold_path_matrix", "chip_hot_compile_vs_recovery_compile")
        | ("hot_cold_path_matrix", "geometry_rich_publication_hot_vs_replay_truth")
        | ("hot_cold_path_matrix", "chip_rich_compile_hot_vs_recovery_compile")
        | (
            "recoverability_policy_matrix",
            "geometry_hot_truth_vs_deferred_trace_policy",
        )
        | ("recoverability_policy_matrix", "chip_compile_reconstructable_policy")
        | ("rocketship_scale_matrix", "hundred_k_nodes_geometry_profile_propagation_wave")
        | ("rocketship_scale_matrix", "hundred_k_nodes_pseudorealistic_propagation_wave")
        | ("rocketship_scale_matrix", "hundred_k_nodes_pseudorealistic_subsystem_round_trip")
        | ("rocketship_scale_matrix", "hundred_k_nodes_zero_diagnostics_narrow_round_trip")
        | ("rocketship_scale_matrix", "hundred_k_nodes_geometry_profile_narrow_round_trip")
        | ("sustained_load_matrix", "rocketship_hot_update_endurance")
        | ("sustained_load_matrix", "rocketship_propagation_endurance")
        | ("sustained_load_matrix", "chip_global_step_endurance")
        | ("replay_recovery_matrix", "durable_replay_lineage_basis")
        | ("sustained_load_matrix", "replay_window_drift_stability") => {
            PerfCaseContract::io_bursty()
        }
        ("commit_delta_matrix", "single_partition_create_burst")
        | ("geometry_kernel_matrix", "topology_identity_survival_recovery_round_trip")
        | ("chip_simulator_matrix", "branch_rollback_compile_step_window")
        | ("index_parity_matrix", "entity_field_equals_warm_generation")
        | ("inspection_budget_matrix", "retention_commit_window")
        | ("sustained_load_matrix", "commit_query_churn_stability")
        | (
            "snapshot_materialization_matrix",
            "projection_entity_identity_surface",
        )
        | ("workflow_matrix", "persisted_recovery_replay_round_trip")
        | ("profile_matrix", "certification_core_rich_commit_query_round_trip")
        | ("profile_matrix", "geometry_kernel_rich_commit_query_round_trip") => {
            PerfCaseContract::volatile()
        }
        ("chip_simulator_matrix", "event_wave_compile_churn_window")
        | ("chip_simulator_matrix", "event_wave_compile_churn_rich_diagnostics") => {
            PerfCaseContract::extreme_volatile()
        }
        ("merge_lineage_matrix", "merge_execution_feature_adoption")
        | ("merge_lineage_matrix", "merge_execution_feature_adoption_zero_diagnostics_budget")
        | ("merge_lineage_matrix", "merge_execute_phase_timing_feature_adoption")
        | ("merge_lineage_matrix", "merge_execution_vs_persisted_commit_floor")
        | ("merge_lineage_matrix", "merge_prepare_vs_execute_feature_adoption")
        | (
            "geometry_artifact_decomposition_matrix",
            "hundred_k_nodes_pseudorealistic_rich_artifact_classes",
        )
        | ("geometry_kernel_matrix", "topology_bridge_connectivity_wave")
        | ("chip_simulator_matrix", "dense_fanout_compile_wave")
        | ("chip_simulator_matrix", "dense_fanout_compile_wave_rich_diagnostics")
        | ("runtime_bridge_mock_matrix", "geometry_commit_bridge_wave_operational")
        | ("runtime_bridge_mock_matrix", "geometry_commit_bridge_wave_development")
        | (
            "runtime_bridge_mock_matrix",
            "geometry_commit_bridge_wave_medium_region_operational",
        )
        | (
            "runtime_bridge_mock_matrix",
            "geometry_commit_bridge_wave_medium_region_development",
        )
        | (
            "runtime_bridge_mock_matrix",
            "geometry_commit_bridge_wave_mixed_locality_operational",
        )
        | ("mixed_load_matrix", "concurrent_relation_index_parity_pressure")
        | ("game_engine_matrix", "local_scene_graph_propagation_wave")
        | ("game_engine_matrix", "mixed_read_write_frame_churn_window")
        | ("query_packet_matrix", "explicit_targets_cross_partition")
        | ("sustained_load_matrix", "mixed_topology_query_churn_stability")
        | ("sustained_load_matrix", "retention_pass_drift_stability")
        | ("workflow_matrix", "trade_correction_analysis_round_trip")
        | ("workflow_matrix", "fintech_intraday_risk_branch_round_trip")
        | ("workflow_matrix", "fintech_trade_correction_audit_round_trip")
        | (
            "profile_matrix",
            "certification_core_zero_diagnostics_commit_query_round_trip",
        ) => {
            PerfCaseContract::bursty()
        }
        ("commit_delta_matrix", "cross_partition_relation_burst")
        | ("geometry_kernel_matrix", "topology_bridge_connectivity_wave_rich_geometry_profile")
        | ("geometry_kernel_matrix", "topology_bridge_connectivity_wave_zero_diagnostics")
        | ("mixed_load_matrix", "concurrent_snapshot_version_read_pressure")
        | ("query_packet_matrix", "connectivity_traversal_cross_partition")
        | ("snapshot_materialization_matrix", "version_read_view_historical")
        | ("replay_recovery_matrix", "checkpoint_recover_suffix_replay") => {
            PerfCaseContract::noisy()
        }
        ("merge_lineage_matrix", "merge_planning_divergent_update")
        | ("index_parity_matrix", "entity_field_equals_build_failed_fallback")
        | ("inspection_budget_matrix", "structural_identity_historical_window")
        | ("merge_lineage_matrix", "lineage_branch_divergence_breadth")
        | ("query_packet_matrix", "entity_kind_scan_partition_matrix")
        | ("retention_reclaim_matrix", "replay_pin_release_deleted_relation")
        | ("snapshot_materialization_matrix", "snapshot_read_view_current") => {
            PerfCaseContract::tiny()
        }
        ("inspection_budget_matrix", "graph_kind_connectivity_bundle")
        | ("retention_reclaim_matrix", "snapshot_release_to_reclaimable_entity")
        | ("workflow_matrix", "retention_release_reclaim_round_trip") => PerfCaseContract::micro(),
        _ => PerfCaseContract::standard(),
    }
}

fn allow_missing_baseline_rows() -> bool {
    std::env::var("RELATIONAL_PERF_ALLOW_MISSING_BASELINE")
        .ok()
        .map(|value| {
            let normalized = value.trim().to_ascii_lowercase();
            normalized == "1" || normalized == "true" || normalized == "yes"
        })
        .unwrap_or(false)
}

fn skip_baseline_asserts() -> bool {
    std::env::var("RELATIONAL_PERF_SKIP_BASELINE_ASSERTS")
        .ok()
        .map(|value| {
            let normalized = value.trim().to_ascii_lowercase();
            normalized == "1" || normalized == "true" || normalized == "yes"
        })
        .unwrap_or(false)
}

fn allowed_perf_regression(baseline: u128, tolerance: f64) -> u128 {
    ((baseline as f64) * tolerance).ceil() as u128
}

fn allowed_metric_regression(baseline: u128, contract: PerfCaseContract) -> u128 {
    if baseline == 0 {
        contract.zero_baseline_metric_floor
    } else {
        allowed_perf_regression(baseline, contract.metric_median_tolerance)
    }
}

fn profile_boundary_metric(metric_name: &str) -> bool {
    matches!(
        metric_name,
        "profile_execution_lane_code"
            | "profile_diagnostics_boundary_code"
            | "profile_matches_defaults"
    )
}

fn assert_elapsed_against_baseline(
    suite: &str,
    case: &str,
    summary: &PerfSummaryRecord<'_>,
) {
    if skip_baseline_asserts() {
        return;
    }
    let Some(baseline) = perf_baseline_elapsed_rows().get(&(suite.to_string(), case.to_string())) else {
        if allow_missing_baseline_rows() {
            return;
        }
        panic!("missing relational perf baseline for {suite}/{case}");
    };
    let contract = perf_case_contract(suite, case);

    let allowed_median =
        allowed_perf_regression(baseline.median_elapsed_micros, contract.elapsed_median_tolerance);
    assert!(
        summary.median_elapsed_micros <= allowed_median,
        "elapsed median regressed for {suite}/{case}: observed {} > allowed {} from baseline {}",
        summary.median_elapsed_micros,
        allowed_median,
        baseline.median_elapsed_micros
    );

    if contract.enforce_elapsed_max {
        let allowed_max =
            allowed_perf_regression(baseline.max_elapsed_micros, contract.elapsed_max_tolerance);
        assert!(
            summary.max_elapsed_micros <= allowed_max,
            "elapsed max regressed for {suite}/{case}: observed {} > allowed {} from baseline {}",
            summary.max_elapsed_micros,
            allowed_max,
            baseline.max_elapsed_micros
        );
    }
}

fn assert_metric_against_baseline(
    suite: &str,
    case: &str,
    metric_name: &str,
    summary: &PerfMetricSummaryRecord<'_>,
) {
    if skip_baseline_asserts() {
        return;
    }
    let Some(baseline) = perf_baseline_metric_rows()
        .get(&(suite.to_string(), case.to_string(), metric_name.to_string()))
    else {
        if allow_missing_baseline_rows() {
            return;
        }
        panic!("missing relational perf baseline metric for {suite}/{case}/{metric_name}");
    };
    if profile_boundary_metric(metric_name) {
        assert!(
            summary.median == baseline.median,
            "profile boundary drift for {suite}/{case}/{metric_name}: observed {} != baseline {}",
            summary.median,
            baseline.median
        );
        return;
    }
    let contract = perf_case_contract(suite, case);
    let allowed_median = allowed_metric_regression(baseline.median, contract);
    assert!(
        summary.median <= allowed_median,
        "phase metric median regressed for {suite}/{case}/{metric_name}: observed {} > allowed {} from baseline {}",
        summary.median,
        allowed_median,
        baseline.median
    );
}

pub(super) fn emit_json(value: impl Serialize) {
    println!(
        "{}",
        serde_json::to_string(&value).expect("performance JSON serialization")
    );
}

pub(super) fn median(mut values: Vec<u128>) -> u128 {
    values.sort_unstable();
    values[values.len() / 2]
}

pub(super) fn metric_u64(metrics: &Value, key: &str) -> u64 {
    metrics[key]
        .as_u64()
        .unwrap_or_else(|| panic!("missing numeric metric `{key}`"))
}

pub(super) fn metric_path_u128(metrics: &Value, path: &[&str]) -> u128 {
    let mut current = metrics;
    for key in path {
        current = &current[*key];
    }
    current
        .as_u64()
        .map(u128::from)
        .unwrap_or_else(|| panic!("missing numeric metric path `{}`", path.join(".")))
}

pub(super) fn counter_u64(metrics: &Value, key: &str) -> u64 {
    metrics["counters"][key]
        .as_u64()
        .unwrap_or_else(|| panic!("missing counter metric `{key}`"))
}

pub(super) fn assert_budget(
    samples: &[PerfMeasurement],
    description: &str,
    predicate: impl Fn(&Value) -> bool,
) {
    assert!(
        samples.iter().all(|sample| predicate(&sample.metrics)),
        "performance budget failed: {description}"
    );
}

pub(super) fn measurement_from(
    started_at: Instant,
    build_metrics: impl FnOnce() -> Value,
) -> PerfMeasurement {
    let elapsed_micros = started_at.elapsed().as_micros();
    measurement_with_elapsed(elapsed_micros, build_metrics)
}

pub(super) fn measurement_with_elapsed(
    elapsed_micros: u128,
    build_metrics: impl FnOnce() -> Value,
) -> PerfMeasurement {
    PerfMeasurement {
        elapsed_micros,
        metrics: build_metrics(),
    }
}

pub(super) fn capture_perf_samples(
    suite: &'static str,
    case: &'static str,
    mut run: impl FnMut() -> PerfMeasurement,
) -> Vec<PerfMeasurement> {
    let mut samples = Vec::with_capacity(perf_samples());
    for sample_index in 0..perf_samples() {
        let measurement = run();
        emit_json(PerfSampleRecord {
            suite,
            case,
            sample: sample_index,
            elapsed_micros: measurement.elapsed_micros,
            metrics: &measurement.metrics,
        });
        samples.push(measurement);
    }

    let elapsed_values = samples
        .iter()
        .map(|measurement| measurement.elapsed_micros)
        .collect::<Vec<_>>();
    let total_elapsed = elapsed_values.iter().copied().sum::<u128>();
    let summary = PerfSummaryRecord {
        suite,
        case,
        samples: samples.len(),
        mean_elapsed_micros: total_elapsed as f64 / samples.len() as f64,
        median_elapsed_micros: median(elapsed_values.clone()),
        min_elapsed_micros: *elapsed_values.iter().min().expect("sample minimum"),
        max_elapsed_micros: *elapsed_values.iter().max().expect("sample maximum"),
    };
    emit_json(&summary);
    assert_elapsed_against_baseline(suite, case, &summary);

    samples
}

pub(super) fn emit_metric_summaries(
    suite: &'static str,
    case: &'static str,
    samples: &[PerfMeasurement],
    metrics: &[(&'static str, &[&str])],
) {
    for (metric_name, path) in metrics {
        let values = samples
            .iter()
            .map(|sample| metric_path_u128(&sample.metrics, path))
            .collect::<Vec<_>>();
        let total = values.iter().copied().sum::<u128>();
        let summary = PerfMetricSummaryRecord {
            suite,
            case,
            metric: metric_name,
            samples: values.len(),
            mean: total as f64 / values.len() as f64,
            median: median(values.clone()),
            min: *values.iter().min().expect("metric minimum"),
            max: *values.iter().max().expect("metric maximum"),
        };
        emit_json(&summary);
        assert_metric_against_baseline(suite, case, metric_name, &summary);
    }
}
