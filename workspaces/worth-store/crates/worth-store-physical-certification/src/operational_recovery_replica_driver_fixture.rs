use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use worth_store_offline_verifier::{
    open_disaster_recovery_bundle, verify_disaster_recovery_bundle,
    DisasterRecoveryVerificationPolicy, IndependentlyVerifiedDisasterRecoveryBundle,
};
use worth_store_operations::{
    ConfiguredFailureDomainId, OperationalControlLocation, OperationalControlStore,
    ProtectedOperationalMediaLocation,
};
use worth_store_replication::{
    DisasterRecoveryArtifactEvidence, DisasterRecoveryComponent,
    DisasterRecoveryComponentSemantics, ReplicaRecoveryFrontier, ReplicationDisasterRecoveryOwner,
    ReplicationLineageIdentity,
};

pub(super) struct DisasterRecoveryFixture {
    workspace: tempfile::TempDir,
    root: PathBuf,
    components: Vec<(PathBuf, Vec<u8>, DisasterRecoveryComponentSemantics)>,
    frontier: ReplicaRecoveryFrontier,
    lineage: ReplicationLineageIdentity,
}

impl DisasterRecoveryFixture {
    pub(super) fn materialize() -> Self {
        let workspace = tempfile::tempdir().unwrap();
        let root = workspace.path().join("bundle");
        let lineage = ReplicationLineageIdentity::from_declared_lineage("lineage/s10").unwrap();
        let lineage_identity = lineage.stable_fingerprint();
        let checkpoint_identity = identity("checkpoint/90");
        let blob_closure_identity = identity("blob-closure/90");
        let components = vec![
            component(
                "authority.manifest",
                b"authority=9",
                DisasterRecoveryComponentSemantics::Authority {
                    lineage_identity,
                    authority_epoch: 9,
                },
            ),
            component(
                "checkpoint.bin",
                b"checkpoint=90",
                DisasterRecoveryComponentSemantics::Checkpoint {
                    lineage_identity,
                    authority_epoch: 9,
                    checkpoint_identity,
                    checkpoint_lsn: 90,
                    blob_closure_identity,
                },
            ),
            component(
                "wal.log",
                b"lsn=91\nlsn=92",
                DisasterRecoveryComponentSemantics::Wal {
                    lineage_identity,
                    authority_epoch: 9,
                    start_lsn: 91,
                    end_lsn_exclusive: 93,
                },
            ),
            component(
                "blobs/chunk-7",
                b"blob-7",
                DisasterRecoveryComponentSemantics::Blob {
                    blob_closure_identity,
                },
            ),
        ];
        materialize_components(&root, &components);
        let root = std::fs::canonicalize(root).unwrap();
        let frontier = ReplicaRecoveryFrontier::admit(94, 93, 92, 92, 9).unwrap();
        let scope = worth_store_test_support::layout_integrity_authority("s10-driver-bootstrap")
            .security_scope()
            .identity();
        ReplicationDisasterRecoveryOwner::record_materialized_bundle(
            &root,
            lineage.clone(),
            frontier,
            scope,
            92,
            component_declarations(&components),
        )
        .unwrap();
        Self {
            workspace,
            root,
            components,
            frontier,
            lineage,
        }
    }

    pub(super) fn verify(&self) -> IndependentlyVerifiedDisasterRecoveryBundle {
        let opened = open_disaster_recovery_bundle(&self.root, 64 * 1024).unwrap();
        verify_disaster_recovery_bundle(opened, 3, &self.policy()).unwrap()
    }

    pub(super) const fn frontier(&self) -> ReplicaRecoveryFrontier {
        self.frontier
    }

    pub(super) fn lineage(&self) -> ReplicationLineageIdentity {
        self.lineage.clone()
    }

    pub(super) fn lease_root(&self) -> PathBuf {
        self.workspace.path().join("leases")
    }

    pub(super) fn materialize_replica_target(&self) -> PathBuf {
        let target = self.workspace.path().join("promoted-target");
        std::fs::create_dir_all(target.join("blobs")).unwrap();
        std::fs::write(target.join("authority.page"), b"authority=promoted").unwrap();
        std::fs::write(target.join("blobs/chunk-7"), b"blob-7").unwrap();
        std::fs::canonicalize(target).unwrap()
    }

    pub(super) fn control_store(&self) -> OperationalControlStore {
        let target = self.workspace.path().join("target");
        std::fs::create_dir_all(&target).unwrap();
        OperationalControlStore::open_with_certified_topology(
            OperationalControlLocation::new(
                self.workspace.path().join("control/operations.log"),
                domain("control"),
            ),
            [
                ProtectedOperationalMediaLocation::source(&self.root, domain("source")),
                ProtectedOperationalMediaLocation::backup_target(target, domain("target")),
            ],
        )
        .unwrap()
    }

    fn policy(&self) -> DisasterRecoveryVerificationPolicy {
        let mut formats = self
            .components
            .iter()
            .map(|(_, _, semantics)| identity(&format!("format/{:?}", semantics.family())))
            .collect::<Vec<_>>();
        formats.sort_unstable();
        formats.dedup();
        DisasterRecoveryVerificationPolicy::from_supported_assumptions(
            formats,
            vec![identity("backend/filesystem-v1")],
        )
        .unwrap()
    }
}

fn materialize_components(
    root: &Path,
    components: &[(PathBuf, Vec<u8>, DisasterRecoveryComponentSemantics)],
) {
    for (relative, bytes, _) in components {
        let path = root.join(relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, bytes).unwrap();
    }
}

fn component(
    path: &str,
    bytes: &[u8],
    semantics: DisasterRecoveryComponentSemantics,
) -> (PathBuf, Vec<u8>, DisasterRecoveryComponentSemantics) {
    (PathBuf::from(path), bytes.to_vec(), semantics)
}

fn component_declarations(
    components: &[(PathBuf, Vec<u8>, DisasterRecoveryComponentSemantics)],
) -> Vec<DisasterRecoveryComponent> {
    components
        .iter()
        .map(|(path, bytes, semantics)| {
            DisasterRecoveryComponent::declare(
                path,
                DisasterRecoveryArtifactEvidence::admit(
                    Sha256::digest(bytes).into(),
                    bytes.len() as u64,
                    identity(&format!("format/{:?}", semantics.family())),
                    identity("backend/filesystem-v1"),
                )
                .unwrap(),
                *semantics,
            )
            .unwrap()
        })
        .collect()
}

fn identity(label: &str) -> [u8; 32] {
    Sha256::digest(label.as_bytes()).into()
}

fn domain(label: &str) -> ConfiguredFailureDomainId {
    ConfiguredFailureDomainId::new(label).unwrap()
}
