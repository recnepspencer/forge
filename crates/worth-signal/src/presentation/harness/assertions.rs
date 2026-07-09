use worth_harness::facade::{HarnessObservedBundle, ObservationStatus, RunRecord, SnapshotRecord};
use serde_json::Value;

use crate::facade::*;

pub struct SignalHarnessAssert;

impl SignalHarnessAssert {
    pub fn run_target_status(run: &RunRecord<String>, target: &str) -> ObservationStatus {
        run.target_statuses
            .iter()
            .find(|status| status.target == target)
            .map(|status| status.status)
            .unwrap_or_else(|| panic!("missing target status for `{target}`"))
    }

    pub fn snapshot_target_status(
        snapshot: &SnapshotRecord<String>,
        target: &str,
    ) -> ObservationStatus {
        snapshot
            .observations
            .iter()
            .find(|observation| observation.target == target)
            .map(|observation| observation.status)
            .unwrap_or_else(|| panic!("missing snapshot observation for `{target}`"))
    }

    pub fn assert_run_target_status(
        run: &RunRecord<String>,
        target: &str,
        expected: ObservationStatus,
    ) {
        assert_eq!(Self::run_target_status(run, target), expected);
    }

    pub fn assert_snapshot_target_status(
        snapshot: &SnapshotRecord<String>,
        target: &str,
        expected: ObservationStatus,
    ) {
        assert_eq!(Self::snapshot_target_status(snapshot, target), expected);
    }

    pub fn execution_report(run: &RunRecord<String>) -> ExecutionReport {
        let value = run
            .extensions
            .get("execution_report")
            .cloned()
            .unwrap_or_else(|| panic!("run record is missing execution_report extension"));
        serde_json::from_value(value)
            .unwrap_or_else(|error| panic!("invalid execution_report extension: {error}"))
    }

    pub fn plan_summary(run: &RunRecord<String>) -> PlanSummary {
        let value = run
            .extensions
            .get("evaluation_plan_summary")
            .cloned()
            .unwrap_or_else(|| panic!("run record is missing evaluation_plan_summary"));
        serde_json::from_value(value)
            .unwrap_or_else(|error| panic!("invalid evaluation_plan_summary extension: {error}"))
    }

    pub fn assert_no_snapshot(snapshot: &Option<SnapshotRecord<String>>) {
        assert!(snapshot.is_none(), "expected no snapshot to be captured");
    }

    pub fn assert_has_snapshot(
        snapshot: &Option<SnapshotRecord<String>>,
    ) -> &SnapshotRecord<String> {
        snapshot
            .as_ref()
            .unwrap_or_else(|| panic!("expected snapshot to be captured"))
    }

    pub fn performance_metric(bundle: &HarnessObservedBundle<String>, metric: &str) -> Option<u64> {
        let mut value = bundle.performance.as_ref()?;
        for segment in metric.split('.') {
            value = value.get(segment)?;
        }
        value.as_u64()
    }

    pub fn diagnostics_field<'a>(
        bundle: &'a HarnessObservedBundle<String>,
        field: &str,
    ) -> Option<&'a Value> {
        bundle.diagnostics.as_ref()?.summary.get(field)
    }
}
