pub(super) fn expected_phase(path: &str) -> &'static str {
    if path.starts_with("crates/worth-store-recovery-runtime") {
        return runtime_phase(path);
    }
    if path.ends_with("worth-store-physical-backend/src/recovery_media/discovery/artifact.rs") {
        return "phase-3";
    }
    if path.ends_with("worth-store-physical-backend/src/recovery_media/staging.rs") {
        return "phase-5";
    }
    if path.ends_with("worth-store-physical-backend/src/recovery_media/publication.rs")
        || path.ends_with("worth-store-physical-backend/src/recovery_media/reopen.rs")
    {
        return "phase-6";
    }
    if path.ends_with("worth-store-physical-backend/src/recovery_media/cleanup.rs")
        || path.ends_with("worth-store-physical-backend/src/recovery_media/cleanup/revalidation.rs")
    {
        return "phase-7";
    }
    if path
        .ends_with("worth-store-physical-backend/src/recovery_media/discovery/addressed_payload.rs")
    {
        return "phase-4";
    }
    if path.contains("worth-store-physical-backend/src/recovery_media/") {
        return "phase-2";
    }
    if path
        .ends_with("worth-store-physical-format/src/manifest/durable_root_routing/decode_limits.rs")
        || path.ends_with(
            "worth-store-wal/src/artifact_store/segment_inventory/segment_inspection/denial.rs",
        )
    {
        return "phase-3";
    }
    if path.contains("worth-store-physical-format/src/recovery_projection")
        || path.ends_with(
            "worth-store-physical-format/src/manifest/durable_segment_routing/codec_primitives.rs",
        )
    {
        return "phase-4";
    }
    if path.ends_with(
        "worth-store-wal/src/artifact_store/segment_inventory/segment_inspection/owned_frame.rs",
    ) {
        return "phase-4";
    }
    if path
        .ends_with("worth-store-recovery-physics/src/source_precedence/checkpoint_covered_wal.rs")
    {
        return "phase-7";
    }
    if path.contains("worth-store-recovery-physics/src/source_precedence/")
        || path.contains("worth-store-recovery-physics/src/wal_prefix/")
    {
        return "phase-3";
    }
    if path.contains("worth-store-recovery-physics/src/page_redo/") {
        return if path.ends_with("/transition.rs") {
            "phase-5"
        } else {
            "phase-4"
        };
    }
    if path.contains("worth-store-recovery-physics/") {
        return "phase-4";
    }
    if path.contains("worth-store/src/physical_runtime/recovery_freshness/") {
        return if path.ends_with("/cleanup.rs") || path.contains("/cleanup/plan/") {
            "phase-7"
        } else if path.ends_with("/binding.rs") || path.contains("/binding/") {
            "phase-4"
        } else {
            "phase-2"
        };
    }
    if path.contains("worth-store/src/physical_runtime/recovery_coordination/") {
        return if path.contains("/recovery_coordination/cleanup/")
            || path.ends_with("/recovery_coordination/effect/cleanup.rs")
        {
            "phase-7"
        } else if path.ends_with("/staging.rs")
            || path.contains("/recovery_coordination/staging/")
            || path.ends_with("/recovery_coordination/effect.rs")
        {
            "phase-5"
        } else if path.contains("/recovery_coordination/publication")
            || path.contains("/recovery_coordination/reopen")
            || path.ends_with("/recovery_coordination/effect/reopen.rs")
        {
            "phase-6"
        } else {
            "phase-2"
        };
    }
    if path.contains("worth-store/src/physical_runtime/recovery_construction/") {
        return "phase-6";
    }
    if path.contains("worth-store-offline-verifier/") {
        return "phase-8";
    }
    "phase-9"
}

fn runtime_phase(path: &str) -> &'static str {
    if path.contains("/src/observation/") || path.ends_with("/src/observation/mod.rs") {
        return "phase-8";
    }
    if path.ends_with("/entry/publication.rs") || path.ends_with("/entry/reopen.rs") {
        return "phase-6";
    }
    if path.ends_with("/entry/staging.rs") {
        return "phase-5";
    }
    if path.ends_with("/entry/source_denial.rs") {
        return "phase-3";
    }
    if path.contains("/entry/")
        || path.ends_with("/orchestration/coordination.rs")
        || runtime_scaffold(path)
    {
        return "phase-2";
    }
    if path.ends_with("/handoff/mod.rs")
        || path.ends_with("/progression/discovered.rs")
        || path.ends_with("/progression/discovered/selection.rs")
        || path.ends_with("/progression/selected.rs")
        || path.ends_with("/orchestration/discovery.rs")
        || path.ends_with("/orchestration/discovery/observation.rs")
        || path.ends_with("/orchestration/discovery/observation/counters.rs")
        || path.ends_with("/orchestration/manifest_facts.rs")
        || path.ends_with("/handoff/blocked.rs")
    {
        return "phase-3";
    }
    if path.ends_with("/progression/planned/cancellation.rs") {
        return "phase-5";
    }
    if path.ends_with("/progression/planned.rs")
        || path.contains("/progression/planned/")
        || path.ends_with("/orchestration/planning.rs")
        || path.contains("/orchestration/planning/")
        || path.ends_with("/handoff/operation_fates.rs")
    {
        return "phase-4";
    }
    if path.ends_with("/progression/staged.rs")
        || path.ends_with("/orchestration/staging.rs")
        || path.contains("/orchestration/staging/")
    {
        return "phase-5";
    }
    if path.contains("/cleanup/") || path.ends_with("/handoff/cleanup_posture.rs") {
        return "phase-7";
    }
    "phase-6"
}

fn runtime_scaffold(path: &str) -> bool {
    !path.contains("/src/progression/")
        && !path.contains("/src/orchestration/")
        && !path.contains("/src/handoff/")
        && !path.contains("/src/cleanup/")
        && !path.contains("/src/observation/")
        || path.ends_with("/progression/mod.rs")
        || path.ends_with("/progression/admitted.rs")
        || path.ends_with("/orchestration/mod.rs")
}
mod phase_four;
mod phase_seven;
mod responsibility;

pub(super) use responsibility::expected_responsibility;
