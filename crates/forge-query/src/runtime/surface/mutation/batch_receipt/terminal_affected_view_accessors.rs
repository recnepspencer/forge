use crate::runtime::{ForgeQueryDerivedMaterializationTarget, ForgeQueryLiveArtifactTarget};

use super::{terminal_derived_view_ids, terminal_live_view_ids, ForgeQueryBatchWriteReceipt};

impl ForgeQueryBatchWriteReceipt {
    pub fn affected_live_view_targets(&self) -> &[ForgeQueryLiveArtifactTarget] {
        &self.affected_live_view_targets
    }

    pub fn affected_derived_view_targets(&self) -> &[ForgeQueryDerivedMaterializationTarget] {
        &self.affected_derived_view_targets
    }

    pub fn terminal_affected_live_view_ids_projection(&self) -> Vec<String> {
        terminal_live_view_ids(self.affected_live_view_targets())
    }

    pub fn terminal_affected_derived_view_ids_projection(&self) -> Vec<String> {
        terminal_derived_view_ids(self.affected_derived_view_targets())
    }
}
