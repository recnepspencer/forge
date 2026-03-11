use crate::query::data::QueryWorkPacket;
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};

use super::{PacketResult, RelationalReadView, RelationalRuntime, SnapshotGuard, SnapshotHandleBinding};

impl RelationalRuntime {
    pub fn snapshot(&mut self) -> SnapshotHandle {
        let (handle, state) = self.snapshot_state_for_current(self.current_version_id());
        self.snapshots.active.insert(
            handle.snapshot_id,
            SnapshotHandleBinding {
                version_id: handle.version_id,
                read_policy: handle.read_policy,
            },
        );
        if self.config.visibility_cache_policy.protect_active_snapshots {
            self.insert_visibility_state(state.clone());
            self.bump_active_snapshot_ref(handle.version_id, 1);
        }
        handle
    }

    pub fn pin_snapshot(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<SnapshotGuard<'_>> {
        if self.read_or_reconstruct_visibility_state(version_id, false).is_none() {
            return None;
        }
        let snapshot_id = SnapshotId(self.snapshots.next_snapshot_id);
        self.snapshots.next_snapshot_id += 1;
        let state = self.build_visibility_state(
            version_id,
            snapshot_id,
            SnapshotReadPolicy::ImmutablePinned,
        );
        self.pin_snapshot_state(&state);
        let handle = state.handle.clone();
        self.snapshots.active.insert(
            handle.snapshot_id,
            SnapshotHandleBinding {
                version_id: handle.version_id,
                read_policy: handle.read_policy,
            },
        );
        if self.config.visibility_cache_policy.protect_active_snapshots {
            self.insert_visibility_state(state);
            self.bump_active_snapshot_ref(handle.version_id, 1);
        }
        Some(SnapshotGuard {
            runtime: self,
            handle,
        })
    }

    pub fn release_snapshot(&mut self, handle: &SnapshotHandle) -> bool {
        if let Some(binding) = self.snapshots.active.remove(&handle.snapshot_id) {
            let state = self
                .visibility_state_for_version(binding.version_id)
                .unwrap_or_else(|| {
                    self.build_visibility_state(
                        binding.version_id,
                        SnapshotId(0),
                        SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
                    )
                });
            self.unpin_snapshot_state(&state);
            if self.config.visibility_cache_policy.protect_active_snapshots {
                self.bump_active_snapshot_ref(binding.version_id, -1);
            }
            self.evict_visibility_cache_if_needed();
            if self.config.mvcc.snapshot_release_policy
                == crate::config::data::SnapshotReleasePolicy::ReleaseOnRetentionPass
            {
                self.run_retention_pass();
            }
            return true;
        }
        self.snapshots
            .published_handles
            .remove(&handle.snapshot_id)
            .is_some()
    }

    pub fn read_snapshot(&self, handle: &SnapshotHandle) -> Option<RelationalReadView> {
        if let Some(binding) = self.snapshots.active.get(&handle.snapshot_id) {
            let state = self.read_or_reconstruct_visibility_state(
                binding.version_id,
                !self.config.visibility_cache_policy.protect_active_snapshots,
            )?;
            let mut read_view = self.read_from_snapshot_state(&state);
            read_view.snapshot = SnapshotHandle {
                snapshot_id: handle.snapshot_id,
                version_id: binding.version_id,
                read_policy: binding.read_policy,
            };
            return Some(read_view);
        }
        let version_id = *self.snapshots.published_handles.get(&handle.snapshot_id)?;
        let mut read_view = self.read_version(version_id);
        read_view.snapshot = handle.clone();
        Some(read_view)
    }

    pub fn read_version(&self, version_id: crate::identity::data::VersionId) -> RelationalReadView {
        let state = self
            .read_or_reconstruct_visibility_state(version_id, true)
            .unwrap_or_else(|| {
                self.build_visibility_state(
                    version_id,
                    SnapshotId(0),
                    SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
                )
            });
        self.read_from_snapshot_state(&state)
    }

    pub fn execute_read_packet(
        &self,
        handle: &SnapshotHandle,
        packet: &QueryWorkPacket,
    ) -> Option<PacketResult> {
        self.read_snapshot(handle)
            .map(|read_view| read_view.execute_packet(packet))
    }
}
