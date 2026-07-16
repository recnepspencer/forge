use std::io::{Seek, Write};

use sha2::{Digest, Sha256};
use tempfile::NamedTempFile;
use worth_store_security::StoreTenantScope;

use crate::test_support::{blob_scope, integrity_proof_for_scope};

use super::{
    artifact::{FOOTER_BYTES, HEADER_BYTES},
    verify_bounded_blob_backup_artifact, BlobBackupChunkArtifact, BoundedBlobBackupDenial,
    BoundedBlobBackupVerificationRequest,
};

#[test]
fn owner_decoder_accepts_the_blob_owners_canonical_artifact() {
    let (file, bytes, identity) = canonical_artifact(b"owner-issued-blob-payload");
    let digest = Sha256::digest(&bytes).into();

    let observation = verify_bounded_blob_backup_artifact(
        file.path(),
        BoundedBlobBackupVerificationRequest {
            expected_identity: &identity,
            expected_bytes: bytes.len() as u64,
            expected_digest: digest,
            max_buffer_bytes: 4 * 1024,
        },
    )
    .expect("the blob owner's canonical artifact must verify");

    assert_eq!(observation.payload_bytes(), 25);
    assert_eq!(observation.bytes_read(), bytes.len() as u64);
    assert!(observation.peak_buffer_bytes() <= 4 * 1024);
}

#[test]
fn recomputing_transport_and_internal_hashes_cannot_forge_blob_meaning() {
    let (mut file, mut bytes, identity) = canonical_artifact(b"owner-issued-blob-payload");
    let metadata_bytes = (0..5)
        .map(|index| u16::from_le_bytes(bytes[42 + index * 2..44 + index * 2].try_into().unwrap()))
        .map(usize::from)
        .sum::<usize>();
    bytes[HEADER_BYTES + metadata_bytes] ^= 0x5a;

    let footer_offset = bytes.len() - FOOTER_BYTES;
    let owner_digest = Sha256::digest(&bytes[..footer_offset]);
    bytes[footer_offset..].copy_from_slice(&owner_digest);
    file.as_file_mut().set_len(0).unwrap();
    file.rewind().unwrap();
    file.write_all(&bytes).unwrap();
    file.flush().unwrap();
    let attacker_recomputed_outer_digest = Sha256::digest(&bytes).into();

    let denial = verify_bounded_blob_backup_artifact(
        file.path(),
        BoundedBlobBackupVerificationRequest {
            expected_identity: &identity,
            expected_bytes: bytes.len() as u64,
            expected_digest: attacker_recomputed_outer_digest,
            max_buffer_bytes: 4 * 1024,
        },
    )
    .expect_err("transport hashes cannot confer blob-owner meaning");

    assert!(
        matches!(denial, BoundedBlobBackupDenial::ChecksumMismatch),
        "unexpected denial: {denial:?}"
    );
}

fn canonical_artifact(payload: &[u8]) -> (NamedTempFile, Vec<u8>, String) {
    let scope = blob_scope(
        "backup-owner-codec",
        StoreTenantScope::TenantPhysicalBoundary,
    );
    let proof = integrity_proof_for_scope(scope, payload);
    let artifact = BlobBackupChunkArtifact::from_integrity_proof(&proof, payload)
        .expect("the owner must encode its own integrity proof");
    let identity = artifact.chunk_identity().to_owned();
    let mut bytes = Vec::new();
    artifact.encode(&mut bytes).unwrap();
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(&bytes).unwrap();
    file.flush().unwrap();
    (file, bytes, identity)
}
