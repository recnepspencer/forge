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

pub(crate) fn digest_artifact_key(
    artifact_family: &AuthoritativeArtifactFamily,
    artifact_id: &str,
    canonicalization_version: u32,
) -> String {
    format!("{artifact_family:?}:{artifact_id}:v{canonicalization_version}")
}
