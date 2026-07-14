use crate::{ArtifactCompatibilityWindow, ArtifactFormatVersion};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackwardReadCompatibilityWitness {
    window: ArtifactCompatibilityWindow,
    admitted_version: ArtifactFormatVersion,
}

impl BackwardReadCompatibilityWitness {
    pub(crate) const fn new(
        window: ArtifactCompatibilityWindow,
        admitted_version: ArtifactFormatVersion,
    ) -> Self {
        Self {
            window,
            admitted_version,
        }
    }

    pub const fn window(self) -> ArtifactCompatibilityWindow {
        self.window
    }

    pub const fn admitted_version(self) -> ArtifactFormatVersion {
        self.admitted_version
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForwardReadCompatibilityWitness {
    window: ArtifactCompatibilityWindow,
    admitted_version: ArtifactFormatVersion,
}

impl ForwardReadCompatibilityWitness {
    pub(crate) const fn new(
        window: ArtifactCompatibilityWindow,
        admitted_version: ArtifactFormatVersion,
    ) -> Self {
        Self {
            window,
            admitted_version,
        }
    }

    pub const fn window(self) -> ArtifactCompatibilityWindow {
        self.window
    }

    pub const fn admitted_version(self) -> ArtifactFormatVersion {
        self.admitted_version
    }
}
