#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionCloseoutNonGoal {
    MeasurementEvidence,
    MountedReceiptEvidence,
    VisualSnapshotEvidence,
    ReplayEvidence,
    RendererLocalExplanation,
    HostLocalExplanation,
    LogLocalExplanation,
}
