use worth_relational::facade::{
    history::{BranchId, RelationalCommitReceipt},
    runtime::RelationalRuntime,
    snapshots::SnapshotHandle,
};

/// Opens an ephemeral snapshot from the Relational owner's exact current
/// branch observation. The descriptive branch name never selects storage on
/// its own.
pub(crate) fn open_current_branch_snapshot(
    runtime: &mut RelationalRuntime,
    branch: &BranchId,
) -> Option<SnapshotHandle> {
    let identity = runtime.branch_identity(branch).ok()?;
    let (_, basis) = runtime.observe_branch(&identity).ok()?;
    runtime
        .snapshots()
        .snapshot_for_observation(&basis.observation())
        .ok()
}

pub(crate) fn open_current_main_snapshot(
    runtime: &mut RelationalRuntime,
) -> Option<SnapshotHandle> {
    let identity = runtime.main_branch_identity();
    let (_, basis) = runtime.observe_branch(&identity).ok()?;
    runtime
        .snapshots()
        .snapshot_for_observation(&basis.observation())
        .ok()
}

/// Reads the current canonical head only through an owner-admitted repeatable
/// observation.
pub(crate) fn current_branch_head(
    runtime: &RelationalRuntime,
    branch: &BranchId,
) -> Option<RelationalCommitReceipt> {
    let identity = runtime.branch_identity(branch).ok()?;
    let (_, basis) = runtime.observe_branch(&identity).ok()?;
    runtime
        .history()
        .branch_head_for_observation(&basis.observation())
        .ok()?
}
