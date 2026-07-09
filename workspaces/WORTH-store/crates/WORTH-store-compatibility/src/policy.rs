use crate::{ArtifactCompatibilityWindow, ArtifactFormatVersion};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompatibilityManifestIndex {
    format_version: ArtifactFormatVersion,
}

impl CompatibilityManifestIndex {
    pub const fn new(format_version: ArtifactFormatVersion) -> Self {
        Self { format_version }
    }

    pub const fn format_version(self) -> ArtifactFormatVersion {
        self.format_version
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompatibilityRegistrySnapshot {
    manifest_index: CompatibilityManifestIndex,
}

impl CompatibilityRegistrySnapshot {
    pub const fn new(manifest_index: CompatibilityManifestIndex) -> Self {
        Self { manifest_index }
    }

    pub const fn manifest_index(self) -> CompatibilityManifestIndex {
        self.manifest_index
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollingUpgradePolicy {
    ReadOldWriteNew,
    DualRead,
    DualWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RollingUpgradeAdmissionPlan {
    window: ArtifactCompatibilityWindow,
    policy: RollingUpgradePolicy,
}

impl RollingUpgradeAdmissionPlan {
    pub const fn new(window: ArtifactCompatibilityWindow, policy: RollingUpgradePolicy) -> Self {
        Self { window, policy }
    }

    pub const fn window(self) -> ArtifactCompatibilityWindow {
        self.window
    }

    pub const fn policy(self) -> RollingUpgradePolicy {
        self.policy
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RollingWindowCompatibilityReceipt {
    plan: RollingUpgradeAdmissionPlan,
}

impl RollingWindowCompatibilityReceipt {
    pub const fn new(plan: RollingUpgradeAdmissionPlan) -> Self {
        Self { plan }
    }

    pub const fn plan(self) -> RollingUpgradeAdmissionPlan {
        self.plan
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestoreCompatibilityPlan {
    window: ArtifactCompatibilityWindow,
    target_version: ArtifactFormatVersion,
}

impl RestoreCompatibilityPlan {
    pub const fn new(
        window: ArtifactCompatibilityWindow,
        target_version: ArtifactFormatVersion,
    ) -> Self {
        Self {
            window,
            target_version,
        }
    }

    pub const fn window(self) -> ArtifactCompatibilityWindow {
        self.window
    }

    pub const fn target_version(self) -> ArtifactFormatVersion {
        self.target_version
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestoreCompatibilityReceipt {
    plan: RestoreCompatibilityPlan,
}

impl RestoreCompatibilityReceipt {
    pub const fn new(plan: RestoreCompatibilityPlan) -> Self {
        Self { plan }
    }

    pub const fn plan(self) -> RestoreCompatibilityPlan {
        self.plan
    }
}
