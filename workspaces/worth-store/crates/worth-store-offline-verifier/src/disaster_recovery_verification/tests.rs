use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use tempfile::TempDir;
use worth_store_physical_isolation::{
    RecoveredRecoverySourceLease, RecoverySourceLeaseKind, RecoverySourceLeaseRegistry,
};
use worth_store_replication::{
    DisasterRecoveryArtifactEvidence, DisasterRecoveryComponent,
    DisasterRecoveryComponentSemantics, ReplicaRecoveryFrontier, ReplicationDisasterRecoveryOwner,
    ReplicationLineageIdentity,
};
use worth_store_security::admitted_store_internal_security_scope_for_io_qos_test;

use super::{
    open_disaster_recovery_bundle, verify_disaster_recovery_bundle, DisasterRecoveryClosureDenial,
    DisasterRecoveryVerificationDenial, DisasterRecoveryVerificationPolicy,
};

const OPERATION_IDENTITY: [u8; 32] = [0xA4; 32];

#[test]
fn independently_verified_bundle_becomes_restart_safe_bootstrap_lease() {
    let fixture = DisasterRecoveryFixture::materialize();
    let materialized = fixture.declare_bundle();
    let expected_source_identity = materialized.manifest_identity();
    drop(materialized);
    let opened = open_disaster_recovery_bundle(fixture.bundle_root(), 64 * 1024)
        .expect("a fresh process should decode the canonical manifest");
    let verified = verify_disaster_recovery_bundle(opened, 3, &fixture.verification_policy())
        .expect("real bundle bytes should independently verify");
    assert_eq!(verified.counters().cross_component_closure_checks(), 10);
    assert_eq!(verified.counters().assumption_checks(), 8);
    let resolved = verified
        .resolve_bootstrap_source_cut(OPERATION_IDENTITY, 2, 32 * 1024)
        .expect("verified DR evidence should resolve through Recovery Physics");

    assert_eq!(resolved.source_identity(), expected_source_identity);
    let expected_resolution_identity = resolved.resolution_identity();
    assert_eq!(resolved.counters().artifacts_reopened(), 4);
    assert_eq!(resolved.counters().bytes_read(), fixture.total_bytes());
    assert_eq!(resolved.counters().resident_buffer_bytes(), 2);

    let registry_root = fixture.workspace.path().join("control/source-leases");
    let admitted = RecoverySourceLeaseRegistry::open(&registry_root)
        .expect("lease registry should open")
        .admit_bootstrap_source_cut(resolved)
        .expect("Isolation should admit only the resolved bootstrap cut");
    let expected_binding = admitted.binding_fingerprint();
    drop(admitted);

    let recovered = RecoverySourceLeaseRegistry::open(&registry_root)
        .expect("lease registry should reopen after simulated process loss")
        .recover_bound(
            OPERATION_IDENTITY,
            RecoverySourceLeaseKind::ReplicaBootstrap,
        )
        .expect("the bootstrap source lease should recover by operation and kind");
    let RecoveredRecoverySourceLease::ReplicaBootstrap(lease) = recovered else {
        panic!("recovered operation must remain a replica-bootstrap lease");
    };
    assert_eq!(lease.source_identity(), expected_source_identity);
    assert_eq!(
        lease.source_evidence_identity(),
        expected_resolution_identity
    );
    assert_eq!(lease.binding_fingerprint(), expected_binding);
    assert_eq!(lease.source_root(), fixture.bundle_root());
    assert_eq!(
        lease.artifact_names(),
        [
            "authority.manifest",
            "blobs/nested/chunk-7",
            "checkpoint.bin",
            "wal.log"
        ]
    );

    let release = lease
        .release()
        .expect("verified target handoff should release the lease");
    assert_eq!(release.source_identity(), expected_source_identity);
    assert_eq!(
        release.source_evidence_identity(),
        expected_resolution_identity
    );
    assert!(RecoverySourceLeaseRegistry::open(&registry_root)
        .expect("lease registry should reopen after release")
        .recover_active()
        .expect("released registry should remain readable")
        .is_empty());
}

#[test]
fn valid_component_digests_cannot_hide_broken_blob_reachability() {
    let mut fixture = DisasterRecoveryFixture::materialize();
    fixture.replace_blob_closure(identity("unrelated-blob-closure"));
    drop(fixture.declare_bundle());
    let opened = open_disaster_recovery_bundle(fixture.bundle_root(), 64 * 1024)
        .expect("the individually valid defective manifest remains materialized");

    assert!(matches!(
        verify_disaster_recovery_bundle(opened, 5, &fixture.verification_policy()),
        Err(DisasterRecoveryVerificationDenial::CrossComponentClosure(
            DisasterRecoveryClosureDenial::BlobClosureMismatch
        ))
    ));
}

#[test]
fn valid_wal_digest_cannot_hide_a_frontier_gap() {
    let mut fixture = DisasterRecoveryFixture::materialize();
    fixture.replace_wal_range(92, 93);
    drop(fixture.declare_bundle());
    let opened = open_disaster_recovery_bundle(fixture.bundle_root(), 64 * 1024)
        .expect("the individually valid defective manifest remains materialized");

    assert!(matches!(
        verify_disaster_recovery_bundle(opened, 5, &fixture.verification_policy()),
        Err(DisasterRecoveryVerificationDenial::CrossComponentClosure(
            DisasterRecoveryClosureDenial::WalCoverageGapOrOverlap
        ))
    ));
}

struct DisasterRecoveryFixture {
    workspace: TempDir,
    root: PathBuf,
    components: Vec<(PathBuf, Vec<u8>, DisasterRecoveryComponentSemantics)>,
}

impl DisasterRecoveryFixture {
    fn materialize() -> Self {
        let workspace = tempfile::tempdir().expect("fixture workspace should exist");
        let root = workspace.path().join("bundle");
        let lineage_identity = lineage().stable_fingerprint();
        let checkpoint_identity = identity("checkpoint/90");
        let blob_closure_identity = identity("blob-closure/checkpoint-90");
        let components = vec![
            (
                PathBuf::from("authority.manifest"),
                b"authority-epoch=9".to_vec(),
                DisasterRecoveryComponentSemantics::Authority {
                    lineage_identity,
                    authority_epoch: 9,
                },
            ),
            (
                PathBuf::from("checkpoint.bin"),
                b"checkpoint-lsn=90".to_vec(),
                DisasterRecoveryComponentSemantics::Checkpoint {
                    lineage_identity,
                    authority_epoch: 9,
                    checkpoint_identity,
                    checkpoint_lsn: 90,
                    blob_closure_identity,
                },
            ),
            (
                PathBuf::from("wal.log"),
                b"lsn=91\nlsn=92".to_vec(),
                DisasterRecoveryComponentSemantics::Wal {
                    lineage_identity,
                    authority_epoch: 9,
                    start_lsn: 91,
                    end_lsn_exclusive: 93,
                },
            ),
            (
                PathBuf::from("blobs/nested/chunk-7"),
                b"content-addressed-blob".to_vec(),
                DisasterRecoveryComponentSemantics::Blob {
                    blob_closure_identity,
                },
            ),
        ];
        for (relative, bytes, _) in &components {
            let path = root.join(relative);
            std::fs::create_dir_all(path.parent().expect("component must have a parent"))
                .expect("component directory should materialize");
            std::fs::write(path, bytes).expect("component bytes should materialize");
        }
        Self {
            workspace,
            root: std::fs::canonicalize(root).expect("bundle root should canonicalize"),
            components,
        }
    }

    fn declare_bundle(&self) -> worth_store_replication::MaterializedDisasterRecoveryBundle {
        let declarations = self
            .components
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
                    .expect("fixture evidence should be complete"),
                    *semantics,
                )
                .expect("fixture component declaration should be valid")
            })
            .collect();
        ReplicationDisasterRecoveryOwner::record_materialized_bundle(
            &self.root,
            lineage(),
            ReplicaRecoveryFrontier::admit(94, 93, 92, 92, 9)
                .expect("fixture frontier should be ordered"),
            admitted_store_internal_security_scope_for_io_qos_test().identity(),
            92,
            declarations,
        )
        .expect("fixture bundle should be structurally admissible")
    }

    fn total_bytes(&self) -> u64 {
        self.components
            .iter()
            .map(|(_, bytes, _)| bytes.len() as u64)
            .sum()
    }

    fn bundle_root(&self) -> &Path {
        &self.root
    }

    fn verification_policy(&self) -> DisasterRecoveryVerificationPolicy {
        let mut formats = self
            .components
            .iter()
            .map(|(_, _, semantics)| identity(&format!("format/{:?}", semantics.family())))
            .collect::<Vec<_>>();
        formats.sort();
        formats.dedup();
        DisasterRecoveryVerificationPolicy::from_supported_assumptions(
            formats,
            vec![identity("backend/filesystem-v1")],
        )
        .expect("fixture verification assumptions should be explicit and unique")
    }

    fn replace_blob_closure(&mut self, replacement: [u8; 32]) {
        let semantics = self
            .components
            .iter_mut()
            .find_map(|(_, _, semantics)| match semantics {
                DisasterRecoveryComponentSemantics::Blob { .. } => Some(semantics),
                _ => None,
            })
            .expect("fixture must contain a blob component");
        *semantics = DisasterRecoveryComponentSemantics::Blob {
            blob_closure_identity: replacement,
        };
    }

    fn replace_wal_range(&mut self, start_lsn: u64, end_lsn_exclusive: u64) {
        let lineage_identity = lineage().stable_fingerprint();
        let semantics = self
            .components
            .iter_mut()
            .find_map(|(_, _, semantics)| match semantics {
                DisasterRecoveryComponentSemantics::Wal { .. } => Some(semantics),
                _ => None,
            })
            .expect("fixture must contain a WAL component");
        *semantics = DisasterRecoveryComponentSemantics::Wal {
            lineage_identity,
            authority_epoch: 9,
            start_lsn,
            end_lsn_exclusive,
        };
    }
}

fn lineage() -> ReplicationLineageIdentity {
    ReplicationLineageIdentity::from_declared_lineage("lineage/production/9")
        .expect("fixture lineage should be valid")
}

fn identity(label: &str) -> [u8; 32] {
    Sha256::digest(label.as_bytes()).into()
}
