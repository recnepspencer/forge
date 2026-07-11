use std::collections::BTreeMap;

use forge_store_readiness::{
    PhysicalIsolationCorrectnessNonClaimEvidence, PhysicalIsolationHarnessMaturityDependency,
    PhysicalIsolationHarnessReadinessDenial,
};

use crate::PhysicalSimulationProfile;

use super::{
    CoverageGapDenial, CoverageRowDimension, CoverageSurfaceKind, GeneratedCoverageMatrix,
    HarnessCoverageStage, HarnessSubsystem, PhysicalIsolationReadinessDependencySet,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HarnessMaturityLevel {
    Exists,
    SmokeWorks,
    CiCertifiable,
    ReleaseCertifiable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessSubsystemMaturity {
    subsystem: HarnessSubsystem,
    level: HarnessMaturityLevel,
    satisfied_surface_count: usize,
    source_identity: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessMaturityEvidence {
    sequence: HarnessCoverageStage,
    profile: Option<PhysicalSimulationProfile>,
    subsystems: Vec<HarnessSubsystemMaturity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationHarnessMaturityDependencyEvidence {
    dependency: PhysicalIsolationHarnessMaturityDependency,
    coverage_row_digest: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationHarnessReadiness {
    dependencies: Vec<PhysicalIsolationHarnessMaturityDependencyEvidence>,
    non_claim: PhysicalIsolationCorrectnessNonClaimEvidence,
}

impl HarnessMaturityEvidence {
    pub(crate) fn from_generated_matrix(matrix: &GeneratedCoverageMatrix) -> Self {
        let mut counts = BTreeMap::<HarnessSubsystem, (usize, [u8; 32])>::new();
        for row in matrix.rows() {
            let entry = counts
                .entry(row.subsystem())
                .or_insert((0usize, *row.source_identity()));
            entry.0 += 1;
        }
        let mut subsystems = counts
            .into_iter()
            .map(|(subsystem, (satisfied_surface_count, source_identity))| {
                HarnessSubsystemMaturity {
                    subsystem,
                    level: level_for_registered_surfaces(satisfied_surface_count),
                    satisfied_surface_count,
                    source_identity,
                }
            })
            .collect::<Vec<_>>();
        subsystems.sort_by_key(|entry| entry.subsystem);
        Self {
            sequence: matrix.sequence(),
            profile: profile_from_generated_matrix(matrix),
            subsystems,
        }
    }

    pub const fn sequence(&self) -> HarnessCoverageStage {
        self.sequence
    }

    pub const fn profile(&self) -> Option<PhysicalSimulationProfile> {
        self.profile
    }

    pub fn subsystems(&self) -> &[HarnessSubsystemMaturity] {
        &self.subsystems
    }

    pub fn level_for(&self, subsystem: HarnessSubsystem) -> Option<HarnessMaturityLevel> {
        self.subsystems
            .iter()
            .find(|entry| entry.subsystem == subsystem)
            .map(|entry| entry.level)
    }

    pub fn require_subsystem_level(
        self,
        dependencies: PhysicalIsolationReadinessDependencySet,
        required: HarnessMaturityLevel,
    ) -> Result<Self, CoverageGapDenial> {
        for subsystem in dependencies.required() {
            let Some(actual) = self.level_for(*subsystem) else {
                return Err(CoverageGapDenial::MissingRegistrationEvidence {
                    surface: surface_for_dependency(*subsystem),
                });
            };
            if actual < required {
                return Err(CoverageGapDenial::SmokeOnlyMaturityDenied {
                    subsystem: *subsystem,
                    actual,
                });
            }
        }
        Ok(self)
    }

    pub fn physical_isolation_readiness_dependency_evidence(
        &self,
    ) -> Result<Vec<PhysicalIsolationHarnessMaturityDependencyEvidence>, CoverageGapDenial> {
        let dependencies = PhysicalIsolationReadinessDependencySet::required_for_ci();
        dependencies
            .required()
            .iter()
            .map(|subsystem| {
                let evidence = self
                    .subsystems
                    .iter()
                    .find(|entry| entry.subsystem == *subsystem)
                    .filter(|entry| entry.level >= HarnessMaturityLevel::CiCertifiable)
                    .ok_or(CoverageGapDenial::SmokeOnlyMaturityDenied {
                        subsystem: *subsystem,
                        actual: self
                            .level_for(*subsystem)
                            .unwrap_or(HarnessMaturityLevel::Exists),
                    })?;
                Ok(PhysicalIsolationHarnessMaturityDependencyEvidence {
                    dependency: dependency_for_subsystem(*subsystem),
                    coverage_row_digest: *evidence.source_identity(),
                })
            })
            .collect()
    }

    pub fn admit_physical_isolation_simulation_harness_readiness(
        &self,
        non_claim: PhysicalIsolationCorrectnessNonClaimEvidence,
    ) -> Result<PhysicalIsolationHarnessReadiness, PhysicalIsolationHarnessReadinessDenial> {
        if self.sequence != HarnessCoverageStage::SimulationAdmission {
            return Err(PhysicalIsolationHarnessReadinessDenial::WrongSequenceMaturityEvidence);
        }
        if self.profile != Some(PhysicalSimulationProfile::CiCertification) {
            return Err(
                PhysicalIsolationHarnessReadinessDenial::UnsupportedProfileMaturityEvidence,
            );
        }
        let dependencies = self
            .physical_isolation_readiness_dependency_evidence()
            .map_err(readiness_denial_from_coverage_gap)?;
        PhysicalIsolationHarnessReadiness::from_generated_maturity(dependencies, non_claim)
    }
}

impl HarnessSubsystemMaturity {
    pub const fn subsystem(&self) -> HarnessSubsystem {
        self.subsystem
    }

    pub const fn level(&self) -> HarnessMaturityLevel {
        self.level
    }

    pub const fn satisfied_surface_count(&self) -> usize {
        self.satisfied_surface_count
    }

    pub const fn source_identity(&self) -> &[u8; 32] {
        &self.source_identity
    }
}

impl PhysicalIsolationHarnessMaturityDependencyEvidence {
    pub const fn dependency(&self) -> PhysicalIsolationHarnessMaturityDependency {
        self.dependency
    }

    pub const fn coverage_row_digest(&self) -> &[u8; 32] {
        &self.coverage_row_digest
    }
}

impl PhysicalIsolationHarnessReadiness {
    fn from_generated_maturity(
        dependencies: Vec<PhysicalIsolationHarnessMaturityDependencyEvidence>,
        non_claim: PhysicalIsolationCorrectnessNonClaimEvidence,
    ) -> Result<Self, PhysicalIsolationHarnessReadinessDenial> {
        for required in PhysicalIsolationHarnessMaturityDependency::required_for_ci() {
            if !dependencies
                .iter()
                .any(|evidence| evidence.dependency() == required)
            {
                return Err(PhysicalIsolationHarnessReadinessDenial::MissingDependency(
                    required,
                ));
            }
        }
        Ok(Self {
            dependencies,
            non_claim,
        })
    }

    pub fn dependencies(&self) -> &[PhysicalIsolationHarnessMaturityDependencyEvidence] {
        &self.dependencies
    }

    pub const fn non_claim(&self) -> PhysicalIsolationCorrectnessNonClaimEvidence {
        self.non_claim
    }

    pub const fn does_not_claim_physical_isolation_correctness(&self) -> bool {
        matches!(
            self.non_claim,
            PhysicalIsolationCorrectnessNonClaimEvidence::ShapeProbeOnly
        )
    }
}

const fn level_for_registered_surfaces(satisfied_surface_count: usize) -> HarnessMaturityLevel {
    if satisfied_surface_count == 0 {
        HarnessMaturityLevel::Exists
    } else {
        HarnessMaturityLevel::CiCertifiable
    }
}

const fn surface_for_dependency(subsystem: HarnessSubsystem) -> CoverageSurfaceKind {
    match subsystem {
        HarnessSubsystem::ScenarioDefinitions => CoverageSurfaceKind::Scenario,
        HarnessSubsystem::DeterministicScheduler => CoverageSurfaceKind::YieldpointSchedule,
        HarnessSubsystem::ActorModel => CoverageSurfaceKind::Actor,
        HarnessSubsystem::ProductionDriverContracts => CoverageSurfaceKind::Driver,
        HarnessSubsystem::CertificationOracleFamilies => CoverageSurfaceKind::Oracle,
        HarnessSubsystem::CounterStrengthContracts => CoverageSurfaceKind::Counter,
        HarnessSubsystem::ReplayableTranscripts => CoverageSurfaceKind::Transcript,
        HarnessSubsystem::MutationValidation => CoverageSurfaceKind::MutationResult,
    }
}

const fn dependency_for_subsystem(
    subsystem: HarnessSubsystem,
) -> PhysicalIsolationHarnessMaturityDependency {
    match subsystem {
        HarnessSubsystem::ScenarioDefinitions => {
            PhysicalIsolationHarnessMaturityDependency::ScenarioDefinitions
        }
        HarnessSubsystem::DeterministicScheduler => {
            PhysicalIsolationHarnessMaturityDependency::DeterministicScheduler
        }
        HarnessSubsystem::ActorModel => PhysicalIsolationHarnessMaturityDependency::ActorModel,
        HarnessSubsystem::ProductionDriverContracts => {
            PhysicalIsolationHarnessMaturityDependency::ProductionDriverContracts
        }
        HarnessSubsystem::CertificationOracleFamilies => {
            PhysicalIsolationHarnessMaturityDependency::CertificationOracleFamilies
        }
        HarnessSubsystem::CounterStrengthContracts => {
            PhysicalIsolationHarnessMaturityDependency::CounterStrengthContracts
        }
        HarnessSubsystem::ReplayableTranscripts => {
            PhysicalIsolationHarnessMaturityDependency::ReplayableTranscripts
        }
        HarnessSubsystem::MutationValidation => {
            PhysicalIsolationHarnessMaturityDependency::MutationValidation
        }
    }
}

fn readiness_denial_from_coverage_gap(
    denial: CoverageGapDenial,
) -> PhysicalIsolationHarnessReadinessDenial {
    match denial {
        CoverageGapDenial::WrongSequenceMaturityEvidence => {
            PhysicalIsolationHarnessReadinessDenial::WrongSequenceMaturityEvidence
        }
        CoverageGapDenial::UnsupportedProfileMaturityEvidence => {
            PhysicalIsolationHarnessReadinessDenial::UnsupportedProfileMaturityEvidence
        }
        CoverageGapDenial::SmokeOnlyMaturityDenied { subsystem, .. } => {
            PhysicalIsolationHarnessReadinessDenial::MissingDependency(dependency_for_subsystem(
                subsystem,
            ))
        }
        CoverageGapDenial::MissingRegistrationEvidence { surface } => {
            PhysicalIsolationHarnessReadinessDenial::MissingDependency(dependency_for_surface(
                surface,
            ))
        }
        _ => PhysicalIsolationHarnessReadinessDenial::MissingDependency(
            PhysicalIsolationHarnessMaturityDependency::ScenarioDefinitions,
        ),
    }
}

fn profile_from_generated_matrix(
    matrix: &GeneratedCoverageMatrix,
) -> Option<PhysicalSimulationProfile> {
    matrix.rows().iter().find_map(|row| {
        row.dimensions()
            .iter()
            .find_map(|dimension| match dimension {
                CoverageRowDimension::ResourceEnvelopeProfile(profile) => Some(*profile),
                _ => None,
            })
    })
}

const fn dependency_for_surface(
    surface: CoverageSurfaceKind,
) -> PhysicalIsolationHarnessMaturityDependency {
    match surface {
        CoverageSurfaceKind::Scenario | CoverageSurfaceKind::Plan => {
            PhysicalIsolationHarnessMaturityDependency::ScenarioDefinitions
        }
        CoverageSurfaceKind::YieldpointSchedule => {
            PhysicalIsolationHarnessMaturityDependency::DeterministicScheduler
        }
        CoverageSurfaceKind::Actor => PhysicalIsolationHarnessMaturityDependency::ActorModel,
        CoverageSurfaceKind::Driver => {
            PhysicalIsolationHarnessMaturityDependency::ProductionDriverContracts
        }
        CoverageSurfaceKind::Oracle => {
            PhysicalIsolationHarnessMaturityDependency::CertificationOracleFamilies
        }
        CoverageSurfaceKind::Counter => {
            PhysicalIsolationHarnessMaturityDependency::CounterStrengthContracts
        }
        CoverageSurfaceKind::Transcript => {
            PhysicalIsolationHarnessMaturityDependency::ReplayableTranscripts
        }
        CoverageSurfaceKind::MutationResult => {
            PhysicalIsolationHarnessMaturityDependency::MutationValidation
        }
    }
}
