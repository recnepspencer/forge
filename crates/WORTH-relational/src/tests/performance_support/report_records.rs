use serde::Serialize;

use super::PerfMetricSet;

#[derive(Debug, Serialize)]
pub(super) struct PerfSampleRecord<'a> {
    pub(super) suite: &'a str,
    pub(super) case: &'a str,
    pub(super) sample: usize,
    pub(super) elapsed_micros: u128,
    pub(super) metrics: &'a PerfMetricSet,
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

pub(super) fn emit_external_performance_record(value: impl Serialize) {
    worth_harness::facade::emit_external_record_line(value);
}
