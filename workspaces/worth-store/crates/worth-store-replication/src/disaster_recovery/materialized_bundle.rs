use std::path::{Path, PathBuf};

use worth_store_security::StoreSecurityScopeIdentity;

use super::{
    DisasterRecoveryComponent, DisasterRecoveryComponentFamily, DisasterRecoveryManifestFormat,
    DisasterRecoverySecurityBinding,
};
use crate::{ReplicaRecoveryFrontier, ReplicationLineageIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisasterRecoveryBundleDenial {
    EmptyComponents,
    InvalidComponent,
    DuplicateComponent,
    MissingAuthority,
    MissingCheckpoint,
    MissingWal,
    MissingBlobClosure,
    InvalidBundleIdentity,
    BundleRootUnavailable,
    ManifestAlreadyExists,
    ManifestUnavailable,
    ManifestTooLarge,
    ManifestMalformed,
    AllocationFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterializedDisasterRecoveryBundle {
    root: PathBuf,
    source_lineage: ReplicationLineageIdentity,
    frontier: ReplicaRecoveryFrontier,
    security: DisasterRecoverySecurityBinding,
    expected_rpo_lsn: u64,
    components: Vec<DisasterRecoveryComponent>,
    manifest_identity: [u8; 32],
}

impl MaterializedDisasterRecoveryBundle {
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn source_lineage(&self) -> &ReplicationLineageIdentity {
        &self.source_lineage
    }

    pub const fn frontier(&self) -> ReplicaRecoveryFrontier {
        self.frontier
    }

    pub const fn security(&self) -> DisasterRecoverySecurityBinding {
        self.security
    }

    pub const fn expected_rpo_lsn(&self) -> u64 {
        self.expected_rpo_lsn
    }

    pub fn components(&self) -> &[DisasterRecoveryComponent] {
        &self.components
    }

    pub const fn manifest_identity(&self) -> [u8; 32] {
        self.manifest_identity
    }

    pub(super) fn from_manifest(
        root: PathBuf,
        source_lineage: ReplicationLineageIdentity,
        frontier: ReplicaRecoveryFrontier,
        security: DisasterRecoverySecurityBinding,
        expected_rpo_lsn: u64,
        components: Vec<DisasterRecoveryComponent>,
        manifest_identity: [u8; 32],
    ) -> Result<Self, DisasterRecoveryBundleDenial> {
        validate_component_set(&components)?;
        if manifest_identity == [0; 32] {
            return Err(DisasterRecoveryBundleDenial::InvalidBundleIdentity);
        }
        Ok(Self {
            root,
            source_lineage,
            frontier,
            security,
            expected_rpo_lsn,
            components,
            manifest_identity,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReplicationDisasterRecoveryOwner;

impl ReplicationDisasterRecoveryOwner {
    pub fn classify_replica_history(
        peer: crate::ReplicationPeerId,
        lineage: ReplicationLineageIdentity,
        frontier: ReplicaRecoveryFrontier,
        blob_closure_complete: bool,
        authoritative_media_admissible: bool,
        durable_media_identity: [u8; 32],
        current_lineage: ReplicationLineageIdentity,
    ) -> crate::DivergentReplicaHistoryReport {
        crate::DivergentReplicaHistoryReport::classify(
            crate::ReplicaHistoryObservation {
                peer,
                lineage,
                frontier,
                blob_closure_complete,
                authoritative_media_admissible,
                durable_media_identity,
            },
            current_lineage,
        )
    }

    pub fn record_materialized_bundle(
        root: impl Into<PathBuf>,
        source_lineage: ReplicationLineageIdentity,
        frontier: ReplicaRecoveryFrontier,
        security_scope: StoreSecurityScopeIdentity,
        expected_rpo_lsn: u64,
        mut components: Vec<DisasterRecoveryComponent>,
    ) -> Result<MaterializedDisasterRecoveryBundle, DisasterRecoveryBundleDenial> {
        components.sort_by(|left, right| left.relative_path().cmp(right.relative_path()));
        validate_component_set(&components)?;
        let root = std::fs::canonicalize(root.into())
            .map_err(|_| DisasterRecoveryBundleDenial::BundleRootUnavailable)?;
        let security = DisasterRecoverySecurityBinding::from_current_scope(security_scope);
        let manifest_identity = DisasterRecoveryManifestFormat::persist(
            &root,
            &source_lineage,
            frontier,
            security,
            expected_rpo_lsn,
            &components,
        )?;
        MaterializedDisasterRecoveryBundle::from_manifest(
            root,
            source_lineage,
            frontier,
            security,
            expected_rpo_lsn,
            components,
            manifest_identity,
        )
    }
}

fn validate_component_set(
    components: &[DisasterRecoveryComponent],
) -> Result<(), DisasterRecoveryBundleDenial> {
    if components.is_empty() {
        return Err(DisasterRecoveryBundleDenial::EmptyComponents);
    }
    if components
        .windows(2)
        .any(|pair| pair[0].relative_path() >= pair[1].relative_path())
    {
        return Err(DisasterRecoveryBundleDenial::DuplicateComponent);
    }
    for (family, denial) in [
        (
            DisasterRecoveryComponentFamily::Authority,
            DisasterRecoveryBundleDenial::MissingAuthority,
        ),
        (
            DisasterRecoveryComponentFamily::Checkpoint,
            DisasterRecoveryBundleDenial::MissingCheckpoint,
        ),
        (
            DisasterRecoveryComponentFamily::Wal,
            DisasterRecoveryBundleDenial::MissingWal,
        ),
        (
            DisasterRecoveryComponentFamily::Blob,
            DisasterRecoveryBundleDenial::MissingBlobClosure,
        ),
    ] {
        if !components
            .iter()
            .any(|component| component.family() == family)
        {
            return Err(denial);
        }
    }
    Ok(())
}
