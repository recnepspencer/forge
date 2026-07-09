use crate::backend::{
    integrity::{
        commit_artifact_id, commit_support_summary_artifact_id, digest_artifact_key,
        lineage_support_artifact_id, parent_artifact_id, schema_support_artifact_id,
        stable_structural_digest,
    },
    records::{AuthoritativeArtifactFamily, StoreState},
};

use super::receipt::AppliedAuthoritativeAppend;

impl StoreState {
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
}
