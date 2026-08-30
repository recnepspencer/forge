use std::sync::Arc;

use worth_store_physical_integrity::VerifiedCheckpointStream;

use super::SelectedPhysicalRoot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalCheckpointBase {
    checkpoint: Arc<VerifiedCheckpointStream>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalCheckpointBaseDenial {
    ForeignStore,
    RootGenerationMismatch,
    RootTreeMismatch,
    CompactionCutoffOutsideCheckpoint,
}

impl PhysicalCheckpointBase {
    pub fn admit(
        root: &SelectedPhysicalRoot,
        checkpoint: VerifiedCheckpointStream,
    ) -> Result<Self, PhysicalCheckpointBaseDenial> {
        let source = checkpoint.source();
        let selected = root.selected();
        if source.identity().store_identity() != selected.selector().store_identity() {
            return Err(PhysicalCheckpointBaseDenial::ForeignStore);
        }
        let matching_root = std::iter::once(selected)
            .chain(root.retained_previous())
            .find(|candidate| source.root().generation() == candidate.manifest().generation());
        let Some(matching_root) = matching_root else {
            return Err(PhysicalCheckpointBaseDenial::RootGenerationMismatch);
        };
        if source.root().tree_identity() != matching_root.manifest().tree_identity() {
            return Err(PhysicalCheckpointBaseDenial::RootTreeMismatch);
        }
        let cutoff = checkpoint.compaction_cutover().wal_cutoff_lsn_exclusive();
        if cutoff < source.wal().admitted_begin_lsn()
            || cutoff > source.wal().covered_end_lsn_exclusive()
        {
            return Err(PhysicalCheckpointBaseDenial::CompactionCutoffOutsideCheckpoint);
        }
        Ok(Self {
            checkpoint: Arc::new(checkpoint),
        })
    }

    pub fn checkpoint(&self) -> &VerifiedCheckpointStream {
        self.checkpoint.as_ref()
    }

    /// Shares the admitted checkpoint with a later recovery owner that must
    /// retain it beyond this borrow. The ordinary observation accessor stays
    /// representation-agnostic; this method makes the ownership transfer and
    /// its reference-counting cost explicit at the call site.
    pub fn share_checkpoint(&self) -> Arc<VerifiedCheckpointStream> {
        Arc::clone(&self.checkpoint)
    }

    pub fn wal_tail_begin_lsn(&self) -> u64 {
        self.checkpoint.source().wal().covered_end_lsn_exclusive()
    }
}
