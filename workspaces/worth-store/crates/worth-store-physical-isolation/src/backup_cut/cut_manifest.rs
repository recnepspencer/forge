use std::collections::HashSet;

use super::{BackupArtifactFamily, BackupArtifactReference};

const REQUIRED_FAMILIES: &[BackupArtifactFamily] = &[
    BackupArtifactFamily::RootManifest,
    BackupArtifactFamily::CheckpointManifest,
    BackupArtifactFamily::WalSegment,
    BackupArtifactFamily::Page,
    BackupArtifactFamily::Extent,
    BackupArtifactFamily::Index,
    BackupArtifactFamily::BlobChunk,
];

const MAX_BACKUP_CUT_ARTIFACTS: usize =
    worth_store_physical_format::BackupBundleManifestReadLimits::canonical().maximum_artifacts()
        as usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupCutManifest {
    artifacts: Vec<BackupArtifactReference>,
    total_bytes: u64,
    artifact_closure_digest: [u8; 32],
    source_authority_identity: Option<worth_store_authority::StoreCurrentAuthorityIdentity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackupCutManifestDenial {
    Empty,
    MissingArtifactFamily(BackupArtifactFamily),
    DuplicateArtifactIdentity {
        family: BackupArtifactFamily,
        identity: String,
    },
    DuplicatePhysicalArtifact,
    DuplicateReclaimReference,
    MultipleRootManifests,
    MultipleCheckpointManifests,
    ArtifactCountNotPersistable {
        artifacts: u64,
        maximum: u64,
    },
    ByteCountOverflow,
    AllocationFailed,
    PortableClosureInvariant,
    MissingOwnerReachability,
    UnexpectedOwnerReachability,
}

impl BackupCutManifest {
    pub fn from_current_root_source(
        source: &worth_store_physical_format::PhysicalCurrentReachabilitySource,
        artifacts: impl IntoIterator<Item = BackupArtifactReference>,
    ) -> Result<Self, BackupCutManifestDenial> {
        let artifacts = collect_artifacts(artifacts)?;
        validate_current_root_reachability(source, &artifacts)?;
        Self::from_artifacts(artifacts, Some(source.store_authority_identity()))
    }

    #[cfg(any(test, feature = "certification-authority"))]
    pub fn canonical(
        artifacts: impl IntoIterator<Item = BackupArtifactReference>,
    ) -> Result<Self, BackupCutManifestDenial> {
        Self::from_artifacts(collect_artifacts(artifacts)?, None)
    }

    fn from_artifacts(
        mut artifacts: Vec<BackupArtifactReference>,
        source_authority_identity: Option<worth_store_authority::StoreCurrentAuthorityIdentity>,
    ) -> Result<Self, BackupCutManifestDenial> {
        if artifacts.is_empty() {
            return Err(BackupCutManifestDenial::Empty);
        }
        if artifacts.len() > MAX_BACKUP_CUT_ARTIFACTS {
            return Err(BackupCutManifestDenial::ArtifactCountNotPersistable {
                artifacts: artifacts.len() as u64,
                maximum: MAX_BACKUP_CUT_ARTIFACTS as u64,
            });
        }
        artifacts.sort_by(|left, right| {
            left.family()
                .cmp(&right.family())
                .then_with(|| left.identity().cmp(right.identity()))
                .then_with(|| left.generation().cmp(&right.generation()))
        });
        if let Some(duplicate) = artifacts.windows(2).find(|pair| {
            pair[0].family() == pair[1].family() && pair[0].identity() == pair[1].identity()
        }) {
            return Err(BackupCutManifestDenial::DuplicateArtifactIdentity {
                family: duplicate[1].family(),
                identity: duplicate[1].identity().to_owned(),
            });
        }
        let mut physical_identities = HashSet::new();
        let mut reclaim_references = HashSet::new();
        physical_identities
            .try_reserve(artifacts.len())
            .map_err(|_| BackupCutManifestDenial::AllocationFailed)?;
        reclaim_references
            .try_reserve(artifacts.len())
            .map_err(|_| BackupCutManifestDenial::AllocationFailed)?;
        let mut total_bytes = 0u64;
        for artifact in &artifacts {
            if !physical_identities.insert(artifact.physical_identity()) {
                return Err(BackupCutManifestDenial::DuplicatePhysicalArtifact);
            }
            let reclaim_owner = artifact.reclaim_reference().owner();
            if !reclaim_references.insert(reclaim_owner) {
                return Err(BackupCutManifestDenial::DuplicateReclaimReference);
            }
            total_bytes = total_bytes
                .checked_add(artifact.bytes())
                .ok_or(BackupCutManifestDenial::ByteCountOverflow)?;
        }
        for required in REQUIRED_FAMILIES {
            if !artifacts
                .iter()
                .any(|artifact| artifact.family() == *required)
            {
                return Err(BackupCutManifestDenial::MissingArtifactFamily(*required));
            }
        }
        if artifacts
            .iter()
            .filter(|artifact| artifact.family() == BackupArtifactFamily::RootManifest)
            .count()
            != 1
        {
            return Err(BackupCutManifestDenial::MultipleRootManifests);
        }
        if artifacts
            .iter()
            .filter(|artifact| artifact.family() == BackupArtifactFamily::CheckpointManifest)
            .count()
            != 1
        {
            return Err(BackupCutManifestDenial::MultipleCheckpointManifests);
        }
        let mut portable_rows = Vec::new();
        portable_rows
            .try_reserve_exact(artifacts.len())
            .map_err(|_| BackupCutManifestDenial::AllocationFailed)?;
        for (index, artifact) in artifacts.iter().enumerate() {
            portable_rows.push(
                portable_row(index, artifact)
                    .ok_or(BackupCutManifestDenial::PortableClosureInvariant)?,
            );
        }
        Ok(Self {
            artifact_closure_digest:
                worth_store_physical_format::backup_canonical_artifact_closure_digest(
                    &portable_rows,
                ),
            artifacts,
            total_bytes,
            source_authority_identity,
        })
    }

    pub(super) fn from_recovered_artifacts(
        artifacts: Vec<BackupArtifactReference>,
        source_authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    ) -> Result<Self, BackupCutManifestDenial> {
        Self::from_artifacts(artifacts, Some(source_authority_identity))
    }
    pub fn artifacts(&self) -> &[BackupArtifactReference] {
        &self.artifacts
    }
    pub fn total_bytes(&self) -> u64 {
        self.total_bytes
    }
    pub const fn artifact_closure_digest(&self) -> [u8; 32] {
        self.artifact_closure_digest
    }
    pub const fn source_authority_identity(
        &self,
    ) -> Option<worth_store_authority::StoreCurrentAuthorityIdentity> {
        self.source_authority_identity
    }
}

fn collect_artifacts(
    artifacts: impl IntoIterator<Item = BackupArtifactReference>,
) -> Result<Vec<BackupArtifactReference>, BackupCutManifestDenial> {
    let artifacts = artifacts.into_iter();
    let initial = artifacts.size_hint().0;
    if initial > MAX_BACKUP_CUT_ARTIFACTS {
        return Err(BackupCutManifestDenial::ArtifactCountNotPersistable {
            artifacts: initial as u64,
            maximum: MAX_BACKUP_CUT_ARTIFACTS as u64,
        });
    }
    let mut collected = Vec::new();
    collected
        .try_reserve_exact(initial)
        .map_err(|_| BackupCutManifestDenial::AllocationFailed)?;
    for artifact in artifacts {
        if collected.len() == MAX_BACKUP_CUT_ARTIFACTS {
            return Err(BackupCutManifestDenial::ArtifactCountNotPersistable {
                artifacts: MAX_BACKUP_CUT_ARTIFACTS as u64 + 1,
                maximum: MAX_BACKUP_CUT_ARTIFACTS as u64,
            });
        }
        if collected.len() == collected.capacity() {
            collected
                .try_reserve(1)
                .map_err(|_| BackupCutManifestDenial::AllocationFailed)?;
        }
        collected.push(artifact);
    }
    Ok(collected)
}

fn validate_current_root_reachability(
    source: &worth_store_physical_format::PhysicalCurrentReachabilitySource,
    artifacts: &[BackupArtifactReference],
) -> Result<(), BackupCutManifestDenial> {
    use worth_store_physical_format::PhysicalReferenceAuthority;

    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let expected_count = 1usize
        .checked_add(source.page_cells().len())
        .and_then(|count| count.checked_add(source.extent_cells().len()))
        .ok_or(BackupCutManifestDenial::AllocationFailed)?;
    let mut expected = HashSet::new();
    expected
        .try_reserve(expected_count)
        .map_err(|_| BackupCutManifestDenial::AllocationFailed)?;
    expected.insert((
        BackupArtifactFamily::RootManifest,
        references
            .admit_root_publication(source.manifest().root_publication())
            .reference()
            .generation_owner(),
    ));
    expected.extend(
        source
            .page_cells()
            .iter()
            .map(|cell| (BackupArtifactFamily::Page, cell.owner())),
    );
    expected.extend(source.extent_cells().iter().map(|cell| {
        (
            BackupArtifactFamily::Extent,
            references
                .admit_extent(*cell)
                .reference()
                .generation_owner(),
        )
    }));
    let mut unexpected_owner = false;
    for artifact in artifacts.iter().filter(|artifact| {
        matches!(
            artifact.family(),
            BackupArtifactFamily::RootManifest
                | BackupArtifactFamily::Page
                | BackupArtifactFamily::Extent
        )
    }) {
        if !expected.remove(&(artifact.family(), artifact.reclaim_reference().owner())) {
            unexpected_owner = true;
        }
    }
    if !expected.is_empty() {
        return Err(BackupCutManifestDenial::MissingOwnerReachability);
    }
    if unexpected_owner {
        return Err(BackupCutManifestDenial::UnexpectedOwnerReachability);
    }
    Ok(())
}

pub(super) fn portable_row(
    index: usize,
    artifact: &BackupArtifactReference,
) -> Option<worth_store_physical_format::BackupBundleArtifactManifestRow> {
    artifact.portable_manifest_row(format!("closure-{index}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct OversizedArtifactHint;

    impl Iterator for OversizedArtifactHint {
        type Item = BackupArtifactReference;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (MAX_BACKUP_CUT_ARTIFACTS + 1, None)
        }
    }

    #[test]
    fn cut_cardinality_cannot_exceed_the_canonical_bundle_reader() {
        assert!(matches!(
            BackupCutManifest::canonical(OversizedArtifactHint),
            Err(BackupCutManifestDenial::ArtifactCountNotPersistable {
                artifacts,
                maximum,
            }) if artifacts == maximum + 1 && maximum == MAX_BACKUP_CUT_ARTIFACTS as u64
        ));
    }
}
