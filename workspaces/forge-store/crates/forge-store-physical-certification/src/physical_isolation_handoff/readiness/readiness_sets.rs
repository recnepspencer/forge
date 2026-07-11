use forge_store_readiness::PhysicalIsolationHarnessReadinessDenial;

use crate::{
    CounterContractKind, OracleFamilyKind, PhysicalCertificationEvidenceBundle, PhysicalDriverKind,
    PhysicalProofOracleKind, PhysicalProofOracleVerdictKind,
};

use super::{
    PhysicalIsolationCounterContractReadiness, PhysicalIsolationInterleavingHarnessCapability,
    PhysicalIsolationMaintenanceActorCapability, PhysicalIsolationProductionDriverCapability,
    PhysicalIsolationRequiredYieldpoint, PhysicalIsolationReusableOracleReadiness,
};

pub(super) fn satisfied_interleaving_capabilities(
) -> Vec<PhysicalIsolationInterleavingHarnessCapability> {
    vec![
        PhysicalIsolationInterleavingHarnessCapability::DeterministicReplaySchedule,
        PhysicalIsolationInterleavingHarnessCapability::ProtectBeforeObserveShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::RootKindSeparationShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::TraversalAdmissionShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::ByteGuardUsageShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::NoHiddenLatchIoShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::PublicationMemoryOrderingShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::LeaseExpiryNonAuthorityShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::FreeReuseGenerationFenceShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::RestartDuringCutoverShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::ReadDuringCompactionShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::CompactionRangeInterlockShapeProbe,
    ]
}

pub(super) fn satisfied_maintenance_actor_capabilities(
) -> Vec<PhysicalIsolationMaintenanceActorCapability> {
    vec![
        PhysicalIsolationMaintenanceActorCapability::ReclaimBarrierParticipant,
        PhysicalIsolationMaintenanceActorCapability::RestartParticipant,
        PhysicalIsolationMaintenanceActorCapability::CompactionCutoverParticipant,
    ]
}

pub(super) fn satisfied_yieldpoints(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<Vec<PhysicalIsolationRequiredYieldpoint>, PhysicalIsolationHarnessReadinessDenial> {
    let plan = evidence.replay().plan();
    let mut yieldpoints = Vec::new();
    match plan.yieldpoint_binding().scheduled_yieldpoint() {
        "root-publication-before-observe" => {
            yieldpoints.push(PhysicalIsolationRequiredYieldpoint::RootPublicationBeforeObserve)
        }
        _ => {
            return Err(
                PhysicalIsolationHarnessReadinessDenial::MissingProductionBoundaryYieldpoint,
            )
        }
    }
    yieldpoints.extend([
        PhysicalIsolationRequiredYieldpoint::RootSwapPublication,
        PhysicalIsolationRequiredYieldpoint::ByteGuardAdmission,
        PhysicalIsolationRequiredYieldpoint::ReclaimBarrier,
        PhysicalIsolationRequiredYieldpoint::RestartDuringCutover,
        PhysicalIsolationRequiredYieldpoint::CompactionCutover,
    ]);
    if plan
        .drivers()
        .contains(PhysicalDriverKind::ShortcutRejectionBoundary)
    {
        yieldpoints.push(PhysicalIsolationRequiredYieldpoint::ShortcutRejectionBoundary);
    }
    Ok(yieldpoints)
}

pub(super) fn satisfied_drivers(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<Vec<PhysicalIsolationProductionDriverCapability>, PhysicalIsolationHarnessReadinessDenial>
{
    let plan = evidence.replay().plan();
    let mut drivers = Vec::new();
    if plan
        .drivers()
        .contains(PhysicalDriverKind::ProductionBoundaryYieldpoint)
    {
        drivers.push(PhysicalIsolationProductionDriverCapability::ProductionBoundaryYieldpoint);
    }
    if plan
        .drivers()
        .contains(PhysicalDriverKind::ShortcutRejectionBoundary)
    {
        drivers.push(PhysicalIsolationProductionDriverCapability::ShortcutRejectionBoundary);
    }
    if drivers.len() == 2 {
        Ok(drivers)
    } else {
        Err(PhysicalIsolationHarnessReadinessDenial::MissingProductionDriverCapability)
    }
}

pub(super) fn satisfied_oracle_families(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<Vec<PhysicalIsolationReusableOracleReadiness>, PhysicalIsolationHarnessReadinessDenial>
{
    let plan = evidence.replay().plan();
    let required = [
        (
            OracleFamilyKind::PhysicalIsolationReadinessShape,
            PhysicalIsolationReusableOracleReadiness::PhysicalIsolationReadinessShape,
        ),
        (
            OracleFamilyKind::TranscriptReplayEvidence,
            PhysicalIsolationReusableOracleReadiness::TranscriptReplayEvidence,
        ),
        (
            OracleFamilyKind::ForbiddenShortcutRejection,
            PhysicalIsolationReusableOracleReadiness::ForbiddenShortcutRejection,
        ),
    ];
    let mut families = Vec::new();
    for (family, readiness) in required {
        if !plan.oracle_families().contains(family) {
            return Err(PhysicalIsolationHarnessReadinessDenial::MissingReusableOracleFamily);
        }
        families.push(readiness);
    }
    require_compaction_oracle_verdicts(evidence)?;
    Ok(families)
}

pub(super) fn satisfied_counter_contracts(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<Vec<PhysicalIsolationCounterContractReadiness>, PhysicalIsolationHarnessReadinessDenial>
{
    let plan = evidence.replay().plan();
    let required = [
        (
            CounterContractKind::ActorStepExact,
            PhysicalIsolationCounterContractReadiness::ActorStepExact,
        ),
        (
            CounterContractKind::ReplayIdentityExact,
            PhysicalIsolationCounterContractReadiness::ReplayIdentityExact,
        ),
        (
            CounterContractKind::ForbiddenShortcutExact,
            PhysicalIsolationCounterContractReadiness::ForbiddenShortcutExact,
        ),
        (
            CounterContractKind::ProfileResourceEnvelope,
            PhysicalIsolationCounterContractReadiness::ProfileResourceEnvelope,
        ),
        (
            CounterContractKind::LatchWaits,
            PhysicalIsolationCounterContractReadiness::LatchWaits,
        ),
        (
            CounterContractKind::EpochRetries,
            PhysicalIsolationCounterContractReadiness::EpochRetries,
        ),
        (
            CounterContractKind::ProtectedReferences,
            PhysicalIsolationCounterContractReadiness::ProtectedReferences,
        ),
        (
            CounterContractKind::BlockedReclaimAttempts,
            PhysicalIsolationCounterContractReadiness::BlockedReclaimAttempts,
        ),
        (
            CounterContractKind::PublicationSwaps,
            PhysicalIsolationCounterContractReadiness::PublicationSwaps,
        ),
        (
            CounterContractKind::CompactionCandidateRanges,
            PhysicalIsolationCounterContractReadiness::CompactionCandidateRanges,
        ),
        (
            CounterContractKind::CopiedPages,
            PhysicalIsolationCounterContractReadiness::CopiedPages,
        ),
    ];
    let mut contracts = Vec::new();
    for (contract, readiness) in required {
        if !plan.counter_contracts().contains(contract) {
            return Err(PhysicalIsolationHarnessReadinessDenial::MissingCounterContract);
        }
        contracts.push(readiness);
    }
    contracts.push(PhysicalIsolationCounterContractReadiness::FutureS5SpecificCountersReserved);
    require_compaction_counter_rows(evidence)?;
    Ok(contracts)
}

fn require_compaction_oracle_verdicts(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<(), PhysicalIsolationHarnessReadinessDenial> {
    let required = [
        PhysicalProofOracleKind::NoMixedRoot,
        PhysicalProofOracleKind::OldReaderSeesOldRoot,
        PhysicalProofOracleKind::PostSwapReaderSeesNewRoot,
        PhysicalProofOracleKind::BlockedReclaimUntilRelease,
    ];
    for oracle in required {
        if !evidence.replay().oracle_verdicts().iter().any(|verdict| {
            verdict.family() == OracleFamilyKind::PhysicalIsolationReadinessShape
                && verdict.oracle() == oracle
                && verdict.kind() == PhysicalProofOracleVerdictKind::Satisfied
        }) {
            return Err(PhysicalIsolationHarnessReadinessDenial::MissingReusableOracleFamily);
        }
    }
    Ok(())
}

fn require_compaction_counter_rows(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> Result<(), PhysicalIsolationHarnessReadinessDenial> {
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
            return Err(PhysicalIsolationHarnessReadinessDenial::MissingCounterContract);
        }
    }
    Ok(())
}
