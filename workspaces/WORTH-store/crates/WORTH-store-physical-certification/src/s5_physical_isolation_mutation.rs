use crate::{
    CoverageGapDenial, FaultDeliveryAttempt, PhysicalMutationCoverageEvidence,
    PhysicalSimulationScenarioFamily, Roadmap2HarnessSequence, S5PhysicalIsolationMutationKind,
    SimulationReplayBundle,
};

#[derive(Debug, Clone)]
pub struct S5PhysicalIsolationMutationEvidence {
    physical: PhysicalMutationCoverageEvidence,
    replay_basis: S5MutationReplayBasis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S5MutationReplayBasis {
    plan_identity: [u8; 32],
    schedule_identity: [u8; 32],
    transcript_identity: [u8; 32],
    replay_basis_identity: [u8; 32],
}

impl S5PhysicalIsolationMutationEvidence {
    pub fn from_replay(
        family: PhysicalSimulationScenarioFamily,
        replay: &SimulationReplayBundle,
    ) -> Self {
        Self::try_from_replay(family, replay).unwrap()
    }

    pub fn try_from_replay(
        family: PhysicalSimulationScenarioFamily,
        replay: &SimulationReplayBundle,
    ) -> Result<Self, CoverageGapDenial> {
        if family != replay.plan().scenario_family() {
            return Err(CoverageGapDenial::MissingMutationResult);
        }
        let physical =
            PhysicalMutationCoverageEvidence::from_replay_private_and_s5_physical_isolation_denials(
                Roadmap2HarnessSequence::S45,
                replay,
                FaultDeliveryAttempt::private_mutation(),
            )?;
        Ok(Self {
            physical,
            replay_basis: S5MutationReplayBasis::from_replay(replay),
        })
    }

    pub const fn physical(&self) -> &PhysicalMutationCoverageEvidence {
        &self.physical
    }

    pub const fn plan_identity(&self) -> &[u8; 32] {
        &self.replay_basis.plan_identity
    }

    pub const fn schedule_identity(&self) -> &[u8; 32] {
        &self.replay_basis.schedule_identity
    }

    pub const fn transcript_identity(&self) -> &[u8; 32] {
        &self.replay_basis.transcript_identity
    }

    pub const fn replay_basis_identity(&self) -> &[u8; 32] {
        &self.replay_basis.replay_basis_identity
    }

    pub fn required_rows(&self) -> &[S5PhysicalIsolationMutationKind] {
        self.physical.s5_physical_isolation_mutations()
    }
}

impl S5MutationReplayBasis {
    fn from_replay(replay: &SimulationReplayBundle) -> Self {
        Self {
            plan_identity: *replay.plan().identity().digest_bytes(),
            schedule_identity: *replay.schedule().identity().digest_bytes(),
            transcript_identity: *replay.transcript_identity().digest_bytes(),
            replay_basis_identity: *replay.replay_basis_identity().digest_bytes(),
        }
    }
}

pub fn s5_physical_isolation_required_mutation_rows(
    family: PhysicalSimulationScenarioFamily,
) -> &'static [S5PhysicalIsolationMutationKind] {
    match family {
        PhysicalSimulationScenarioFamily::S5CompactionInterlock => &COMPACTION_ROWS,
        PhysicalSimulationScenarioFamily::S5CheckpointPublicationInterlock => &CHECKPOINT_ROWS,
        PhysicalSimulationScenarioFamily::S5ReclaimReachability => &RECLAIM_ROWS,
        PhysicalSimulationScenarioFamily::S5TierMovementStability => &TIER_ROWS,
        PhysicalSimulationScenarioFamily::S5FutureChunkStability => &FUTURE_CHUNK_ROWS,
        PhysicalSimulationScenarioFamily::S5RestartDuringCutover => &RESTART_ROWS,
        _ => &[],
    }
}

const COMPACTION_ROWS: [S5PhysicalIsolationMutationKind; 6] = [
    S5PhysicalIsolationMutationKind::CompactionInPlaceOverwriteDenied,
    S5PhysicalIsolationMutationKind::CompactionEarlyReclaimDenied,
    S5PhysicalIsolationMutationKind::CompactionStaleEpochReuseDenied,
    S5PhysicalIsolationMutationKind::CompactionBackendResidueSelectionDenied,
    S5PhysicalIsolationMutationKind::CompactionLatchHierarchyInversionDenied,
    S5PhysicalIsolationMutationKind::CompactionMixedRootReadDenied,
];
const CHECKPOINT_ROWS: [S5PhysicalIsolationMutationKind; 1] =
    [S5PhysicalIsolationMutationKind::CheckpointMixedRootReadDenied];
const RECLAIM_ROWS: [S5PhysicalIsolationMutationKind; 1] =
    [S5PhysicalIsolationMutationKind::ReclaimEarlyReachabilityDenied];
const TIER_ROWS: [S5PhysicalIsolationMutationKind; 1] =
    [S5PhysicalIsolationMutationKind::TierMovementStabilityNonClaim];
const FUTURE_CHUNK_ROWS: [S5PhysicalIsolationMutationKind; 1] =
    [S5PhysicalIsolationMutationKind::FutureChunkStabilityNonClaim];
const RESTART_ROWS: [S5PhysicalIsolationMutationKind; 1] =
    [S5PhysicalIsolationMutationKind::RestartLatchHierarchyInversionDenied];
