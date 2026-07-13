use crate::catalog::{ArtifactFamilyAccessLane, DurableArtifactMigrationPosture};
use crate::materialization::LayoutCoverageWitness;
use crate::strategy::{LayoutStrategyFamily, StrategyDenial};

use super::maintenance_mode::IndexMaintenanceMode;
use super::mutation_plan::PhysicalMutationShape;
use super::publication_protocol::IndexPublicationProtocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationProofRequirement {
    WalBeforeData,
    StableReadIsolation,
    PageLsnConsistency,
    ChecksumRewrite,
    TornWriteProtection,
    CrashReplayPublication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationProofRequirement {
    RootPublicationValidation,
    RootEpochPublicationBinding,
    ManifestPublicationValidation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexMaintenanceFailureOutcome {
    StrategyDenied {
        denial: StrategyDenial,
    },
    MaintenanceModeIncompatibleWithRequestedLane {
        family: LayoutStrategyFamily,
        maintenance_mode: IndexMaintenanceMode,
        requested_lane: ArtifactFamilyAccessLane,
    },
    MutationShapeIncompatibleWithStrategy {
        family: LayoutStrategyFamily,
        mutation_shape: PhysicalMutationShape,
    },
    PublicationProtocolIncompatibleWithStrategy {
        family: LayoutStrategyFamily,
        publication_protocol: IndexPublicationProtocol,
    },
    ExactPublicationAuthorityRequired {
        family: LayoutStrategyFamily,
        publication_protocol: IndexPublicationProtocol,
    },
    PublicationAuthorityDoesNotMatchExactCoverage {
        publication_protocol: IndexPublicationProtocol,
        coverage: LayoutCoverageWitness,
    },
    ExactCoverageRequired {
        family: LayoutStrategyFamily,
        maintenance_mode: IndexMaintenanceMode,
    },
    CoverageFamilyDoesNotMatchStrategy {
        coverage_family: crate::catalog::PhysicalArtifactFamily,
        strategy_family: crate::catalog::PhysicalArtifactFamily,
    },
    LagWitnessRequired {
        family: LayoutStrategyFamily,
        maintenance_mode: IndexMaintenanceMode,
    },
    LagWitnessUnexpected {
        family: LayoutStrategyFamily,
        maintenance_mode: IndexMaintenanceMode,
    },
    MigrationPostureIncompatibleWithStrategy {
        family: LayoutStrategyFamily,
        required_migration_posture: DurableArtifactMigrationPosture,
        admitted_migration_posture: DurableArtifactMigrationPosture,
    },
    LowerMutationCapabilityRequired {
        family: LayoutStrategyFamily,
        mutation_shape: PhysicalMutationShape,
        missing: MutationProofRequirement,
    },
    LowerPublicationCapabilityRequired {
        family: LayoutStrategyFamily,
        publication_protocol: IndexPublicationProtocol,
        missing: PublicationProofRequirement,
    },
    LagCoverageDoesNotMatchRequest {
        expected: LayoutCoverageWitness,
        actual: LayoutCoverageWitness,
    },
}

impl IndexMaintenanceFailureOutcome {
    pub(crate) const fn case_name(&self) -> &'static str {
        match self {
            Self::StrategyDenied { .. } => "maintenance.admission.denied.strategy",
            Self::MaintenanceModeIncompatibleWithRequestedLane { .. } => {
                "maintenance.admission.denied.lane"
            }
            Self::MutationShapeIncompatibleWithStrategy { .. } => {
                "maintenance.admission.denied.mutation_shape"
            }
            Self::PublicationProtocolIncompatibleWithStrategy { .. } => {
                "maintenance.admission.denied.publication_protocol"
            }
            Self::ExactPublicationAuthorityRequired { .. } => {
                "maintenance.admission.denied.exact_publication_authority"
            }
            Self::PublicationAuthorityDoesNotMatchExactCoverage { .. } => {
                "maintenance.admission.denied.publication_coverage_binding"
            }
            Self::ExactCoverageRequired { .. } => "maintenance.admission.denied.exact_coverage",
            Self::CoverageFamilyDoesNotMatchStrategy { .. } => {
                "maintenance.admission.denied.coverage_family"
            }
            Self::LagWitnessRequired { .. } => "maintenance.admission.denied.lag_witness_missing",
            Self::LagWitnessUnexpected { .. } => {
                "maintenance.admission.denied.lag_witness_unexpected"
            }
            Self::MigrationPostureIncompatibleWithStrategy { .. } => {
                "maintenance.admission.denied.migration_posture"
            }
            Self::LowerMutationCapabilityRequired { .. } => {
                "maintenance.admission.denied.lower_mutation_capability"
            }
            Self::LowerPublicationCapabilityRequired { .. } => {
                "maintenance.admission.denied.lower_publication_capability"
            }
            Self::LagCoverageDoesNotMatchRequest { .. } => {
                "maintenance.admission.denied.lag_coverage_binding"
            }
        }
    }
}
