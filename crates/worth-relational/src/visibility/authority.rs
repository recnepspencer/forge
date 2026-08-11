use crate::capabilities::VisibilityPolicySource;
use crate::logic::runtime::{RelationalRuntime, SnapshotGuard, SnapshotHandleBinding};
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::visibility::cache_state::{
    bump_active_snapshot_ref, cached_state_for_version, evict_cache_if_needed, insert_state,
    residency_for_version, retained_state,
};
use crate::visibility::exact_commit_snapshot::{
    open_retained_commit_snapshot, projection_binding_denial,
    RelationalRetainedCommitEntityProjection, RelationalRetainedCommitSnapshot,
    RelationalRetainedCommitSnapshotDenial, RelationalRetainedCommitSnapshotDenialKind,
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
            branch_id: crate::visibility::branch_scope::authoritative_branch_for_version(
                self.runtime,
                version_id,
            ),
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
            SnapshotHandleBinding::new(
                handle.branch_id.clone(),
                handle.version_id,
                handle.read_policy,
            ),
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

    /// Opens the current immutable snapshot for one exact branch.
    ///
    /// This is the branch-qualified counterpart to [`Self::snapshot`]. It
    /// refuses to infer another branch when the requested branch has no
    /// current head or when retained version ownership disagrees.
    pub fn snapshot_for_branch(
        &mut self,
        branch_id: &crate::history::data::BranchId,
    ) -> Option<SnapshotHandle> {
        let version_id = self
            .runtime
            .history()
            .branch_head(branch_id)
            .map(|head| head.version_id)
            .or_else(|| {
                (branch_id == &self.runtime.config.history.main_branch)
                    .then(|| self.runtime.current_version_id())
            })?;
        let handle = self.open_active_snapshot(
            version_id,
            SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        );
        if &handle.branch_id == branch_id {
            Some(handle)
        } else {
            self.release_snapshot(&handle);
            None
        }
    }

    /// Observes the already-published snapshot for one exact canonical commit.
    ///
    /// This does not open a current branch head, allocate a snapshot identity,
    /// reconstruct historical state, or acquire replay retention. A pruned
    /// publication is therefore a typed denial rather than a reconstruction
    /// request.
    pub fn retained_snapshot_for_commit(
        &self,
        expected_runtime_instance_id: u64,
        commit: &crate::history::data::CommitReference,
    ) -> Result<RelationalRetainedCommitSnapshot, RelationalRetainedCommitSnapshotDenial> {
        open_retained_commit_snapshot(self.runtime, expected_runtime_instance_id, commit)
    }

    /// Projects one entity directly from an exact retained canonical commit and reports the
    /// structural work performed by this owner operation.
    pub fn project_retained_entity_for_commit<T>(
        &self,
        expected_runtime_instance_id: u64,
        commit: &crate::history::data::CommitReference,
        entity_id: crate::identity::data::EntityId,
        expected_kind_id: crate::identity::data::KindId,
        projection_scope: crate::visibility::materialization::read_records::ProjectionAspectScope,
        mut project: impl FnMut(
            crate::visibility::materialization::read_records::EntityProjectionRecord<'_>,
        ) -> Option<T>,
    ) -> Result<RelationalRetainedCommitEntityProjection<T>, RelationalRetainedCommitSnapshotDenial>
    {
        let retained =
            open_retained_commit_snapshot(self.runtime, expected_runtime_instance_id, commit)?;
        let projected_fields = projection_scope
            .requirements()
            .iter()
            .map(|requirement| requirement.mask().paths().len())
            .sum();
        let value = self
            .runtime
            .read_truth()
            .project_snapshot(retained.snapshot_handle())
            .ok_or_else(projection_binding_denial)?
            .entity_record_of_expected_kind_with_projection_scope(
                entity_id,
                expected_kind_id,
                projection_scope,
                |record| project(record),
            )
            .map_err(|_| {
                RelationalRetainedCommitSnapshotDenial::new(
                    RelationalRetainedCommitSnapshotDenialKind::EntityKindMismatch,
                    "retained entity kind differs from the requested owner projection",
                )
            })?;
        let work = retained
            .work()
            .record_projection(usize::from(value.is_some()), projected_fields);
        Ok(RelationalRetainedCommitEntityProjection::new(value, work))
    }

    pub fn pin_snapshot(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<SnapshotGuard> {
        retained_state(self.runtime, version_id)?;
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
        self.runtime
            .visibility
            .remove_published_handle(handle.snapshot_id)
            .is_some()
    }

    pub fn admit_execution_basis(
        &mut self,
        branch_id: &crate::history::data::BranchId,
        version_id: crate::identity::data::VersionId,
    ) -> Result<
        crate::visibility::execution_basis::RelationalExecutionBasisLease,
        crate::visibility::execution_basis::RelationalExecutionBasisDenial,
    > {
        crate::visibility::execution_basis::admit_execution_basis(
            self.runtime,
            branch_id,
            version_id,
        )
    }

    pub fn execution_basis_is_live(
        &self,
        identity: &crate::visibility::execution_basis::RelationalExecutionBasisIdentity,
    ) -> bool {
        identity.runtime_instance_id() == self.runtime.runtime_instance_id()
            && self.runtime.visibility.execution_basis_is_live(identity)
    }
}
