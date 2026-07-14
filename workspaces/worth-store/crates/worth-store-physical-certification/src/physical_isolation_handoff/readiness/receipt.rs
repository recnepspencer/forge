use crate::{
    PhysicalIsolationCorrectnessNonClaimEvidence, PhysicalIsolationHarnessMaturityDependency,
    PhysicalIsolationHarnessReadinessDenial,
};

use crate::{
    CoverageSurfaceKind, GeneratedCoverageMatrix, HarnessCoverageStage, HarnessMaturityLevel,
    OracleFamilyKind, PhysicalCertificationEvidenceBundle, PhysicalIsolationHarnessReadiness,
    PhysicalIsolationReadinessDependencySet, PhysicalOracleNonClaim,
    PhysicalProofOracleVerdictKind, PhysicalScenarioActorRole, PhysicalSimulationProfile,
    ShortcutRejectionObservationKind, SyntheticHarnessShortcutRejectionReport,
};

use super::{
    readiness_sets::{
        satisfied_counter_contracts, satisfied_drivers, satisfied_interleaving_capabilities,
        satisfied_maintenance_actor_capabilities, satisfied_oracle_families, satisfied_yieldpoints,
    },
    PhysicalIsolationCounterContractReadiness, PhysicalIsolationInterleavingHarnessCapability,
    PhysicalIsolationMaintenanceActorCapability, PhysicalIsolationProductionDriverCapability,
    PhysicalIsolationRequiredYieldpoint, PhysicalIsolationReusableOracleReadiness,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationHarnessReadinessReceipt {
    readiness: PhysicalIsolationHarnessReadiness,
    interleaving: Vec<PhysicalIsolationInterleavingHarnessCapability>,
    maintenance_actors: Vec<PhysicalIsolationMaintenanceActorCapability>,
    yieldpoints: Vec<PhysicalIsolationRequiredYieldpoint>,
    production_drivers: Vec<PhysicalIsolationProductionDriverCapability>,
    oracle_families: Vec<PhysicalIsolationReusableOracleReadiness>,
    counter_contracts: Vec<PhysicalIsolationCounterContractReadiness>,
    transcript_digest: [u8; 32],
    shortcut_denial_count: usize,
}

impl PhysicalIsolationHarnessReadinessReceipt {
    pub fn from_store_harness_evidence(
        matrix: &GeneratedCoverageMatrix,
        evidence: &PhysicalCertificationEvidenceBundle,
        shortcut_report: &SyntheticHarnessShortcutRejectionReport,
        non_claim: PhysicalIsolationCorrectnessNonClaimEvidence,
    ) -> Result<Self, PhysicalIsolationHarnessReadinessDenial> {
        require_matrix_matches_evidence(matrix, evidence)?;
        require_physical_isolation_shape_probe_evidence(evidence)?;
        require_shortcut_denials(evidence, shortcut_report)?;
        let readiness = matrix
            .derive_maturity()
            .require_subsystem_level(
                PhysicalIsolationReadinessDependencySet::required_for_ci(),
                HarnessMaturityLevel::CiCertifiable,
            )
            .map_err(|_| {
                PhysicalIsolationHarnessReadinessDenial::MissingDependency(
                    PhysicalIsolationHarnessMaturityDependency::ScenarioDefinitions,
                )
            })?
            .admit_physical_isolation_simulation_harness_readiness(non_claim)?;
        Ok(Self {
            readiness,
            interleaving: satisfied_interleaving_capabilities(),
            maintenance_actors: satisfied_maintenance_actor_capabilities(),
            yieldpoints: satisfied_yieldpoints(evidence)?,
            production_drivers: satisfied_drivers(evidence)?,
            oracle_families: satisfied_oracle_families(evidence)?,
            counter_contracts: satisfied_counter_contracts(evidence)?,
            transcript_digest: *evidence.primary().transcript_digest(),
            shortcut_denial_count: shortcut_report.receipts().len(),
        })
    }

    pub fn into_readiness(self) -> PhysicalIsolationHarnessReadiness {
        self.readiness
    }

    pub const fn readiness(&self) -> &PhysicalIsolationHarnessReadiness {
        &self.readiness
    }

    pub fn interleaving(&self) -> &[PhysicalIsolationInterleavingHarnessCapability] {
        &self.interleaving
    }

    pub fn maintenance_actors(&self) -> &[PhysicalIsolationMaintenanceActorCapability] {
        &self.maintenance_actors
    }

    pub fn yieldpoints(&self) -> &[PhysicalIsolationRequiredYieldpoint] {
        &self.yieldpoints
    }

    pub fn production_drivers(&self) -> &[PhysicalIsolationProductionDriverCapability] {
        &self.production_drivers
    }

    pub fn oracle_families(&self) -> &[PhysicalIsolationReusableOracleReadiness] {
        &self.oracle_families
    }

    pub fn counter_contracts(&self) -> &[PhysicalIsolationCounterContractReadiness] {
        &self.counter_contracts
    }

    pub const fn transcript_digest(&self) -> &[u8; 32] {
        &self.transcript_digest
    }

    pub const fn shortcut_denial_count(&self) -> usize {
        self.shortcut_denial_count
    }
}

fn require_matrix_matches_evidence(
    matrix: &GeneratedCoverageMatrix,
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<(), PhysicalIsolationHarnessReadinessDenial> {
    if matrix.sequence() != HarnessCoverageStage::SimulationAdmission {
        return Err(PhysicalIsolationHarnessReadinessDenial::WrongSequenceMaturityEvidence);
    }
    let primary = evidence.primary();
    require_row_identity(
        matrix,
        CoverageSurfaceKind::Scenario,
        primary.scenario_digest(),
    )?;
    require_row_identity(matrix, CoverageSurfaceKind::Plan, primary.plan_digest())?;
    require_row_identity(
        matrix,
        CoverageSurfaceKind::Transcript,
        primary.transcript_digest(),
    )?;
    Ok(())
}

fn require_physical_isolation_shape_probe_evidence(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<(), PhysicalIsolationHarnessReadinessDenial> {
    let plan = evidence.replay().plan();
    if plan.profile() != PhysicalSimulationProfile::CiCertification {
        return Err(PhysicalIsolationHarnessReadinessDenial::UnsupportedProfileMaturityEvidence);
    }
    if !evidence
        .replay()
        .schedule()
        .replay_identity_matches_plan(plan)
    {
        return Err(PhysicalIsolationHarnessReadinessDenial::MissingInterleavingCapability);
    }
    require_maintenance_actor(plan)?;
    require_physical_isolation_oracle_non_claim(evidence)?;
    Ok(())
}

fn require_shortcut_denials(
    evidence: &PhysicalCertificationEvidenceBundle,
    shortcut_report: &SyntheticHarnessShortcutRejectionReport,
) -> Result<(), PhysicalIsolationHarnessReadinessDenial> {
    if !shortcut_report.all_required_shortcuts_denied() {
        return Err(PhysicalIsolationHarnessReadinessDenial::MissingShortcutDenialReport);
    }
    if !evidence
        .replay()
        .trace()
        .shortcut_rejections()
        .iter()
        .any(|entry| entry.kind() == ShortcutRejectionObservationKind::PrivateMutationDenied)
    {
        return Err(PhysicalIsolationHarnessReadinessDenial::MissingShortcutDenialReport);
    }
    Ok(())
}

fn require_row_identity(
    matrix: &GeneratedCoverageMatrix,
    surface: CoverageSurfaceKind,
    expected: &[u8; 32],
) -> Result<(), PhysicalIsolationHarnessReadinessDenial> {
    if matrix
        .rows()
        .iter()
        .any(|row| row.surface() == surface && row.source_identity() == expected)
    {
        Ok(())
    } else {
        Err(PhysicalIsolationHarnessReadinessDenial::MissingDependency(
            dependency_for_surface(surface),
        ))
    }
}

fn require_maintenance_actor(
    plan: &crate::PhysicalSimulationPlan,
) -> Result<(), PhysicalIsolationHarnessReadinessDenial> {
    if plan
        .actors()
        .contains_role(PhysicalScenarioActorRole::MaintenanceReclaimer)
    {
        Ok(())
    } else {
        Err(PhysicalIsolationHarnessReadinessDenial::MissingMaintenanceActorCapability)
    }
}

fn require_physical_isolation_oracle_non_claim(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<(), PhysicalIsolationHarnessReadinessDenial> {
    if evidence.replay().oracle_verdicts().iter().any(|verdict| {
        verdict.family() == OracleFamilyKind::PhysicalIsolationReadinessShape
            && verdict.kind() == PhysicalProofOracleVerdictKind::Satisfied
            && verdict
                .non_claims()
                .contains(&PhysicalOracleNonClaim::PhysicalIsolationCorrectness)
    }) {
        Ok(())
    } else {
        Err(PhysicalIsolationHarnessReadinessDenial::MissingPhysicalIsolationCorrectnessNonClaim)
    }
}

fn dependency_for_surface(
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
