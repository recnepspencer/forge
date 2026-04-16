use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::history::{BranchId, CommitId};

use crate::{
    authority::AuthoritativeExportBundle,
    backend::{
        integrity::{
            branch_key, commit_artifact_id, commit_support_summary_artifact_id,
            durable_cursor_identity_artifact_id, lineage_support_artifact_id, parent_artifact_id,
            schema_support_artifact_id, stable_structural_digest,
            subscriber_checkpoint_artifact_id,
        },
        records::{
            AuthoritativeArtifactDigestRecord, AuthoritativeArtifactFamily, BranchHeadRecord,
            CommitParentRecord, CommitSupportSummaryRecord, DurableCursorIdentityRecord,
            LineageSupportRecord, SchemaSupportRecord, StoreState, SubscriberCheckpointRecord,
        },
    },
    failure::{StoreError, StoreErrorKind},
    publication::{
        admit_local_snapshot_basis_source, admit_local_snapshot_image_source,
        classify_snapshot_publication, PublicationClassification,
    },
    snapshot::{stable_snapshot_digest, SnapshotId, SnapshotImageBundle},
};

impl StoreState {
    pub fn snapshot_image(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotImageBundle, StoreError> {
        let basis = self.snapshot_basis_records.get(&snapshot_id.0).cloned();
        let record = self.snapshot_image_records.get(&snapshot_id.0).cloned();
        let publication = classify_snapshot_publication(
            crate::media::DurableMediaReport::new(
                crate::media::DurableBackendFamily::InMemory,
                crate::media::DurabilityBarrierClass::FileContentDurable,
                crate::media::DurabilityBarrierClass::FileAndRequiredMetadataDurable,
                crate::media::DurabilityBarrierClass::FileContentDurable,
            ),
            basis.clone(),
            record.clone(),
        )?;
        match publication.classification() {
            PublicationClassification::RequireRebuild => {
                return Err(StoreError::new(
                    StoreErrorKind::SnapshotPublicationStateGap,
                    format!(
                        "snapshot {} image missing while basis exists",
                        snapshot_id.0
                    ),
                ));
            }
            PublicationClassification::RequireQuarantine => {
                return Err(StoreError::new(
                    StoreErrorKind::SnapshotPublicationStateGap,
                    format!(
                        "snapshot {} image exists without an admitted basis",
                        snapshot_id.0
                    ),
                ));
            }
            PublicationClassification::DiscardUnpublished => {
                return Err(StoreError::new(
                    StoreErrorKind::SnapshotBasisUnsupported,
                    format!("snapshot {} basis not found", snapshot_id.0),
                ));
            }
            PublicationClassification::FinishPublication
            | PublicationClassification::RetainTrusted => {}
        }
        let basis = admit_local_snapshot_basis_source(
            basis.expect("published snapshot should include basis record"),
        )?
        .into_inner();
        let record = admit_local_snapshot_image_source(
            record.expect("published snapshot should include image record"),
        )?
        .into_inner();
        let digest = stable_snapshot_digest(&record.image);
        if digest != basis.snapshot_image_digest {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotDigestMismatch,
                format!(
                    "snapshot {} image digest {} did not match basis {}",
                    snapshot_id.0, digest, basis.snapshot_image_digest
                ),
            ));
        }
        Ok(record.image.clone())
    }

    #[cfg(test)]
    pub fn remove_snapshot_image(&mut self, snapshot_id: SnapshotId) {
        self.snapshot_image_records.remove(&snapshot_id.0);
    }

    pub fn build_snapshot_image(
        &self,
        branch_id: &BranchId,
        frontier_commit_id: CommitId,
    ) -> Result<SnapshotImageBundle, StoreError> {
        let frontier_record = self.commit_record(frontier_commit_id).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::SnapshotBasisUnsupported,
                format!("frontier commit {} not found", frontier_commit_id.0),
            )
        })?;
        if &frontier_record.envelope.branch_context != branch_id {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotBasisAmbiguous,
                format!(
                    "frontier commit {} belongs to branch `{}` not requested branch `{}`",
                    frontier_commit_id.0, frontier_record.envelope.branch_context.0, branch_id.0
                ),
            ));
        }
        let branch_record = self
            .branch_records
            .get(&branch_key(branch_id))
            .cloned()
            .ok_or_else(|| StoreError::unknown_branch(branch_id))?;
        let history_range = self.snapshot_history_range(frontier_commit_id)?;
        let commit_set: BTreeSet<_> = history_range.iter().copied().collect();

        let commit_envelopes = history_range
            .iter()
            .map(|commit_id| {
                self.commit_record(*commit_id).cloned().ok_or_else(|| {
                    StoreError::backend_integrity("snapshot history range commit missing")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let commit_parent_records = history_range
            .iter()
            .flat_map(|commit_id| {
                self.commit_record(*commit_id)
                    .into_iter()
                    .flat_map(|record| record.envelope.commit.parents.iter().copied().enumerate())
                    .map(
                        move |(parent_position, parent_commit_id)| CommitParentRecord {
                            commit_id: *commit_id,
                            parent_position,
                            parent_commit_id,
                        },
                    )
            })
            .filter(|record| commit_set.contains(&record.parent_commit_id))
            .collect::<Vec<_>>();
        let commit_support_summaries = history_range
            .iter()
            .filter_map(|commit_id| self.commit_support_summaries.get(&commit_id.0).cloned())
            .collect::<Vec<CommitSupportSummaryRecord>>();
        let schema_support_records = history_range
            .iter()
            .filter_map(|commit_id| {
                self.schema_support_records
                    .get(&schema_support_artifact_id(*commit_id))
                    .cloned()
            })
            .collect::<Vec<SchemaSupportRecord>>();
        let lineage_support_records = history_range
            .iter()
            .filter_map(|commit_id| {
                self.lineage_support_records
                    .get(&lineage_support_artifact_id(*commit_id))
                    .cloned()
            })
            .collect::<Vec<LineageSupportRecord>>();
        let durable_cursor_identity_records = self
            .durable_cursor_identity_records
            .values()
            .filter(|record| {
                record.branch_id == *branch_id
                    && commit_set.contains(&record.latest_basis_commit_id)
            })
            .cloned()
            .collect::<Vec<DurableCursorIdentityRecord>>();
        let subscriber_checkpoint_records = self
            .subscriber_checkpoint_records
            .values()
            .filter(|record| {
                record.branch_id == *branch_id && commit_set.contains(&record.basis_commit_id)
            })
            .cloned()
            .collect::<Vec<SubscriberCheckpointRecord>>();

        let branch_head_record = BranchHeadRecord {
            branch_id: branch_id.clone(),
            head_commit_id: Some(frontier_commit_id),
            head_commit_digest: Some(frontier_record.envelope_digest.clone()),
            head_update_sequence: frontier_record.commit_sequence,
        };

        let mut authoritative_artifact_digests = BTreeMap::new();
        let branch_digest = stable_structural_digest(&branch_record)?;
        authoritative_artifact_digests.insert(
            format!(
                "{:?}:{}:v{}",
                AuthoritativeArtifactFamily::BranchRecord,
                branch_record.branch_id.0,
                self.canonicalization_version
            ),
            AuthoritativeArtifactDigestRecord {
                artifact_family: AuthoritativeArtifactFamily::BranchRecord,
                artifact_id: branch_record.branch_id.0.clone(),
                canonicalization_version: self.canonicalization_version,
                digest_algorithm: "sha256".to_string(),
                artifact_digest: branch_digest,
            },
        );
        let head_digest = stable_structural_digest(&branch_head_record)?;
        authoritative_artifact_digests.insert(
            format!(
                "{:?}:{}:v{}",
                AuthoritativeArtifactFamily::BranchHeadRecord,
                branch_head_record.branch_id.0,
                self.canonicalization_version
            ),
            AuthoritativeArtifactDigestRecord {
                artifact_family: AuthoritativeArtifactFamily::BranchHeadRecord,
                artifact_id: branch_head_record.branch_id.0.clone(),
                canonicalization_version: self.canonicalization_version,
                digest_algorithm: "sha256".to_string(),
                artifact_digest: head_digest,
            },
        );
        for commit in &commit_envelopes {
            let artifact_id = commit_artifact_id(commit.envelope.commit.commit_id);
            authoritative_artifact_digests.insert(
                format!(
                    "{:?}:{}:v{}",
                    AuthoritativeArtifactFamily::CommitEnvelope,
                    artifact_id,
                    self.canonicalization_version
                ),
                AuthoritativeArtifactDigestRecord {
                    artifact_family: AuthoritativeArtifactFamily::CommitEnvelope,
                    artifact_id,
                    canonicalization_version: self.canonicalization_version,
                    digest_algorithm: "sha256".to_string(),
                    artifact_digest: commit.envelope_digest.clone(),
                },
            );
        }
        for parent in &commit_parent_records {
            let digest = stable_structural_digest(parent)?;
            let artifact_id = parent_artifact_id(parent.commit_id, parent.parent_position);
            authoritative_artifact_digests.insert(
                format!(
                    "{:?}:{}:v{}",
                    AuthoritativeArtifactFamily::CommitParentRecord,
                    artifact_id,
                    self.canonicalization_version
                ),
                AuthoritativeArtifactDigestRecord {
                    artifact_family: AuthoritativeArtifactFamily::CommitParentRecord,
                    artifact_id,
                    canonicalization_version: self.canonicalization_version,
                    digest_algorithm: "sha256".to_string(),
                    artifact_digest: digest,
                },
            );
        }
        for summary in &commit_support_summaries {
            let digest = stable_structural_digest(summary)?;
            let artifact_id = commit_support_summary_artifact_id(summary.commit_id);
            authoritative_artifact_digests.insert(
                format!(
                    "{:?}:{}:v{}",
                    AuthoritativeArtifactFamily::CommitSupportSummary,
                    artifact_id,
                    self.canonicalization_version
                ),
                AuthoritativeArtifactDigestRecord {
                    artifact_family: AuthoritativeArtifactFamily::CommitSupportSummary,
                    artifact_id,
                    canonicalization_version: self.canonicalization_version,
                    digest_algorithm: "sha256".to_string(),
                    artifact_digest: digest,
                },
            );
        }
        for record in &schema_support_records {
            let digest = stable_structural_digest(record)?;
            authoritative_artifact_digests.insert(
                format!(
                    "{:?}:{}:v{}",
                    AuthoritativeArtifactFamily::SchemaSupportRecord,
                    record.artifact_id,
                    self.canonicalization_version
                ),
                AuthoritativeArtifactDigestRecord {
                    artifact_family: AuthoritativeArtifactFamily::SchemaSupportRecord,
                    artifact_id: record.artifact_id.clone(),
                    canonicalization_version: self.canonicalization_version,
                    digest_algorithm: "sha256".to_string(),
                    artifact_digest: digest,
                },
            );
        }
        for record in &lineage_support_records {
            let digest = stable_structural_digest(record)?;
            authoritative_artifact_digests.insert(
                format!(
                    "{:?}:{}:v{}",
                    AuthoritativeArtifactFamily::LineageSupportRecord,
                    record.artifact_id,
                    self.canonicalization_version
                ),
                AuthoritativeArtifactDigestRecord {
                    artifact_family: AuthoritativeArtifactFamily::LineageSupportRecord,
                    artifact_id: record.artifact_id.clone(),
                    canonicalization_version: self.canonicalization_version,
                    digest_algorithm: "sha256".to_string(),
                    artifact_digest: digest,
                },
            );
        }
        for record in &durable_cursor_identity_records {
            let digest = stable_structural_digest(record)?;
            authoritative_artifact_digests.insert(
                format!(
                    "{:?}:{}:v{}",
                    AuthoritativeArtifactFamily::DurableCursorIdentityRecord,
                    record.artifact_id,
                    self.canonicalization_version
                ),
                AuthoritativeArtifactDigestRecord {
                    artifact_family: AuthoritativeArtifactFamily::DurableCursorIdentityRecord,
                    artifact_id: record.artifact_id.clone(),
                    canonicalization_version: self.canonicalization_version,
                    digest_algorithm: "sha256".to_string(),
                    artifact_digest: digest,
                },
            );
        }
        for record in &subscriber_checkpoint_records {
            let digest = stable_structural_digest(record)?;
            authoritative_artifact_digests.insert(
                format!(
                    "{:?}:{}:v{}",
                    AuthoritativeArtifactFamily::SubscriberCheckpointRecord,
                    record.artifact_id,
                    self.canonicalization_version
                ),
                AuthoritativeArtifactDigestRecord {
                    artifact_family: AuthoritativeArtifactFamily::SubscriberCheckpointRecord,
                    artifact_id: record.artifact_id.clone(),
                    canonicalization_version: self.canonicalization_version,
                    digest_algorithm: "sha256".to_string(),
                    artifact_digest: digest,
                },
            );
        }
        let mut bundle = AuthoritativeExportBundle {
            canonicalization_version: self.canonicalization_version,
            branch_records: vec![branch_record],
            branch_head_records: vec![branch_head_record],
            commit_envelopes,
            commit_parent_records,
            commit_support_summaries,
            schema_support_records,
            lineage_support_records,
            durable_cursor_identity_records,
            subscriber_checkpoint_records,
            authoritative_artifact_digests: authoritative_artifact_digests.into_values().collect(),
        };
        bundle.canonicalize_order();
        Ok(SnapshotImageBundle::new(bundle))
    }

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
            let commit = self.commit_record(*commit_id).cloned().ok_or_else(|| {
                StoreError::backend_integrity("snapshot tail commit missing during replay")
            })?;
            export.commit_envelopes.push(commit.clone());
            export
                .authoritative_artifact_digests
                .push(AuthoritativeArtifactDigestRecord {
                    artifact_family: AuthoritativeArtifactFamily::CommitEnvelope,
                    artifact_id: commit_artifact_id(commit.envelope.commit.commit_id),
                    canonicalization_version: self.canonicalization_version,
                    digest_algorithm: "sha256".to_string(),
                    artifact_digest: commit.envelope_digest,
                });

            for (parent_position, parent_commit_id) in
                commit.envelope.commit.parents.iter().copied().enumerate()
            {
                let parent = CommitParentRecord {
                    commit_id: *commit_id,
                    parent_position,
                    parent_commit_id,
                };
                let digest = stable_structural_digest(&parent)?;
                export.commit_parent_records.push(parent);
                export
                    .authoritative_artifact_digests
                    .push(AuthoritativeArtifactDigestRecord {
                        artifact_family: AuthoritativeArtifactFamily::CommitParentRecord,
                        artifact_id: parent_artifact_id(*commit_id, parent_position),
                        canonicalization_version: self.canonicalization_version,
                        digest_algorithm: "sha256".to_string(),
                        artifact_digest: digest,
                    });
            }
            if let Some(summary) = self.commit_support_summaries.get(&commit_id.0).cloned() {
                let digest = stable_structural_digest(&summary)?;
                export.commit_support_summaries.push(summary.clone());
                export
                    .authoritative_artifact_digests
                    .push(AuthoritativeArtifactDigestRecord {
                        artifact_family: AuthoritativeArtifactFamily::CommitSupportSummary,
                        artifact_id: commit_support_summary_artifact_id(*commit_id),
                        canonicalization_version: self.canonicalization_version,
                        digest_algorithm: "sha256".to_string(),
                        artifact_digest: digest,
                    });
            }
            if let Some(record) = self
                .schema_support_records
                .get(&schema_support_artifact_id(*commit_id))
                .cloned()
            {
                let digest = stable_structural_digest(&record)?;
                export.schema_support_records.push(record.clone());
                export
                    .authoritative_artifact_digests
                    .push(AuthoritativeArtifactDigestRecord {
                        artifact_family: AuthoritativeArtifactFamily::SchemaSupportRecord,
                        artifact_id: record.artifact_id.clone(),
                        canonicalization_version: self.canonicalization_version,
                        digest_algorithm: "sha256".to_string(),
                        artifact_digest: digest,
                    });
            }
            if let Some(record) = self
                .lineage_support_records
                .get(&lineage_support_artifact_id(*commit_id))
                .cloned()
            {
                let digest = stable_structural_digest(&record)?;
                export.lineage_support_records.push(record.clone());
                export
                    .authoritative_artifact_digests
                    .push(AuthoritativeArtifactDigestRecord {
                        artifact_family: AuthoritativeArtifactFamily::LineageSupportRecord,
                        artifact_id: record.artifact_id.clone(),
                        canonicalization_version: self.canonicalization_version,
                        digest_algorithm: "sha256".to_string(),
                        artifact_digest: digest,
                    });
            }
            for record in self
                .durable_cursor_identity_records
                .values()
                .filter(|record| {
                    record.branch_id == basis.snapshot_branch_id
                        && record.latest_basis_commit_id == *commit_id
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
                        canonicalization_version: self.canonicalization_version,
                        digest_algorithm: "sha256".to_string(),
                        artifact_digest: digest,
                    });
            }
            for record in self
                .subscriber_checkpoint_records
                .values()
                .filter(|record| {
                    record.branch_id == basis.snapshot_branch_id
                        && record.basis_commit_id == *commit_id
                })
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
                        canonicalization_version: self.canonicalization_version,
                        digest_algorithm: "sha256".to_string(),
                        artifact_digest: digest,
                    });
            }
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
