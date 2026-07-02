use crate::{
    FaultDeliveryAttempt, FaultDeliveryDenial, ShortcutRejectionObservationKind,
    SimulationReplayBundle,
};
use forge_store_physical_isolation::CompactionReadInterlockDenial;

use super::{CoverageGapDenial, MutationValidationPosture, Roadmap2HarnessSequence};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalMutationCoverageEvidence {
    sequence: Roadmap2HarnessSequence,
    plan_identity: [u8; 32],
    posture: MutationValidationPosture,
    denial: FaultDeliveryDenial,
    compaction_mutations: Vec<S5CompactionMutationCoverageRow>,
    s5_physical_isolation_mutations: Vec<S5PhysicalIsolationMutationKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S5CompactionMutationKind {
    InPlaceOverwriteDenied,
    EarlyReclaimDenied,
    StaleEpochReuseDenied,
    BackendResidueCandidateSelectionDenied,
    LatchHierarchyInversionDenied,
    MixedRootReadDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S5PhysicalIsolationMutationKind {
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
pub struct S5CompactionMutationCoverageRow {
    kind: S5CompactionMutationKind,
    denial: CompactionReadInterlockDenial,
}

impl PhysicalMutationCoverageEvidence {
    pub fn from_replay_private_mutation_denial(
        sequence: Roadmap2HarnessSequence,
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
                s5_physical_isolation_mutations: Vec::new(),
            }),
            _ => Err(CoverageGapDenial::MissingMutationResult),
        }
    }

    pub fn from_replay_private_and_compaction_mutation_denials(
        sequence: Roadmap2HarnessSequence,
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

    pub fn from_replay_private_and_s5_physical_isolation_denials(
        sequence: Roadmap2HarnessSequence,
        replay: &SimulationReplayBundle,
        attempt: FaultDeliveryAttempt,
    ) -> Result<Self, CoverageGapDenial> {
        let family = replay.plan().scenario_family();
        let mut evidence = if requires_compaction_mutation_rows(family) {
            Self::from_replay_private_and_compaction_mutation_denials(sequence, replay, attempt)?
        } else {
            Self::from_replay_private_mutation_denial(sequence, replay, attempt)?
        };
        evidence.s5_physical_isolation_mutations =
            s5_family_mutation_rows(family, replay, evidence.compaction_mutations())?;
        Ok(evidence)
    }

    pub const fn sequence(&self) -> Roadmap2HarnessSequence {
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

    pub fn compaction_mutations(&self) -> &[S5CompactionMutationCoverageRow] {
        &self.compaction_mutations
    }

    pub fn s5_physical_isolation_mutations(&self) -> &[S5PhysicalIsolationMutationKind] {
        &self.s5_physical_isolation_mutations
    }
}

impl S5CompactionMutationCoverageRow {
    pub fn observed(
        kind: S5CompactionMutationKind,
        denial: CompactionReadInterlockDenial,
    ) -> Result<Self, CoverageGapDenial> {
        if !kind.matches_denial(denial) {
            return Err(CoverageGapDenial::MissingMutationResult);
        }
        Ok(Self { kind, denial })
    }

    pub const fn kind(&self) -> S5CompactionMutationKind {
        self.kind
    }

    pub const fn denial(&self) -> CompactionReadInterlockDenial {
        self.denial
    }
}

fn requires_compaction_mutation_rows(family: crate::PhysicalSimulationScenarioFamily) -> bool {
    matches!(
        family,
        crate::PhysicalSimulationScenarioFamily::S5CompactionInterlock
            | crate::PhysicalSimulationScenarioFamily::S5CheckpointPublicationInterlock
            | crate::PhysicalSimulationScenarioFamily::S5ReclaimReachability
            | crate::PhysicalSimulationScenarioFamily::S5TierMovementStability
            | crate::PhysicalSimulationScenarioFamily::S5RestartDuringCutover
    )
}

fn s5_family_mutation_rows(
    family: crate::PhysicalSimulationScenarioFamily,
    replay: &SimulationReplayBundle,
    compaction: &[S5CompactionMutationCoverageRow],
) -> Result<Vec<S5PhysicalIsolationMutationKind>, CoverageGapDenial> {
    match family {
        crate::PhysicalSimulationScenarioFamily::S5CompactionInterlock => {
            require_all_compaction_rows(compaction)
        }
        crate::PhysicalSimulationScenarioFamily::S5CheckpointPublicationInterlock => {
            require_checkpoint_trace(replay)?;
            require_compaction_kind(compaction, S5CompactionMutationKind::MixedRootReadDenied)?;
            Ok(vec![
                S5PhysicalIsolationMutationKind::CheckpointMixedRootReadDenied,
            ])
        }
        crate::PhysicalSimulationScenarioFamily::S5ReclaimReachability => {
            require_compaction_trace(replay)?;
            require_compaction_kind(compaction, S5CompactionMutationKind::EarlyReclaimDenied)?;
            Ok(vec![
                S5PhysicalIsolationMutationKind::ReclaimEarlyReachabilityDenied,
            ])
        }
        crate::PhysicalSimulationScenarioFamily::S5TierMovementStability => {
            require_independent_verifier_trace(replay)?;
            require_compaction_kind(
                compaction,
                S5CompactionMutationKind::LatchHierarchyInversionDenied,
            )?;
            Ok(vec![
                S5PhysicalIsolationMutationKind::TierMovementStabilityNonClaim,
            ])
        }
        crate::PhysicalSimulationScenarioFamily::S5FutureChunkStability => {
            require_independent_verifier_trace(replay)?;
            if replay.trace().compaction_mutations().is_some() || !compaction.is_empty() {
                return Err(CoverageGapDenial::MissingMutationResult);
            }
            Ok(vec![
                S5PhysicalIsolationMutationKind::FutureChunkStabilityNonClaim,
            ])
        }
        crate::PhysicalSimulationScenarioFamily::S5RestartDuringCutover => {
            require_checkpoint_trace(replay)?;
            require_compaction_kind(
                compaction,
                S5CompactionMutationKind::LatchHierarchyInversionDenied,
            )?;
            Ok(vec![
                S5PhysicalIsolationMutationKind::RestartLatchHierarchyInversionDenied,
            ])
        }
        _ => Ok(Vec::new()),
    }
}

fn require_all_compaction_rows(
    compaction: &[S5CompactionMutationCoverageRow],
) -> Result<Vec<S5PhysicalIsolationMutationKind>, CoverageGapDenial> {
    let mut rows = Vec::new();
    for required in S5CompactionMutationKind::REQUIRED_FOR_S5_INTERLEAVING {
        require_compaction_kind(compaction, required)?;
        rows.push(S5PhysicalIsolationMutationKind::from(required));
    }
    Ok(rows)
}

fn require_compaction_kind(
    compaction: &[S5CompactionMutationCoverageRow],
    required: S5CompactionMutationKind,
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

impl From<S5CompactionMutationKind> for S5PhysicalIsolationMutationKind {
    fn from(kind: S5CompactionMutationKind) -> Self {
        match kind {
            S5CompactionMutationKind::InPlaceOverwriteDenied => {
                Self::CompactionInPlaceOverwriteDenied
            }
            S5CompactionMutationKind::EarlyReclaimDenied => Self::CompactionEarlyReclaimDenied,
            S5CompactionMutationKind::StaleEpochReuseDenied => {
                Self::CompactionStaleEpochReuseDenied
            }
            S5CompactionMutationKind::BackendResidueCandidateSelectionDenied => {
                Self::CompactionBackendResidueSelectionDenied
            }
            S5CompactionMutationKind::LatchHierarchyInversionDenied => {
                Self::CompactionLatchHierarchyInversionDenied
            }
            S5CompactionMutationKind::MixedRootReadDenied => Self::CompactionMixedRootReadDenied,
        }
    }
}

impl S5CompactionMutationKind {
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
                    forge_store_physical_isolation::LatchAcquisitionDenial::HierarchyInversion
                )
            ) | (
                Self::MixedRootReadDenied,
                CompactionReadInterlockDenial::MixedRootDuringCompaction
            )
        )
    }
}
