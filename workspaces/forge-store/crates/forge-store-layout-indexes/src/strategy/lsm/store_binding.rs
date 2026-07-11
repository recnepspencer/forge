use std::path::Path;

use super::super::BaselineLsmExecutionAdmissionDenial;

pub(super) struct ParsedManifestMembership {
    pub(super) checkpoint_epoch: u64,
    pub(super) covered_lsn_start: u64,
    pub(super) covered_lsn_end: u64,
    pub(super) digest: String,
}

pub(super) fn manifest_membership(
    path: &Path,
    expected_bytes: u64,
    wal_root: &Path,
) -> Result<ParsedManifestMembership, BaselineLsmExecutionAdmissionDenial> {
    const PREFIX: &[u8] = b"forge-store:baseline-lsm-manifest:v1\0";
    let artifact = std::fs::read(path)
        .map_err(|_| BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch)?;
    if !super::artifact_belongs_to_wal_root(path, wal_root)
        || artifact.len() as u64 != expected_bytes
        || artifact.len() < PREFIX.len() + 24
        || !artifact.starts_with(PREFIX)
    {
        return Err(BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch);
    }
    let number_at = |offset: usize| {
        artifact[offset..offset + 8]
            .try_into()
            .map(u64::from_le_bytes)
            .map_err(|_| BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch)
    };
    Ok(ParsedManifestMembership {
        checkpoint_epoch: number_at(PREFIX.len())?,
        covered_lsn_start: number_at(PREFIX.len() + 8)?,
        covered_lsn_end: number_at(PREFIX.len() + 16)?,
        digest: String::from_utf8(artifact[PREFIX.len() + 24..].to_vec())
            .map_err(|_| BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch)?,
    })
}

pub(super) fn store_binding(
    artifact_root: &Path,
    segment: u64,
    generation: u64,
) -> Result<String, BaselineLsmExecutionAdmissionDenial> {
    let root = std::fs::canonicalize(artifact_root)
        .map_err(|_| BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)?;
    Ok(format!(
        "{}:{segment}:{generation}",
        super::persisted_codec::hex(root.as_os_str().to_string_lossy().as_bytes())
    ))
}
