use forge_store_readiness::{S5CorrectnessNonClaimEvidence, S5SimulationHarnessReadinessDenial};

use crate::{
    CounterContractKind, CoverageSurfaceKind, GeneratedCoverageMatrix, HarnessMaturityLevel,
    OracleFamilyKind, PhysicalCertificationEvidenceBundle, PhysicalDriverKind,
    PhysicalOracleNonClaim, PhysicalProofOracleVerdictKind, PhysicalScenarioActorRole,
    PhysicalSimulationProfile, Roadmap2HarnessSequence, S5ReadinessDependencySet,
    S5SimulationHarnessReadiness, ShortcutRejectionObservationKind,
    SyntheticHarnessShortcutRejectionReport,
};

use super::{
    S5CounterContractReadiness, S5InterleavingHarnessCapability, S5MaintenanceActorCapability,
    S5ProductionDriverCapability, S5RequiredYieldpoint, S5ReusableOracleReadiness,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5HarnessReadinessReceipt {
    readiness: S5SimulationHarnessReadiness,
    interleaving: Vec<S5InterleavingHarnessCapability>,
    maintenance_actors: Vec<S5MaintenanceActorCapability>,
    yieldpoints: Vec<S5RequiredYieldpoint>,
    production_drivers: Vec<S5ProductionDriverCapability>,
    oracle_families: Vec<S5ReusableOracleReadiness>,
    counter_contracts: Vec<S5CounterContractReadiness>,
    transcript_digest: [u8; 32],
    shortcut_denial_count: usize,
}

impl S5HarnessReadinessReceipt {
    pub fn from_store_harness_evidence(
        matrix: &GeneratedCoverageMatrix,
        evidence: &PhysicalCertificationEvidenceBundle,
        shortcut_report: &SyntheticHarnessShortcutRejectionReport,
        non_claim: S5CorrectnessNonClaimEvidence,
    ) -> Result<Self, S5SimulationHarnessReadinessDenial> {
        require_matrix_matches_evidence(matrix, evidence)?;
        require_s5_shape_probe_evidence(evidence)?;
        require_shortcut_denials(evidence, shortcut_report)?;
        let readiness = matrix
            .derive_maturity()
            .require_subsystem_level(
                S5ReadinessDependencySet::required_for_ci(),
                HarnessMaturityLevel::CiCertifiable,
            )
            .map_err(|_| {
                S5SimulationHarnessReadinessDenial::MissingDependency(
                    forge_store_readiness::S5HarnessMaturityDependency::ScenarioDefinitions,
                )
            })?
            .admit_s5_simulation_harness_readiness(non_claim)?;
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

    pub fn into_readiness(self) -> S5SimulationHarnessReadiness {
        self.readiness
    }

    pub const fn readiness(&self) -> &S5SimulationHarnessReadiness {
        &self.readiness
    }

    pub fn interleaving(&self) -> &[S5InterleavingHarnessCapability] {
        &self.interleaving
    }

    pub fn maintenance_actors(&self) -> &[S5MaintenanceActorCapability] {
        &self.maintenance_actors
    }

    pub fn yieldpoints(&self) -> &[S5RequiredYieldpoint] {
        &self.yieldpoints
    }

    pub fn production_drivers(&self) -> &[S5ProductionDriverCapability] {
        &self.production_drivers
    }

    pub fn oracle_families(&self) -> &[S5ReusableOracleReadiness] {
        &self.oracle_families
    }

    pub fn counter_contracts(&self) -> &[S5CounterContractReadiness] {
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
) -> Result<(), S5SimulationHarnessReadinessDenial> {
    if matrix.sequence() != Roadmap2HarnessSequence::S45 {
        return Err(S5SimulationHarnessReadinessDenial::WrongSequenceMaturityEvidence);
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

fn require_s5_shape_probe_evidence(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<(), S5SimulationHarnessReadinessDenial> {
    let plan = evidence.replay().plan();
    if plan.profile() != PhysicalSimulationProfile::CiCertification {
        return Err(S5SimulationHarnessReadinessDenial::UnsupportedProfileMaturityEvidence);
    }
    if !evidence
        .replay()
        .schedule()
        .replay_identity_matches_plan(plan)
    {
        return Err(S5SimulationHarnessReadinessDenial::MissingInterleavingCapability);
    }
    require_maintenance_actor(plan)?;
    require_s5_oracle_non_claim(evidence)?;
    Ok(())
}

fn require_shortcut_denials(
    evidence: &PhysicalCertificationEvidenceBundle,
    shortcut_report: &SyntheticHarnessShortcutRejectionReport,
) -> Result<(), S5SimulationHarnessReadinessDenial> {
    if !shortcut_report.all_required_shortcuts_denied() {
        return Err(S5SimulationHarnessReadinessDenial::MissingShortcutDenialReport);
    }
    if !evidence
        .replay()
        .trace()
        .shortcut_rejections()
        .iter()
        .any(|entry| entry.kind() == ShortcutRejectionObservationKind::PrivateMutationDenied)
    {
        return Err(S5SimulationHarnessReadinessDenial::MissingShortcutDenialReport);
    }
    Ok(())
}

fn require_row_identity(
    matrix: &GeneratedCoverageMatrix,
    surface: CoverageSurfaceKind,
    expected: &[u8; 32],
) -> Result<(), S5SimulationHarnessReadinessDenial> {
    if matrix
        .rows()
        .iter()
        .any(|row| row.surface() == surface && row.source_identity() == expected)
    {
        Ok(())
    } else {
        Err(S5SimulationHarnessReadinessDenial::MissingDependency(
            dependency_for_surface(surface),
        ))
    }
}

fn require_maintenance_actor(
    plan: &crate::PhysicalSimulationPlan,
) -> Result<(), S5SimulationHarnessReadinessDenial> {
    if plan
        .actors()
        .contains_role(PhysicalScenarioActorRole::MaintenanceReclaimer)
    {
        Ok(())
    } else {
        Err(S5SimulationHarnessReadinessDenial::MissingMaintenanceActorCapability)
    }
}

fn require_s5_oracle_non_claim(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<(), S5SimulationHarnessReadinessDenial> {
    if evidence.replay().oracle_verdicts().iter().any(|verdict| {
        verdict.family() == OracleFamilyKind::S5ReadinessShape
            && verdict.kind() == PhysicalProofOracleVerdictKind::Satisfied
            && verdict
                .non_claims()
                .contains(&PhysicalOracleNonClaim::S5PhysicalIsolationCorrectness)
    }) {
        Ok(())
    } else {
        Err(S5SimulationHarnessReadinessDenial::MissingS5CorrectnessNonClaim)
    }
}

fn satisfied_interleaving_capabilities() -> Vec<S5InterleavingHarnessCapability> {
    vec![
        S5InterleavingHarnessCapability::DeterministicReplaySchedule,
        S5InterleavingHarnessCapability::ProtectBeforeObserveShapeProbe,
        S5InterleavingHarnessCapability::RootKindSeparationShapeProbe,
        S5InterleavingHarnessCapability::TraversalAdmissionShapeProbe,
        S5InterleavingHarnessCapability::ByteGuardUsageShapeProbe,
        S5InterleavingHarnessCapability::NoHiddenLatchIoShapeProbe,
        S5InterleavingHarnessCapability::PublicationMemoryOrderingShapeProbe,
        S5InterleavingHarnessCapability::LeaseExpiryNonAuthorityShapeProbe,
        S5InterleavingHarnessCapability::FreeReuseGenerationFenceShapeProbe,
        S5InterleavingHarnessCapability::RestartDuringCutoverShapeProbe,
    ]
}

fn satisfied_maintenance_actor_capabilities() -> Vec<S5MaintenanceActorCapability> {
    vec![
        S5MaintenanceActorCapability::ReclaimBarrierParticipant,
        S5MaintenanceActorCapability::RestartParticipant,
    ]
}

fn satisfied_yieldpoints(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<Vec<S5RequiredYieldpoint>, S5SimulationHarnessReadinessDenial> {
    let plan = evidence.replay().plan();
    let mut yieldpoints = Vec::new();
    match plan.yieldpoint_binding().scheduled_yieldpoint() {
        "root-publication-before-observe" => {
            yieldpoints.push(S5RequiredYieldpoint::RootPublicationBeforeObserve)
        }
        _ => return Err(S5SimulationHarnessReadinessDenial::MissingProductionBoundaryYieldpoint),
    }
    yieldpoints.extend([
        S5RequiredYieldpoint::RootSwapPublication,
        S5RequiredYieldpoint::ByteGuardAdmission,
        S5RequiredYieldpoint::ReclaimBarrier,
        S5RequiredYieldpoint::RestartDuringCutover,
    ]);
    if plan
        .drivers()
        .contains(PhysicalDriverKind::ShortcutRejectionBoundary)
    {
        yieldpoints.push(S5RequiredYieldpoint::ShortcutRejectionBoundary);
    }
    Ok(yieldpoints)
}

fn satisfied_drivers(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<Vec<S5ProductionDriverCapability>, S5SimulationHarnessReadinessDenial> {
    let plan = evidence.replay().plan();
    let mut drivers = Vec::new();
    if plan
        .drivers()
        .contains(PhysicalDriverKind::ProductionBoundaryYieldpoint)
    {
        drivers.push(S5ProductionDriverCapability::ProductionBoundaryYieldpoint);
    }
    if plan
        .drivers()
        .contains(PhysicalDriverKind::ShortcutRejectionBoundary)
    {
        drivers.push(S5ProductionDriverCapability::ShortcutRejectionBoundary);
    }
    if drivers.len() == 2 {
        Ok(drivers)
    } else {
        Err(S5SimulationHarnessReadinessDenial::MissingProductionDriverCapability)
    }
}

fn satisfied_oracle_families(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<Vec<S5ReusableOracleReadiness>, S5SimulationHarnessReadinessDenial> {
    let plan = evidence.replay().plan();
    let required = [
        (
            OracleFamilyKind::S5ReadinessShape,
            S5ReusableOracleReadiness::S5ReadinessShape,
        ),
        (
            OracleFamilyKind::TranscriptReplayEvidence,
            S5ReusableOracleReadiness::TranscriptReplayEvidence,
        ),
        (
            OracleFamilyKind::ForbiddenShortcutRejection,
            S5ReusableOracleReadiness::ForbiddenShortcutRejection,
        ),
    ];
    let mut families = Vec::new();
    for (family, readiness) in required {
        if !plan.oracle_families().contains(family) {
            return Err(S5SimulationHarnessReadinessDenial::MissingReusableOracleFamily);
        }
        families.push(readiness);
    }
    Ok(families)
}

fn satisfied_counter_contracts(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<Vec<S5CounterContractReadiness>, S5SimulationHarnessReadinessDenial> {
    let plan = evidence.replay().plan();
    let required = [
        (
            CounterContractKind::ActorStepExact,
            S5CounterContractReadiness::ActorStepExact,
        ),
        (
            CounterContractKind::ReplayIdentityExact,
            S5CounterContractReadiness::ReplayIdentityExact,
        ),
        (
            CounterContractKind::ForbiddenShortcutExact,
            S5CounterContractReadiness::ForbiddenShortcutExact,
        ),
        (
            CounterContractKind::ProfileResourceEnvelope,
            S5CounterContractReadiness::ProfileResourceEnvelope,
        ),
        (
            CounterContractKind::LatchWaits,
            S5CounterContractReadiness::LatchWaits,
        ),
        (
            CounterContractKind::EpochRetries,
            S5CounterContractReadiness::EpochRetries,
        ),
        (
            CounterContractKind::ProtectedReferences,
            S5CounterContractReadiness::ProtectedReferences,
        ),
        (
            CounterContractKind::BlockedReclaimAttempts,
            S5CounterContractReadiness::BlockedReclaimAttempts,
        ),
        (
            CounterContractKind::PublicationSwaps,
            S5CounterContractReadiness::PublicationSwaps,
        ),
    ];
    let mut contracts = Vec::new();
    for (contract, readiness) in required {
        if !plan.counter_contracts().contains(contract) {
            return Err(S5SimulationHarnessReadinessDenial::MissingCounterContract);
        }
        contracts.push(readiness);
    }
    contracts.push(S5CounterContractReadiness::FutureS5SpecificCountersReserved);
    if evidence.replay().counter_receipt().rows().is_empty() {
        return Err(S5SimulationHarnessReadinessDenial::MissingCounterContract);
    }
    Ok(contracts)
}

fn dependency_for_surface(
    surface: CoverageSurfaceKind,
) -> forge_store_readiness::S5HarnessMaturityDependency {
    match surface {
        CoverageSurfaceKind::Scenario | CoverageSurfaceKind::Plan => {
            forge_store_readiness::S5HarnessMaturityDependency::ScenarioDefinitions
        }
        CoverageSurfaceKind::YieldpointSchedule => {
            forge_store_readiness::S5HarnessMaturityDependency::DeterministicScheduler
        }
        CoverageSurfaceKind::Actor => {
            forge_store_readiness::S5HarnessMaturityDependency::ActorModel
        }
        CoverageSurfaceKind::Driver => {
            forge_store_readiness::S5HarnessMaturityDependency::ProductionDriverContracts
        }
        CoverageSurfaceKind::Oracle => {
            forge_store_readiness::S5HarnessMaturityDependency::CertificationOracleFamilies
        }
        CoverageSurfaceKind::Counter => {
            forge_store_readiness::S5HarnessMaturityDependency::CounterStrengthContracts
        }
        CoverageSurfaceKind::Transcript => {
            forge_store_readiness::S5HarnessMaturityDependency::ReplayableTranscripts
        }
        CoverageSurfaceKind::MutationResult => {
            forge_store_readiness::S5HarnessMaturityDependency::MutationValidation
        }
    }
}
