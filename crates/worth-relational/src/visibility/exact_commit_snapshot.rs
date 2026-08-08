use crate::history::data::CommitReference;
use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::{SnapshotHandle, SnapshotReadPolicy};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationalRetainedCommitSnapshotDenialKind {
    ForeignRuntime,
    VersionUnavailable,
    BranchMismatch,
    CommitMismatch,
    SnapshotNotRetained,
    SnapshotBindingMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalRetainedCommitSnapshotDenial {
    kind: RelationalRetainedCommitSnapshotDenialKind,
    detail: &'static str,
}

impl RelationalRetainedCommitSnapshotDenial {
    fn new(kind: RelationalRetainedCommitSnapshotDenialKind, detail: &'static str) -> Self {
        Self { kind, detail }
    }

    pub const fn kind(&self) -> RelationalRetainedCommitSnapshotDenialKind {
        self.kind
    }

    pub const fn detail(&self) -> &'static str {
        self.detail
    }
}

/// Read-only evidence that one exact canonical commit still has a published
/// snapshot handle in this runtime.
///
/// This observation does not own a release lease. Its borrowed handle is for
/// immediate use with ordinary visibility APIs.
#[derive(Debug, Eq, PartialEq)]
pub struct RelationalRetainedCommitSnapshot {
    commit: CommitReference,
    snapshot_handle: SnapshotHandle,
}

impl RelationalRetainedCommitSnapshot {
    pub const fn commit(&self) -> &CommitReference {
        &self.commit
    }

    pub const fn snapshot_handle(&self) -> &SnapshotHandle {
        &self.snapshot_handle
    }
}

pub(crate) fn open_retained_commit_snapshot(
    runtime: &RelationalRuntime,
    expected_runtime_instance_id: u64,
    requested_commit: &CommitReference,
) -> Result<RelationalRetainedCommitSnapshot, RelationalRetainedCommitSnapshotDenial> {
    if expected_runtime_instance_id != runtime.runtime_instance_id() {
        return Err(denial(
            RelationalRetainedCommitSnapshotDenialKind::ForeignRuntime,
            "retained commit snapshot belongs to a different runtime instance",
        ));
    }

    let Some(committed) = runtime
        .history()
        .committed_version(requested_commit.version_id)
    else {
        return Err(denial(
            RelationalRetainedCommitSnapshotDenialKind::VersionUnavailable,
            "requested version has no canonical commit in this runtime",
        ));
    };
    let canonical_commit = committed.commit();
    if canonical_commit.branch_id != requested_commit.branch_id {
        return Err(denial(
            RelationalRetainedCommitSnapshotDenialKind::BranchMismatch,
            "requested branch does not own the canonical commit version",
        ));
    }
    if canonical_commit != requested_commit {
        return Err(denial(
            RelationalRetainedCommitSnapshotDenialKind::CommitMismatch,
            "requested commit reference differs from canonical history",
        ));
    }

    let Some((snapshot_id, binding)) = runtime
        .visibility
        .published_snapshot_binding_for_version(requested_commit.version_id)
    else {
        return Err(denial(
            RelationalRetainedCommitSnapshotDenialKind::SnapshotNotRetained,
            "canonical commit no longer has an already-retained published snapshot",
        ));
    };
    if binding.branch_id != requested_commit.branch_id
        || binding.version_id != requested_commit.version_id
        || binding.read_policy != SnapshotReadPolicy::ImmutablePinnedNoLazyMutation
    {
        return Err(denial(
            RelationalRetainedCommitSnapshotDenialKind::SnapshotBindingMismatch,
            "published snapshot binding disagrees with canonical commit identity",
        ));
    }

    Ok(RelationalRetainedCommitSnapshot {
        commit: canonical_commit.clone(),
        snapshot_handle: SnapshotHandle {
            runtime_instance_id: runtime.runtime_instance_id(),
            branch_id: binding.branch_id,
            snapshot_id,
            version_id: binding.version_id,
            read_policy: binding.read_policy,
        },
    })
}

fn denial(
    kind: RelationalRetainedCommitSnapshotDenialKind,
    detail: &'static str,
) -> RelationalRetainedCommitSnapshotDenial {
    RelationalRetainedCommitSnapshotDenial::new(kind, detail)
}
