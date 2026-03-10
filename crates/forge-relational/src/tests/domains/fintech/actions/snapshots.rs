use crate::facade::SnapshotHandle;

use super::super::fixture::FintechWorld;

pub(crate) fn capture_world_snapshot(world: &mut FintechWorld) -> SnapshotHandle {
    world.runtime.snapshot()
}

pub(crate) fn release_snapshot_handle(
    world: &mut FintechWorld,
    snapshot: &SnapshotHandle,
) -> bool {
    world.runtime.release_snapshot(snapshot)
}
