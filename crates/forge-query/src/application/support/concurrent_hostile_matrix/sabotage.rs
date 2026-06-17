use super::artifact::ForgeQueryConcurrentHostileMatrixArtifact;
use super::posture::{
    classify_concurrent_hostile_matrix_posture, ForgeQueryConcurrentHostileMatrixPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryConcurrentHostileMatrixSabotageKind {
    CommittedReadHotPathLock,
    SharedReadMintRowClone,
    ReaderDerivedEvaluation,
    OrphanedSnapshotGeneration,
    UnretiredReadPin,
    JournalGap,
    ReplayResidue,
    DeliveryResidue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConcurrentHostileMatrixSabotage {
    kind: ForgeQueryConcurrentHostileMatrixSabotageKind,
    opened_counter_residue_count: usize,
    posture_after_sabotage: ForgeQueryConcurrentHostileMatrixPosture,
}

impl ForgeQueryConcurrentHostileMatrixSabotage {
    pub fn perturb(
        kind: ForgeQueryConcurrentHostileMatrixSabotageKind,
        artifact: &ForgeQueryConcurrentHostileMatrixArtifact,
    ) -> Self {
        let opened_counter_residue_count = artifact.counters().exact_zero_residue_count() + 1;
        let posture_after_sabotage = classify_concurrent_hostile_matrix_posture(
            artifact.topology().satisfies_phase_sixteen_minimums(),
            artifact.artifact_replay_equal(),
            artifact.repeated_run_equal(),
            opened_counter_residue_count,
            artifact
                .counters()
                .published_artifact_registry_lease_count(),
            artifact.sabotage_sensitive(),
        );
        Self {
            kind,
            opened_counter_residue_count,
            posture_after_sabotage,
        }
    }

    pub fn kind(&self) -> ForgeQueryConcurrentHostileMatrixSabotageKind {
        self.kind
    }

    pub fn opens_posture(&self) -> bool {
        self.posture_after_sabotage != ForgeQueryConcurrentHostileMatrixPosture::Closed
    }

    pub fn opened_counter_residue_count(&self) -> usize {
        self.opened_counter_residue_count
    }

    pub fn posture_after_sabotage(&self) -> ForgeQueryConcurrentHostileMatrixPosture {
        self.posture_after_sabotage
    }
}
