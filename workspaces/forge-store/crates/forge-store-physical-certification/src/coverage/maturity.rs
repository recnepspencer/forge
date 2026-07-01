use std::collections::BTreeMap;

use forge_store_readiness::{
    S5CorrectnessNonClaimEvidence, S5HarnessMaturityDependency, S5SimulationHarnessReadinessDenial,
};

use crate::PhysicalSimulationProfile;

use super::{
    CoverageGapDenial, CoverageRowDimension, CoverageSurfaceKind, GeneratedCoverageMatrix,
    HarnessSubsystem, Roadmap2HarnessSequence, S5ReadinessDependencySet,
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
    sequence: Roadmap2HarnessSequence,
    profile: Option<PhysicalSimulationProfile>,
    subsystems: Vec<HarnessSubsystemMaturity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5HarnessMaturityDependencyEvidence {
    dependency: S5HarnessMaturityDependency,
    coverage_row_digest: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5SimulationHarnessReadiness {
    dependencies: Vec<S5HarnessMaturityDependencyEvidence>,
    non_claim: S5CorrectnessNonClaimEvidence,
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

    pub const fn sequence(&self) -> Roadmap2HarnessSequence {
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
        dependencies: S5ReadinessDependencySet,
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

    pub fn s5_readiness_dependency_evidence(
        &self,
    ) -> Result<Vec<S5HarnessMaturityDependencyEvidence>, CoverageGapDenial> {
        let dependencies = S5ReadinessDependencySet::required_for_ci();
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
                Ok(S5HarnessMaturityDependencyEvidence {
                    dependency: dependency_for_subsystem(*subsystem),
                    coverage_row_digest: *evidence.source_identity(),
                })
            })
            .collect()
    }

    pub fn admit_s5_simulation_harness_readiness(
        &self,
        non_claim: S5CorrectnessNonClaimEvidence,
    ) -> Result<S5SimulationHarnessReadiness, S5SimulationHarnessReadinessDenial> {
        if self.sequence != Roadmap2HarnessSequence::S45 {
            return Err(S5SimulationHarnessReadinessDenial::WrongSequenceMaturityEvidence);
        }
        if self.profile != Some(PhysicalSimulationProfile::CiCertification) {
            return Err(S5SimulationHarnessReadinessDenial::UnsupportedProfileMaturityEvidence);
        }
        let dependencies = self
            .s5_readiness_dependency_evidence()
            .map_err(readiness_denial_from_coverage_gap)?;
        S5SimulationHarnessReadiness::from_generated_maturity(dependencies, non_claim)
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

impl S5HarnessMaturityDependencyEvidence {
    pub const fn dependency(&self) -> S5HarnessMaturityDependency {
        self.dependency
    }

    pub const fn coverage_row_digest(&self) -> &[u8; 32] {
        &self.coverage_row_digest
    }
}

impl S5SimulationHarnessReadiness {
    fn from_generated_maturity(
        dependencies: Vec<S5HarnessMaturityDependencyEvidence>,
        non_claim: S5CorrectnessNonClaimEvidence,
    ) -> Result<Self, S5SimulationHarnessReadinessDenial> {
        for required in S5HarnessMaturityDependency::required_for_ci() {
            if !dependencies
                .iter()
                .any(|evidence| evidence.dependency() == required)
            {
                return Err(S5SimulationHarnessReadinessDenial::MissingDependency(
                    required,
                ));
            }
        }
        Ok(Self {
            dependencies,
            non_claim,
        })
    }

    pub fn dependencies(&self) -> &[S5HarnessMaturityDependencyEvidence] {
        &self.dependencies
    }

    pub const fn non_claim(&self) -> S5CorrectnessNonClaimEvidence {
        self.non_claim
    }

    pub const fn does_not_claim_s5_correctness(&self) -> bool {
        matches!(
            self.non_claim,
            S5CorrectnessNonClaimEvidence::ShapeProbeOnly
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

const fn dependency_for_subsystem(subsystem: HarnessSubsystem) -> S5HarnessMaturityDependency {
    match subsystem {
        HarnessSubsystem::ScenarioDefinitions => S5HarnessMaturityDependency::ScenarioDefinitions,
        HarnessSubsystem::DeterministicScheduler => {
            S5HarnessMaturityDependency::DeterministicScheduler
        }
        HarnessSubsystem::ActorModel => S5HarnessMaturityDependency::ActorModel,
        HarnessSubsystem::ProductionDriverContracts => {
            S5HarnessMaturityDependency::ProductionDriverContracts
        }
        HarnessSubsystem::CertificationOracleFamilies => {
            S5HarnessMaturityDependency::CertificationOracleFamilies
        }
        HarnessSubsystem::CounterStrengthContracts => {
            S5HarnessMaturityDependency::CounterStrengthContracts
        }
        HarnessSubsystem::ReplayableTranscripts => {
            S5HarnessMaturityDependency::ReplayableTranscripts
        }
        HarnessSubsystem::MutationValidation => S5HarnessMaturityDependency::MutationValidation,
    }
}

fn readiness_denial_from_coverage_gap(
    denial: CoverageGapDenial,
) -> S5SimulationHarnessReadinessDenial {
    match denial {
        CoverageGapDenial::WrongSequenceMaturityEvidence => {
            S5SimulationHarnessReadinessDenial::WrongSequenceMaturityEvidence
        }
        CoverageGapDenial::UnsupportedProfileMaturityEvidence => {
            S5SimulationHarnessReadinessDenial::UnsupportedProfileMaturityEvidence
        }
        CoverageGapDenial::SmokeOnlyMaturityDenied { subsystem, .. } => {
            S5SimulationHarnessReadinessDenial::MissingDependency(dependency_for_subsystem(
                subsystem,
            ))
        }
        CoverageGapDenial::MissingRegistrationEvidence { surface } => {
            S5SimulationHarnessReadinessDenial::MissingDependency(dependency_for_surface(surface))
        }
        _ => S5SimulationHarnessReadinessDenial::MissingDependency(
            S5HarnessMaturityDependency::ScenarioDefinitions,
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

const fn dependency_for_surface(surface: CoverageSurfaceKind) -> S5HarnessMaturityDependency {
    match surface {
        CoverageSurfaceKind::Scenario | CoverageSurfaceKind::Plan => {
            S5HarnessMaturityDependency::ScenarioDefinitions
        }
        CoverageSurfaceKind::YieldpointSchedule => {
            S5HarnessMaturityDependency::DeterministicScheduler
        }
        CoverageSurfaceKind::Actor => S5HarnessMaturityDependency::ActorModel,
        CoverageSurfaceKind::Driver => S5HarnessMaturityDependency::ProductionDriverContracts,
        CoverageSurfaceKind::Oracle => S5HarnessMaturityDependency::CertificationOracleFamilies,
        CoverageSurfaceKind::Counter => S5HarnessMaturityDependency::CounterStrengthContracts,
        CoverageSurfaceKind::Transcript => S5HarnessMaturityDependency::ReplayableTranscripts,
        CoverageSurfaceKind::MutationResult => S5HarnessMaturityDependency::MutationValidation,
    }
}
