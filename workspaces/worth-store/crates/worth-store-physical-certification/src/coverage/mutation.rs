use crate::{
    FaultDeliveryAttempt, FaultDeliveryDenial, PhysicalSimulationPlan,
    ShortcutRejectionObservationKind, SimulationReplayBundle,
};
use worth_store_physical_isolation::CompactionReadInterlockDenial;

use super::{CoverageGapDenial, HarnessCoverageStage, MutationValidationPosture};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalMutationCoverageEvidence {
    sequence: HarnessCoverageStage,
    plan_identity: [u8; 32],
    posture: MutationValidationPosture,
    denial: FaultDeliveryDenial,
    compaction_mutations: Vec<PhysicalIsolationCompactionMutationCoverageRow>,
    physical_isolation_mutations: Vec<PhysicalIsolationMutationKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalIsolationCompactionMutationKind {
    InPlaceOverwriteDenied,
    EarlyReclaimDenied,
    StaleEpochReuseDenied,
    BackendResidueCandidateSelectionDenied,
    LatchHierarchyInversionDenied,
    MixedRootReadDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalIsolationMutationKind {
    CompactionInPlaceOverwriteDenied,
    CompactionEarlyReclaimDenied,
    CompactionStaleEpochReuseDenied,
    CompactionBackendResidueSelectionDenied,
    CompactionLatchHierarchyInversionDenied,
    CompactionMixedRootReadDenied,
    CheckpointMixedRootReadDenied,
    ReclaimEarlyReachabilityDenied,
    TierMovementStabilityNonClaim,
    FutureChunkStabilityNonClaim,
    RestartLatchHierarchyInversionDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalIsolationCompactionMutationCoverageRow {
    kind: PhysicalIsolationCompactionMutationKind,
    denial: CompactionReadInterlockDenial,
}

impl PhysicalMutationCoverageEvidence {
    pub fn from_private_mutation_denial_plan(
        sequence: HarnessCoverageStage,
        plan: &PhysicalSimulationPlan,
        attempt: FaultDeliveryAttempt,
    ) -> Result<Self, CoverageGapDenial> {
        match attempt.admit() {
            Err(FaultDeliveryDenial::PrivateMutationDenied) => Ok(Self {
                sequence,
                plan_identity: *plan.identity().digest_bytes(),
                posture: MutationValidationPosture::ExpectedFailureObserved,
                denial: FaultDeliveryDenial::PrivateMutationDenied,
                compaction_mutations: Vec::new(),
                physical_isolation_mutations: Vec::new(),
            }),
            _ => Err(CoverageGapDenial::MissingMutationResult),
        }
    }

    pub fn from_replay_private_mutation_denial(
        sequence: HarnessCoverageStage,
        replay: &SimulationReplayBundle,
        attempt: FaultDeliveryAttempt,
    ) -> Result<Self, CoverageGapDenial> {
        if !replay
            .trace()
            .shortcut_rejections()
            .iter()
            .any(|observation| {
                observation.kind() == ShortcutRejectionObservationKind::PrivateMutationDenied
            })
        {
            return Err(CoverageGapDenial::MissingMutationResult);
        }
        match attempt.admit() {
            Err(FaultDeliveryDenial::PrivateMutationDenied) => Ok(Self {
                sequence,
                plan_identity: *replay.plan().identity().digest_bytes(),
                posture: MutationValidationPosture::ExpectedFailureObserved,
                denial: FaultDeliveryDenial::PrivateMutationDenied,
                compaction_mutations: Vec::new(),
                physical_isolation_mutations: Vec::new(),
            }),
            _ => Err(CoverageGapDenial::MissingMutationResult),
        }
    }

    pub fn from_replay_private_and_compaction_mutation_denials(
        sequence: HarnessCoverageStage,
        replay: &SimulationReplayBundle,
        attempt: FaultDeliveryAttempt,
    ) -> Result<Self, CoverageGapDenial> {
        let mut evidence = Self::from_replay_private_mutation_denial(sequence, replay, attempt)?;
        if replay.trace().compaction_interlock().is_none() {
            return Err(CoverageGapDenial::MissingMutationResult);
        }
        let mutations = replay
            .trace()
            .compaction_mutations()
            .ok_or(CoverageGapDenial::MissingMutationResult)?;
        if mutations.plan_identity() != replay.plan().identity().digest_bytes()
            || mutations.schedule_identity() != replay.schedule().identity().digest_bytes()
        {
            return Err(CoverageGapDenial::MissingMutationResult);
        }
        evidence.compaction_mutations = mutations.rows().to_vec();
        Ok(evidence)
    }

    pub fn from_replay_private_and_physical_isolation_denials(
        sequence: HarnessCoverageStage,
        replay: &SimulationReplayBundle,
        attempt: FaultDeliveryAttempt,
    ) -> Result<Self, CoverageGapDenial> {
        let family = replay.plan().scenario_family();
        let mut evidence = if requires_compaction_mutation_rows(family) {
            Self::from_replay_private_and_compaction_mutation_denials(sequence, replay, attempt)?
        } else {
            Self::from_replay_private_mutation_denial(sequence, replay, attempt)?
        };
        evidence.physical_isolation_mutations = physical_isolation_family_mutation_rows(
            family,
            replay,
            evidence.compaction_mutations(),
        )?;
        Ok(evidence)
    }

    pub const fn sequence(&self) -> HarnessCoverageStage {
        self.sequence
    }

    pub const fn plan_identity(&self) -> &[u8; 32] {
        &self.plan_identity
    }

    pub const fn posture(&self) -> MutationValidationPosture {
        self.posture
    }

    pub const fn denial(&self) -> &FaultDeliveryDenial {
        &self.denial
    }

    pub fn compaction_mutations(&self) -> &[PhysicalIsolationCompactionMutationCoverageRow] {
        &self.compaction_mutations
    }

    pub fn physical_isolation_mutations(&self) -> &[PhysicalIsolationMutationKind] {
        &self.physical_isolation_mutations
    }
}

impl PhysicalIsolationCompactionMutationCoverageRow {
    pub fn observed(
        kind: PhysicalIsolationCompactionMutationKind,
        denial: CompactionReadInterlockDenial,
    ) -> Result<Self, CoverageGapDenial> {
        if !kind.matches_denial(denial) {
            return Err(CoverageGapDenial::MissingMutationResult);
        }
        Ok(Self { kind, denial })
    }

    pub const fn kind(&self) -> PhysicalIsolationCompactionMutationKind {
        self.kind
    }

    pub const fn denial(&self) -> CompactionReadInterlockDenial {
        self.denial
    }
}

fn requires_compaction_mutation_rows(family: crate::PhysicalSimulationScenarioFamily) -> bool {
    matches!(
        family,
        crate::PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock
            | crate::PhysicalSimulationScenarioFamily::PhysicalIsolationCheckpointPublicationInterlock
            | crate::PhysicalSimulationScenarioFamily::PhysicalIsolationReclaimReachability
            | crate::PhysicalSimulationScenarioFamily::PhysicalIsolationTierMovementStability
            | crate::PhysicalSimulationScenarioFamily::PhysicalIsolationRestartDuringCutover
    )
}

fn physical_isolation_family_mutation_rows(
    family: crate::PhysicalSimulationScenarioFamily,
    replay: &SimulationReplayBundle,
    compaction: &[PhysicalIsolationCompactionMutationCoverageRow],
) -> Result<Vec<PhysicalIsolationMutationKind>, CoverageGapDenial> {
    match family {
        crate::PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock => {
            require_all_compaction_rows(compaction)
        }
        crate::PhysicalSimulationScenarioFamily::PhysicalIsolationCheckpointPublicationInterlock => {
            require_checkpoint_trace(replay)?;
            require_compaction_kind(compaction, PhysicalIsolationCompactionMutationKind::MixedRootReadDenied)?;
            Ok(vec![
                PhysicalIsolationMutationKind::CheckpointMixedRootReadDenied,
            ])
        }
        crate::PhysicalSimulationScenarioFamily::PhysicalIsolationReclaimReachability => {
            require_compaction_trace(replay)?;
            require_compaction_kind(compaction, PhysicalIsolationCompactionMutationKind::EarlyReclaimDenied)?;
            Ok(vec![
                PhysicalIsolationMutationKind::ReclaimEarlyReachabilityDenied,
            ])
        }
        crate::PhysicalSimulationScenarioFamily::PhysicalIsolationTierMovementStability => {
            require_independent_verifier_trace(replay)?;
            require_compaction_kind(
                compaction,
                PhysicalIsolationCompactionMutationKind::LatchHierarchyInversionDenied,
            )?;
            Ok(vec![
                PhysicalIsolationMutationKind::TierMovementStabilityNonClaim,
            ])
        }
        crate::PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability => {
            require_independent_verifier_trace(replay)?;
            if replay.trace().compaction_mutations().is_some() || !compaction.is_empty() {
                return Err(CoverageGapDenial::MissingMutationResult);
            }
            Ok(vec![
                PhysicalIsolationMutationKind::FutureChunkStabilityNonClaim,
            ])
        }
        crate::PhysicalSimulationScenarioFamily::PhysicalIsolationRestartDuringCutover => {
            require_checkpoint_trace(replay)?;
            require_compaction_kind(
                compaction,
                PhysicalIsolationCompactionMutationKind::LatchHierarchyInversionDenied,
            )?;
            Ok(vec![
                PhysicalIsolationMutationKind::RestartLatchHierarchyInversionDenied,
            ])
        }
        _ => Ok(Vec::new()),
    }
}

fn require_all_compaction_rows(
    compaction: &[PhysicalIsolationCompactionMutationCoverageRow],
) -> Result<Vec<PhysicalIsolationMutationKind>, CoverageGapDenial> {
    let mut rows = Vec::new();
    for required in PhysicalIsolationCompactionMutationKind::REQUIRED_FOR_S5_INTERLEAVING {
        require_compaction_kind(compaction, required)?;
        rows.push(PhysicalIsolationMutationKind::from(required));
    }
    Ok(rows)
}

fn require_compaction_kind(
    compaction: &[PhysicalIsolationCompactionMutationCoverageRow],
    required: PhysicalIsolationCompactionMutationKind,
) -> Result<(), CoverageGapDenial> {
    if compaction.iter().any(|row| row.kind() == required) {
        Ok(())
    } else {
        Err(CoverageGapDenial::MissingMutationResult)
    }
}

fn require_checkpoint_trace(replay: &SimulationReplayBundle) -> Result<(), CoverageGapDenial> {
    replay
        .trace()
        .checkpoint_interlock()
        .map(|_| ())
        .ok_or(CoverageGapDenial::MissingMutationResult)
}

fn require_compaction_trace(replay: &SimulationReplayBundle) -> Result<(), CoverageGapDenial> {
    replay
        .trace()
        .compaction_interlock()
        .map(|_| ())
        .ok_or(CoverageGapDenial::MissingMutationResult)
}

fn require_independent_verifier_trace(
    replay: &SimulationReplayBundle,
) -> Result<(), CoverageGapDenial> {
    replay
        .trace()
        .independent_verifier()
        .map(|_| ())
        .ok_or(CoverageGapDenial::MissingMutationResult)
}

impl From<PhysicalIsolationCompactionMutationKind> for PhysicalIsolationMutationKind {
    fn from(kind: PhysicalIsolationCompactionMutationKind) -> Self {
        match kind {
            PhysicalIsolationCompactionMutationKind::InPlaceOverwriteDenied => {
                Self::CompactionInPlaceOverwriteDenied
            }
            PhysicalIsolationCompactionMutationKind::EarlyReclaimDenied => {
                Self::CompactionEarlyReclaimDenied
            }
            PhysicalIsolationCompactionMutationKind::StaleEpochReuseDenied => {
                Self::CompactionStaleEpochReuseDenied
            }
            PhysicalIsolationCompactionMutationKind::BackendResidueCandidateSelectionDenied => {
                Self::CompactionBackendResidueSelectionDenied
            }
            PhysicalIsolationCompactionMutationKind::LatchHierarchyInversionDenied => {
                Self::CompactionLatchHierarchyInversionDenied
            }
            PhysicalIsolationCompactionMutationKind::MixedRootReadDenied => {
                Self::CompactionMixedRootReadDenied
            }
        }
    }
}

impl PhysicalIsolationCompactionMutationKind {
    pub const REQUIRED_FOR_PHASE8: [Self; 4] = [
        Self::InPlaceOverwriteDenied,
        Self::EarlyReclaimDenied,
        Self::StaleEpochReuseDenied,
        Self::BackendResidueCandidateSelectionDenied,
    ];

    pub const REQUIRED_FOR_S5_INTERLEAVING: [Self; 6] = [
        Self::InPlaceOverwriteDenied,
        Self::EarlyReclaimDenied,
        Self::StaleEpochReuseDenied,
        Self::BackendResidueCandidateSelectionDenied,
        Self::LatchHierarchyInversionDenied,
        Self::MixedRootReadDenied,
    ];

    const fn matches_denial(self, denial: CompactionReadInterlockDenial) -> bool {
        matches!(
            (self, denial),
            (
                Self::InPlaceOverwriteDenied,
                CompactionReadInterlockDenial::InPlaceOverwriteOfProtectedStructure
            ) | (
                Self::EarlyReclaimDenied,
                CompactionReadInterlockDenial::EarlyReclaimBeforeReadRelease { .. }
            ) | (
                Self::StaleEpochReuseDenied,
                CompactionReadInterlockDenial::StaleCompactionSourceEpoch { .. }
            ) | (
                Self::StaleEpochReuseDenied,
                CompactionReadInterlockDenial::StaleEpochReuse { .. }
            ) | (
                Self::BackendResidueCandidateSelectionDenied,
                CompactionReadInterlockDenial::BackendResidueCandidateSelection(_)
            ) | (
                Self::LatchHierarchyInversionDenied,
                CompactionReadInterlockDenial::LatchAcquisition(
                    worth_store_physical_isolation::LatchAcquisitionDenial::HierarchyInversion
                )
            ) | (
                Self::MixedRootReadDenied,
                CompactionReadInterlockDenial::MixedRootDuringCompaction
            )
        )
    }
}
