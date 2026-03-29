use crate::logic::runtime::{RelationalRuntime, SnapshotGuard, SnapshotHandleBinding};
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::visibility::cache_state::{
    bump_active_snapshot_ref, cached_state_for_version, evict_cache_if_needed, insert_state,
    reconstruct_state,
};
use crate::visibility::snapshot_states::build_visibility_state;

pub struct VisibilityAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl RelationalRuntime {
    pub fn visibility_authority(&mut self) -> VisibilityAuthority<'_> {
        VisibilityAuthority::new(self)
    }
}

impl<'runtime> VisibilityAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    fn snapshot_state_for_current(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> (SnapshotHandle, crate::storage::logic::state::SnapshotState) {
        let snapshot_id = self.runtime.visibility.allocate_snapshot_id();
        let state = build_visibility_state(
            self.runtime,
            version_id,
            snapshot_id,
            SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        );
        self.runtime.visibility_pins().pin_snapshot_state(&state);
        (state.handle.clone(), state)
    }

    pub fn snapshot(&mut self) -> SnapshotHandle {
        let (handle, state) = self.snapshot_state_for_current(self.runtime.current_version_id());
        self.runtime.visibility.insert_active_handle(
            handle.snapshot_id,
            SnapshotHandleBinding {
                version_id: handle.version_id,
                read_policy: handle.read_policy,
            },
        );
        if self
            .runtime
            .config
            .visibility
            .cache_policy
            .protect_active_snapshots
        {
            insert_state(self.runtime, state.clone());
            bump_active_snapshot_ref(self.runtime, handle.version_id, 1);
        }
        handle
    }

    pub fn pin_snapshot(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<SnapshotGuard> {
        if reconstruct_state(self.runtime, version_id, false).is_none() {
            return None;
        }
        let snapshot_id = self.runtime.visibility.allocate_snapshot_id();
        let state = build_visibility_state(
            self.runtime,
            version_id,
            snapshot_id,
            SnapshotReadPolicy::ImmutablePinned,
        );
        self.runtime.visibility_pins().pin_snapshot_state(&state);
        let handle = state.handle.clone();
        self.runtime.visibility.insert_active_handle(
            handle.snapshot_id,
            SnapshotHandleBinding {
                version_id: handle.version_id,
                read_policy: handle.read_policy,
            },
        );
        if self
            .runtime
            .config
            .visibility
            .cache_policy
            .protect_active_snapshots
        {
            insert_state(self.runtime, state);
            bump_active_snapshot_ref(self.runtime, handle.version_id, 1);
        }
        Some(SnapshotGuard::new(handle))
    }

    pub fn release_snapshot(&mut self, handle: &SnapshotHandle) -> bool {
        if let Some(binding) = self
            .runtime
            .visibility
            .remove_active_handle(handle.snapshot_id)
        {
            let state =
                cached_state_for_version(self.runtime, binding.version_id).unwrap_or_else(|| {
                    build_visibility_state(
                        self.runtime,
                        binding.version_id,
                        SnapshotId(0),
                        SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
                    )
                });
            self.runtime.visibility_pins().unpin_snapshot_state(&state);
            if self
                .runtime
                .config
                .visibility
                .cache_policy
                .protect_active_snapshots
            {
                bump_active_snapshot_ref(self.runtime, binding.version_id, -1);
            }
            evict_cache_if_needed(self.runtime);
            if self.runtime.config.storage.mvcc.snapshot_release_policy
                == crate::config::data::SnapshotReleasePolicy::ReleaseOnRetentionPass
            {
                let _ = self.runtime.retention_authority().run_pass();
            }
            return true;
        }
        if self
            .runtime
            .visibility
            .remove_published_handle(handle.snapshot_id)
            .is_some()
        {
            true
        } else {
            false
        }
    }
}
