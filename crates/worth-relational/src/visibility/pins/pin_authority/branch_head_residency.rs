use crate::capabilities::VisibilityPolicySource;
use crate::visibility::cache_state::{
    bump_visibility_ref, evict_cache_if_needed, maybe_remove_unprotected_state,
    protect_branch_head_state,
};
use crate::visibility::snapshot_states::VisibilitySnapshotBasis;

use super::*;

impl<'runtime> VisibilityPinAuthority<'runtime> {
    pub(crate) fn rebuild_branch_head_visibility_residency(&mut self) {
        let tracked_states = self.runtime.visibility.tracked_branch_head_states();
        self.runtime
            .visibility
            .clear_branch_head_residency(&tracked_states);
        for key in &tracked_states {
            maybe_remove_unprotected_state(self.runtime, key);
        }
        if !self.runtime.protect_branch_heads() {
            evict_cache_if_needed(self.runtime);
            return;
        }
        let branch_ids = self.runtime.history.branch_ids_snapshot();
        for branch_id in branch_ids {
            let Some(version_id) = self
                .runtime
                .history()
                .branch_head(&branch_id)
                .map(|head| head.version_id)
            else {
                continue;
            };
            let Some(basis) =
                VisibilitySnapshotBasis::capture_current(self.runtime, &branch_id, version_id)
            else {
                continue;
            };
            protect_branch_head_state(self.runtime, &basis);
            self.runtime.services.instrumentation.count(|counters| {
                counters.visibility_cache_branch_head_promotions += 1;
            });
        }
        evict_cache_if_needed(self.runtime);
    }

    pub(crate) fn move_branch_head_visibility_residency(
        &mut self,
        branch_id: &crate::history::data::BranchId,
        previous_head: Option<crate::identity::data::VersionId>,
        next_head: Option<crate::identity::data::VersionId>,
    ) {
        if !self.runtime.protect_branch_heads() || previous_head == next_head {
            return;
        }
        if let Some(version_id) = previous_head {
            let previous_key = self
                .runtime
                .visibility
                .tracked_branch_head_states()
                .into_iter()
                .find(|key| key.branch_id() == branch_id && key.version_id() == version_id);
            if let Some(previous_key) = previous_key {
                bump_visibility_ref(self.runtime, &previous_key, |residency| {
                    residency.branch_head_refs = residency.branch_head_refs.saturating_sub(1);
                });
            }
        }
        if let Some(version_id) = next_head {
            if let Some(basis) =
                VisibilitySnapshotBasis::capture_current(self.runtime, branch_id, version_id)
            {
                protect_branch_head_state(self.runtime, &basis);
            }
            self.runtime.services.instrumentation.count(|counters| {
                counters.visibility_cache_branch_head_promotions += 1;
            });
        }
        evict_cache_if_needed(self.runtime);
    }
}
