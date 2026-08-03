mod cache;
mod replay_retention;
mod snapshot_handles;

pub(crate) use cache::{VisibilityCache, VisibilityResidency};
pub(crate) use replay_retention::{ReplayRetentionIndex, ReplayRetentionState};
pub(crate) use snapshot_handles::{
    ExecutionBasisBinding, ExecutionBasisRegistry, SnapshotHandleBinding, SnapshotHandles,
};

use std::sync::Arc;

use crate::logic::runtime::state::subsystems::RuntimeSubsystem;
use crate::logic::runtime::RelationalRuntimeConfig;
use crate::snapshots::data::SnapshotId;

#[derive(Debug)]
pub(crate) struct VisibilitySubsystem {
    pub(crate) handles: SnapshotHandles,
    execution_bases: Arc<ExecutionBasisRegistry>,
    pub(crate) cache: VisibilityCache,
    pub(crate) replay_retention: ReplayRetentionIndex,
}

impl VisibilitySubsystem {
    fn build_from_config(config: &RelationalRuntimeConfig) -> Self {
        Self {
            handles: SnapshotHandles::new(),
            execution_bases: ExecutionBasisRegistry::new(),
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
            execution_bases: ExecutionBasisRegistry::new(),
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

    pub(crate) fn allocate_snapshot_id(&self) -> SnapshotId {
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
            || self.execution_bases.binding(snapshot_id).is_some()
    }

    pub(crate) fn admit_execution_basis(
        &self,
        snapshot_id: SnapshotId,
        branch_id: crate::history::data::BranchId,
        version_id: crate::identity::data::VersionId,
        read_policy: crate::snapshots::data::SnapshotReadPolicy,
    ) -> (u64, Arc<ExecutionBasisRegistry>) {
        let lease_ordinal =
            self.execution_bases
                .admit(snapshot_id, branch_id, version_id, read_policy);
        (lease_ordinal, Arc::clone(&self.execution_bases))
    }

    pub(crate) fn execution_basis_binding(
        &self,
        snapshot_id: SnapshotId,
    ) -> Option<ExecutionBasisBinding> {
        self.execution_bases.binding(snapshot_id)
    }

    pub(crate) fn execution_basis_is_live(
        &self,
        identity: &crate::visibility::execution_basis::RelationalExecutionBasisIdentity,
    ) -> bool {
        self.execution_bases
            .retains_identity(identity.snapshot_id(), identity.lease_ordinal())
    }

    pub(crate) fn active_versions(
        &self,
    ) -> impl Iterator<Item = crate::identity::data::VersionId> + '_ {
        self.handles.active_versions()
    }

    pub(crate) fn retains_published_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> bool {
        self.handles.retains_published_version(version_id)
    }

    pub(crate) fn retains_execution_basis_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> bool {
        self.execution_bases.retains_version(version_id)
    }

    pub(crate) fn retention_fence_version(
        &self,
        published_version: crate::identity::data::VersionId,
    ) -> crate::identity::data::VersionId {
        let non_execution_fence = self
            .active_versions()
            .chain(self.replay_retention.versions())
            .min()
            .unwrap_or(published_version);
        self.execution_bases
            .oldest_version()
            .map_or(non_execution_fence, |version| {
                version.min(non_execution_fence)
            })
    }

    pub(crate) fn historical_reconstruction_fence_version(
        &self,
        published_version: crate::identity::data::VersionId,
    ) -> crate::identity::data::VersionId {
        let retention_fence = self.retention_fence_version(published_version);
        self.handles
            .published_versions()
            .min()
            .map_or(retention_fence, |version| version.min(retention_fence))
    }

    pub(crate) fn insert_published_handle(
        &mut self,
        snapshot_id: SnapshotId,
        binding: SnapshotHandleBinding,
    ) {
        self.handles.insert_published(snapshot_id, binding);
    }

    pub(crate) fn remove_published_handle(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Option<SnapshotHandleBinding> {
        self.handles.remove_published(snapshot_id)
    }

    pub(crate) fn published_snapshot_version(
        &self,
        snapshot_id: SnapshotId,
    ) -> Option<crate::identity::data::VersionId> {
        self.handles.published_version(snapshot_id)
    }

    pub(crate) fn published_snapshot_binding(
        &self,
        snapshot_id: SnapshotId,
    ) -> Option<SnapshotHandleBinding> {
        self.handles.published_binding(snapshot_id).cloned()
    }

    pub(crate) fn oldest_published_snapshot_id(&self) -> Option<SnapshotId> {
        self.handles.oldest_published_snapshot_id()
    }

    pub(crate) fn cached_visibility_version_count(&self) -> usize {
        self.cache.cached_version_count()
    }

    pub(crate) fn protected_visibility_version_count(
        &self,
        protect_active_snapshots: bool,
    ) -> usize {
        self.cache.protected_version_count(protect_active_snapshots)
    }

    pub(crate) fn recent_visibility_cache_count(&self) -> usize {
        self.cache.recent_visibility_count()
    }

    pub(crate) fn tracked_branch_head_versions(&self) -> Vec<crate::identity::data::VersionId> {
        self.cache.tracked_branch_head_versions()
    }

    pub(crate) fn clear_branch_head_residency(
        &self,
        tracked_versions: &[crate::identity::data::VersionId],
    ) {
        self.cache.clear_branch_head_residency(tracked_versions);
    }

    pub(crate) fn increment_replay_retention(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<usize> {
        let retained = self.replay_retention.retained_mut(version_id)?;
        retained.ref_count += 1;
        Some(retained.ref_count)
    }

    pub(crate) fn insert_replay_retention(
        &mut self,
        version_id: crate::identity::data::VersionId,
        state: ReplayRetentionState,
    ) {
        self.replay_retention.insert_retained(version_id, state);
    }

    pub(crate) fn take_replay_retention(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<ReplayRetentionState> {
        self.replay_retention.take_retained(version_id)
    }

    pub(crate) fn restore_replay_retention(
        &mut self,
        version_id: crate::identity::data::VersionId,
        state: ReplayRetentionState,
    ) {
        self.replay_retention.insert_retained(version_id, state);
    }
}
