use std::collections::BTreeMap;

use crate::{
    backend::{
        integrity::{
            commit_artifact_id, commit_support_summary_artifact_id, parent_artifact_id,
            stable_structural_digest,
        },
        records::{
            AuthoritativeArtifactDigestRecord, AuthoritativeArtifactFamily, BranchHeadRecord,
            CommitParentRecord, CommitSupportSummaryRecord, DurableCursorIdentityRecord,
            LineageSupportRecord, SchemaSupportRecord, StableBasisRecord, StoredCommitEnvelope,
            SubscriberCheckpointRecord,
        },
    },
    failure::StoreError,
};

pub(super) fn insert_branch_artifact_digest(
    digests: &mut BTreeMap<String, AuthoritativeArtifactDigestRecord>,
    canonicalization_version: u32,
    branch_record: &crate::backend::records::BranchRecord,
) -> Result<(), StoreError> {
    let branch_digest = stable_structural_digest(branch_record)?;
    digests.insert(
        format!(
            "{:?}:{}:v{}",
            AuthoritativeArtifactFamily::BranchRecord,
            branch_record.branch_id.0,
            canonicalization_version
        ),
        AuthoritativeArtifactDigestRecord {
            artifact_family: AuthoritativeArtifactFamily::BranchRecord,
            artifact_id: branch_record.branch_id.0.clone(),
            canonicalization_version,
            digest_algorithm: "sha256".to_string(),
            artifact_digest: branch_digest,
        },
    );
    Ok(())
}

pub(super) fn insert_branch_head_artifact_digest(
    digests: &mut BTreeMap<String, AuthoritativeArtifactDigestRecord>,
    canonicalization_version: u32,
    branch_head_record: &BranchHeadRecord,
) -> Result<(), StoreError> {
    let head_digest = stable_structural_digest(branch_head_record)?;
    digests.insert(
        format!(
            "{:?}:{}:v{}",
            AuthoritativeArtifactFamily::BranchHeadRecord,
            branch_head_record.branch_id.0,
            canonicalization_version
        ),
        AuthoritativeArtifactDigestRecord {
            artifact_family: AuthoritativeArtifactFamily::BranchHeadRecord,
            artifact_id: branch_head_record.branch_id.0.clone(),
            canonicalization_version,
            digest_algorithm: "sha256".to_string(),
            artifact_digest: head_digest,
        },
    );
    Ok(())
}

pub(super) fn insert_commit_artifact_digests(
    digests: &mut BTreeMap<String, AuthoritativeArtifactDigestRecord>,
    canonicalization_version: u32,
    commit_envelopes: &[StoredCommitEnvelope],
) {
    for commit in commit_envelopes {
        let artifact_id = commit_artifact_id(commit.envelope.commit.commit_id);
        digests.insert(
            format!(
                "{:?}:{}:v{}",
                AuthoritativeArtifactFamily::CommitEnvelope,
                artifact_id,
                canonicalization_version
            ),
            AuthoritativeArtifactDigestRecord {
                artifact_family: AuthoritativeArtifactFamily::CommitEnvelope,
                artifact_id,
                canonicalization_version,
                digest_algorithm: "sha256".to_string(),
                artifact_digest: commit.envelope_digest.clone(),
            },
        );
    }
}

pub(super) fn insert_commit_parent_artifact_digests(
    digests: &mut BTreeMap<String, AuthoritativeArtifactDigestRecord>,
    canonicalization_version: u32,
    commit_parent_records: &[CommitParentRecord],
) -> Result<(), StoreError> {
    for parent in commit_parent_records {
        let digest = stable_structural_digest(parent)?;
        let artifact_id = parent_artifact_id(parent.commit_id, parent.parent_position);
        digests.insert(
            format!(
                "{:?}:{}:v{}",
                AuthoritativeArtifactFamily::CommitParentRecord,
                artifact_id,
                canonicalization_version
            ),
            AuthoritativeArtifactDigestRecord {
                artifact_family: AuthoritativeArtifactFamily::CommitParentRecord,
                artifact_id,
                canonicalization_version,
                digest_algorithm: "sha256".to_string(),
                artifact_digest: digest,
            },
        );
    }
    Ok(())
}

pub(super) fn insert_support_artifact_digests(
    digests: &mut BTreeMap<String, AuthoritativeArtifactDigestRecord>,
    canonicalization_version: u32,
    commit_support_summaries: &[CommitSupportSummaryRecord],
    schema_support_records: &[SchemaSupportRecord],
    lineage_support_records: &[LineageSupportRecord],
    durable_cursor_identity_records: &[DurableCursorIdentityRecord],
    subscriber_checkpoint_records: &[SubscriberCheckpointRecord],
    stable_basis_records: &[StableBasisRecord],
) -> Result<(), StoreError> {
    for summary in commit_support_summaries {
        let digest = stable_structural_digest(summary)?;
        let artifact_id = commit_support_summary_artifact_id(summary.commit_id);
        digests.insert(
            format!(
                "{:?}:{}:v{}",
                AuthoritativeArtifactFamily::CommitSupportSummary,
                artifact_id,
                canonicalization_version
            ),
            AuthoritativeArtifactDigestRecord {
                artifact_family: AuthoritativeArtifactFamily::CommitSupportSummary,
                artifact_id,
                canonicalization_version,
                digest_algorithm: "sha256".to_string(),
                artifact_digest: digest,
            },
        );
    }
    for record in schema_support_records {
        insert_structural_digest(
            digests,
            canonicalization_version,
            AuthoritativeArtifactFamily::SchemaSupportRecord,
            &record.artifact_id,
            record,
        )?;
    }
    for record in lineage_support_records {
        insert_structural_digest(
            digests,
            canonicalization_version,
            AuthoritativeArtifactFamily::LineageSupportRecord,
            &record.artifact_id,
            record,
        )?;
    }
    for record in durable_cursor_identity_records {
        insert_structural_digest(
            digests,
            canonicalization_version,
            AuthoritativeArtifactFamily::DurableCursorIdentityRecord,
            &record.artifact_id,
            record,
        )?;
    }
    for record in subscriber_checkpoint_records {
        insert_structural_digest(
            digests,
            canonicalization_version,
            AuthoritativeArtifactFamily::SubscriberCheckpointRecord,
            &record.artifact_id,
            record,
        )?;
    }
    for record in stable_basis_records {
        insert_structural_digest(
            digests,
            canonicalization_version,
            AuthoritativeArtifactFamily::StableBasisRecord,
            &record.artifact_id,
            record,
        )?;
    }
    Ok(())
}

fn insert_structural_digest<T: serde::Serialize>(
    digests: &mut BTreeMap<String, AuthoritativeArtifactDigestRecord>,
    canonicalization_version: u32,
    artifact_family: AuthoritativeArtifactFamily,
    artifact_id: &str,
    record: &T,
) -> Result<(), StoreError> {
    let digest = stable_structural_digest(record)?;
    digests.insert(
        format!(
            "{:?}:{}:v{}",
            artifact_family, artifact_id, canonicalization_version
        ),
        AuthoritativeArtifactDigestRecord {
            artifact_family,
            artifact_id: artifact_id.to_string(),
            canonicalization_version,
            digest_algorithm: "sha256".to_string(),
            artifact_digest: digest,
        },
    );
    Ok(())
}
