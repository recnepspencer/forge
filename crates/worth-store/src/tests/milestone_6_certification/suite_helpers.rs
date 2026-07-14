pub(super) fn canonical_truth(bundle: &crate::Milestone6CertificationBundle) -> serde_json::Value {
    serde_json::json!({
        "truth_digest": bundle.truth_digest,
        "layout_read_report": bundle.layout_read_report,
        "physical_layout_report": bundle.physical_layout_report,
    })
}

pub(super) fn fallback_surface(
    error: &crate::StoreError,
    counters: crate::StoreCounterSnapshot,
) -> serde_json::Value {
    serde_json::json!({
        "error_kind": format!("{:?}", error.kind()),
        "aspect_layout_plan_count": counters.aspect_layout_plan_count,
        "aspect_layout_admitted_count": counters.aspect_layout_admitted_count,
        "aspect_layout_fallback_count": counters.aspect_layout_fallback_count,
        "aspect_layout_rejected_count": counters.aspect_layout_rejected_count,
    })
}

pub(super) fn rebuild_identity_surface(
    bundle: &crate::Milestone6CertificationBundle,
) -> serde_json::Value {
    serde_json::json!({
        "truth_digest": bundle.truth_digest,
        "artifact_digest": bundle.artifact_digest,
        "structural_block_id": bundle.physical_layout_report.structural_block_id,
        "physical_chunk_id": bundle.physical_layout_report.physical_chunk_id,
        "determinism_digest": bundle.physical_layout_report.determinism_digest,
    })
}

pub(super) fn chunk_export_surface(
    export: &crate::Milestone6ChunkModelExport,
) -> serde_json::Value {
    serde_json::json!({
        "physical_chunk_id": export.physical_chunk_id().as_str(),
        "chunk_membership_artifact_id": export.chunk_membership_artifact_id(),
        "determinism_digest": export.determinism_digest(),
        "chunk_member_count": export.chunk_member_count(),
        "layout_materialization_artifact_id": export.layout_materialization_artifact_id(),
    })
}

pub(super) fn execution_surface(
    read: &crate::AspectLayoutReadExecutionResult,
    dedup: &crate::DedupBackedReadResult,
) -> serde_json::Value {
    serde_json::json!({
        "request_scope_class": read.plan().request().scope_class().label(),
        "layout_materialization_artifact_id": read.layout_materialization_artifact_id(),
        "scope_membership_artifact_id": read.scope_membership_artifact_id(),
        "chunk_membership_artifact_id": read.chunk_membership_artifact_id(),
        "semantic_truth_digest": read.semantic_truth_digest(),
        "authoritative_commit_count": read.authoritative_commit_count(),
        "structural_block_id": dedup.structural_block_lookup().structural_block_id().as_str(),
        "structural_block_slice_ids": dedup.structural_block_lookup().slice_ids(),
    })
}

pub(super) fn overlap_branch_parity_surface(
    read: &crate::AspectLayoutReadExecutionResult,
    dedup: &crate::DedupBackedReadResult,
    control: &crate::AspectLayoutControlTruth,
) -> serde_json::Value {
    serde_json::json!({
        "execution_branch_id": read.plan().request().target().branch_id().0,
        "execution_frontier_commit_id": read.plan().request().target().frontier_commit_id().0,
        "execution_scope_class": read.plan().request().scope_class().label(),
        "execution_slice_ids": read.plan().slice_ids(),
        "execution_semantic_truth_digest": read.semantic_truth_digest(),
        "execution_authoritative_commit_count": read.authoritative_commit_count(),
        "dedup_semantic_truth_digest": dedup.read().semantic_truth_digest(),
        "dedup_authoritative_commit_count": dedup.read().authoritative_commit_count(),
        "dedup_structural_block_id": dedup.structural_block_lookup().structural_block_id().as_str(),
        "dedup_slice_ids": dedup.structural_block_lookup().slice_ids(),
        "control_branch_id": control.branch_id().0,
        "control_frontier_commit_id": control.frontier_commit_id().0,
        "control_scope_class": control.scope_class(),
        "control_authoritative_truth_digest": control.authoritative_truth_digest(),
        "control_authoritative_commit_count": control.authoritative_commit_count(),
    })
}
