use std::time::Instant;

use baseline_assertions::{assert_elapsed_against_baseline, assert_metric_against_baseline};
use report_records::{
    emit_external_performance_record, PerfMetricSummaryRecord, PerfSampleRecord, PerfSummaryRecord,
};

pub(super) use super::performance_metrics::{perf_metrics, PerfMetricSet};

mod baseline_assertions;
mod baseline_records;
mod report_records;

#[derive(Debug, Clone)]
pub(super) struct PerfMeasurement {
    pub(super) elapsed_micros: u128,
    pub(super) metrics: PerfMetricSet,
}

pub(super) fn perf_samples() -> usize {
    std::env::var("FORGE_RELATIONAL_PERF_SAMPLES")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(3)
}

pub(super) fn median(mut values: Vec<u128>) -> u128 {
    values.sort_unstable();
    values[values.len() / 2]
}

pub(super) fn metric_u64(metrics: &PerfMetricSet, key: &str) -> u64 {
    metrics
        .metric_u64(key)
        .unwrap_or_else(|| panic!("missing numeric metric `{key}`"))
}

pub(super) fn metric_path_u128(metrics: &PerfMetricSet, path: &[&str]) -> u128 {
    metrics
        .metric_path_u128(path)
        .unwrap_or_else(|| panic!("missing numeric metric path `{}`", path.join(".")))
}

pub(super) fn counter_u64(metrics: &PerfMetricSet, key: &str) -> u64 {
    metrics["counters"][key]
        .as_u64()
        .unwrap_or_else(|| panic!("missing counter metric `{key}`"))
}

pub(super) fn assert_budget(
    samples: &[PerfMeasurement],
    description: &str,
    predicate: impl Fn(&PerfMetricSet) -> bool,
) {
    assert!(
        samples.iter().all(|sample| predicate(&sample.metrics)),
        "performance budget failed: {description}"
    );
}

pub(super) fn measurement_from(
    started_at: Instant,
    build_metrics: impl FnOnce() -> PerfMetricSet,
) -> PerfMeasurement {
    let elapsed_micros = started_at.elapsed().as_micros();
    measurement_with_elapsed(elapsed_micros, build_metrics)
}

pub(super) fn measurement_with_elapsed(
    elapsed_micros: u128,
    build_metrics: impl FnOnce() -> PerfMetricSet,
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
        emit_external_performance_record(PerfSampleRecord {
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
    emit_external_performance_record(&summary);
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
        emit_external_performance_record(&summary);
        assert_metric_against_baseline(suite, case, metric_name, &summary);
    }
}
