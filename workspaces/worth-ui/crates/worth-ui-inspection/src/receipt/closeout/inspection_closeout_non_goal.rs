#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionCloseoutNonGoal {
    MeasurementEvidence,
    MountEligibilityEvidence,
    VisualSnapshotEvidence,
    ReplayEvidence,
    RendererLocalExplanation,
    HostLocalExplanation,
    LogLocalExplanation,
}
