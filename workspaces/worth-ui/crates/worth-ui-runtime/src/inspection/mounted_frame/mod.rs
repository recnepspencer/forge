mod receipt;
mod request;
mod retention_report;

pub use receipt::{
    UiMountedInspectedFrame, UiMountedInspectionOmission, UiMountedInspectionReceipt,
    UiMountedInspectionRelation,
};
pub use request::{UiMountedInspectionRequest, UiMountedInspectionTarget};
pub use retention_report::{
    UiMountedRetentionClassReport, UiMountedRetentionEvictionPosture,
    UiMountedRetentionQueueBudget, UiMountedRetentionReport,
};

pub(crate) fn inspect(
    retention: &crate::mounting::UiMountedFrameRetentionCoordinator,
    request: UiMountedInspectionRequest,
) -> UiMountedInspectionReceipt {
    match retention.inspect(request.into_selection()) {
        Ok(basis) => UiMountedInspectionReceipt::available(basis),
        Err(denial) => UiMountedInspectionReceipt::omitted(denial),
    }
}

pub(crate) fn retention_report(
    retention: &crate::mounting::UiMountedFrameRetentionCoordinator,
    observations: &crate::host_exchange::observation_report_validation::UiHostObservationReportValidation,
) -> UiMountedRetentionReport {
    retention_report::compose(
        retention.retention_snapshot(),
        observations.retention_snapshot(),
    )
}
