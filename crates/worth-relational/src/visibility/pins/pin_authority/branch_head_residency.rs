use crate::capabilities::VisibilityPolicySource;
use crate::visibility::cache_state::{
    bump_visibility_ref, evict_cache_if_needed, maybe_remove_unprotected_state,
    protect_branch_head_version,
};

use super::*;

impl<'runtime> VisibilityPinAuthority<'runtime> {
    pub(crate) fn rebuild_branch_head_visibility_residency(&mut self) {
        let tracked_versions = self.runtime.visibility.tracked_branch_head_versions();
        self.runtime
            .visibility
            .clear_branch_head_residency(&tracked_versions);
        for version_id in tracked_versions {
            maybe_remove_unprotected_state(self.runtime, version_id);
        }
        if !self.runtime.protect_branch_heads() {
            evict_cache_if_needed(self.runtime);
            return;
        }
        let head_versions = self.runtime.history().branch_head_versions();
        for version_id in head_versions {
            protect_branch_head_version(self.runtime, version_id);
            self.runtime.services.instrumentation.count(|counters| {
                counters.visibility_cache_branch_head_promotions += 1;
            });
        }
        evict_cache_if_needed(self.runtime);
    }

    pub(crate) fn move_branch_head_visibility_residency(
        &mut self,
        previous_head: Option<crate::identity::data::VersionId>,
        next_head: Option<crate::identity::data::VersionId>,
    ) {
        if !self.runtime.protect_branch_heads() || previous_head == next_head {
            return;
        }
        if let Some(version_id) = previous_head {
            bump_visibility_ref(self.runtime, version_id, |residency| {
                residency.branch_head_refs = residency.branch_head_refs.saturating_sub(1);
            });
        }
        if let Some(version_id) = next_head {
            protect_branch_head_version(self.runtime, version_id);
            self.runtime.services.instrumentation.count(|counters| {
                counters.visibility_cache_branch_head_promotions += 1;
            });
        }
        evict_cache_if_needed(self.runtime);
    }
}
