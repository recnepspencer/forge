use crate::memory_workspace::ForgeQuerySnapshotIdentity;

use super::ForgeQueryPublishedArtifactCounterSnapshot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPublishedArtifactDiagnostics {
    counters: ForgeQueryPublishedArtifactCounterSnapshot,
    generations: Vec<ForgeQueryPublishedArtifactGenerationDiagnostic>,
}

impl ForgeQueryPublishedArtifactDiagnostics {
    pub(in crate::runtime) fn new(
        counters: ForgeQueryPublishedArtifactCounterSnapshot,
        generations: Vec<ForgeQueryPublishedArtifactGenerationDiagnostic>,
    ) -> Self {
        Self {
            counters,
            generations,
        }
    }

    pub fn counters(&self) -> ForgeQueryPublishedArtifactCounterSnapshot {
        self.counters
    }

    pub fn generations(&self) -> &[ForgeQueryPublishedArtifactGenerationDiagnostic] {
        &self.generations
    }

    pub fn retained_generation_count(&self) -> usize {
        self.generations.len()
    }

    pub fn contains_generation(&self, ordinal: u64) -> bool {
        self.generations
            .iter()
            .any(|generation| generation.ordinal == ordinal)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPublishedArtifactGenerationDiagnostic {
    ordinal: u64,
    snapshot_identity: ForgeQuerySnapshotIdentity,
    artifact_count: usize,
}

impl ForgeQueryPublishedArtifactGenerationDiagnostic {
    pub(in crate::runtime) fn new(
        ordinal: u64,
        snapshot_identity: ForgeQuerySnapshotIdentity,
        artifact_count: usize,
    ) -> Self {
        Self {
            ordinal,
            snapshot_identity,
            artifact_count,
        }
    }

    pub fn ordinal(&self) -> u64 {
        self.ordinal
    }

    pub fn snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn artifact_count(&self) -> usize {
        self.artifact_count
    }
}
