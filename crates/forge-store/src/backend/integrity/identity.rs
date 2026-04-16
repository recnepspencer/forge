use crate::backend::records::AuthoritativeArtifactFamily;

pub(crate) fn branch_key(branch_id: &forge_relational::facade::history::BranchId) -> String {
    branch_id.0.clone()
}

pub(crate) fn commit_artifact_id(commit_id: forge_relational::facade::history::CommitId) -> String {
    format!("commit:{}", commit_id.0)
}

pub(crate) fn parent_artifact_id(
    commit_id: forge_relational::facade::history::CommitId,
    parent_position: usize,
) -> String {
    format!("commit-parent:{}:{parent_position}", commit_id.0)
}

pub(crate) fn commit_support_summary_artifact_id(
    commit_id: forge_relational::facade::history::CommitId,
) -> String {
    format!("commit-support-summary:{}", commit_id.0)
}

pub(crate) fn schema_support_artifact_id(
    commit_id: forge_relational::facade::history::CommitId,
) -> String {
    format!("schema-support:{}", commit_id.0)
}

pub(crate) fn lineage_support_artifact_id(
    commit_id: forge_relational::facade::history::CommitId,
) -> String {
    format!("lineage-support:{}", commit_id.0)
}

pub(crate) fn durable_cursor_identity_artifact_id(cursor_id: &str) -> String {
    format!("durable-cursor:{cursor_id}")
}

pub(crate) fn subscriber_checkpoint_artifact_id(
    cursor_id: &str,
    checkpoint_sequence: u64,
) -> String {
    format!("subscriber-checkpoint:{cursor_id}:{checkpoint_sequence}")
}

pub(crate) fn bulk_program_artifact_id(program_id: &str) -> String {
    format!("bulk-program:{program_id}")
}

pub(crate) fn frozen_bulk_manifest_artifact_id(program_id: &str, manifest_digest: &str) -> String {
    format!("bulk-manifest:{program_id}:{manifest_digest}")
}

pub(crate) fn frozen_transform_basis_artifact_id(program_id: &str, basis_digest: &str) -> String {
    format!("bulk-transform-basis:{program_id}:{basis_digest}")
}

pub(crate) fn frozen_transform_partition_artifact_id(
    program_id: &str,
    partition_digest: &str,
) -> String {
    format!("bulk-transform-partition:{program_id}:{partition_digest}")
}

pub(crate) fn bulk_plan_artifact_id(program_id: &str, plan_id: &str) -> String {
    format!("bulk-plan:{program_id}:{plan_id}")
}

pub(crate) fn bulk_witness_artifact_id(program_id: &str, plan_id: &str, ordinal: u64) -> String {
    format!("bulk-chunk-witness:{program_id}:{plan_id}:{ordinal}")
}

pub(crate) fn bulk_checkpoint_artifact_id(
    program_id: &str,
    plan_id: &str,
    checkpoint_sequence: u64,
) -> String {
    format!("bulk-checkpoint:{program_id}:{plan_id}:{checkpoint_sequence}")
}

pub(crate) fn bulk_witness_index_artifact_id(program_id: &str, plan_id: &str) -> String {
    format!("bulk-witness-index:{program_id}:{plan_id}")
}

pub(crate) fn digest_artifact_key(
    artifact_family: &AuthoritativeArtifactFamily,
    artifact_id: &str,
    canonicalization_version: u32,
) -> String {
    format!("{artifact_family:?}:{artifact_id}:v{canonicalization_version}")
}
