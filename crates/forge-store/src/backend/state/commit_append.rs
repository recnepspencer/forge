use crate::authority::{CommitCoupledSupportAppendWitness, VerifiedAuthoritativeAppend};
use crate::failure::StoreError;
use forge_relational::facade::history::CommitId;

use crate::backend::{
    integrity::{
        branch_key, commit_artifact_id, commit_support_summary_artifact_id, digest_artifact_key,
        lineage_support_artifact_id, parent_artifact_id, schema_support_artifact_id,
        stable_structural_digest,
    },
    records::{
        AuthoritativeArtifactFamily, BranchHeadRecord, BranchRecord, CommitParentRecord,
        CommitSupportSummaryRecord, LineageSupportRecord, SchemaSupportRecord, StoreState,
        StoredCommitEnvelope,
    },
};

#[derive(Debug)]
pub(crate) struct AppliedAuthoritativeAppend {
    branch_identity: String,
    commit_id: CommitId,
    parent_count: usize,
    created_branch: bool,
    previous_next_commit_sequence: u64,
    previous_next_head_update_sequence: u64,
    previous_branch_record: Option<BranchRecord>,
    previous_branch_head_record: Option<BranchHeadRecord>,
    inserted_support_summary: bool,
    inserted_schema_support: bool,
    inserted_lineage_support: bool,
    inserted_branch_delta_layer_id: Option<u64>,
}

impl StoreState {
    pub fn has_commit(&self, commit_id: CommitId) -> bool {
        self.commit_envelopes.contains_key(&commit_id.0)
    }

    pub fn apply_verified_append_in_place(
        &mut self,
        verified: &VerifiedAuthoritativeAppend,
    ) -> Result<AppliedAuthoritativeAppend, StoreError> {
        let envelope = verified.envelope();
        let commit_id = envelope.commit.commit_id;
        let branch_identity = branch_key(&envelope.branch_context);
        let previous_branch_record = self.branch_records.get(&branch_identity).cloned();
        let previous_branch_head_record = self.branch_head_records.get(&branch_identity).cloned();
        let created_branch = previous_branch_record.is_none();

        if created_branch {
            self.branch_records.insert(
                branch_identity.clone(),
                BranchRecord {
                    branch_id: envelope.branch_context.clone(),
                    created_from_branch: None,
                    created_from_commit_id: None,
                    created_at_commit_sequence: Some(self.next_commit_sequence),
                },
            );
            self.branch_head_records.insert(
                branch_identity.clone(),
                BranchHeadRecord {
                    branch_id: envelope.branch_context.clone(),
                    head_commit_id: None,
                    head_commit_digest: None,
                    head_update_sequence: 0,
                },
            );
            self.upsert_digest_record(
                AuthoritativeArtifactFamily::BranchRecord,
                branch_identity.clone(),
                stable_structural_digest(&self.branch_records[&branch_identity])?,
            );
            self.upsert_digest_record(
                AuthoritativeArtifactFamily::BranchHeadRecord,
                branch_identity.clone(),
                stable_structural_digest(&self.branch_head_records[&branch_identity])?,
            );
        }

        let previous_next_commit_sequence = self.next_commit_sequence;
        let previous_next_head_update_sequence = self.next_head_update_sequence;
        let commit_sequence = self.next_commit_sequence;
        self.next_commit_sequence += 1;
        let head_update_sequence = self.next_head_update_sequence;
        self.next_head_update_sequence += 1;

        self.commit_envelopes.insert(
            commit_id.0,
            StoredCommitEnvelope {
                envelope: envelope.clone(),
                envelope_digest: verified.digest().as_str().to_string(),
                canonicalization_version: verified.canonicalization_version(),
                commit_sequence,
            },
        );
        self.upsert_digest_record(
            AuthoritativeArtifactFamily::CommitEnvelope,
            commit_artifact_id(commit_id),
            verified.digest().as_str().to_string(),
        );

        let schema_support_artifact_id = schema_support_artifact_id(commit_id);
        let lineage_support_artifact_id = lineage_support_artifact_id(commit_id);
        let schema_support_record = if envelope.schema_transition.is_some()
            || envelope.schema_continuation_descriptor.is_some()
            || envelope.schema_reconciliation_descriptor.is_some()
        {
            Some(SchemaSupportRecord {
                artifact_id: schema_support_artifact_id.clone(),
                commit_id,
                branch_id: envelope.branch_context.clone(),
                schema_version_id: envelope.schema_version,
                descriptor_semantics_version: envelope.descriptor_semantics_version,
                schema_transition: envelope.schema_transition.clone(),
                schema_continuation_descriptor: envelope.schema_continuation_descriptor.clone(),
                schema_reconciliation_descriptor: envelope.schema_reconciliation_descriptor.clone(),
            })
        } else {
            None
        };

        let lineage_support_record =
            if !envelope.lineage_event_ids().is_empty() || !envelope.lineage_events().is_empty() {
                Some(LineageSupportRecord {
                    artifact_id: lineage_support_artifact_id.clone(),
                    commit_id,
                    branch_id: envelope.branch_context.clone(),
                    lineage_event_ids: envelope.lineage_event_ids().to_vec(),
                    lineage_events: envelope.lineage_events().to_vec(),
                    lineage_digest_basis: envelope.lineage_digest_basis().clone(),
                    event_batch_digest_basis: envelope.event_batch_digest_basis().clone(),
                    decision_log_digest_basis: envelope.decision_log_digest_basis().clone(),
                    lineage_artifact_counters: envelope.lineage_artifact_counters(),
                })
            } else {
                None
            };
        let support_append_witness = CommitCoupledSupportAppendWitness::new(
            commit_id,
            envelope.branch_context.clone(),
            schema_support_record.is_some(),
            lineage_support_record.is_some(),
        );

        let support_summary = CommitSupportSummaryRecord {
            commit_id: support_append_witness.commit_id(),
            branch_id: support_append_witness.branch_id().clone(),
            schema_support_artifact_id: schema_support_record
                .as_ref()
                .map(|record| record.artifact_id.clone()),
            lineage_support_artifact_id: lineage_support_record
                .as_ref()
                .map(|record| record.artifact_id.clone()),
            emitted_schema_artifact: support_append_witness.emits_schema_support(),
            emitted_lineage_artifact: support_append_witness.emits_lineage_support(),
        };
        self.commit_support_summaries
            .insert(commit_id.0, support_summary.clone());
        self.upsert_digest_record(
            AuthoritativeArtifactFamily::CommitSupportSummary,
            commit_support_summary_artifact_id(commit_id),
            stable_structural_digest(&support_summary)?,
        );

        let inserted_schema_support = if let Some(record) = schema_support_record {
            self.schema_support_records
                .insert(record.artifact_id.clone(), record.clone());
            self.upsert_digest_record(
                AuthoritativeArtifactFamily::SchemaSupportRecord,
                record.artifact_id.clone(),
                stable_structural_digest(&record)?,
            );
            true
        } else {
            false
        };

        let inserted_lineage_support = if let Some(record) = lineage_support_record {
            self.lineage_support_records
                .insert(record.artifact_id.clone(), record.clone());
            self.upsert_digest_record(
                AuthoritativeArtifactFamily::LineageSupportRecord,
                record.artifact_id.clone(),
                stable_structural_digest(&record)?,
            );
            true
        } else {
            false
        };

        for (parent_position, parent_commit_id) in
            envelope.commit.parents.iter().copied().enumerate()
        {
            let parent_record = CommitParentRecord {
                commit_id,
                parent_position,
                parent_commit_id,
            };
            self.commit_parent_records.insert(
                parent_artifact_id(commit_id, parent_position),
                parent_record.clone(),
            );
            self.upsert_digest_record(
                AuthoritativeArtifactFamily::CommitParentRecord,
                parent_artifact_id(commit_id, parent_position),
                stable_structural_digest(&parent_record)?,
            );
        }

        self.branch_head_records.insert(
            branch_identity.clone(),
            BranchHeadRecord {
                branch_id: envelope.branch_context.clone(),
                head_commit_id: Some(commit_id),
                head_commit_digest: Some(verified.digest().as_str().to_string()),
                head_update_sequence,
            },
        );
        self.upsert_digest_record(
            AuthoritativeArtifactFamily::BranchHeadRecord,
            branch_identity.clone(),
            stable_structural_digest(&self.branch_head_records[&branch_identity])?,
        );
        let inserted_branch_delta_layer_id = self.publish_branch_delta_layer_for_append(
            envelope.branch_context.clone(),
            previous_branch_head_record
                .as_ref()
                .and_then(|record| record.head_commit_id),
            commit_id,
            vec![commit_id],
        );

        Ok(AppliedAuthoritativeAppend {
            branch_identity,
            commit_id,
            parent_count: envelope.commit.parents.len(),
            created_branch,
            previous_next_commit_sequence,
            previous_next_head_update_sequence,
            previous_branch_record,
            previous_branch_head_record,
            inserted_support_summary: true,
            inserted_schema_support,
            inserted_lineage_support,
            inserted_branch_delta_layer_id,
        })
    }

    pub fn rollback_verified_append(&mut self, applied: AppliedAuthoritativeAppend) {
        self.next_commit_sequence = applied.previous_next_commit_sequence;
        self.next_head_update_sequence = applied.previous_next_head_update_sequence;

        self.commit_envelopes.remove(&applied.commit_id.0);
        self.authoritative_artifact_digests
            .remove(&digest_artifact_key(
                &AuthoritativeArtifactFamily::CommitEnvelope,
                &commit_artifact_id(applied.commit_id),
                self.canonicalization_version,
            ));
        if applied.inserted_support_summary {
            self.commit_support_summaries.remove(&applied.commit_id.0);
            self.authoritative_artifact_digests
                .remove(&digest_artifact_key(
                    &AuthoritativeArtifactFamily::CommitSupportSummary,
                    &commit_support_summary_artifact_id(applied.commit_id),
                    self.canonicalization_version,
                ));
        }
        if applied.inserted_schema_support {
            let artifact_id = schema_support_artifact_id(applied.commit_id);
            self.schema_support_records.remove(&artifact_id);
            self.authoritative_artifact_digests
                .remove(&digest_artifact_key(
                    &AuthoritativeArtifactFamily::SchemaSupportRecord,
                    &artifact_id,
                    self.canonicalization_version,
                ));
        }
        if applied.inserted_lineage_support {
            let artifact_id = lineage_support_artifact_id(applied.commit_id);
            self.lineage_support_records.remove(&artifact_id);
            self.authoritative_artifact_digests
                .remove(&digest_artifact_key(
                    &AuthoritativeArtifactFamily::LineageSupportRecord,
                    &artifact_id,
                    self.canonicalization_version,
                ));
        }
        if let Some(layer_id) = applied.inserted_branch_delta_layer_id {
            self.branch_delta_layer_records.remove(&layer_id);
            self.next_branch_delta_layer_id = layer_id;
        }

        for parent_position in 0..applied.parent_count {
            let artifact_id = parent_artifact_id(applied.commit_id, parent_position);
            self.commit_parent_records.remove(&artifact_id);
            self.authoritative_artifact_digests
                .remove(&digest_artifact_key(
                    &AuthoritativeArtifactFamily::CommitParentRecord,
                    &artifact_id,
                    self.canonicalization_version,
                ));
        }

        match applied.previous_branch_head_record {
            Some(record) => {
                let restored_digest = stable_structural_digest(&record)
                    .expect("restoring previous branch head digest should serialize");
                self.branch_head_records
                    .insert(applied.branch_identity.clone(), record);
                self.upsert_digest_record(
                    AuthoritativeArtifactFamily::BranchHeadRecord,
                    applied.branch_identity.clone(),
                    restored_digest,
                );
            }
            None => {
                self.branch_head_records.remove(&applied.branch_identity);
                self.authoritative_artifact_digests
                    .remove(&digest_artifact_key(
                        &AuthoritativeArtifactFamily::BranchHeadRecord,
                        &applied.branch_identity,
                        self.canonicalization_version,
                    ));
            }
        }

        match applied.previous_branch_record {
            Some(record) => {
                let restored_digest = stable_structural_digest(&record)
                    .expect("restoring previous branch digest should serialize");
                self.branch_records
                    .insert(applied.branch_identity.clone(), record);
                self.upsert_digest_record(
                    AuthoritativeArtifactFamily::BranchRecord,
                    applied.branch_identity.clone(),
                    restored_digest,
                );
            }
            None => {
                self.branch_records.remove(&applied.branch_identity);
                self.authoritative_artifact_digests
                    .remove(&digest_artifact_key(
                        &AuthoritativeArtifactFamily::BranchRecord,
                        &applied.branch_identity,
                        self.canonicalization_version,
                    ));
            }
        }
    }

    pub fn verify_applied_authoritative_append(
        &self,
        applied: &AppliedAuthoritativeAppend,
    ) -> Result<(), StoreError> {
        let commit_record = self
            .commit_envelopes
            .get(&applied.commit_id.0)
            .ok_or_else(|| {
                StoreError::backend_integrity(format!(
                    "commit {} missing after in-place authoritative append",
                    applied.commit_id.0
                ))
            })?;
        self.verify_commit_record(commit_record)?;
        if applied.created_branch {
            self.verify_branch_record(&applied.branch_identity)?;
        }
        self.verify_support_record_family()?;
        self.verify_delta_record_family()?;
        self.verify_branch_head_record(&applied.branch_identity)?;
        Ok(())
    }
}
