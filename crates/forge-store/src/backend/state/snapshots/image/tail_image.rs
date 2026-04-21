use std::collections::BTreeSet;

use forge_relational::facade::history::CommitId;

use crate::{
    backend::{
        integrity::{
            commit_artifact_id, commit_support_summary_artifact_id,
            durable_cursor_identity_artifact_id, lineage_support_artifact_id, parent_artifact_id,
            schema_support_artifact_id, stable_structural_digest,
            subscriber_checkpoint_artifact_id,
        },
        records::{
            AuthoritativeArtifactDigestRecord, AuthoritativeArtifactFamily, BranchHeadRecord,
            CommitParentRecord, StoreState,
        },
    },
    failure::{StoreError, StoreErrorKind},
    snapshot::{SnapshotId, SnapshotImageBundle},
};

impl StoreState {
    pub fn build_snapshot_tail_image(
        &self,
        snapshot_id: SnapshotId,
        target_commit_id: CommitId,
    ) -> Result<(SnapshotImageBundle, usize), StoreError> {
        let basis = self.snapshot_basis(snapshot_id)?;
        self.require_snapshot_restore_target(&basis, target_commit_id)?;
        if target_commit_id == basis.snapshot_frontier_commit_id {
            return Ok((self.snapshot_image(snapshot_id)?, 0));
        }

        let mut export = self
            .snapshot_image(snapshot_id)?
            .authoritative_export()
            .clone();
        let prefix_commit_ids: BTreeSet<_> = basis.snapshot_history_range.iter().copied().collect();
        let target_history_range = self.snapshot_history_range(target_commit_id)?;
        let tail_commit_ids = target_history_range
            .into_iter()
            .filter(|commit_id| !prefix_commit_ids.contains(commit_id))
            .collect::<Vec<_>>();
        let target_record = self.commit_record(target_commit_id).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::SnapshotRestoreTargetIllegal,
                format!("target commit {} does not exist", target_commit_id.0),
            )
        })?;

        for commit_id in &tail_commit_ids {
            extend_export_for_commit(self, &basis.snapshot_branch_id, *commit_id, &mut export)?;
        }

        let target_head = BranchHeadRecord {
            branch_id: basis.snapshot_branch_id.clone(),
            head_commit_id: Some(target_commit_id),
            head_commit_digest: Some(target_record.envelope_digest.clone()),
            head_update_sequence: target_record.commit_sequence,
        };
        export.branch_head_records = vec![target_head.clone()];
        export.authoritative_artifact_digests.retain(|record| {
            !(record.artifact_family == AuthoritativeArtifactFamily::BranchHeadRecord
                && record.artifact_id == basis.snapshot_branch_id.0)
        });
        export
            .authoritative_artifact_digests
            .push(AuthoritativeArtifactDigestRecord {
                artifact_family: AuthoritativeArtifactFamily::BranchHeadRecord,
                artifact_id: basis.snapshot_branch_id.0.clone(),
                canonicalization_version: self.canonicalization_version,
                digest_algorithm: "sha256".to_string(),
                artifact_digest: stable_structural_digest(&target_head)?,
            });
        export.canonicalize_order();
        Ok((SnapshotImageBundle::new(export), tail_commit_ids.len()))
    }
}

fn extend_export_for_commit(
    state: &StoreState,
    branch_id: &forge_relational::facade::history::BranchId,
    commit_id: CommitId,
    export: &mut crate::authority::AuthoritativeExportBundle,
) -> Result<(), StoreError> {
    let commit = state.commit_record(commit_id).cloned().ok_or_else(|| {
        StoreError::backend_integrity("snapshot tail commit missing during replay")
    })?;
    export.commit_envelopes.push(commit.clone());
    export
        .authoritative_artifact_digests
        .push(AuthoritativeArtifactDigestRecord {
            artifact_family: AuthoritativeArtifactFamily::CommitEnvelope,
            artifact_id: commit_artifact_id(commit.envelope.commit.commit_id),
            canonicalization_version: state.canonicalization_version,
            digest_algorithm: "sha256".to_string(),
            artifact_digest: commit.envelope_digest,
        });

    for (parent_position, parent_commit_id) in
        commit.envelope.commit.parents.iter().copied().enumerate()
    {
        let parent = CommitParentRecord {
            commit_id,
            parent_position,
            parent_commit_id,
        };
        let digest = stable_structural_digest(&parent)?;
        export.commit_parent_records.push(parent);
        export
            .authoritative_artifact_digests
            .push(AuthoritativeArtifactDigestRecord {
                artifact_family: AuthoritativeArtifactFamily::CommitParentRecord,
                artifact_id: parent_artifact_id(commit_id, parent_position),
                canonicalization_version: state.canonicalization_version,
                digest_algorithm: "sha256".to_string(),
                artifact_digest: digest,
            });
    }
    extend_support_records(state, branch_id, commit_id, export)
}

fn extend_support_records(
    state: &StoreState,
    branch_id: &forge_relational::facade::history::BranchId,
    commit_id: CommitId,
    export: &mut crate::authority::AuthoritativeExportBundle,
) -> Result<(), StoreError> {
    if let Some(summary) = state.commit_support_summaries.get(&commit_id.0).cloned() {
        let digest = stable_structural_digest(&summary)?;
        export.commit_support_summaries.push(summary.clone());
        export
            .authoritative_artifact_digests
            .push(AuthoritativeArtifactDigestRecord {
                artifact_family: AuthoritativeArtifactFamily::CommitSupportSummary,
                artifact_id: commit_support_summary_artifact_id(commit_id),
                canonicalization_version: state.canonicalization_version,
                digest_algorithm: "sha256".to_string(),
                artifact_digest: digest,
            });
    }
    if let Some(record) = state
        .schema_support_records
        .get(&schema_support_artifact_id(commit_id))
        .cloned()
    {
        let digest = stable_structural_digest(&record)?;
        export.schema_support_records.push(record.clone());
        export
            .authoritative_artifact_digests
            .push(AuthoritativeArtifactDigestRecord {
                artifact_family: AuthoritativeArtifactFamily::SchemaSupportRecord,
                artifact_id: record.artifact_id.clone(),
                canonicalization_version: state.canonicalization_version,
                digest_algorithm: "sha256".to_string(),
                artifact_digest: digest,
            });
    }
    if let Some(record) = state
        .lineage_support_records
        .get(&lineage_support_artifact_id(commit_id))
        .cloned()
    {
        let digest = stable_structural_digest(&record)?;
        export.lineage_support_records.push(record.clone());
        export
            .authoritative_artifact_digests
            .push(AuthoritativeArtifactDigestRecord {
                artifact_family: AuthoritativeArtifactFamily::LineageSupportRecord,
                artifact_id: record.artifact_id.clone(),
                canonicalization_version: state.canonicalization_version,
                digest_algorithm: "sha256".to_string(),
                artifact_digest: digest,
            });
    }
    extend_cursor_records(state, branch_id, commit_id, export)
}

fn extend_cursor_records(
    state: &StoreState,
    branch_id: &forge_relational::facade::history::BranchId,
    commit_id: CommitId,
    export: &mut crate::authority::AuthoritativeExportBundle,
) -> Result<(), StoreError> {
    for record in state
        .durable_cursor_identity_records
        .values()
        .filter(|record| {
            record.branch_id == *branch_id && record.latest_basis_commit_id == commit_id
        })
        .cloned()
        .collect::<Vec<_>>()
    {
        let digest = stable_structural_digest(&record)?;
        export.durable_cursor_identity_records.push(record.clone());
        export
            .authoritative_artifact_digests
            .push(AuthoritativeArtifactDigestRecord {
                artifact_family: AuthoritativeArtifactFamily::DurableCursorIdentityRecord,
                artifact_id: durable_cursor_identity_artifact_id(&record.cursor_id),
                canonicalization_version: state.canonicalization_version,
                digest_algorithm: "sha256".to_string(),
                artifact_digest: digest,
            });
    }
    for record in state
        .subscriber_checkpoint_records
        .values()
        .filter(|record| record.branch_id == *branch_id && record.basis_commit_id == commit_id)
        .cloned()
        .collect::<Vec<_>>()
    {
        let digest = stable_structural_digest(&record)?;
        export.subscriber_checkpoint_records.push(record.clone());
        export
            .authoritative_artifact_digests
            .push(AuthoritativeArtifactDigestRecord {
                artifact_family: AuthoritativeArtifactFamily::SubscriberCheckpointRecord,
                artifact_id: subscriber_checkpoint_artifact_id(
                    &record.cursor_id,
                    record.checkpoint_sequence,
                ),
                canonicalization_version: state.canonicalization_version,
                digest_algorithm: "sha256".to_string(),
                artifact_digest: digest,
            });
    }
    Ok(())
}

pub(crate) fn snapshot_image_record_count(image: &SnapshotImageBundle) -> usize {
    image.authoritative_export().commit_envelopes.len()
        + image.authoritative_export().commit_parent_records.len()
        + image.authoritative_export().commit_support_summaries.len()
        + image.authoritative_export().schema_support_records.len()
        + image.authoritative_export().lineage_support_records.len()
        + image
            .authoritative_export()
            .durable_cursor_identity_records
            .len()
        + image
            .authoritative_export()
            .subscriber_checkpoint_records
            .len()
        + image.authoritative_export().branch_records.len()
        + image.authoritative_export().branch_head_records.len()
        + image
            .authoritative_export()
            .authoritative_artifact_digests
            .len()
}
