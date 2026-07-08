use crate::artifact_family::{ArtifactFamilyAccessLane, DurableArtifactMigrationPosture};
use crate::materialization::S8LayoutCoverageWitness;
use crate::strategy::{S8LayoutStrategyFamily, S8StrategyDenial};

use super::maintenance_mode::S8IndexMaintenanceMode;
use super::mutation_plan::S8PhysicalMutationShape;
use super::publication_protocol::S8IndexPublicationProtocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8MutationProofRequirement {
    WalBeforeData,
    StableReadIsolation,
    PageLsnConsistency,
    ChecksumRewrite,
    TornWriteProtection,
    CrashReplayPublication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8PublicationProofRequirement {
    RootPublicationValidation,
    RootEpochPublicationBinding,
    ManifestPublicationValidation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8IndexMaintenanceFailureOutcome {
    StrategyDenied {
        denial: S8StrategyDenial,
    },
    MaintenanceModeIncompatibleWithRequestedLane {
        family: S8LayoutStrategyFamily,
        maintenance_mode: S8IndexMaintenanceMode,
        requested_lane: ArtifactFamilyAccessLane,
    },
    MutationShapeIncompatibleWithStrategy {
        family: S8LayoutStrategyFamily,
        mutation_shape: S8PhysicalMutationShape,
    },
    PublicationProtocolIncompatibleWithStrategy {
        family: S8LayoutStrategyFamily,
        publication_protocol: S8IndexPublicationProtocol,
    },
    ReplayStablePublicationRequired {
        family: S8LayoutStrategyFamily,
        publication_protocol: S8IndexPublicationProtocol,
    },
    ExactPublicationAuthorityRequired {
        family: S8LayoutStrategyFamily,
        publication_protocol: S8IndexPublicationProtocol,
    },
    PublicationAuthorityDoesNotMatchExactCoverage {
        publication_protocol: S8IndexPublicationProtocol,
        coverage: S8LayoutCoverageWitness,
    },
    ExactCoverageRequired {
        family: S8LayoutStrategyFamily,
        maintenance_mode: S8IndexMaintenanceMode,
    },
    CoverageFamilyDoesNotMatchStrategy {
        coverage_family: crate::artifact_family::PhysicalArtifactFamily,
        strategy_family: crate::artifact_family::PhysicalArtifactFamily,
    },
    LagWitnessRequired {
        family: S8LayoutStrategyFamily,
        maintenance_mode: S8IndexMaintenanceMode,
    },
    LagWitnessUnexpected {
        family: S8LayoutStrategyFamily,
        maintenance_mode: S8IndexMaintenanceMode,
    },
    MigrationPostureIncompatibleWithStrategy {
        family: S8LayoutStrategyFamily,
        required_migration_posture: DurableArtifactMigrationPosture,
        admitted_migration_posture: DurableArtifactMigrationPosture,
    },
    LowerMutationCapabilityRequired {
        family: S8LayoutStrategyFamily,
        mutation_shape: S8PhysicalMutationShape,
        missing: S8MutationProofRequirement,
    },
    LowerPublicationCapabilityRequired {
        family: S8LayoutStrategyFamily,
        publication_protocol: S8IndexPublicationProtocol,
        missing: S8PublicationProofRequirement,
    },
    LagCoverageDoesNotMatchRequest {
        expected: S8LayoutCoverageWitness,
        actual: S8LayoutCoverageWitness,
    },
}
