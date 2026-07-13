use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalReference,
    PhysicalReferenceAuthority, PhysicalRootReference, PhysicalSegmentId,
};

use crate::{
    CheckpointCandidate, CheckpointCandidateDiscoverySource, CheckpointCoveredLsnRange,
    CheckpointManifest, CheckpointPageLsnFrontier, CheckpointRedoBoundary, CheckpointRootPosture,
    CheckpointValidation, CheckpointValidationDenialKind, LogSequenceNumber, PageLsn,
    SharpCheckpointCertificationMode,
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

fn page_cell() -> forge_store_physical_format::PageGenerationCell {
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
