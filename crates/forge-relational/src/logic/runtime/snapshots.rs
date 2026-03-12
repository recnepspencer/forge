use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};

use super::{RelationalRuntime, SnapshotGuard, SnapshotHandleBinding};

pub struct SnapshotAccess<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl<'runtime> SnapshotAccess<'runtime> {
    fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn snapshot(&mut self) -> SnapshotHandle {
        let (handle, state) = self
            .runtime
            .snapshot_state_for_current(self.runtime.current_version_id());
        self.runtime.visibility.insert_active_handle(
            handle.snapshot_id,
            SnapshotHandleBinding {
                version_id: handle.version_id,
                read_policy: handle.read_policy,
            },
        );
        if self.runtime.config.visibility.cache_policy.protect_active_snapshots {
            self.runtime.insert_visibility_state(state.clone());
            self.runtime.bump_active_snapshot_ref(handle.version_id, 1);
        }
        handle
    }

    pub fn pin_snapshot(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<SnapshotGuard> {
        if self
            .runtime
            .read_or_reconstruct_visibility_state(version_id, false)
            .is_none()
        {
            return None;
        }
        let snapshot_id = self.runtime.visibility.allocate_snapshot_id();
        let state =
            self.runtime
                .build_visibility_state(version_id, snapshot_id, SnapshotReadPolicy::ImmutablePinned);
        self.runtime.visibility_pins().pin_snapshot_state(&state);
        let handle = state.handle.clone();
        self.runtime.visibility.insert_active_handle(
            handle.snapshot_id,
            SnapshotHandleBinding {
                version_id: handle.version_id,
                read_policy: handle.read_policy,
            },
        );
        if self.runtime.config.visibility.cache_policy.protect_active_snapshots {
            self.runtime.insert_visibility_state(state);
            self.runtime.bump_active_snapshot_ref(handle.version_id, 1);
        }
        Some(SnapshotGuard { handle })
    }

    pub fn release_snapshot(&mut self, handle: &SnapshotHandle) -> bool {
        if let Some(binding) = self.runtime.visibility.remove_active_handle(handle.snapshot_id) {
            let state = self
                .runtime
                .visibility_state_for_version(binding.version_id)
                .unwrap_or_else(|| {
                    self.runtime.build_visibility_state(
                        binding.version_id,
                        SnapshotId(0),
                        SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
                    )
                });
            self.runtime.visibility_pins().unpin_snapshot_state(&state);
            if self.runtime.config.visibility.cache_policy.protect_active_snapshots {
                self.runtime.bump_active_snapshot_ref(binding.version_id, -1);
            }
            self.runtime.evict_visibility_cache_if_needed();
            if self.runtime.config.storage.mvcc.snapshot_release_policy
                == crate::config::data::SnapshotReleasePolicy::ReleaseOnRetentionPass
            {
                let _ = self.runtime.retention_access().run_pass();
            }
            return true;
        }
        self.runtime
            .visibility
            .remove_published_handle(handle.snapshot_id)
            .is_some()
    }
}

impl RelationalRuntime {
    pub fn snapshot_access(&mut self) -> SnapshotAccess<'_> {
        SnapshotAccess::new(self)
    }
}
