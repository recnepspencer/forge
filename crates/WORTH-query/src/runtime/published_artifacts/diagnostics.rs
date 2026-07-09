use crate::memory_workspace::WorthQuerySnapshotIdentity;

use super::WorthQueryPublishedArtifactCounterSnapshot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPublishedArtifactDiagnostics {
    counters: WorthQueryPublishedArtifactCounterSnapshot,
    generations: Vec<WorthQueryPublishedArtifactGenerationDiagnostic>,
}

impl WorthQueryPublishedArtifactDiagnostics {
    pub(in crate::runtime) fn new(
        counters: WorthQueryPublishedArtifactCounterSnapshot,
        generations: Vec<WorthQueryPublishedArtifactGenerationDiagnostic>,
    ) -> Self {
        Self {
            counters,
            generations,
        }
    }

    pub fn counters(&self) -> WorthQueryPublishedArtifactCounterSnapshot {
        self.counters
    }

    pub fn generations(&self) -> &[WorthQueryPublishedArtifactGenerationDiagnostic] {
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
pub struct WorthQueryPublishedArtifactGenerationDiagnostic {
    ordinal: u64,
    snapshot_identity: WorthQuerySnapshotIdentity,
    artifact_count: usize,
}

impl WorthQueryPublishedArtifactGenerationDiagnostic {
    pub(in crate::runtime) fn new(
        ordinal: u64,
        snapshot_identity: WorthQuerySnapshotIdentity,
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

    pub fn snapshot_identity(&self) -> &WorthQuerySnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn artifact_count(&self) -> usize {
        self.artifact_count
    }
}
