use worth_query::facade::runtime::WorthQueryRuntimeBuilder;

use crate::WorthUiDomainEntry;

use super::{
    measurement_recording::{
        WorthUiMeasurementRecording, WorthUiMeasurementRecordingExecutor,
        WorthUiMeasurementRecordingFamily,
    },
    snapshot_measurement::{
        WorthUiSnapshotMeasurement, WorthUiSnapshotMeasurementExecutor,
        WorthUiSnapshotMeasurementFamily,
    },
};

#[cfg(any(test, feature = "certification-construction"))]
use super::snapshot_measurement::WorthUiPartialSnapshotMeasurementExecutor;

/// Installs only Worth UI's volatile Query mechanics. Portable operation
/// meaning remains owned by `worth_ui_domain_package()`.
pub fn install_worth_ui_operation_executors(
    builder: WorthQueryRuntimeBuilder,
) -> WorthQueryRuntimeBuilder {
    builder
        .domain_operation_executor(
            WorthUiDomainEntry,
            WorthUiSnapshotMeasurement,
            WorthUiSnapshotMeasurementFamily,
            WorthUiSnapshotMeasurementExecutor,
        )
        .workflow_stage_executor(
            WorthUiDomainEntry,
            WorthUiMeasurementRecording,
            WorthUiMeasurementRecordingFamily,
            WorthUiMeasurementRecordingExecutor,
        )
}

#[cfg(any(test, feature = "certification-construction"))]
pub fn install_worth_ui_test_operation_executors(
    builder: worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder,
) -> worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder {
    builder
        .domain_operation_executor(
            WorthUiDomainEntry,
            WorthUiSnapshotMeasurement,
            WorthUiSnapshotMeasurementFamily,
            WorthUiSnapshotMeasurementExecutor,
        )
        .workflow_stage_executor(
            WorthUiDomainEntry,
            WorthUiMeasurementRecording,
            WorthUiMeasurementRecordingFamily,
            WorthUiMeasurementRecordingExecutor,
        )
}

#[cfg(any(test, feature = "certification-construction"))]
pub fn install_worth_ui_partial_test_operation_executors(
    builder: worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder,
) -> worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder {
    builder
        .domain_operation_executor(
            WorthUiDomainEntry,
            WorthUiSnapshotMeasurement,
            WorthUiSnapshotMeasurementFamily,
            WorthUiPartialSnapshotMeasurementExecutor,
        )
        .workflow_stage_executor(
            WorthUiDomainEntry,
            WorthUiMeasurementRecording,
            WorthUiMeasurementRecordingFamily,
            WorthUiMeasurementRecordingExecutor,
        )
}
