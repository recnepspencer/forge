use crate::catalog::{
    ArtifactFamilyAccessLane, ArtifactScopePartitionWitness, DurableArtifactMigrationPosture,
    PhysicalArtifactFamily,
};
use crate::keyspace::PhysicalKeyDomainWitness;
use crate::maintenance::{S8IndexMaintenanceMode, S8PhysicalMutationShape};
use crate::materialization::S8MaterializationDenial;
use crate::strategy::{S8LayoutStrategyFamily, S8StrategyDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutAdmissionDenial {
    StrategyVocabularyDenied(S8StrategyDenial),
    RequestedLaneDoesNotMatchFamilyLane {
        family: S8LayoutStrategyFamily,
        requested_lane: ArtifactFamilyAccessLane,
        declared_lane: ArtifactFamilyAccessLane,
    },
    RequestedScopeDoesNotMatchKeyDomain {
        requested_scope: ArtifactScopePartitionWitness,
        key_domain_scope: ArtifactScopePartitionWitness,
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
    MigrationPostureIncompatibleWithStrategy {
        family: S8LayoutStrategyFamily,
        required_migration_posture: DurableArtifactMigrationPosture,
        admitted_migration_posture: DurableArtifactMigrationPosture,
    },
    StrategyDoesNotSupportRequestedCapability {
        family: S8LayoutStrategyFamily,
        capability: super::S8LayoutRequestedCapability,
    },
    ComparatorLawRequired {
        family: S8LayoutStrategyFamily,
        capability: super::S8LayoutRequestedCapability,
    },
    PrefixLawRequired {
        family: S8LayoutStrategyFamily,
    },
    RangeBoundLawRequired {
        family: S8LayoutStrategyFamily,
    },
    HashEqualityLawDoesNotMatchKeyDomain {
        requested_domain: PhysicalKeyDomainWitness,
        strategy_domain: PhysicalKeyDomainWitness,
    },
    CompositeOrderingLawDoesNotMatchKeyDomain {
        requested_domain: PhysicalKeyDomainWitness,
        strategy_domain: PhysicalKeyDomainWitness,
    },
    CoverageFamilyDoesNotMatchStrategy {
        coverage_family: PhysicalArtifactFamily,
        strategy_family: PhysicalArtifactFamily,
    },
    LiveExactMaintenanceWitnessDoesNotMatchStrategy {
        witness_family: PhysicalArtifactFamily,
        strategy_family: PhysicalArtifactFamily,
    },
    LiveExactMaintenanceCoverageDoesNotMatchRequest {
        witness_coverage: crate::materialization::S8LayoutCoverageWitness,
        requested_coverage: crate::materialization::S8LayoutCoverageWitness,
    },
    ExactCoverageDenied(S8MaterializationDenial),
    ExactAbsenceProofDenied(S8MaterializationDenial),
}
