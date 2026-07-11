use crate::catalog::ArtifactFamilyAccessLane;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8IndexMaintenanceMode {
    SynchronousExact,
    AsynchronousLagged,
    RebuildOnly,
    LazyMaterializedOnDemand,
    AdvisoryOnly,
    VerifierOnly,
    MigrationOnly,
}

impl S8IndexMaintenanceMode {
    pub const fn synchronous_exact() -> Self {
        Self::SynchronousExact
    }

    pub const fn asynchronous_lagged() -> Self {
        Self::AsynchronousLagged
    }

    pub const fn rebuild_only() -> Self {
        Self::RebuildOnly
    }

    pub const fn lazy_materialized_on_demand() -> Self {
        Self::LazyMaterializedOnDemand
    }

    pub const fn advisory_only() -> Self {
        Self::AdvisoryOnly
    }

    pub const fn verifier_only() -> Self {
        Self::VerifierOnly
    }

    pub const fn migration_only() -> Self {
        Self::MigrationOnly
    }

    pub const fn permits_exact_answers(self) -> bool {
        matches!(self, Self::SynchronousExact)
    }

    pub const fn requires_lag_witness(self) -> bool {
        matches!(
            self,
            Self::AsynchronousLagged
                | Self::RebuildOnly
                | Self::LazyMaterializedOnDemand
                | Self::AdvisoryOnly
                | Self::VerifierOnly
                | Self::MigrationOnly
        )
    }

    pub const fn is_verifier_only(self) -> bool {
        matches!(self, Self::VerifierOnly)
    }

    pub const fn supports_lane(self, lane: ArtifactFamilyAccessLane) -> bool {
        match self {
            Self::SynchronousExact => matches!(lane, ArtifactFamilyAccessLane::HotPath),
            Self::AsynchronousLagged
            | Self::RebuildOnly
            | Self::LazyMaterializedOnDemand
            | Self::MigrationOnly => matches!(
                lane,
                ArtifactFamilyAccessLane::HotPath | ArtifactFamilyAccessLane::MaintenancePath
            ),
            Self::AdvisoryOnly => matches!(
                lane,
                ArtifactFamilyAccessLane::MaintenancePath | ArtifactFamilyAccessLane::TerminalPath
            ),
            Self::VerifierOnly => matches!(
                lane,
                ArtifactFamilyAccessLane::VerifierPath | ArtifactFamilyAccessLane::TerminalPath
            ),
        }
    }
}
