use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::measurement_capture::{PerfCaseContract, PerfCaseSummary, PerfTimingPolicy};

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

pub(super) fn certify_against_baseline(contract: PerfCaseContract<'_>, summary: &PerfCaseSummary) {
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
    env::var("WORTH_SIGNAL_UPDATE_PERF_BASELINE")
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
