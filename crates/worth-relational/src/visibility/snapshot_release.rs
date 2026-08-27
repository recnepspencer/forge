#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalSnapshotReleaseDenial {
    ForeignRuntime {
        expected_runtime_instance_id: u64,
        actual_runtime_instance_id: u64,
    },
    UnknownSnapshot,
    BindingMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalSnapshotReleaseReceipt {
    snapshot_id: crate::snapshots::data::SnapshotId,
}

impl RelationalSnapshotReleaseReceipt {
    pub(crate) const fn new(snapshot_id: crate::snapshots::data::SnapshotId) -> Self {
        Self { snapshot_id }
    }

    pub const fn snapshot_id(&self) -> crate::snapshots::data::SnapshotId {
        self.snapshot_id
    }
}

impl super::authority::VisibilityAuthority<'_> {
    pub fn release_snapshot(
        &mut self,
        handle: &crate::snapshots::data::SnapshotHandle,
    ) -> Result<RelationalSnapshotReleaseReceipt, RelationalSnapshotReleaseDenial> {
        if handle.runtime_instance_id() != self.runtime.runtime_instance_id() {
            return Err(RelationalSnapshotReleaseDenial::ForeignRuntime {
                expected_runtime_instance_id: self.runtime.runtime_instance_id(),
                actual_runtime_instance_id: handle.runtime_instance_id(),
            });
        }
        let active_binding = self
            .runtime
            .visibility
            .active_handle_binding(handle.snapshot_id());
        let published_binding = self
            .runtime
            .visibility
            .published_snapshot_binding(handle.snapshot_id());
        let binding = active_binding
            .or(published_binding.as_ref())
            .ok_or(RelationalSnapshotReleaseDenial::UnknownSnapshot)?;
        if binding.branch_id != *handle.branch_id()
            || binding.version_id != handle.version_id()
            || binding.read_policy != handle.read_policy()
        {
            return Err(RelationalSnapshotReleaseDenial::BindingMismatch);
        }
        if let Some(binding) = self
            .runtime
            .visibility
            .remove_active_handle(handle.snapshot_id())
        {
            super::cache_state::bump_active_snapshot_ref(self.runtime, &binding.basis, -1);
            return Ok(RelationalSnapshotReleaseReceipt::new(handle.snapshot_id()));
        }
        self.runtime
            .visibility
            .remove_published_handle(handle.snapshot_id())
            .map(|_| RelationalSnapshotReleaseReceipt::new(handle.snapshot_id()))
            .ok_or(RelationalSnapshotReleaseDenial::UnknownSnapshot)
    }
}
