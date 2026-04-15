use crate::{
    authority::AuthoritativeExportBundle,
    failure::{StoreError, StoreErrorKind},
};
use forge_relational::facade::history::CommitId;

use super::records::StoreState;

impl StoreState {
    pub fn authoritative_export_bundle(&self) -> AuthoritativeExportBundle {
        let mut bundle = AuthoritativeExportBundle {
            canonicalization_version: self.canonicalization_version,
            branch_records: self.branch_records.values().cloned().collect(),
            branch_head_records: self.branch_head_records.values().cloned().collect(),
            commit_envelopes: self.commit_envelopes.values().cloned().collect(),
            commit_parent_records: self.commit_parent_records.values().cloned().collect(),
            authoritative_artifact_digests: self
                .authoritative_artifact_digests
                .values()
                .cloned()
                .collect(),
        };
        bundle.canonicalize_order();
        bundle
    }

    pub fn from_authoritative_export_bundle(
        bundle: AuthoritativeExportBundle,
    ) -> Result<Self, StoreError> {
        let bundle = bundle.into_canonicalized();
        let mut state = StoreState::default();
        state.canonicalization_version = bundle.canonicalization_version;
        for branch_record in bundle.branch_records {
            let branch_id = branch_record.branch_id.0.clone();
            if state
                .branch_records
                .insert(branch_id.clone(), branch_record)
                .is_some()
            {
                return Err(StoreError::new(
                    StoreErrorKind::DuplicateArtifactIdentity,
                    format!("duplicate branch record `{branch_id}` in authoritative export"),
                ));
            }
        }
        for branch_head_record in bundle.branch_head_records {
            let branch_id = branch_head_record.branch_id.0.clone();
            if state
                .branch_head_records
                .insert(branch_id.clone(), branch_head_record)
                .is_some()
            {
                return Err(StoreError::new(
                    StoreErrorKind::DuplicateArtifactIdentity,
                    format!("duplicate branch head record `{branch_id}` in authoritative export"),
                ));
            }
        }
        for commit_envelope in bundle.commit_envelopes {
            let commit_id = commit_envelope.envelope.commit.commit_id.0;
            if state
                .commit_envelopes
                .insert(commit_id, commit_envelope)
                .is_some()
            {
                return Err(StoreError::duplicate_conflict(CommitId(commit_id)));
            }
        }
        for parent_record in bundle.commit_parent_records {
            let artifact_id = super::integrity::parent_artifact_id(
                parent_record.commit_id,
                parent_record.parent_position,
            );
            if state
                .commit_parent_records
                .insert(artifact_id.clone(), parent_record)
                .is_some()
            {
                return Err(StoreError::new(
                    StoreErrorKind::DuplicateArtifactIdentity,
                    format!(
                        "duplicate commit parent record `{artifact_id}` in authoritative export"
                    ),
                ));
            }
        }
        for digest_record in bundle.authoritative_artifact_digests {
            let artifact_key = format!(
                "{:?}:{}:v{}",
                digest_record.artifact_family,
                digest_record.artifact_id,
                digest_record.canonicalization_version
            );
            if state
                .authoritative_artifact_digests
                .insert(artifact_key.clone(), digest_record)
                .is_some()
            {
                return Err(StoreError::new(
                    StoreErrorKind::DuplicateArtifactIdentity,
                    format!("duplicate digest record `{artifact_key}` in authoritative export"),
                ));
            }
        }
        state.next_commit_sequence = state
            .commit_envelopes
            .values()
            .map(|record| record.commit_sequence)
            .max()
            .map(|sequence| sequence + 1)
            .unwrap_or(1);
        state.next_head_update_sequence = state
            .branch_head_records
            .values()
            .map(|record| record.head_update_sequence)
            .max()
            .map(|sequence| sequence + 1)
            .unwrap_or(1);
        state.verify_integrity()?;
        Ok(state)
    }
}
