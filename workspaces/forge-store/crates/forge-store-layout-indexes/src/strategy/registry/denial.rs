use crate::catalog::{
    ArtifactFamilyAccessLane, ArtifactScopePartitionWitness, DurableArtifactMigrationPosture,
    PhysicalArtifactFamily,
};
use crate::keyspace::PhysicalKeyDomainWitness;
use crate::maintenance::{IndexMaintenanceMode, PhysicalMutationShape};
use crate::materialization::MaterializationDenial;
use crate::strategy::{LayoutStrategyFamily, StrategyDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutAdmissionDenial {
    StrategyVocabularyDenied(StrategyDenial),
    RequestedLaneDoesNotMatchFamilyLane {
        family: LayoutStrategyFamily,
        requested_lane: ArtifactFamilyAccessLane,
        declared_lane: ArtifactFamilyAccessLane,
    },
    RequestedScopeDoesNotMatchKeyDomain {
        requested_scope: ArtifactScopePartitionWitness,
        key_domain_scope: ArtifactScopePartitionWitness,
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
    MigrationPostureIncompatibleWithStrategy {
        family: LayoutStrategyFamily,
        required_migration_posture: DurableArtifactMigrationPosture,
        admitted_migration_posture: DurableArtifactMigrationPosture,
    },
    StrategyDoesNotSupportRequestedCapability {
        family: LayoutStrategyFamily,
        capability: super::LayoutRequestedCapability,
    },
    ComparatorLawRequired {
        family: LayoutStrategyFamily,
        capability: super::LayoutRequestedCapability,
    },
    PrefixLawRequired {
        family: LayoutStrategyFamily,
    },
    RangeBoundLawRequired {
        family: LayoutStrategyFamily,
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
    ExactMaterializationRequired,
    ExactCoverageDenied(MaterializationDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutAdmissionDenialCase {
    StrategyVocabularyDenied,
    RequestedLaneDoesNotMatchFamilyLane,
    RequestedScopeDoesNotMatchKeyDomain,
    MaintenanceModeIncompatibleWithRequestedLane,
    MutationShapeIncompatibleWithStrategy,
    MigrationPostureIncompatibleWithStrategy,
    StrategyDoesNotSupportRequestedCapability,
    ComparatorLawRequired,
    PrefixLawRequired,
    RangeBoundLawRequired,
    HashEqualityLawDoesNotMatchKeyDomain,
    CompositeOrderingLawDoesNotMatchKeyDomain,
    CoverageFamilyDoesNotMatchStrategy,
    ExactMaterializationRequired,
    ExactCoverageDenied,
}

impl LayoutAdmissionDenialCase {
    pub const ALL: [Self; 15] = [
        Self::StrategyVocabularyDenied,
        Self::RequestedLaneDoesNotMatchFamilyLane,
        Self::RequestedScopeDoesNotMatchKeyDomain,
        Self::MaintenanceModeIncompatibleWithRequestedLane,
        Self::MutationShapeIncompatibleWithStrategy,
        Self::MigrationPostureIncompatibleWithStrategy,
        Self::StrategyDoesNotSupportRequestedCapability,
        Self::ComparatorLawRequired,
        Self::PrefixLawRequired,
        Self::RangeBoundLawRequired,
        Self::HashEqualityLawDoesNotMatchKeyDomain,
        Self::CompositeOrderingLawDoesNotMatchKeyDomain,
        Self::CoverageFamilyDoesNotMatchStrategy,
        Self::ExactMaterializationRequired,
        Self::ExactCoverageDenied,
    ];
}

impl LayoutAdmissionDenial {
    pub const fn case(&self) -> LayoutAdmissionDenialCase {
        match self {
            Self::StrategyVocabularyDenied(_) => {
                LayoutAdmissionDenialCase::StrategyVocabularyDenied
            }
            Self::RequestedLaneDoesNotMatchFamilyLane { .. } => {
                LayoutAdmissionDenialCase::RequestedLaneDoesNotMatchFamilyLane
            }
            Self::RequestedScopeDoesNotMatchKeyDomain { .. } => {
                LayoutAdmissionDenialCase::RequestedScopeDoesNotMatchKeyDomain
            }
            Self::MaintenanceModeIncompatibleWithRequestedLane { .. } => {
                LayoutAdmissionDenialCase::MaintenanceModeIncompatibleWithRequestedLane
            }
            Self::MutationShapeIncompatibleWithStrategy { .. } => {
                LayoutAdmissionDenialCase::MutationShapeIncompatibleWithStrategy
            }
            Self::MigrationPostureIncompatibleWithStrategy { .. } => {
                LayoutAdmissionDenialCase::MigrationPostureIncompatibleWithStrategy
            }
            Self::StrategyDoesNotSupportRequestedCapability { .. } => {
                LayoutAdmissionDenialCase::StrategyDoesNotSupportRequestedCapability
            }
            Self::ComparatorLawRequired { .. } => LayoutAdmissionDenialCase::ComparatorLawRequired,
            Self::PrefixLawRequired { .. } => LayoutAdmissionDenialCase::PrefixLawRequired,
            Self::RangeBoundLawRequired { .. } => LayoutAdmissionDenialCase::RangeBoundLawRequired,
            Self::HashEqualityLawDoesNotMatchKeyDomain { .. } => {
                LayoutAdmissionDenialCase::HashEqualityLawDoesNotMatchKeyDomain
            }
            Self::CompositeOrderingLawDoesNotMatchKeyDomain { .. } => {
                LayoutAdmissionDenialCase::CompositeOrderingLawDoesNotMatchKeyDomain
            }
            Self::CoverageFamilyDoesNotMatchStrategy { .. } => {
                LayoutAdmissionDenialCase::CoverageFamilyDoesNotMatchStrategy
            }
            Self::ExactMaterializationRequired => {
                LayoutAdmissionDenialCase::ExactMaterializationRequired
            }
            Self::ExactCoverageDenied(_) => LayoutAdmissionDenialCase::ExactCoverageDenied,
        }
    }
}
