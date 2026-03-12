mod cache;
mod replay_retention;
mod runtime_cache;
mod snapshot_handles;

pub(crate) use cache::{VisibilityCache, VisibilityResidency};
pub(crate) use replay_retention::{ReplayRetentionIndex, ReplayRetentionState};
pub(crate) use snapshot_handles::{SnapshotHandleBinding, SnapshotHandles};

use crate::logic::runtime::RelationalRuntimeConfig;
use crate::snapshots::data::SnapshotId;
use crate::logic::runtime::state::subsystems::RuntimeSubsystem;

#[derive(Debug)]
pub(crate) struct VisibilitySubsystem {
    pub(crate) handles: SnapshotHandles,
    pub(crate) cache: VisibilityCache,
    pub(crate) replay_retention: ReplayRetentionIndex,
}

impl VisibilitySubsystem {
    fn build_from_config(config: &RelationalRuntimeConfig) -> Self {
        Self {
            handles: SnapshotHandles::new(),
            cache: VisibilityCache::new(config),
            replay_retention: ReplayRetentionIndex::new(),
        }
    }
}

impl RuntimeSubsystem for VisibilitySubsystem {
    type Config = RelationalRuntimeConfig;

    fn new(config: &Self::Config) -> Self {
        Self::build_from_config(config)
    }

    fn fork(&self) -> Self {
        Self {
            handles: self.handles.fork(),
            cache: self.cache.fork(),
            replay_retention: self.replay_retention.fork(),
        }
    }
}

impl VisibilitySubsystem {
    pub(crate) fn active_snapshot_count(&self) -> usize {
        self.handles.active_count()
    }

    pub(crate) fn published_snapshot_handle_count(&self) -> usize {
        self.handles.published_count()
    }

    pub(crate) fn allocate_snapshot_id(&mut self) -> SnapshotId {
        self.handles.next_snapshot_id()
    }

    pub(crate) fn insert_active_handle(
        &mut self,
        snapshot_id: SnapshotId,
        binding: SnapshotHandleBinding,
    ) {
        self.handles.insert_active(snapshot_id, binding);
    }

    pub(crate) fn remove_active_handle(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Option<SnapshotHandleBinding> {
        self.handles.remove_active(snapshot_id)
    }

    pub(crate) fn active_handle_binding(
        &self,
        snapshot_id: SnapshotId,
    ) -> Option<&SnapshotHandleBinding> {
        self.handles.active_binding(snapshot_id)
    }

    pub(crate) fn is_known_snapshot(&self, snapshot_id: SnapshotId) -> bool {
        self.handles.is_known_snapshot(snapshot_id)
    }

    pub(crate) fn active_versions(
        &self,
    ) -> impl Iterator<Item = crate::identity::data::VersionId> + '_ {
        self.handles.active_versions()
    }

    pub(crate) fn insert_published_handle(
        &mut self,
        snapshot_id: SnapshotId,
        version_id: crate::identity::data::VersionId,
    ) {
        self.handles.insert_published(snapshot_id, version_id);
    }

    pub(crate) fn remove_published_handle(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Option<crate::identity::data::VersionId> {
        self.handles.remove_published(snapshot_id)
    }

    pub(crate) fn published_snapshot_version(
        &self,
        snapshot_id: SnapshotId,
    ) -> Option<crate::identity::data::VersionId> {
        self.handles.published_version(snapshot_id)
    }

    pub(crate) fn oldest_published_snapshot_id(&self) -> Option<SnapshotId> {
        self.handles.oldest_published_snapshot_id()
    }
}
