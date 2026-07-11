use forge_store_authority::StoreCurrentAuthorityWitness;
use forge_store_physical_isolation::PhysicalIsolationEvidenceProfile;

use crate::{
    physical_isolation_required_mutation_rows, CounterContractKind,
    ExecutedPhysicalIsolationFinding, ExecutedPhysicalIsolationRequiredCounters,
    ExecutedPhysicalIsolationSourceBasis, PhysicalCounterEvidenceRow,
    PhysicalIsolationMutationEvidence, PhysicalProofOracleKind, PhysicalProofOracleVerdictKind,
    PhysicalSimulationScenarioFamily, SimulationReplayBundle,
};

#[derive(Debug, Clone)]
pub struct ExecutedPhysicalIsolationEvidenceSource {
    store_authority: StoreCurrentAuthorityWitness,
    replay: SimulationReplayBundle,
    mutation: PhysicalIsolationMutationEvidence,
    finding: ExecutedPhysicalIsolationFinding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutedPhysicalIsolationSourceDenial {
    ReadinessProbeCannotMaterialize,
    NonS5FamilyCannotMaterialize,
    MissingMutationEvidence,
    MutationReplayBasisMismatch,
    MutationRowsDoNotMatchFamily,
    MissingPhysicalIsolationOracleVerdict,
}

impl ExecutedPhysicalIsolationEvidenceSource {
    pub fn from_executed_replay(
        store_authority: StoreCurrentAuthorityWitness,
        replay: SimulationReplayBundle,
        mutation: PhysicalIsolationMutationEvidence,
        profile: PhysicalIsolationEvidenceProfile,
    ) -> Result<Self, ExecutedPhysicalIsolationSourceDenial> {
        require_physical_isolation_owned_family(replay.plan().scenario_family())?;
        require_mutation_evidence_matches_replay(&replay, &mutation)?;
        if physical_isolation_satisfied_oracle_verdict_count(&replay) == 0 {
            return Err(
                ExecutedPhysicalIsolationSourceDenial::MissingPhysicalIsolationOracleVerdict,
            );
        }

        let finding = ExecutedPhysicalIsolationFinding::from_admitted_executed_source(
            replay.plan().scenario_family(),
            source_basis(&replay),
            required_counters(&replay),
            profile,
        );
        Ok(Self {
            store_authority,
            replay,
            mutation,
            finding,
        })
    }

    pub const fn store_authority(&self) -> &StoreCurrentAuthorityWitness {
        &self.store_authority
    }

    pub const fn replay(&self) -> &SimulationReplayBundle {
        &self.replay
    }

    pub const fn mutation(&self) -> &PhysicalIsolationMutationEvidence {
        &self.mutation
    }

    pub const fn finding(&self) -> &ExecutedPhysicalIsolationFinding {
        &self.finding
    }
}

fn require_physical_isolation_owned_family(
    family: PhysicalSimulationScenarioFamily,
) -> Result<(), ExecutedPhysicalIsolationSourceDenial> {
    match family {
        PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe => {
            Err(ExecutedPhysicalIsolationSourceDenial::ReadinessProbeCannotMaterialize)
        }
        PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock
        | PhysicalSimulationScenarioFamily::PhysicalIsolationCheckpointPublicationInterlock
        | PhysicalSimulationScenarioFamily::PhysicalIsolationReclaimReachability
        | PhysicalSimulationScenarioFamily::PhysicalIsolationTierMovementStability
        | PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability
        | PhysicalSimulationScenarioFamily::PhysicalIsolationRestartDuringCutover => Ok(()),
        _ => Err(ExecutedPhysicalIsolationSourceDenial::NonS5FamilyCannotMaterialize),
    }
}

fn require_mutation_evidence_matches_replay(
    replay: &SimulationReplayBundle,
    mutation: &PhysicalIsolationMutationEvidence,
) -> Result<(), ExecutedPhysicalIsolationSourceDenial> {
    if mutation.required_rows().is_empty() {
        return Err(ExecutedPhysicalIsolationSourceDenial::MissingMutationEvidence);
    }
    if mutation.plan_identity() != replay.plan().identity().digest_bytes()
        || mutation.schedule_identity() != replay.schedule().identity().digest_bytes()
        || mutation.transcript_identity() != replay.transcript_identity().digest_bytes()
        || mutation.replay_basis_identity() != replay.replay_basis_identity().digest_bytes()
    {
        return Err(ExecutedPhysicalIsolationSourceDenial::MutationReplayBasisMismatch);
    }
    if mutation.required_rows()
        != physical_isolation_required_mutation_rows(replay.plan().scenario_family())
    {
        return Err(ExecutedPhysicalIsolationSourceDenial::MutationRowsDoNotMatchFamily);
    }
    Ok(())
}

fn source_basis(replay: &SimulationReplayBundle) -> ExecutedPhysicalIsolationSourceBasis {
    ExecutedPhysicalIsolationSourceBasis::new(
        family_token(replay.plan().scenario_family()),
        *replay.plan().identity().digest_bytes(),
        *replay.schedule().identity().digest_bytes(),
        *replay.transcript_identity().digest_bytes(),
        *replay.replay_basis_identity().digest_bytes(),
    )
}

fn required_counters(replay: &SimulationReplayBundle) -> ExecutedPhysicalIsolationRequiredCounters {
    let rows = replay.counter_receipt().rows();
    ExecutedPhysicalIsolationRequiredCounters::new(
        physical_isolation_satisfied_oracle_verdict_count(replay),
        count(rows, CounterContractKind::EpochRetries),
        count(rows, CounterContractKind::LatchWaits),
        count(rows, CounterContractKind::BlockedReclaimAttempts),
    )
}

fn count(rows: &[PhysicalCounterEvidenceRow], kind: CounterContractKind) -> u64 {
    rows.iter()
        .find(|row| row.kind() == kind)
        .map(|row| row.observed_count())
        .unwrap_or(0)
}

fn physical_isolation_satisfied_oracle_verdict_count(replay: &SimulationReplayBundle) -> u64 {
    replay
        .oracle_verdicts()
        .iter()
        .filter(|verdict| {
            verdict.oracle() == PhysicalProofOracleKind::PhysicalIsolationInterleaving
                && verdict.kind() == PhysicalProofOracleVerdictKind::Satisfied
        })
        .count() as u64
}

fn family_token(family: PhysicalSimulationScenarioFamily) -> &'static str {
    match family {
        PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock => {
            "s5.compaction_interlock"
        }
        PhysicalSimulationScenarioFamily::PhysicalIsolationCheckpointPublicationInterlock => {
            "s5.checkpoint_publication_interlock"
        }
        PhysicalSimulationScenarioFamily::PhysicalIsolationReclaimReachability => {
            "s5.reclaim_reachability"
        }
        PhysicalSimulationScenarioFamily::PhysicalIsolationTierMovementStability => {
            "s5.tier_movement_stability"
        }
        PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability => {
            "s5.future_chunk_stability"
        }
        PhysicalSimulationScenarioFamily::PhysicalIsolationRestartDuringCutover => {
            "s5.restart_during_cutover"
        }
        _ => "s5.unsupported",
    }
}
