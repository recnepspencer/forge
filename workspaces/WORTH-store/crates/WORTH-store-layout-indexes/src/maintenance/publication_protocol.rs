use crate::strategy::S8StrategyPublicationInvariant;

use super::maintenance_mode::S8IndexMaintenanceMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8IndexPublicationProtocol {
    StableRootSwap,
    StableManifestInstall,
    DeferredCatchUp,
    CompactionCutover,
    MigrationCutover,
    VerifierObservationOnly,
}

impl S8IndexPublicationProtocol {
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

    pub const fn supports_mode(self, mode: S8IndexMaintenanceMode) -> bool {
        match self {
            Self::StableRootSwap | Self::StableManifestInstall => {
                matches!(mode, S8IndexMaintenanceMode::SynchronousExact)
            }
            Self::DeferredCatchUp => matches!(
                mode,
                S8IndexMaintenanceMode::AsynchronousLagged
                    | S8IndexMaintenanceMode::LazyMaterializedOnDemand
                    | S8IndexMaintenanceMode::AdvisoryOnly
            ),
            Self::CompactionCutover => matches!(
                mode,
                S8IndexMaintenanceMode::AsynchronousLagged
                    | S8IndexMaintenanceMode::RebuildOnly
                    | S8IndexMaintenanceMode::MigrationOnly
            ),
            Self::MigrationCutover => matches!(mode, S8IndexMaintenanceMode::MigrationOnly),
            Self::VerifierObservationOnly => matches!(mode, S8IndexMaintenanceMode::VerifierOnly),
        }
    }

    pub const fn matches_invariant(self, invariant: S8StrategyPublicationInvariant) -> bool {
        match (self, invariant) {
            (
                Self::StableRootSwap | Self::CompactionCutover | Self::MigrationCutover,
                S8StrategyPublicationInvariant::RootPublication,
            ) => true,
            (
                Self::StableManifestInstall
                | Self::DeferredCatchUp
                | Self::CompactionCutover
                | Self::MigrationCutover,
                S8StrategyPublicationInvariant::ManifestPublication,
            ) => true,
            (Self::VerifierObservationOnly, _) => true,
            _ => false,
        }
    }
}
