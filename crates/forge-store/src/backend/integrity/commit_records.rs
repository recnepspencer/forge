use crate::{authority::digest_envelope, failure::StoreError};
use forge_relational::facade::history::CommitId;

use crate::backend::records::{AuthoritativeArtifactFamily, StoreState, StoredCommitEnvelope};

use super::{commit_artifact_id, parent_artifact_id};

impl StoreState {
    fn verify_commit_parent_record_entry(
        &self,
        parent_record: &crate::backend::records::CommitParentRecord,
    ) -> Result<(), StoreError> {
        let artifact_id =
            parent_artifact_id(parent_record.commit_id, parent_record.parent_position);
        if !self.commit_parent_records.contains_key(&artifact_id) {
            return Err(StoreError::backend_integrity(format!(
                "commit parent record `{artifact_id}` missing during targeted verification"
            )));
        }
        if artifact_id != parent_artifact_id(parent_record.commit_id, parent_record.parent_position)
        {
            return Err(StoreError::backend_integrity(
                "commit parent record key does not match stored payload",
            ));
        }
        if !self
            .commit_envelopes
            .contains_key(&parent_record.commit_id.0)
        {
            return Err(StoreError::backend_integrity(format!(
                "commit parent record references missing commit {}",
                parent_record.commit_id.0
            )));
        }
        if !self
            .commit_envelopes
            .contains_key(&parent_record.parent_commit_id.0)
        {
            return Err(StoreError::backend_integrity(format!(
                "commit parent record references missing parent {}",
                parent_record.parent_commit_id.0
            )));
        }
        self.require_digest_record(
            AuthoritativeArtifactFamily::CommitParentRecord,
            artifact_id,
            &super::stable_structural_digest(parent_record)?,
        )?;
        Ok(())
    }

    pub fn verify_commit_parent_records(
        &self,
        commit_id: CommitId,
        parent_count: usize,
    ) -> Result<(), StoreError> {
        for parent_position in 0..parent_count {
            let artifact_id = parent_artifact_id(commit_id, parent_position);
            let parent_record = self
                .commit_parent_records
                .get(&artifact_id)
                .ok_or_else(|| {
                    StoreError::backend_integrity(format!(
                        "commit {} missing parent record at position {}",
                        commit_id.0, parent_position
                    ))
                })?;
            self.verify_commit_parent_record_entry(parent_record)?;
        }
        Ok(())
    }

    pub fn verify_commit_record(&self, record: &StoredCommitEnvelope) -> Result<(), StoreError> {
        let redigested = digest_envelope(&record.envelope, record.canonicalization_version)?;
        if redigested.as_str() != record.envelope_digest {
            return Err(StoreError::backend_integrity(format!(
                "commit {} digest drifted from canonical export basis",
                record.envelope.commit.commit_id.0
            )));
        }
        self.require_digest_record(
            AuthoritativeArtifactFamily::CommitEnvelope,
            commit_artifact_id(record.envelope.commit.commit_id),
            &record.envelope_digest,
        )?;

        for (position, parent_commit_id) in
            record.envelope.commit.parents.iter().copied().enumerate()
        {
            let parent = self
                .commit_parent_records
                .get(&parent_artifact_id(
                    record.envelope.commit.commit_id,
                    position,
                ))
                .ok_or_else(|| {
                    StoreError::backend_integrity(format!(
                        "commit {} missing parent record at position {}",
                        record.envelope.commit.commit_id.0, position
                    ))
                })?;
            if parent.parent_commit_id != parent_commit_id {
                return Err(StoreError::backend_integrity(format!(
                    "commit {} parent record at position {} does not match envelope",
                    record.envelope.commit.commit_id.0, position
                )));
            }
        }

        self.verify_commit_parent_records(
            record.envelope.commit.commit_id,
            record.envelope.commit.parents.len(),
        )?;

        Ok(())
    }

    pub fn verify_commit_record_family(&self) -> Result<(), StoreError> {
        for record in self.commit_envelopes.values() {
            self.verify_commit_record(record)?;
        }

        for parent_record in self.commit_parent_records.values() {
            self.verify_commit_parent_record_entry(parent_record)?;
        }

        Ok(())
    }
}
