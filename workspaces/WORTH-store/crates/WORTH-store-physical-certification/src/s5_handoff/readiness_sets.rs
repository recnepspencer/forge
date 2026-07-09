use worth_store_readiness::S5SimulationHarnessReadinessDenial;

use crate::{
    CounterContractKind, OracleFamilyKind, PhysicalCertificationEvidenceBundle, PhysicalDriverKind,
    PhysicalProofOracleKind, PhysicalProofOracleVerdictKind,
};

use super::{
    S5CounterContractReadiness, S5InterleavingHarnessCapability, S5MaintenanceActorCapability,
    S5ProductionDriverCapability, S5RequiredYieldpoint, S5ReusableOracleReadiness,
};

pub(super) fn satisfied_interleaving_capabilities() -> Vec<S5InterleavingHarnessCapability> {
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
        S5InterleavingHarnessCapability::ReadDuringCompactionShapeProbe,
        S5InterleavingHarnessCapability::CompactionRangeInterlockShapeProbe,
    ]
}

pub(super) fn satisfied_maintenance_actor_capabilities() -> Vec<S5MaintenanceActorCapability> {
    vec![
        S5MaintenanceActorCapability::ReclaimBarrierParticipant,
        S5MaintenanceActorCapability::RestartParticipant,
        S5MaintenanceActorCapability::CompactionCutoverParticipant,
    ]
}

pub(super) fn satisfied_yieldpoints(
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
        S5RequiredYieldpoint::CompactionCutover,
    ]);
    if plan
        .drivers()
        .contains(PhysicalDriverKind::ShortcutRejectionBoundary)
    {
        yieldpoints.push(S5RequiredYieldpoint::ShortcutRejectionBoundary);
    }
    Ok(yieldpoints)
}

pub(super) fn satisfied_drivers(
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

pub(super) fn satisfied_oracle_families(
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
    require_compaction_oracle_verdicts(evidence)?;
    Ok(families)
}

pub(super) fn satisfied_counter_contracts(
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
        (
            CounterContractKind::CompactionCandidateRanges,
            S5CounterContractReadiness::CompactionCandidateRanges,
        ),
        (
            CounterContractKind::CopiedPages,
            S5CounterContractReadiness::CopiedPages,
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
    require_compaction_counter_rows(evidence)?;
    Ok(contracts)
}

fn require_compaction_oracle_verdicts(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<(), S5SimulationHarnessReadinessDenial> {
    let required = [
        PhysicalProofOracleKind::NoMixedRoot,
        PhysicalProofOracleKind::OldReaderSeesOldRoot,
        PhysicalProofOracleKind::PostSwapReaderSeesNewRoot,
        PhysicalProofOracleKind::BlockedReclaimUntilRelease,
    ];
    for oracle in required {
        if !evidence.replay().oracle_verdicts().iter().any(|verdict| {
            verdict.family() == OracleFamilyKind::S5ReadinessShape
                && verdict.oracle() == oracle
                && verdict.kind() == PhysicalProofOracleVerdictKind::Satisfied
        }) {
            return Err(S5SimulationHarnessReadinessDenial::MissingReusableOracleFamily);
        }
    }
    Ok(())
}

fn require_compaction_counter_rows(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<(), S5SimulationHarnessReadinessDenial> {
    for kind in [
        CounterContractKind::CompactionCandidateRanges,
        CounterContractKind::CopiedPages,
    ] {
        if !evidence
            .replay()
            .counter_receipt()
            .rows()
            .iter()
            .any(|row| row.kind() == kind && row.observed_count() > 0)
        {
            return Err(S5SimulationHarnessReadinessDenial::MissingCounterContract);
        }
    }
    Ok(())
}
