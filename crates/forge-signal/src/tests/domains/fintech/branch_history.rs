use crate::facade::{
    LineageRecord, ReplaySlice, SignalBranchHandle, SignalError, SignalSnapshotV1,
};

use super::fixture::FintechDomainFixture;

pub(super) fn create_branch(
    fixture: &mut FintechDomainFixture,
    name: &str,
) -> Result<SignalBranchHandle, SignalError> {
    let branch = fixture.runtime.create_branch(name)?;
    fixture.runtime.switch_branch(branch.clone())?;
    Ok(branch)
}

pub(super) fn capture_active_snapshot(fixture: &mut FintechDomainFixture) -> SignalSnapshotV1 {
    fixture.runtime.capture_snapshot()
}

pub(super) fn capture_branch_snapshot(
    fixture: &mut FintechDomainFixture,
    branch: SignalBranchHandle,
) -> Result<SignalSnapshotV1, SignalError> {
    fixture.runtime.capture_branch_snapshot(branch)
}

pub(super) fn restore_branch_snapshot(
    fixture: &mut FintechDomainFixture,
    branch: SignalBranchHandle,
    snapshot: &SignalSnapshotV1,
) -> Result<(), SignalError> {
    fixture.runtime.restore_branch_snapshot(branch, snapshot)
}

pub(super) fn replay_for_branch(
    fixture: &FintechDomainFixture,
    branch: SignalBranchHandle,
) -> ReplaySlice {
    fixture.runtime.replay_for_branch(branch.id)
}

pub(super) fn replay_around_snapshot(
    fixture: &FintechDomainFixture,
    snapshot: &SignalSnapshotV1,
) -> ReplaySlice {
    fixture
        .runtime
        .replay_around_snapshot(snapshot.meta.snapshot_id)
}

pub(super) fn lineage_for_main_risk(fixture: &FintechDomainFixture) -> Vec<LineageRecord> {
    fixture
        .runtime
        .lineage_chain_for_node(fixture.main_risk_node())
}
