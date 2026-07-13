use crate::strategy::StrategyPublicationInvariant;

use super::maintenance_mode::IndexMaintenanceMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexPublicationProtocol {
    StableRootSwap,
    StableManifestInstall,
    DeferredCatchUp,
    CompactionCutover,
    MigrationCutover,
    VerifierObservationOnly,
}

impl IndexPublicationProtocol {
    pub const fn stable_root_swap() -> Self {
        Self::StableRootSwap
    }

    pub const fn stable_manifest_install() -> Self {
        Self::StableManifestInstall
    }

    pub const fn deferred_catch_up() -> Self {
        Self::DeferredCatchUp
    }

    pub const fn compaction_cutover() -> Self {
        Self::CompactionCutover
    }

    pub const fn migration_cutover() -> Self {
        Self::MigrationCutover
    }

    pub const fn verifier_observation_only() -> Self {
        Self::VerifierObservationOnly
    }

    pub const fn is_replay_stable(self) -> bool {
        !matches!(self, Self::DeferredCatchUp | Self::VerifierObservationOnly)
    }

    pub const fn supports_mode(self, mode: IndexMaintenanceMode) -> bool {
        match self {
            Self::StableRootSwap | Self::StableManifestInstall => {
                matches!(mode, IndexMaintenanceMode::SynchronousExact)
            }
            Self::DeferredCatchUp => matches!(
                mode,
                IndexMaintenanceMode::AsynchronousLagged
                    | IndexMaintenanceMode::LazyMaterializedOnDemand
                    | IndexMaintenanceMode::AdvisoryOnly
            ),
            Self::CompactionCutover => matches!(
                mode,
                IndexMaintenanceMode::AsynchronousLagged
                    | IndexMaintenanceMode::RebuildOnly
                    | IndexMaintenanceMode::MigrationOnly
            ),
            Self::MigrationCutover => matches!(mode, IndexMaintenanceMode::MigrationOnly),
            Self::VerifierObservationOnly => matches!(mode, IndexMaintenanceMode::VerifierOnly),
        }
    }

    pub const fn matches_invariant(self, invariant: StrategyPublicationInvariant) -> bool {
        match (self, invariant) {
            (
                Self::StableRootSwap | Self::CompactionCutover | Self::MigrationCutover,
                StrategyPublicationInvariant::RootPublication,
            ) => true,
            (
                Self::StableManifestInstall
                | Self::DeferredCatchUp
                | Self::CompactionCutover
                | Self::MigrationCutover,
                StrategyPublicationInvariant::ManifestPublication,
            ) => true,
            (Self::VerifierObservationOnly, _) => true,
            _ => false,
        }
    }
}
