use crate::capabilities::SnapshotSource;
use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::{SnapshotHandle, SnapshotInspectionSummary};
use crate::storage::logic::state::SnapshotState;
use crate::visibility::cache_state::reconstruct_state;

use super::read_view_from_snapshot_state;

pub(crate) struct ResolvedVisibilitySnapshot {
    pub(crate) handle: SnapshotHandle,
    pub(crate) state: SnapshotState,
    pub(crate) keeps_storage_pins: bool,
}

pub(crate) fn resolve_snapshot_handle(
    runtime: &RelationalRuntime,
    handle: &SnapshotHandle,
) -> Option<SnapshotHandle> {
    if let Some((version_id, read_policy)) = runtime.active_snapshot_binding(handle.snapshot_id) {
        return Some(SnapshotHandle {
            runtime_instance_id: runtime.runtime_instance_id(),
            snapshot_id: handle.snapshot_id,
            version_id,
            read_policy,
        });
    }

    if let Some(binding) = runtime
        .visibility
        .execution_basis_binding(handle.snapshot_id)
    {
        return Some(SnapshotHandle {
            runtime_instance_id: runtime.runtime_instance_id(),
            snapshot_id: handle.snapshot_id,
            version_id: binding.version_id,
            read_policy: binding.read_policy,
        });
    }

    let version_id = runtime.published_snapshot_version(handle.snapshot_id)?;
    let read_policy = runtime
        .visibility
        .published_snapshot_binding(handle.snapshot_id)?
        .read_policy;
    Some(SnapshotHandle {
        runtime_instance_id: runtime.runtime_instance_id(),
        snapshot_id: handle.snapshot_id,
        version_id,
        read_policy,
    })
}

pub(crate) fn resolve_snapshot_state(
    runtime: &RelationalRuntime,
    handle: &SnapshotHandle,
) -> Option<ResolvedVisibilitySnapshot> {
    if let Some((version_id, read_policy)) = runtime.active_snapshot_binding(handle.snapshot_id) {
        let resolved_handle = SnapshotHandle {
            runtime_instance_id: runtime.runtime_instance_id(),
            snapshot_id: handle.snapshot_id,
            version_id,
            read_policy,
        };
        let state = reconstruct_state(runtime, version_id)?;
        return Some(ResolvedVisibilitySnapshot {
            handle: resolved_handle,
            state,
            keeps_storage_pins: true,
        });
    }

    if let Some(binding) = runtime
        .visibility
        .execution_basis_binding(handle.snapshot_id)
    {
        let resolved_handle = SnapshotHandle {
            runtime_instance_id: runtime.runtime_instance_id(),
            snapshot_id: handle.snapshot_id,
            version_id: binding.version_id,
            read_policy: binding.read_policy,
        };
        let state = reconstruct_state(runtime, binding.version_id)?;
        return Some(ResolvedVisibilitySnapshot {
            handle: resolved_handle,
            state,
            keeps_storage_pins: false,
        });
    }

    let version_id = runtime.published_snapshot_version(handle.snapshot_id)?;
    let read_policy = runtime
        .visibility
        .published_snapshot_binding(handle.snapshot_id)?
        .read_policy;
    let resolved_handle = SnapshotHandle {
        runtime_instance_id: runtime.runtime_instance_id(),
        snapshot_id: handle.snapshot_id,
        version_id,
        read_policy,
    };
    let state = reconstruct_state(runtime, version_id)?;
    Some(ResolvedVisibilitySnapshot {
        handle: resolved_handle,
        state,
        keeps_storage_pins: false,
    })
}

pub(crate) fn resolve_snapshot_inspection(
    runtime: &RelationalRuntime,
    handle: &SnapshotHandle,
) -> Option<SnapshotInspectionSummary> {
    let resolved = resolve_snapshot_state(runtime, handle)?;
    if resolved.keeps_storage_pins {
        return Some(SnapshotInspectionSummary {
            version_id: resolved.handle.version_id,
            entity_count: resolved.state.pinned_entity_count,
            relation_count: resolved.state.pinned_relation_count,
            pinned_entity_count: resolved.state.pinned_entity_count,
            pinned_relation_count: resolved.state.pinned_relation_count,
        });
    }
    let read_view = read_view_from_snapshot_state(runtime, &resolved.state);
    Some(SnapshotInspectionSummary {
        version_id: resolved.handle.version_id,
        entity_count: read_view.entities.len(),
        relation_count: read_view.relations.len(),
        pinned_entity_count: 0,
        pinned_relation_count: 0,
    })
}
