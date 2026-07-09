use std::collections::BTreeSet;

use crate::{
    backend::records::{
        compatibility_manifest_artifact_id, StoreState,
        COMPATIBILITY_MANIFEST_RECORD_FAMILY_VERSION,
    },
    compatibility::CompatibilityRegistry,
    failure::{StoreError, StoreErrorKind},
};

impl StoreState {
    pub(super) fn verify_compatibility_record_family(&self) -> Result<(), StoreError> {
        let snapshot = CompatibilityRegistry::first_ship();
        let expected_declarations = snapshot.declarations();
        let expected_artifact_ids = expected_declarations
            .iter()
            .map(|declaration| compatibility_manifest_artifact_id(declaration.family_id()))
            .collect::<BTreeSet<_>>();

        let mut observed_sequences = Vec::with_capacity(self.compatibility_manifest_records.len());
        for record in self.compatibility_manifest_records.values() {
            if record.family_version != COMPATIBILITY_MANIFEST_RECORD_FAMILY_VERSION {
                return Err(StoreError::new(
                    StoreErrorKind::CompatibilityArtifactManifestMalformed,
                    format!(
                        "compatibility manifest `{}` has unsupported family version {}",
                        record.artifact_id, record.family_version
                    ),
                ));
            }
            observed_sequences.push(record.record.publication_sequence());
            if !expected_artifact_ids.contains(record.artifact_id()) {
                return Err(StoreError::new(
                    StoreErrorKind::CompatibilityArtifactFamilyUndeclared,
                    format!(
                        "compatibility manifest `{}` does not belong to the first-ship registry",
                        record.artifact_id()
                    ),
                ));
            }
        }

        observed_sequences.sort_unstable();
        for (offset, observed) in observed_sequences.iter().enumerate() {
            let expected = offset as u64 + 1;
            if *observed != expected {
                return Err(StoreError::new(
                    StoreErrorKind::CompatibilityManifestPublicationGap,
                    format!(
                        "compatibility manifest publication sequence gap detected: expected {expected}, found {observed}"
                    ),
                ));
            }
        }

        for declaration in expected_declarations {
            let artifact_id = compatibility_manifest_artifact_id(declaration.family_id());
            let Some(record) = self.compatibility_manifest_records.get(&artifact_id) else {
                return Err(StoreError::new(
                    StoreErrorKind::CompatibilityManifestPublicationGap,
                    format!(
                        "compatibility manifest `{artifact_id}` is missing from persisted state"
                    ),
                ));
            };
            if record.record.family_id() != declaration.family_id() {
                return Err(StoreError::new(
                    StoreErrorKind::CompatibilityArtifactFamilyUndeclared,
                    format!(
                        "compatibility manifest `{artifact_id}` drifted to family `{}`",
                        record.record.family_id().as_str()
                    ),
                ));
            }
            if record.record.authority_classification() != declaration.authority_classification() {
                return Err(StoreError::new(
                    StoreErrorKind::CompatibilityArtifactManifestMalformed,
                    format!(
                        "compatibility manifest `{artifact_id}` changed authority classification"
                    ),
                ));
            }
            if record.record.window() != declaration.manifest().window() {
                return Err(StoreError::new(
                    StoreErrorKind::CompatibilityArtifactManifestMalformed,
                    format!("compatibility manifest `{artifact_id}` window drifted from registry"),
                ));
            }
            if record.record.manifest_digest() != declaration.manifest().digest() {
                return Err(StoreError::new(
                    StoreErrorKind::CompatibilityArtifactManifestMalformed,
                    format!("compatibility manifest `{artifact_id}` digest drifted from registry"),
                ));
            }
        }
        Ok(())
    }
}
