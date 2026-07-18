use sha2::{Digest, Sha256};
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalReference,
    PhysicalReferenceAuthority, PhysicalRootReference, PhysicalSegmentId,
};

use crate::{
    verify_bounded_checkpoint_backup_artifact, BoundedCheckpointBackupDenial,
    BoundedCheckpointBackupVerificationRequest, CheckpointBackupArtifact, CheckpointCandidate,
    CheckpointCandidateDiscoverySource, CheckpointCoveredLsnRange, CheckpointManifest,
    CheckpointPageLsnFrontier, CheckpointRedoBoundary, CheckpointRootPosture, CheckpointValidation,
    CheckpointValidationDenialKind, LogSequenceNumber, PageLsn, SharpCheckpointCertificationMode,
};

#[test]
fn unlocated_candidate_cannot_be_validated() {
    let candidate = CheckpointCandidate::from_manifest(
        manifest(10, 20, 19),
        CheckpointCandidateDiscoverySource::DirectoryListing,
    );

    let denial = CheckpointValidation::require_locator(candidate).unwrap_err();

    assert_eq!(
        denial.kind(),
        CheckpointValidationDenialKind::MissingCheckpointLocator
    );
}

#[test]
fn backup_checkpoint_round_trips_through_the_owner_decoder() {
    let checkpoint = CheckpointBackupArtifact::from_sharp_manifest(&manifest(1, 11, 10), 3, 10)
        .expect("owner checkpoint artifact");
    let mut bytes = Vec::new();
    checkpoint.encode(&mut bytes).expect("checkpoint encoding");
    let file = temporary_checkpoint_file("round-trip");
    let path = file.path();
    std::fs::write(path, &bytes).expect("checkpoint media");
    let request = checkpoint_request(&checkpoint, &bytes);

    let observation = verify_bounded_checkpoint_backup_artifact(path, request)
        .expect("bounded owner verification");

    assert_eq!(observation.page_count(), 1);
    assert_eq!(observation.bytes_read(), bytes.len() as u64);
}

#[test]
fn rehashed_checkpoint_row_corruption_fails_owner_integrity() {
    let checkpoint = CheckpointBackupArtifact::from_sharp_manifest(&manifest(1, 11, 10), 3, 10)
        .expect("owner checkpoint artifact");
    let mut bytes = Vec::new();
    checkpoint.encode(&mut bytes).expect("checkpoint encoding");
    bytes[78] ^= 0x20;
    let file = temporary_checkpoint_file("corrupt-row");
    let path = file.path();
    std::fs::write(path, &bytes).expect("corrupt checkpoint media");

    let denial =
        verify_bounded_checkpoint_backup_artifact(path, checkpoint_request(&checkpoint, &bytes))
            .expect_err("outer digest cannot replace checkpoint owner integrity");

    assert!(matches!(
        denial,
        BoundedCheckpointBackupDenial::InvalidPageFrontier
            | BoundedCheckpointBackupDenial::InternalDigestMismatch
    ));
}

fn checkpoint_request<'a>(
    checkpoint: &'a CheckpointBackupArtifact,
    bytes: &[u8],
) -> BoundedCheckpointBackupVerificationRequest<'a> {
    BoundedCheckpointBackupVerificationRequest {
        checkpoint_identity: checkpoint.checkpoint_identity(),
        manifest_generation: checkpoint.manifest_generation(),
        durable_checkpoint_lsn: checkpoint.durable_checkpoint_lsn(),
        root_generation: checkpoint.root().generation().get(),
        expected_bytes: bytes.len() as u64,
        expected_digest: Sha256::digest(bytes).into(),
        max_buffer_bytes: 256,
    }
}

fn temporary_checkpoint_file(case: &str) -> tempfile::NamedTempFile {
    tempfile::Builder::new()
        .prefix(&format!("worth-store-checkpoint-{case}-"))
        .suffix(".bin")
        .tempfile()
        .unwrap()
}

fn manifest(start: u64, end: u64, redo: u64) -> CheckpointManifest {
    CheckpointManifest::sharp(
        CheckpointRootPosture::root_present(root_record_reference()),
        frontier(redo),
        CheckpointCoveredLsnRange::new(lsn(start), lsn(end)).unwrap(),
        CheckpointRedoBoundary::from_page_lsn(PageLsn::from_lsn(lsn(redo))),
        SharpCheckpointCertificationMode::certified(),
    )
    .unwrap()
}

fn frontier(redo: u64) -> CheckpointPageLsnFrontier {
    CheckpointPageLsnFrontier::from_pages([(page_cell(), PageLsn::from_lsn(lsn(redo)))]).unwrap()
}

fn page_cell() -> worth_store_physical_format::PageGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
        )
        .with_page_generation(PhysicalGeneration::from_raw(1).unwrap())
}

fn root_reference() -> PhysicalRootReference {
    PhysicalRootReference::from_raw(1).unwrap()
}

fn root_record_reference() -> PhysicalReference {
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .root_publication_cell(root_reference())
        .with_root_publication_generation(PhysicalGeneration::from_raw(1).unwrap());
    PhysicalReferenceAuthority::for_canonical_physical_format()
        .admit_root_publication(cell)
        .reference()
}

fn lsn(value: u64) -> LogSequenceNumber {
    LogSequenceNumber::new(value)
}
