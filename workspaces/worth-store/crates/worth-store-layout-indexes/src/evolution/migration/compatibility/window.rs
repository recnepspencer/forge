use worth_store_compatibility::{
    ArtifactCompatibilityDenial, ArtifactCompatibilityWindow, ArtifactFormatVersion,
};

use super::LayoutVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutReadCompatibilityPosture {
    CurrentOnly,
    ReadOldWriteNew,
    DualRead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutWriteCompatibilityPosture {
    CurrentOnly,
    WriteNewDuringRollingUpgrade,
    DualWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutCompatibilityWindow {
    artifact_window: ArtifactCompatibilityWindow,
    read_posture: LayoutReadCompatibilityPosture,
    write_posture: LayoutWriteCompatibilityPosture,
}

impl LayoutCompatibilityWindow {
    pub fn new(
        minimum_readable: ArtifactFormatVersion,
        write_version: ArtifactFormatVersion,
        maximum_readable: ArtifactFormatVersion,
        read_posture: LayoutReadCompatibilityPosture,
        write_posture: LayoutWriteCompatibilityPosture,
    ) -> Result<Self, ArtifactCompatibilityDenial> {
        Ok(Self {
            artifact_window: ArtifactCompatibilityWindow::new(
                minimum_readable,
                write_version,
                maximum_readable,
            )?,
            read_posture,
            write_posture,
        })
    }

    pub const fn artifact_window(self) -> ArtifactCompatibilityWindow {
        self.artifact_window
    }

    pub const fn read_posture(self) -> LayoutReadCompatibilityPosture {
        self.read_posture
    }

    pub const fn write_posture(self) -> LayoutWriteCompatibilityPosture {
        self.write_posture
    }

    pub fn supports_read(self, version: LayoutVersion) -> bool {
        self.artifact_window
            .admit_backward_read(version.format_version())
            .is_ok()
            || matches!(self.read_posture, LayoutReadCompatibilityPosture::DualRead)
                && self
                    .artifact_window
                    .admit_forward_read(version.format_version())
                    .is_ok()
    }
}
