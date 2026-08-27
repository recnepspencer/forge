use crate::history::data::RelationalCommitReceipt;
use crate::publication::patch::data::PatchStreamPosition;

use super::HistoryAccess;

impl<'runtime> HistoryAccess<'runtime> {
    /// Resolve the immutable commit published at one exact canonical patch
    /// stream position.
    ///
    /// This lookup does not infer a branch head or catalog latest. It is for
    /// consumers that carry a performed publication position and need the
    /// corresponding commit identity without reopening branch currentness.
    pub fn immutable_commit_receipt_at_patch_stream_position(
        &self,
        position: PatchStreamPosition,
    ) -> Option<RelationalCommitReceipt> {
        self.runtime
            .history
            .canonical_envelope_at_patch(position)
            .map(|envelope| envelope.commit.clone())
    }
}
