use crate::{
    CoverageGapDenial, FaultDeliveryAttempt, HarnessCoverageStage, PhysicalIsolationMutationKind,
    PhysicalMutationCoverageEvidence, PhysicalSimulationScenarioFamily, SimulationReplayBundle,
};

#[derive(Debug, Clone)]
pub struct PhysicalIsolationMutationEvidence {
    physical: PhysicalMutationCoverageEvidence,
    replay_basis: PhysicalIsolationMutationReplayBasis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalIsolationMutationReplayBasis {
    plan_identity: [u8; 32],
    schedule_identity: [u8; 32],
    transcript_identity: [u8; 32],
    replay_basis_identity: [u8; 32],
}

impl PhysicalIsolationMutationEvidence {
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
            PhysicalMutationCoverageEvidence::from_replay_private_and_physical_isolation_denials(
                HarnessCoverageStage::SimulationAdmission,
                replay,
                FaultDeliveryAttempt::private_mutation(),
            )?;
        Ok(Self {
            physical,
            replay_basis: PhysicalIsolationMutationReplayBasis::from_replay(replay),
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

    pub fn required_rows(&self) -> &[PhysicalIsolationMutationKind] {
        self.physical.physical_isolation_mutations()
    }
}

impl PhysicalIsolationMutationReplayBasis {
    fn from_replay(replay: &SimulationReplayBundle) -> Self {
        Self {
            plan_identity: *replay.plan().identity().digest_bytes(),
            schedule_identity: *replay.schedule().identity().digest_bytes(),
            transcript_identity: *replay.transcript_identity().digest_bytes(),
            replay_basis_identity: *replay.replay_basis_identity().digest_bytes(),
        }
    }
}

pub fn physical_isolation_required_mutation_rows(
    family: PhysicalSimulationScenarioFamily,
) -> &'static [PhysicalIsolationMutationKind] {
    match family {
        PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock => &COMPACTION_ROWS,
        PhysicalSimulationScenarioFamily::PhysicalIsolationCheckpointPublicationInterlock => {
            &CHECKPOINT_ROWS
        }
        PhysicalSimulationScenarioFamily::PhysicalIsolationReclaimReachability => &RECLAIM_ROWS,
        PhysicalSimulationScenarioFamily::PhysicalIsolationTierMovementStability => &TIER_ROWS,
        PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability => {
            &FUTURE_CHUNK_ROWS
        }
        PhysicalSimulationScenarioFamily::PhysicalIsolationRestartDuringCutover => &RESTART_ROWS,
        _ => &[],
    }
}

const COMPACTION_ROWS: [PhysicalIsolationMutationKind; 6] = [
    PhysicalIsolationMutationKind::CompactionInPlaceOverwriteDenied,
    PhysicalIsolationMutationKind::CompactionEarlyReclaimDenied,
    PhysicalIsolationMutationKind::CompactionStaleEpochReuseDenied,
    PhysicalIsolationMutationKind::CompactionBackendResidueSelectionDenied,
    PhysicalIsolationMutationKind::CompactionLatchHierarchyInversionDenied,
    PhysicalIsolationMutationKind::CompactionMixedRootReadDenied,
];
const CHECKPOINT_ROWS: [PhysicalIsolationMutationKind; 1] =
    [PhysicalIsolationMutationKind::CheckpointMixedRootReadDenied];
const RECLAIM_ROWS: [PhysicalIsolationMutationKind; 1] =
    [PhysicalIsolationMutationKind::ReclaimEarlyReachabilityDenied];
const TIER_ROWS: [PhysicalIsolationMutationKind; 1] =
    [PhysicalIsolationMutationKind::TierMovementStabilityNonClaim];
const FUTURE_CHUNK_ROWS: [PhysicalIsolationMutationKind; 1] =
    [PhysicalIsolationMutationKind::FutureChunkStabilityNonClaim];
const RESTART_ROWS: [PhysicalIsolationMutationKind; 1] =
    [PhysicalIsolationMutationKind::RestartLatchHierarchyInversionDenied];
