use crate::failure::StoreError;
use serde::Serialize;

use crate::backend::records::{
    AuthoritativeArtifactDigestRecord, AuthoritativeArtifactFamily, StoreState,
};

use super::identity::digest_artifact_key;

impl StoreState {
    pub fn upsert_digest_record(
        &mut self,
        artifact_family: AuthoritativeArtifactFamily,
        artifact_id: String,
        artifact_digest: String,
    ) {
        self.authoritative_artifact_digests.insert(
            digest_artifact_key(
                &artifact_family,
                &artifact_id,
                self.canonicalization_version,
            ),
            AuthoritativeArtifactDigestRecord {
                artifact_family,
                artifact_id,
                canonicalization_version: self.canonicalization_version,
                digest_algorithm: "sha256".to_string(),
                artifact_digest,
            },
        );
    }

    pub fn require_digest_record(
        &self,
        artifact_family: AuthoritativeArtifactFamily,
        artifact_id: String,
        expected_digest: &str,
    ) -> Result<(), StoreError> {
        let record = self
            .authoritative_artifact_digests
            .get(&digest_artifact_key(
                &artifact_family,
                &artifact_id,
                self.canonicalization_version,
            ))
            .ok_or_else(|| {
                StoreError::backend_integrity(format!(
                    "missing authoritative digest record for artifact `{artifact_id}`"
                ))
            })?;
        if record.artifact_digest != expected_digest {
            return Err(StoreError::backend_integrity(format!(
                "authoritative digest record drifted for artifact `{artifact_id}`"
            )));
        }
        Ok(())
    }
}

pub(crate) fn stable_structural_digest<T: Serialize>(value: &T) -> Result<String, StoreError> {
    let normalized = serde_json::to_value(value)?;
    let bytes = serde_json::to_vec(&normalized)?;
    use sha2::{Digest, Sha256};
    Ok(format!("{:x}", Sha256::digest(bytes)))
}
