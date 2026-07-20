use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use worth_store_offline_verifier::{
    open_disaster_recovery_bundle, verify_disaster_recovery_bundle,
    DisasterRecoveryVerificationPolicy, IndependentlyVerifiedDisasterRecoveryBundle,
};
use worth_store_replication::{
    DisasterRecoveryArtifactEvidence, DisasterRecoveryComponent,
    DisasterRecoveryComponentSemantics, ReplicaRecoveryFrontier, ReplicationDisasterRecoveryOwner,
    ReplicationLineageIdentity,
};

pub struct ScenarioDisasterRecoveryMedia {
    root: PathBuf,
    frontier: ReplicaRecoveryFrontier,
    lineage: ReplicationLineageIdentity,
    supported_formats: Vec<[u8; 32]>,
}

impl ScenarioDisasterRecoveryMedia {
    pub fn materialize(
        workspace: &Path,
        security_scope: worth_store_security::StoreSecurityScopeIdentity,
        identity_label: &str,
    ) -> Self {
        let root = workspace.join("disaster-recovery-bundle");
        let lineage =
            ReplicationLineageIdentity::from_declared_lineage(format!("lineage/{identity_label}"))
                .expect("scenario DR lineage");
        let lineage_identity = lineage.stable_fingerprint();
        let checkpoint_identity = identity(b"checkpoint/90");
        let blob_closure_identity = identity(b"blob-closure/90");
        let components = [
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
        let mut supported_formats = Vec::new();
        let mut declarations = Vec::new();
        for (relative, bytes, semantics) in &components {
            let path = root.join(relative);
            std::fs::create_dir_all(path.parent().expect("DR component parent"))
                .expect("DR component directory");
            std::fs::write(&path, bytes).expect("DR component media");
            let format = format_identity(semantics.family());
            supported_formats.push(format);
            declarations.push(
                DisasterRecoveryComponent::declare(
                    relative,
                    DisasterRecoveryArtifactEvidence::admit(
                        Sha256::digest(bytes).into(),
                        bytes.len() as u64,
                        format,
                        backend_identity(),
                    )
                    .expect("DR artifact evidence"),
                    *semantics,
                )
                .expect("DR component declaration"),
            );
        }
        supported_formats.sort_unstable();
        supported_formats.dedup();
        let root = std::fs::canonicalize(root).expect("canonical DR media");
        let frontier =
            ReplicaRecoveryFrontier::admit(94, 93, 92, 92, 9).expect("admissible DR frontier");
        ReplicationDisasterRecoveryOwner::record_materialized_bundle(
            &root,
            lineage.clone(),
            frontier,
            security_scope,
            92,
            declarations,
        )
        .expect("owner-recorded DR bundle");
        Self {
            root,
            frontier,
            lineage,
            supported_formats,
        }
    }

    pub fn verify(&self) -> IndependentlyVerifiedDisasterRecoveryBundle {
        let opened = open_disaster_recovery_bundle(&self.root, 64 * 1024)
            .expect("fresh DR media acquisition");
        let policy = DisasterRecoveryVerificationPolicy::from_supported_assumptions(
            self.supported_formats.clone(),
            vec![backend_identity()],
        )
        .expect("explicit DR verification policy");
        verify_disaster_recovery_bundle(opened, 3, &policy)
            .expect("independently verified DR bundle")
    }

    pub const fn frontier(&self) -> ReplicaRecoveryFrontier {
        self.frontier
    }

    pub fn lineage(&self) -> ReplicationLineageIdentity {
        self.lineage.clone()
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

fn component(
    path: &str,
    bytes: &'static [u8],
    semantics: DisasterRecoveryComponentSemantics,
) -> (PathBuf, &'static [u8], DisasterRecoveryComponentSemantics) {
    (PathBuf::from(path), bytes, semantics)
}

fn format_identity(family: worth_store_replication::DisasterRecoveryComponentFamily) -> [u8; 32] {
    identity(format!("format/{family:?}").as_bytes())
}

fn backend_identity() -> [u8; 32] {
    identity(b"backend/filesystem-v1")
}

fn identity(value: &[u8]) -> [u8; 32] {
    Sha256::digest(value).into()
}
