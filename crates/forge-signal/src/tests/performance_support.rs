use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

static PERF_ALLOC_LOCK: Mutex<()> = Mutex::new(());

#[global_allocator]
static GLOBAL_ALLOCATOR: &StatsAlloc<std::alloc::System> = &INSTRUMENTED_SYSTEM;

#[derive(Debug, Clone, Copy)]
struct AllocationStats {
    allocation_calls: u64,
    deallocation_calls: u64,
    allocated_bytes: usize,
    deallocated_bytes: usize,
    live_bytes: usize,
    peak_live_bytes: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct PerfMeasurement {
    pub elapsed_micros: u128,
    pub metrics: serde_json::Value,
}

impl PerfMeasurement {
    pub(crate) fn new(elapsed_micros: u128, metrics: serde_json::Value) -> Self {
        Self {
            elapsed_micros,
            metrics,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum PerfTimingPolicy {
    StrictHeavy,
    MedianOnly,
    StructuralOnly,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PerfCaseContract<'a> {
    pub suite: &'a str,
    pub profile: &'a str,
    pub executor: &'a str,
    pub timing_policy: PerfTimingPolicy,
    pub phase_metrics: &'a [&'a str],
    pub access_counter_maxima: &'a [(&'a str, u128)],
}

impl<'a> PerfCaseContract<'a> {
    pub(crate) const fn new(
        suite: &'a str,
        profile: &'a str,
        executor: &'a str,
        timing_policy: PerfTimingPolicy,
        phase_metrics: &'a [&'a str],
        access_counter_maxima: &'a [(&'a str, u128)],
    ) -> Self {
        Self {
            suite,
            profile,
            executor,
            timing_policy,
            phase_metrics,
            access_counter_maxima,
        }
    }
}

#[derive(Debug, Serialize)]
struct PerfSampleRecord<'a> {
    kind: &'static str,
    suite: &'a str,
    profile: &'a str,
    executor: &'a str,
    sample_index: usize,
    elapsed_micros: u128,
    metrics: &'a serde_json::Value,
}

#[derive(Debug, Serialize)]
struct PerfSummaryRecord<'a> {
    kind: &'static str,
    suite: &'a str,
    profile: &'a str,
    executor: &'a str,
    sample_count: usize,
    min_elapsed_micros: u128,
    median_elapsed_micros: u128,
    p95_elapsed_micros: u128,
    p99_elapsed_micros: u128,
    max_elapsed_micros: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PerfCaseSummary {
    suite: String,
    profile: String,
    executor: String,
    sample_count: usize,
    elapsed_micros: NumericSummary,
    allocated_bytes: NumericSummary,
    peak_live_bytes: NumericSummary,
    access_counters: BTreeMap<String, NumericSummary>,
    #[serde(default)]
    phase_metrics: BTreeMap<String, NumericSummary>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct NumericSummary {
    min: u128,
    median: u128,
    p95: u128,
    p99: u128,
    max: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PerfBaselineFile {
    version: u32,
    #[serde(default)]
    environment: Option<PerfEnvironmentFingerprint>,
    cases: BTreeMap<String, PerfCaseSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PerfEnvironmentFingerprint {
    target_os: String,
    target_arch: String,
    profile: String,
    toolchain: Option<String>,
    processor_identifier: Option<String>,
}

pub(crate) fn capture_and_certify_perf_samples<F>(
    contract: PerfCaseContract<'_>,
    measure: F,
) -> Vec<PerfMeasurement>
where
    F: FnMut() -> PerfMeasurement,
{
    let samples = capture_perf_samples(contract, measure);
    let summary = summarize_perf_samples(contract, &samples);
    certify_against_baseline(contract, &summary);
    samples
}

pub(crate) fn capture_perf_samples<F>(
    contract: PerfCaseContract<'_>,
    mut measure: F,
) -> Vec<PerfMeasurement>
where
    F: FnMut() -> PerfMeasurement,
{
    let sample_count = perf_sample_count(contract.timing_policy);
    let mut samples = Vec::with_capacity(sample_count);
    let _alloc_guard = PERF_ALLOC_LOCK
        .lock()
        .expect("perf allocation instrumentation lock should not be poisoned");

    // Prime allocator pages, graph storage interners, and branch-local caches before
    // we start recording cert samples. Without this, targeted perf runs can fail on
    // cold-start noise even when the full suite passes under the same code.
    for _ in 0..perf_warmup_count(contract.timing_policy) {
        let _ = measure();
    }

    for sample_index in 0..sample_count {
        let access_before = crate::data::access_counters::snapshot();
        let region = Region::new(GLOBAL_ALLOCATOR);
        let mut measurement = measure();
        let access_after = crate::data::access_counters::snapshot();
        attach_allocation_stats(&mut measurement.metrics, snapshot_allocation_stats(&region));
        attach_access_counters(
            &mut measurement.metrics,
            access_after.delta_since(access_before),
        );
        emit(&PerfSampleRecord {
            kind: "sample",
            suite: contract.suite,
            profile: contract.profile,
            executor: contract.executor,
            sample_index,
            elapsed_micros: measurement.elapsed_micros,
            metrics: &measurement.metrics,
        });
        samples.push(measurement);
    }

    let mut elapsed = samples
        .iter()
        .map(|sample| sample.elapsed_micros)
        .collect::<Vec<_>>();
    elapsed.sort_unstable();
    emit(&PerfSummaryRecord {
        kind: "summary",
        suite: contract.suite,
        profile: contract.profile,
        executor: contract.executor,
        sample_count,
        min_elapsed_micros: elapsed[0],
        median_elapsed_micros: percentile(&elapsed, 50),
        p95_elapsed_micros: percentile(&elapsed, 95),
        p99_elapsed_micros: percentile(&elapsed, 99),
        max_elapsed_micros: elapsed[elapsed.len() - 1],
    });

    samples
}

fn summarize_perf_samples(
    contract: PerfCaseContract<'_>,
    samples: &[PerfMeasurement],
) -> PerfCaseSummary {
    let elapsed = samples
        .iter()
        .map(|sample| sample.elapsed_micros)
        .collect::<Vec<_>>();
    let allocated_bytes = samples
        .iter()
        .map(|sample| numeric_metric(&sample.metrics, &["allocation_metrics", "allocated_bytes"]))
        .collect::<Vec<_>>();
    let peak_live_bytes = samples
        .iter()
        .map(|sample| numeric_metric(&sample.metrics, &["allocation_metrics", "peak_live_bytes"]))
        .collect::<Vec<_>>();

    let mut access_counters = BTreeMap::new();
    for key in [
        "materialized_entry_reads",
        "materialized_entry_writes",
        "runtime_artifact_warm_reads",
        "runtime_artifact_state_reads",
        "retained_artifact_reads",
        "reconstructed_artifact_reads",
    ] {
        let values = samples
            .iter()
            .map(|sample| numeric_metric(&sample.metrics, &["access_counters", key]))
            .collect::<Vec<_>>();
        access_counters.insert(key.to_string(), summarize_u128(&values));
    }

    let mut phase_metrics = BTreeMap::new();
    for key in contract.phase_metrics {
        let values = samples
            .iter()
            .map(|sample| numeric_metric(&sample.metrics, &[key]))
            .collect::<Vec<_>>();
        phase_metrics.insert((*key).to_string(), summarize_u128(&values));
    }

    let summary = PerfCaseSummary {
        suite: contract.suite.to_string(),
        profile: contract.profile.to_string(),
        executor: contract.executor.to_string(),
        sample_count: samples.len(),
        elapsed_micros: summarize_u128(&elapsed),
        allocated_bytes: summarize_u128(&allocated_bytes),
        peak_live_bytes: summarize_u128(&peak_live_bytes),
        access_counters,
        phase_metrics,
    };

    emit(&summary);
    summary
}

fn perf_sample_count(timing_policy: PerfTimingPolicy) -> usize {
    env::var("FORGE_SIGNAL_PERF_SAMPLES")
        .ok()
        .and_then(|raw| raw.parse::<usize>().ok())
        .filter(|count| *count > 0)
        .unwrap_or(match timing_policy {
            PerfTimingPolicy::StrictHeavy => 5,
            PerfTimingPolicy::MedianOnly => 7,
            PerfTimingPolicy::StructuralOnly => 3,
        })
}

fn perf_warmup_count(timing_policy: PerfTimingPolicy) -> usize {
    env::var("FORGE_SIGNAL_PERF_WARMUPS")
        .ok()
        .and_then(|raw| raw.parse::<usize>().ok())
        .unwrap_or(match timing_policy {
            PerfTimingPolicy::StrictHeavy => 0,
            PerfTimingPolicy::MedianOnly => 2,
            PerfTimingPolicy::StructuralOnly => 0,
        })
}

fn percentile(sorted: &[u128], percentile: usize) -> u128 {
    debug_assert!(!sorted.is_empty());
    debug_assert!(percentile <= 100);

    let rank = (sorted.len() * percentile).div_ceil(100).saturating_sub(1);
    sorted[rank]
}

fn emit(record: &impl Serialize) {
    eprintln!(
        "{}",
        serde_json::to_string(record).expect("perf record should serialize")
    );
}

fn numeric_metric(metrics: &serde_json::Value, path: &[&str]) -> u128 {
    let mut current = metrics;
    for segment in path {
        current = current
            .get(*segment)
            .unwrap_or_else(|| panic!("missing perf metric path {}", path.join(".")));
    }
    current
        .as_u64()
        .map(u128::from)
        .unwrap_or_else(|| panic!("non-numeric perf metric path {}", path.join(".")))
}

fn summarize_u128(values: &[u128]) -> NumericSummary {
    debug_assert!(!values.is_empty());
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    NumericSummary {
        min: sorted[0],
        median: percentile(&sorted, 50),
        p95: percentile(&sorted, 95),
        p99: percentile(&sorted, 99),
        max: sorted[sorted.len() - 1],
    }
}

fn snapshot_allocation_stats(region: &Region<'_, std::alloc::System>) -> AllocationStats {
    let stats = region.change();
    let live_bytes = stats
        .bytes_allocated
        .saturating_sub(stats.bytes_deallocated);
    AllocationStats {
        allocation_calls: stats.allocations as u64,
        deallocation_calls: stats.deallocations as u64,
        allocated_bytes: stats.bytes_allocated,
        deallocated_bytes: stats.bytes_deallocated,
        live_bytes,
        peak_live_bytes: live_bytes,
    }
}

fn attach_allocation_stats(metrics: &mut serde_json::Value, stats: AllocationStats) {
    let allocation_metrics = serde_json::json!({
        "allocation_calls": stats.allocation_calls,
        "deallocation_calls": stats.deallocation_calls,
        "allocated_bytes": stats.allocated_bytes,
        "deallocated_bytes": stats.deallocated_bytes,
        "live_bytes": stats.live_bytes,
        "peak_live_bytes": stats.peak_live_bytes,
    });

    match metrics {
        serde_json::Value::Object(map) => {
            map.insert("allocation_metrics".into(), allocation_metrics);
        }
        _ => {
            *metrics = serde_json::json!({
                "reported_metrics": metrics.clone(),
                "allocation_metrics": allocation_metrics,
            });
        }
    }
}

fn attach_access_counters(
    metrics: &mut serde_json::Value,
    counters: crate::data::access_counters::AccessCounterSnapshot,
) {
    let access_metrics = serde_json::json!(counters);

    match metrics {
        serde_json::Value::Object(map) => {
            map.insert("access_counters".into(), access_metrics);
        }
        _ => {
            *metrics = serde_json::json!({
                "reported_metrics": metrics.clone(),
                "access_counters": access_metrics,
            });
        }
    }
}

fn certify_against_baseline(contract: PerfCaseContract<'_>, summary: &PerfCaseSummary) {
    let key = baseline_case_key(&summary.suite, &summary.profile, &summary.executor);
    let mut baseline = load_baseline_file();
    if update_perf_baseline() {
        baseline.version = 2;
        baseline.environment = Some(current_environment_fingerprint());
        baseline.cases.insert(key, summary.clone());
        write_baseline_file(&baseline);
        return;
    }

    let expected = baseline
        .cases
        .get(&key)
        .unwrap_or_else(|| panic!("missing perf baseline case for {key}"));

    if baseline.environment.as_ref() == Some(&current_environment_fingerprint()) {
        match contract.timing_policy {
            PerfTimingPolicy::StrictHeavy => {
                assert_perf_regression_budget(
                    "elapsed median",
                    summary.elapsed_micros.median,
                    expected.elapsed_micros.median,
                    1.20,
                );
                assert_perf_regression_budget(
                    "elapsed p95",
                    summary.elapsed_micros.p95,
                    expected.elapsed_micros.p95,
                    1.25,
                );
                assert_perf_regression_budget(
                    "elapsed max",
                    summary.elapsed_micros.max,
                    expected.elapsed_micros.max,
                    1.35,
                );
            }
            PerfTimingPolicy::MedianOnly => {
                assert_perf_regression_budget(
                    "elapsed median",
                    summary.elapsed_micros.median,
                    expected.elapsed_micros.median,
                    1.35,
                );
            }
            PerfTimingPolicy::StructuralOnly => {}
        }
    }

    for (counter, maximum) in contract.access_counter_maxima {
        let observed = summary
            .access_counters
            .get(*counter)
            .unwrap_or_else(|| panic!("missing summarized access counter {counter}"))
            .p95;
        assert!(
            observed <= *maximum,
            "access counter {} exceeded allowed maximum: observed {} > {}",
            counter,
            observed,
            maximum
        );
    }

    assert_perf_regression_budget(
        "allocated bytes median",
        summary.allocated_bytes.median,
        expected.allocated_bytes.median,
        1.10,
    );
    assert_perf_regression_budget(
        "allocated bytes max",
        summary.allocated_bytes.max,
        expected.allocated_bytes.max,
        1.10,
    );
    assert_perf_regression_budget(
        "peak live bytes median",
        summary.peak_live_bytes.median,
        expected.peak_live_bytes.median,
        1.10,
    );
    assert_perf_regression_budget(
        "peak live bytes max",
        summary.peak_live_bytes.max,
        expected.peak_live_bytes.max,
        1.10,
    );

    for (counter, observed) in &summary.access_counters {
        let expected = expected
            .access_counters
            .get(counter)
            .unwrap_or_else(|| panic!("missing baseline access counter {counter} for {key}"));
        assert!(
            observed.max <= expected.max,
            "access counter {counter} regressed for {key}: observed max {} > baseline max {}",
            observed.max,
            expected.max
        );
    }

    if !matches!(contract.timing_policy, PerfTimingPolicy::StructuralOnly) {
        for phase_metric in contract.phase_metrics {
            let observed = summary.phase_metrics.get(*phase_metric).unwrap_or_else(|| {
                panic!("missing observed phase metric {phase_metric} for {key}")
            });
            let expected = expected
                .phase_metrics
                .get(*phase_metric)
                .unwrap_or_else(|| {
                    panic!("missing baseline phase metric {phase_metric} for {key}")
                });
            assert_perf_regression_budget(
                &format!("phase metric {phase_metric} median"),
                observed.median,
                expected.median,
                1.25,
            );
        }
    }
}

fn assert_perf_regression_budget(label: &str, observed: u128, expected: u128, tolerance: f64) {
    let allowed = ((expected as f64) * tolerance).ceil() as u128;
    assert!(
        observed <= allowed,
        "{label} regressed: observed {observed} exceeds allowed {allowed} from baseline {expected}"
    );
}

fn baseline_case_key(suite: &str, profile: &str, executor: &str) -> String {
    format!("{suite}|{profile}|{executor}")
}

fn baseline_file_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("tests")
        .join("performance_baseline.json")
}

fn load_baseline_file() -> PerfBaselineFile {
    let path = baseline_file_path();
    let raw = fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "failed to read perf baseline file {}: {err}",
            path.display()
        )
    });
    serde_json::from_str(&raw).unwrap_or_else(|err| {
        panic!(
            "failed to deserialize perf baseline file {}: {err}",
            path.display()
        )
    })
}

fn write_baseline_file(baseline: &PerfBaselineFile) {
    let path = baseline_file_path();
    let raw = serde_json::to_string_pretty(baseline).expect("perf baseline file should serialize");
    fs::write(&path, raw).unwrap_or_else(|err| {
        panic!(
            "failed to write perf baseline file {}: {err}",
            path.display()
        )
    });
}

fn update_perf_baseline() -> bool {
    env::var("FORGE_SIGNAL_UPDATE_PERF_BASELINE")
        .ok()
        .map(|value| {
            let value = value.trim().to_ascii_lowercase();
            value == "1" || value == "true" || value == "yes"
        })
        .unwrap_or(false)
}

fn current_environment_fingerprint() -> PerfEnvironmentFingerprint {
    PerfEnvironmentFingerprint {
        target_os: env::consts::OS.to_string(),
        target_arch: env::consts::ARCH.to_string(),
        profile: env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string()),
        toolchain: env::var("RUSTUP_TOOLCHAIN").ok(),
        processor_identifier: env::var("PROCESSOR_IDENTIFIER").ok(),
    }
}
