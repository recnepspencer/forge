use crate::capabilities::VisibilityPolicySource;
use crate::logic::runtime::{RelationalRuntime, SnapshotGuard, SnapshotHandleBinding};
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::visibility::cache_state::{
    bump_active_snapshot_ref, cached_state_for_version, evict_cache_if_needed, insert_state,
    reconstruct_state, residency_for_version,
};
use crate::visibility::snapshot_states::build_visibility_state;

pub struct VisibilityAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl RelationalRuntime {
    pub(crate) fn visibility_authority(&mut self) -> VisibilityAuthority<'_> {
        VisibilityAuthority::new(self)
    }
}

impl<'runtime> VisibilityAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    fn open_active_snapshot(
        &mut self,
        version_id: crate::identity::data::VersionId,
        read_policy: SnapshotReadPolicy,
    ) -> SnapshotHandle {
        let snapshot_id = self.runtime.visibility.allocate_snapshot_id();
        let handle = SnapshotHandle {
            runtime_instance_id: self.runtime.runtime_instance_id(),
            snapshot_id,
            version_id,
            read_policy,
        };
        let first_active_snapshot =
            residency_for_version(self.runtime, version_id).active_snapshot_refs == 0;
        if first_active_snapshot {
            let state = cached_state_for_version(self.runtime, version_id).unwrap_or_else(|| {
                build_visibility_state(self.runtime, version_id, snapshot_id, read_policy)
            });
            self.runtime.visibility_pins().pin_snapshot_state(&state);
            if self.runtime.protect_active_snapshots() {
                insert_state(self.runtime, state);
            }
        }
        self.runtime.visibility.insert_active_handle(
            handle.snapshot_id,
            SnapshotHandleBinding {
                version_id: handle.version_id,
                read_policy: handle.read_policy,
            },
        );
        bump_active_snapshot_ref(self.runtime, handle.version_id, 1);
        handle
    }

    pub fn snapshot(&mut self) -> SnapshotHandle {
        self.open_active_snapshot(
            self.runtime.current_version_id(),
            SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        )
    }

    pub fn pin_snapshot(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<SnapshotGuard> {
        if reconstruct_state(self.runtime, version_id, false).is_none() {
            return None;
        }
        let handle = self.open_active_snapshot(version_id, SnapshotReadPolicy::ImmutablePinned);
        Some(SnapshotGuard::new(handle))
    }

    pub fn release_snapshot(&mut self, handle: &SnapshotHandle) -> bool {
        if let Some(binding) = self
            .runtime
            .visibility
            .remove_active_handle(handle.snapshot_id)
        {
            let last_active_snapshot =
                residency_for_version(self.runtime, binding.version_id).active_snapshot_refs <= 1;
            if last_active_snapshot {
                let state = cached_state_for_version(self.runtime, binding.version_id)
                    .unwrap_or_else(|| {
                        build_visibility_state(
                            self.runtime,
                            binding.version_id,
                            SnapshotId(0),
                            SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
                        )
                    });
                self.runtime.visibility_pins().unpin_snapshot_state(&state);
            }
            bump_active_snapshot_ref(self.runtime, binding.version_id, -1);
            evict_cache_if_needed(self.runtime);
            if self.runtime.config.storage.mvcc.snapshot_release_policy
                == crate::config::data::SnapshotReleasePolicy::ReleaseOnRetentionPass
            {
                let _ = self.runtime.retention().run_pass();
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
