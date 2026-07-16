use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use worth_store_security::StoreSecurityScopeIdentity;

use crate::{ReplicaRecoveryFrontier, ReplicationLineageIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DisasterRecoveryComponentFamily {
    Authority,
    Checkpoint,
    Wal,
    Page,
    Blob,
    Layout,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisasterRecoveryComponent {
    family: DisasterRecoveryComponentFamily,
    relative_path: PathBuf,
    expected_digest: [u8; 32],
    byte_length: u64,
}

impl DisasterRecoveryComponent {
    pub fn declare(
        family: DisasterRecoveryComponentFamily,
        relative_path: impl Into<PathBuf>,
        expected_digest: [u8; 32],
        byte_length: u64,
    ) -> Result<Self, DisasterRecoveryBundleDenial> {
        let relative_path = relative_path.into();
        if relative_path.as_os_str().is_empty()
            || relative_path.is_absolute()
            || relative_path.components().any(|component| {
                matches!(
                    component,
                    std::path::Component::ParentDir | std::path::Component::CurDir
                )
            })
            || expected_digest == [0; 32]
        {
            return Err(DisasterRecoveryBundleDenial::InvalidComponent);
        }
        Ok(Self {
            family,
            relative_path,
            expected_digest,
            byte_length,
        })
    }

    pub const fn family(&self) -> DisasterRecoveryComponentFamily {
        self.family
    }

    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    pub const fn expected_digest(&self) -> [u8; 32] {
        self.expected_digest
    }

    pub const fn byte_length(&self) -> u64 {
        self.byte_length
    }
}

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterializedDisasterRecoveryBundle {
    root: PathBuf,
    source_lineage: ReplicationLineageIdentity,
    frontier: ReplicaRecoveryFrontier,
    security_scope: StoreSecurityScopeIdentity,
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

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_scope
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
        current_lineage: ReplicationLineageIdentity,
    ) -> crate::DivergentReplicaHistoryReport {
        crate::DivergentReplicaHistoryReport::classify(
            crate::ReplicaHistoryObservation {
                peer,
                lineage,
                frontier,
                blob_closure_complete,
                authoritative_media_admissible,
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
        if components.is_empty() {
            return Err(DisasterRecoveryBundleDenial::EmptyComponents);
        }
        components.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
        if components
            .windows(2)
            .any(|pair| pair[0].relative_path == pair[1].relative_path)
        {
            return Err(DisasterRecoveryBundleDenial::DuplicateComponent);
        }
        require_component_families(&components)?;
        let root = root.into();
        let manifest_identity = manifest_identity(
            &source_lineage,
            frontier,
            security_scope,
            expected_rpo_lsn,
            &components,
        );
        if manifest_identity == [0; 32] {
            return Err(DisasterRecoveryBundleDenial::InvalidBundleIdentity);
        }
        Ok(MaterializedDisasterRecoveryBundle {
            root,
            source_lineage,
            frontier,
            security_scope,
            expected_rpo_lsn,
            components,
            manifest_identity,
        })
    }
}

fn require_component_families(
    components: &[DisasterRecoveryComponent],
) -> Result<(), DisasterRecoveryBundleDenial> {
    for (family, denial) in [
        (DisasterRecoveryComponentFamily::Authority, DisasterRecoveryBundleDenial::MissingAuthority),
        (DisasterRecoveryComponentFamily::Checkpoint, DisasterRecoveryBundleDenial::MissingCheckpoint),
        (DisasterRecoveryComponentFamily::Wal, DisasterRecoveryBundleDenial::MissingWal),
        (DisasterRecoveryComponentFamily::Blob, DisasterRecoveryBundleDenial::MissingBlobClosure),
    ] {
        if !components.iter().any(|component| component.family == family) {
            return Err(denial);
        }
    }
    Ok(())
}

fn manifest_identity(
    lineage: &ReplicationLineageIdentity,
    frontier: ReplicaRecoveryFrontier,
    scope: StoreSecurityScopeIdentity,
    expected_rpo_lsn: u64,
    components: &[DisasterRecoveryComponent],
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-materialized-dr-bundle-v1");
    digest.update(lineage.as_str().as_bytes());
    digest.update(frontier.observed_lsn().to_be_bytes());
    digest.update(frontier.durable_lsn().to_be_bytes());
    digest.update(frontier.client_acknowledged_lsn().to_be_bytes());
    digest.update(frontier.replication_acknowledged_lsn().to_be_bytes());
    digest.update(frontier.authority_epoch().to_be_bytes());
    digest.update(scope.stable_fingerprint());
    digest.update(expected_rpo_lsn.to_be_bytes());
    for component in components {
        digest.update([component.family as u8]);
        digest.update(component.relative_path.as_os_str().to_string_lossy().as_bytes());
        digest.update(component.expected_digest);
        digest.update(component.byte_length.to_be_bytes());
    }
    digest.finalize().into()
}
