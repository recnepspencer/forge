use crate::runtime::{WorthQueryDerivedMaterializationTarget, WorthQueryLiveArtifactTarget};

use super::{terminal_derived_view_ids, terminal_live_view_ids, WorthQueryBatchWriteReceipt};

impl WorthQueryBatchWriteReceipt {
    pub fn affected_live_view_targets(&self) -> &[WorthQueryLiveArtifactTarget] {
        &self.affected_live_view_targets
    }

    pub fn affected_derived_view_targets(&self) -> &[WorthQueryDerivedMaterializationTarget] {
        &self.affected_derived_view_targets
    }

    pub fn terminal_affected_live_view_ids_projection(&self) -> Vec<String> {
        terminal_live_view_ids(self.affected_live_view_targets())
    }

    pub fn terminal_affected_derived_view_ids_projection(&self) -> Vec<String> {
        terminal_derived_view_ids(self.affected_derived_view_targets())
    }
}
