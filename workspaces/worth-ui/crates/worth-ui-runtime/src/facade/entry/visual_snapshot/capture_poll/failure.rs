pub(super) enum UiVisualCaptureFailure {
    Superseded(worth_ui_inspection::UiVisualSnapshotSuperseded),
    Omitted(worth_ui_inspection::UiVisualSnapshotOmission),
    Denied(worth_ui_inspection::UiVisualSnapshotDenial),
    Indeterminate(worth_ui_inspection::UiVisualSnapshotIndeterminate),
}

impl UiVisualCaptureFailure {
    pub(super) fn into_outcome<ArtifactPosture>(
        self,
    ) -> crate::inspection::visual_snapshot::UiVisualSnapshotOutcome<ArtifactPosture>
    where
        ArtifactPosture: worth_ui_inspection::UiVisualArtifactPolicy,
    {
        use crate::inspection::visual_snapshot::UiVisualSnapshotOutcome;

        match self {
            Self::Superseded(posture) => UiVisualSnapshotOutcome::Superseded(posture),
            Self::Omitted(posture) => UiVisualSnapshotOutcome::Omitted(posture),
            Self::Denied(posture) => UiVisualSnapshotOutcome::Denied(posture),
            Self::Indeterminate(posture) => UiVisualSnapshotOutcome::Indeterminate(posture),
        }
    }
}
