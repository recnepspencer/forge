use std::collections::BTreeMap;

use crate::{authority::AuthoritativeExportBundle, failure::StoreError};

use crate::backend::{
    integrity::{
        commit_artifact_id, commit_support_summary_artifact_id,
        durable_cursor_identity_artifact_id, parent_artifact_id, stable_structural_digest,
        subscriber_checkpoint_artifact_id,
    },
    records::{AuthoritativeArtifactDigestRecord, AuthoritativeArtifactFamily, StoreState},
};

impl StoreState {
    pub(crate) fn rebuild_authoritative_export_digests(
        &self,
        export: &AuthoritativeExportBundle,
    ) -> Result<BTreeMap<String, AuthoritativeArtifactDigestRecord>, StoreError> {
        let mut digests = BTreeMap::new();
        for branch_record in &export.branch_records {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::BranchRecord,
                branch_record.branch_id.0.clone(),
                self.canonicalization_version,
                stable_structural_digest(branch_record)?,
            );
        }
        for branch_head_record in &export.branch_head_records {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::BranchHeadRecord,
                branch_head_record.branch_id.0.clone(),
                self.canonicalization_version,
                stable_structural_digest(branch_head_record)?,
            );
        }
        for commit_record in &export.commit_envelopes {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::CommitEnvelope,
                commit_artifact_id(commit_record.envelope.commit.commit_id),
                self.canonicalization_version,
                commit_record.envelope_digest.clone(),
            );
        }
        for parent_record in &export.commit_parent_records {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::CommitParentRecord,
                parent_artifact_id(parent_record.commit_id, parent_record.parent_position),
                self.canonicalization_version,
                stable_structural_digest(parent_record)?,
            );
        }
        for summary in &export.commit_support_summaries {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::CommitSupportSummary,
                commit_support_summary_artifact_id(summary.commit_id),
                self.canonicalization_version,
                stable_structural_digest(summary)?,
            );
        }
        for record in &export.schema_support_records {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::SchemaSupportRecord,
                record.artifact_id.clone(),
                self.canonicalization_version,
                stable_structural_digest(record)?,
            );
        }
        for record in &export.lineage_support_records {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::LineageSupportRecord,
                record.artifact_id.clone(),
                self.canonicalization_version,
                stable_structural_digest(record)?,
            );
        }
        for record in &export.durable_cursor_identity_records {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::DurableCursorIdentityRecord,
                durable_cursor_identity_artifact_id(&record.cursor_id),
                self.canonicalization_version,
                stable_structural_digest(record)?,
            );
        }
        for record in &export.subscriber_checkpoint_records {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::SubscriberCheckpointRecord,
                subscriber_checkpoint_artifact_id(&record.cursor_id, record.checkpoint_sequence),
                self.canonicalization_version,
                stable_structural_digest(record)?,
            );
        }
        Ok(digests)
    }
}

pub(super) fn empty_authoritative_export(
    canonicalization_version: u32,
) -> AuthoritativeExportBundle {
    AuthoritativeExportBundle {
        canonicalization_version,
        branch_records: Vec::new(),
        branch_head_records: Vec::new(),
        commit_envelopes: Vec::new(),
        commit_parent_records: Vec::new(),
        commit_support_summaries: Vec::new(),
        schema_support_records: Vec::new(),
        lineage_support_records: Vec::new(),
        durable_cursor_identity_records: Vec::new(),
        subscriber_checkpoint_records: Vec::new(),
        stable_basis_records: Vec::new(),
        authoritative_artifact_digests: Vec::new(),
    }
}

fn insert_digest_record(
    digests: &mut BTreeMap<String, AuthoritativeArtifactDigestRecord>,
    artifact_family: AuthoritativeArtifactFamily,
    artifact_id: String,
    canonicalization_version: u32,
    artifact_digest: String,
) {
    let key = format!(
        "{:?}:{}:v{}",
        artifact_family, artifact_id, canonicalization_version
    );
    digests.insert(
        key,
        AuthoritativeArtifactDigestRecord {
            artifact_family,
            artifact_id,
            canonicalization_version,
            digest_algorithm: "sha256".to_string(),
            artifact_digest,
        },
    );
}
