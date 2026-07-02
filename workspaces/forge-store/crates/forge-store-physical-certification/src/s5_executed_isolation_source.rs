use forge_store_authority::StoreCurrentAuthorityWitness;
use forge_store_physical_isolation::S5IsolationEvidenceProfile;

use crate::{
    s5_physical_isolation_required_mutation_rows, CounterContractKind, PhysicalCounterEvidenceRow,
    PhysicalProofOracleKind, PhysicalProofOracleVerdictKind, PhysicalSimulationScenarioFamily,
    S5ExecutedIsolationFinding, S5ExecutedIsolationRequiredCounters,
    S5ExecutedIsolationSourceBasis, S5PhysicalIsolationMutationEvidence, SimulationReplayBundle,
};

#[derive(Debug, Clone)]
pub struct S5ExecutedIsolationEvidenceSource {
    store_authority: StoreCurrentAuthorityWitness,
    replay: SimulationReplayBundle,
    mutation: S5PhysicalIsolationMutationEvidence,
    finding: S5ExecutedIsolationFinding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S5ExecutedIsolationSourceDenial {
    ReadinessProbeCannotMaterialize,
    NonS5FamilyCannotMaterialize,
    MissingMutationEvidence,
    MutationReplayBasisMismatch,
    MutationRowsDoNotMatchFamily,
    MissingS5OracleVerdict,
}

impl S5ExecutedIsolationEvidenceSource {
    pub fn from_executed_replay(
        store_authority: StoreCurrentAuthorityWitness,
        replay: SimulationReplayBundle,
        mutation: S5PhysicalIsolationMutationEvidence,
        profile: S5IsolationEvidenceProfile,
    ) -> Result<Self, S5ExecutedIsolationSourceDenial> {
        require_s5_owned_family(replay.plan().scenario_family())?;
        require_mutation_evidence_matches_replay(&replay, &mutation)?;
        if s5_satisfied_oracle_verdict_count(&replay) == 0 {
            return Err(S5ExecutedIsolationSourceDenial::MissingS5OracleVerdict);
        }

        let finding = S5ExecutedIsolationFinding::from_admitted_executed_source(
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

    pub const fn mutation(&self) -> &S5PhysicalIsolationMutationEvidence {
        &self.mutation
    }

    pub const fn finding(&self) -> &S5ExecutedIsolationFinding {
        &self.finding
    }
}

fn require_s5_owned_family(
    family: PhysicalSimulationScenarioFamily,
) -> Result<(), S5ExecutedIsolationSourceDenial> {
    match family {
        PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe => {
            Err(S5ExecutedIsolationSourceDenial::ReadinessProbeCannotMaterialize)
        }
        PhysicalSimulationScenarioFamily::S5CompactionInterlock
        | PhysicalSimulationScenarioFamily::S5CheckpointPublicationInterlock
        | PhysicalSimulationScenarioFamily::S5ReclaimReachability
        | PhysicalSimulationScenarioFamily::S5TierMovementStability
        | PhysicalSimulationScenarioFamily::S5FutureChunkStability
        | PhysicalSimulationScenarioFamily::S5RestartDuringCutover => Ok(()),
        _ => Err(S5ExecutedIsolationSourceDenial::NonS5FamilyCannotMaterialize),
    }
}

fn require_mutation_evidence_matches_replay(
    replay: &SimulationReplayBundle,
    mutation: &S5PhysicalIsolationMutationEvidence,
) -> Result<(), S5ExecutedIsolationSourceDenial> {
    if mutation.required_rows().is_empty() {
        return Err(S5ExecutedIsolationSourceDenial::MissingMutationEvidence);
    }
    if mutation.plan_identity() != replay.plan().identity().digest_bytes()
        || mutation.schedule_identity() != replay.schedule().identity().digest_bytes()
        || mutation.transcript_identity() != replay.transcript_identity().digest_bytes()
        || mutation.replay_basis_identity() != replay.replay_basis_identity().digest_bytes()
    {
        return Err(S5ExecutedIsolationSourceDenial::MutationReplayBasisMismatch);
    }
    if mutation.required_rows()
        != s5_physical_isolation_required_mutation_rows(replay.plan().scenario_family())
    {
        return Err(S5ExecutedIsolationSourceDenial::MutationRowsDoNotMatchFamily);
    }
    Ok(())
}

fn source_basis(replay: &SimulationReplayBundle) -> S5ExecutedIsolationSourceBasis {
    S5ExecutedIsolationSourceBasis::new(
        family_token(replay.plan().scenario_family()),
        *replay.plan().identity().digest_bytes(),
        *replay.schedule().identity().digest_bytes(),
        *replay.transcript_identity().digest_bytes(),
        *replay.replay_basis_identity().digest_bytes(),
    )
}

fn required_counters(replay: &SimulationReplayBundle) -> S5ExecutedIsolationRequiredCounters {
    let rows = replay.counter_receipt().rows();
    S5ExecutedIsolationRequiredCounters::new(
        s5_satisfied_oracle_verdict_count(replay),
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

fn s5_satisfied_oracle_verdict_count(replay: &SimulationReplayBundle) -> u64 {
    replay
        .oracle_verdicts()
        .iter()
        .filter(|verdict| {
            verdict.oracle() == PhysicalProofOracleKind::S5PhysicalIsolationInterleaving
                && verdict.kind() == PhysicalProofOracleVerdictKind::Satisfied
        })
        .count() as u64
}

fn family_token(family: PhysicalSimulationScenarioFamily) -> &'static str {
    match family {
        PhysicalSimulationScenarioFamily::S5CompactionInterlock => "s5.compaction_interlock",
        PhysicalSimulationScenarioFamily::S5CheckpointPublicationInterlock => {
            "s5.checkpoint_publication_interlock"
        }
        PhysicalSimulationScenarioFamily::S5ReclaimReachability => "s5.reclaim_reachability",
        PhysicalSimulationScenarioFamily::S5TierMovementStability => "s5.tier_movement_stability",
        PhysicalSimulationScenarioFamily::S5FutureChunkStability => "s5.future_chunk_stability",
        PhysicalSimulationScenarioFamily::S5RestartDuringCutover => "s5.restart_during_cutover",
        _ => "s5.unsupported",
    }
}
