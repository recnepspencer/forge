use crate::capabilities::VisibilityPolicySource;
use crate::logic::runtime::{RelationalRuntime, VisibilityResidency};
use crate::snapshots::data::{SnapshotId, SnapshotReadPolicy};
use crate::storage::overlay::SnapshotState;

pub(crate) fn cached_state_for_version(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
) -> Option<SnapshotState> {
    runtime.visibility.cache.state_for_version(version_id)
}

pub(crate) fn residency_for_version(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
) -> VisibilityResidency {
    runtime.visibility.cache.residency_for_version(version_id)
}

pub(crate) fn insert_state(runtime: &RelationalRuntime, state: SnapshotState) {
    runtime.visibility.cache.insert_state(state);
}

pub(crate) fn bump_active_snapshot_ref(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
    delta: i32,
) {
    let was_active = residency_for_version(runtime, version_id).active_snapshot_refs > 0;
    bump_visibility_ref(runtime, version_id, |residency| {
        residency.active_snapshot_refs =
            residency.active_snapshot_refs.saturating_add_signed(delta);
    });
    let is_active = residency_for_version(runtime, version_id).active_snapshot_refs > 0;
    if delta > 0 && !was_active && is_active {
        runtime
            .services
            .instrumentation
            .count(|counters| counters.visibility_cache_snapshot_promotions += 1);
    }
}

pub(crate) fn bump_replay_ref(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
    delta: i32,
) {
    bump_visibility_ref(runtime, version_id, |residency| {
        residency.replay_refs = residency.replay_refs.saturating_add_signed(delta);
    });
    if delta > 0 {
        runtime
            .services
            .instrumentation
            .count(|counters| counters.visibility_cache_replay_promotions += delta as usize);
    }
}

pub(crate) fn bump_visibility_ref(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
    update: impl FnOnce(&mut VisibilityResidency),
) {
    runtime
        .visibility
        .cache
        .update_residency(version_id, update);
    maybe_remove_unprotected_state(runtime, version_id);
}

pub(crate) fn protect_branch_head_version(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
) {
    bump_visibility_ref(runtime, version_id, |residency| {
        residency.branch_head_refs += 1;
    });
}

pub(crate) fn ensure_state(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
    recent_candidate: bool,
) -> SnapshotState {
    if let Some(state) = cached_state_for_version(runtime, version_id) {
        runtime
            .services
            .instrumentation
            .count(|counters| counters.visibility_cache_hits += 1);
        return state;
    }
    runtime
        .services
        .instrumentation
        .count(|counters| counters.visibility_cache_miss_reconstructions += 1);
    let state = crate::visibility::snapshot_states::build_visibility_state(
        runtime,
        version_id,
        SnapshotId(0),
        SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
    );
    insert_state(runtime, state.clone());
    if recent_candidate {
        mark_recent_state(runtime, version_id);
    }
    state
}

pub(crate) fn reconstruct_state(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
    allow_recent_admission: bool,
) -> Option<SnapshotState> {
    if version_id.is_zero() || version_id.as_u64() > runtime.current_version_id().as_u64() {
        return None;
    }
    if let Some(state) = cached_state_for_version(runtime, version_id) {
        runtime
            .services
            .instrumentation
            .count(|counters| counters.visibility_cache_hits += 1);
        return Some(state);
    }
    let recent_candidate = allow_recent_admission
        && runtime.config.visibility.cache_policy.enabled
        && runtime.visibility.cache.recent_window() > 0
        && !is_protected_version(runtime, version_id);
    if recent_candidate || is_protected_version(runtime, version_id) {
        return Some(ensure_state(runtime, version_id, recent_candidate));
    }
    runtime
        .services
        .instrumentation
        .count(|counters| counters.visibility_cache_miss_reconstructions += 1);
    Some(crate::visibility::snapshot_states::build_visibility_state(
        runtime,
        version_id,
        SnapshotId(0),
        SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
    ))
}

pub(crate) fn retained_state(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
) -> Option<SnapshotState> {
    if version_id.is_zero() || version_id.as_u64() > runtime.current_version_id().as_u64() {
        return None;
    }
    if let Some(state) = cached_state_for_version(runtime, version_id) {
        return Some(state);
    }
    if version_id != runtime.current_version_id()
        && !is_protected_version(runtime, version_id)
        && !runtime.visibility.retains_published_version(version_id)
    {
        return None;
    }
    Some(ensure_state(runtime, version_id, false))
}

pub(crate) fn is_protected_version(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
) -> bool {
    let residency = residency_for_version(runtime, version_id);
    residency.branch_head_refs > 0
        || residency.replay_refs > 0
        || (runtime.protect_active_snapshots() && residency.active_snapshot_refs > 0)
        || runtime
            .visibility
            .retains_execution_basis_version(version_id)
}

pub(crate) fn mark_recent_state(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
) {
    if !runtime.config.visibility.cache_policy.enabled
        || runtime.visibility.cache.recent_window() == 0
    {
        return;
    }
    if !runtime.visibility.cache.mark_recent_resident(version_id) {
        return;
    }
    evict_cache_if_needed(runtime);
}

pub(crate) fn evict_cache_if_needed(runtime: &RelationalRuntime) {
    let window = runtime.visibility.cache.recent_window();
    if !runtime.config.visibility.cache_policy.enabled || window == 0 {
        return;
    }
    loop {
        if runtime.visibility.cache.resident_recent_count() <= window {
            break;
        }
        let scan_len = runtime.visibility.cache.recent_candidate_count();
        if scan_len == 0 {
            break;
        }
        let mut evicted = false;
        for _ in 0..scan_len {
            let candidate = runtime.visibility.cache.pop_oldest_recent_candidate();
            let Some(version_id) = candidate else {
                break;
            };
            if is_protected_version(runtime, version_id) {
                runtime
                    .visibility
                    .cache
                    .enqueue_recent_candidate(version_id);
                continue;
            }
            if !runtime
                .visibility
                .cache
                .evict_recent_resident_if_unprotected(version_id)
            {
                continue;
            }
            runtime.visibility.cache.remove_state(version_id);
            runtime
                .services
                .instrumentation
                .count(|counters| counters.visibility_cache_recent_evictions += 1);
            evicted = true;
            break;
        }
        if !evicted {
            break;
        }
    }
}

pub(crate) fn maybe_remove_unprotected_state(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
) {
    let residency = residency_for_version(runtime, version_id);
    if residency.branch_head_refs == 0
        && residency.replay_refs == 0
        && residency.active_snapshot_refs == 0
        && !residency.recent_resident
    {
        runtime.visibility.cache.remove_state(version_id);
    }
}
