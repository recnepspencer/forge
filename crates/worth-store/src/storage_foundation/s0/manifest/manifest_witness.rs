use super::super::evidence::S0StableDigest;
use super::audit_manifest::S0AuditInputManifest;
use super::manifest_validation::S0ScanScopeRejection;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0InputManifestWitness {
    schema_version: &'static str,
    source_revision: String,
    manifest_digest: S0StableDigest,
}

impl S0InputManifestWitness {
    pub fn source_revision(&self) -> &str {
        &self.source_revision
    }

    pub fn manifest_digest(&self) -> &S0StableDigest {
        &self.manifest_digest
    }
}

impl S0AuditInputManifest {
    pub fn witness(&self) -> S0InputManifestWitness {
        S0InputManifestWitness {
            schema_version: self.schema_version,
            source_revision: self.source_revision.clone(),
            manifest_digest: self.manifest_digest.clone(),
        }
    }

    pub fn validate_witness(
        &self,
        witness: &S0InputManifestWitness,
    ) -> Result<(), S0ScanScopeRejection> {
        if witness.schema_version != self.schema_version {
            return Err(S0ScanScopeRejection::StaleSchemaVersion);
        }
        if witness.source_revision != self.source_revision {
            return Err(S0ScanScopeRejection::StaleSourceRevision);
        }
        if witness.manifest_digest != self.manifest_digest {
            return Err(S0ScanScopeRejection::StaleManifestDigest);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0InputManifestDelta {
    reused_file_count: u64,
    rescanned_file_count: u64,
    added_file_count: u64,
    removed_file_count: u64,
}

impl S0InputManifestDelta {
    pub fn between(previous: &S0AuditInputManifest, current: &S0AuditInputManifest) -> Self {
        let previous_files = previous
            .matched_files()
            .iter()
            .map(|file| (file.path(), file.digest().as_str()))
            .collect::<BTreeMap<_, _>>();
        let current_files = current
            .matched_files()
            .iter()
            .map(|file| (file.path(), file.digest().as_str()))
            .collect::<BTreeMap<_, _>>();

        let reused_file_count = current_files
            .iter()
            .filter(|(path, digest)| previous_files.get(**path) == Some(digest))
            .count() as u64;
        let rescanned_file_count = current_files
            .iter()
            .filter(|(path, digest)| {
                previous_files
                    .get(**path)
                    .is_some_and(|previous_digest| previous_digest != *digest)
            })
            .count() as u64;
        let added_file_count = current_files
            .keys()
            .filter(|path| !previous_files.contains_key(**path))
            .count() as u64;
        let removed_file_count = previous_files
            .keys()
            .filter(|path| !current_files.contains_key(**path))
            .count() as u64;

        Self {
            reused_file_count,
            rescanned_file_count,
            added_file_count,
            removed_file_count,
        }
    }

    pub fn reused_file_count(&self) -> u64 {
        self.reused_file_count
    }

    pub fn rescanned_file_count(&self) -> u64 {
        self.rescanned_file_count
    }

    pub fn added_file_count(&self) -> u64 {
        self.added_file_count
    }

    pub fn removed_file_count(&self) -> u64 {
        self.removed_file_count
    }
}
