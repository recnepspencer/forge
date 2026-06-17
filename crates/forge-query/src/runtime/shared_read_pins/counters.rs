use crate::runtime::published_artifacts::ForgeQueryPublishedArtifactCounterSnapshot;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ForgeQuerySharedReadCounters {
    committed_read_hot_path_lock_count: usize,
    orphaned_generation_count: usize,
    unretired_pin_count: usize,
    shared_read_mint_row_clone_count: usize,
    published_artifact_registry_lease_count: usize,
    reader_derived_evaluation_count: usize,
    published_artifact_dropped_generation_count: usize,
}

impl ForgeQuerySharedReadCounters {
    pub(in crate::runtime) fn new(
        committed_read_hot_path_lock_count: usize,
        orphaned_generation_count: usize,
        unretired_pin_count: usize,
    ) -> Self {
        Self {
            committed_read_hot_path_lock_count,
            orphaned_generation_count,
            unretired_pin_count,
            shared_read_mint_row_clone_count: 0,
            published_artifact_registry_lease_count: 0,
            reader_derived_evaluation_count: 0,
            published_artifact_dropped_generation_count: 0,
        }
    }

    pub(in crate::runtime) fn with_published_artifacts(
        self,
        published: ForgeQueryPublishedArtifactCounterSnapshot,
    ) -> Self {
        Self {
            shared_read_mint_row_clone_count: published.shared_read_mint_row_clone_count(),
            published_artifact_registry_lease_count: published
                .published_artifact_registry_lease_count(),
            reader_derived_evaluation_count: published.reader_derived_evaluation_count(),
            published_artifact_dropped_generation_count: published.dropped_generation_count(),
            ..self
        }
    }

    #[allow(dead_code)]
    pub fn committed_read_hot_path_lock_count(self) -> usize {
        self.committed_read_hot_path_lock_count
    }

    #[allow(dead_code)]
    pub fn orphaned_generation_count(self) -> usize {
        self.orphaned_generation_count
    }

    #[allow(dead_code)]
    pub fn unretired_pin_count(self) -> usize {
        self.unretired_pin_count
    }

    #[allow(dead_code)]
    pub fn shared_read_mint_row_clone_count(self) -> usize {
        self.shared_read_mint_row_clone_count
    }

    #[allow(dead_code)]
    pub fn published_artifact_registry_lease_count(self) -> usize {
        self.published_artifact_registry_lease_count
    }

    #[allow(dead_code)]
    pub fn reader_derived_evaluation_count(self) -> usize {
        self.reader_derived_evaluation_count
    }

    #[allow(dead_code)]
    pub fn published_artifact_dropped_generation_count(self) -> usize {
        self.published_artifact_dropped_generation_count
    }
}
