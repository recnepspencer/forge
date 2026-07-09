use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::OnceLock;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct PerfBaselineElapsedRow {
    pub(super) suite: String,
    pub(super) case: String,
    pub(super) median_elapsed_micros: u128,
    pub(super) max_elapsed_micros: u128,
}

#[derive(Debug, Deserialize)]
pub(super) struct PerfBaselineMetricRow {
    pub(super) suite: String,
    pub(super) case: String,
    pub(super) metric: String,
    pub(super) median: u128,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PerfBaselineRow {
    Metric(PerfBaselineMetricRow),
    Elapsed(PerfBaselineElapsedRow),
}

static PERF_BASELINE_ELAPSED_ROWS: OnceLock<BTreeMap<(String, String), PerfBaselineElapsedRow>> =
    OnceLock::new();
static PERF_BASELINE_METRIC_ROWS: OnceLock<
    BTreeMap<(String, String, String), PerfBaselineMetricRow>,
> = OnceLock::new();

pub(super) fn perf_baseline_elapsed_rows(
) -> &'static BTreeMap<(String, String), PerfBaselineElapsedRow> {
    PERF_BASELINE_ELAPSED_ROWS.get_or_init(|| {
        perf_external_baseline_rows()
            .into_iter()
            .filter_map(|row| match row {
                PerfBaselineRow::Elapsed(parsed) => {
                    Some(((parsed.suite.clone(), parsed.case.clone()), parsed))
                }
                PerfBaselineRow::Metric(_) => None,
            })
            .collect()
    })
}

pub(super) fn perf_baseline_metric_rows(
) -> &'static BTreeMap<(String, String, String), PerfBaselineMetricRow> {
    PERF_BASELINE_METRIC_ROWS.get_or_init(|| {
        perf_external_baseline_rows()
            .into_iter()
            .filter_map(|row| match row {
                PerfBaselineRow::Metric(parsed) => Some((
                    (
                        parsed.suite.clone(),
                        parsed.case.clone(),
                        parsed.metric.clone(),
                    ),
                    parsed,
                )),
                PerfBaselineRow::Elapsed(_) => None,
            })
            .collect()
    })
}

fn perf_external_baseline_record_path() -> PathBuf {
    std::env::var("RELATIONAL_PERF_BASELINE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            worth_harness::facade::engineering_external_record_lines_path(
                "worth_relational_performance_baseline",
            )
        })
}

fn perf_external_baseline_rows() -> Vec<PerfBaselineRow> {
    let path = perf_external_baseline_record_path();
    worth_harness::facade::read_external_record_lines(&path, "relational performance baseline")
}
