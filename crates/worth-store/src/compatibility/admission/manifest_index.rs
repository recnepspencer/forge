use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityManifestIndexEntry {
    family_id: ArtifactFamilyId,
    minimum_format: ArtifactFormatVersion,
    maximum_format: ArtifactFormatVersion,
    minimum_semantic: ArtifactSemanticVersion,
    maximum_semantic: ArtifactSemanticVersion,
    manifest_digest: CompatibilityManifestDigest,
}

impl CompatibilityManifestIndexEntry {
    fn from_declaration(declaration: &CompatibilityFamilyDeclaration) -> Self {
        let manifest = declaration.manifest();
        let window = manifest.window();
        Self {
            family_id: manifest.family_id().clone(),
            minimum_format: window.minimum_format(),
            maximum_format: window.maximum_format(),
            minimum_semantic: window.minimum_semantic(),
            maximum_semantic: window.maximum_semantic(),
            manifest_digest: manifest.digest().clone(),
        }
    }

    fn from_publication_record(
        record: &super::super::manifests::CompatibilityManifestPublicationRecord,
    ) -> Self {
        let window = record.window();
        Self {
            family_id: record.family_id().clone(),
            minimum_format: window.minimum_format(),
            maximum_format: window.maximum_format(),
            minimum_semantic: window.minimum_semantic(),
            maximum_semantic: window.maximum_semantic(),
            manifest_digest: record.manifest_digest().clone(),
        }
    }

    fn rejection_kind(
        &self,
        format_version: ArtifactFormatVersion,
        semantic_version: ArtifactSemanticVersion,
        manifest_digest: &CompatibilityManifestDigest,
        recovered: bool,
    ) -> Option<CompatibilityRejectionKind> {
        if format_version < self.minimum_format || self.maximum_format < format_version {
            return Some(if recovered {
                CompatibilityRejectionKind::RecoveredManifestWindowMismatch
            } else {
                CompatibilityRejectionKind::UnsupportedFormatVersion
            });
        }
        if semantic_version < self.minimum_semantic || self.maximum_semantic < semantic_version {
            return Some(if recovered {
                CompatibilityRejectionKind::RecoveredManifestWindowMismatch
            } else {
                CompatibilityRejectionKind::UnsupportedSemanticVersion
            });
        }
        if &self.manifest_digest != manifest_digest {
            return Some(if recovered {
                CompatibilityRejectionKind::RecoveredManifestDigestMismatch
            } else {
                CompatibilityRejectionKind::ManifestDigestMismatch
            });
        }
        None
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn manifest_digest(&self) -> &CompatibilityManifestDigest {
        &self.manifest_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityManifestIndex {
    entries_by_family: BTreeMap<ArtifactFamilyId, CompatibilityManifestIndexEntry>,
    rebuild_counters: CompatibilityAdmissionCounters,
    registry_snapshot_identity: String,
    manifest_frontier_identity: String,
    recovered: bool,
}

impl CompatibilityManifestIndex {
    pub fn rebuild_from_registry(snapshot: &CompatibilityRegistrySnapshot) -> Self {
        let mut entries_by_family = BTreeMap::new();
        let mut counters = CompatibilityAdmissionCounters::default();
        counters.manifest_index_rebuild_count = 1;
        for declaration in snapshot.declarations() {
            counters.manifest_entries_visited += 1;
            let entry = CompatibilityManifestIndexEntry::from_declaration(declaration);
            entries_by_family.insert(entry.family_id.clone(), entry);
        }
        Self {
            entries_by_family,
            rebuild_counters: counters,
            registry_snapshot_identity: registry_snapshot_identity(snapshot),
            manifest_frontier_identity: "registry-declaration-frontier".to_string(),
            recovered: false,
        }
    }

    pub fn rebuild_from_recovered_manifests(
        snapshot: &CompatibilityRegistrySnapshot,
        recovered: &CompatibilityRecoveredManifestIndex,
    ) -> Self {
        let mut entries_by_family = BTreeMap::new();
        let mut counters = CompatibilityAdmissionCounters::default();
        counters.manifest_index_rebuild_count = 1;
        counters.manifest_publication_count = recovered.frontier().publication_count();
        for declaration in snapshot.declarations() {
            counters.manifest_entries_visited += 1;
            if let Some(record) = recovered.get(declaration.family_id()) {
                counters.manifest_recovery_record_count += 1;
                let entry = CompatibilityManifestIndexEntry::from_publication_record(record);
                entries_by_family.insert(entry.family_id.clone(), entry);
            } else {
                counters.manifest_publication_gap_count += 1;
            }
        }
        Self {
            entries_by_family,
            rebuild_counters: counters,
            registry_snapshot_identity: registry_snapshot_identity(snapshot),
            manifest_frontier_identity: recovered.frontier().identity().to_string(),
            recovered: true,
        }
    }

    pub fn entries(&self) -> impl Iterator<Item = &CompatibilityManifestIndexEntry> {
        self.entries_by_family.values()
    }

    pub fn rebuild_counters(&self) -> &CompatibilityAdmissionCounters {
        &self.rebuild_counters
    }

    pub fn registry_snapshot_identity(&self) -> &str {
        &self.registry_snapshot_identity
    }

    pub fn manifest_frontier_identity(&self) -> &str {
        &self.manifest_frontier_identity
    }

    pub(in crate::compatibility::admission) fn lookup(
        &self,
        artifact: &QuarantinedDecodedArtifact,
        counters: &mut CompatibilityAdmissionCounters,
    ) -> Result<&CompatibilityManifestIndexEntry, CompatibilityRejection> {
        counters.manifest_index_lookup_count += 1;
        counters.manifest_digest_check_count += 1;
        let Some(entry) = self.entries_by_family.get(artifact.family_id()) else {
            if self.recovered {
                counters.manifest_publication_gap_count += 1;
            }
            return Err(CompatibilityRejection::new(
                if self.recovered {
                    CompatibilityRejectionKind::MissingManifestPublication
                } else {
                    CompatibilityRejectionKind::UndeclaredFamily
                },
                artifact.family_id().clone(),
                "compatibility manifest publication is missing or family is undeclared",
            ));
        };
        if let Some(kind) = entry.rejection_kind(
            artifact.format_version(),
            artifact.semantic_version(),
            artifact.manifest_digest(),
            self.recovered,
        ) {
            match kind {
                CompatibilityRejectionKind::RecoveredManifestDigestMismatch
                | CompatibilityRejectionKind::ManifestDigestMismatch => {
                    counters.manifest_digest_mismatch_count += 1;
                }
                CompatibilityRejectionKind::RecoveredManifestWindowMismatch
                | CompatibilityRejectionKind::UnsupportedFormatVersion
                | CompatibilityRejectionKind::UnsupportedSemanticVersion => {
                    counters.manifest_window_mismatch_count += 1;
                }
                _ => {}
            }
            return Err(CompatibilityRejection::new(
                kind,
                artifact.family_id().clone(),
                "compatibility manifest window or digest rejected artifact",
            ));
        }
        Ok(entry)
    }
}

fn registry_snapshot_identity(snapshot: &CompatibilityRegistrySnapshot) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    for declaration in snapshot.declarations() {
        hasher.update(declaration.family_id().as_str().as_bytes());
        hasher.update(declaration.manifest().digest().as_str().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}
