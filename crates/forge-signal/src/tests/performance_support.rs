use std::env;

use serde::Serialize;

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

pub(crate) fn capture_perf_samples<F>(
    suite: &str,
    profile: &str,
    executor: &str,
    mut measure: F,
) -> Vec<PerfMeasurement>
where
    F: FnMut() -> PerfMeasurement,
{
    let sample_count = perf_sample_count();
    let mut samples = Vec::with_capacity(sample_count);

    for sample_index in 0..sample_count {
        let measurement = measure();
        emit(&PerfSampleRecord {
            kind: "sample",
            suite,
            profile,
            executor,
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
        suite,
        profile,
        executor,
        sample_count,
        min_elapsed_micros: elapsed[0],
        median_elapsed_micros: percentile(&elapsed, 50),
        p95_elapsed_micros: percentile(&elapsed, 95),
        p99_elapsed_micros: percentile(&elapsed, 99),
        max_elapsed_micros: elapsed[elapsed.len() - 1],
    });

    samples
}

fn perf_sample_count() -> usize {
    env::var("FORGE_SIGNAL_PERF_SAMPLES")
        .ok()
        .and_then(|raw| raw.parse::<usize>().ok())
        .filter(|count| *count > 0)
        .unwrap_or(1)
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
