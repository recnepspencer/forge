use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalReference,
    PhysicalReferenceAuthority, PhysicalRootReference, PhysicalSegmentId,
};
use worth_store_recovery_physics::{
    CheckpointCoveredLsnRange, CheckpointManifest, CheckpointPageLsnFrontier,
    CheckpointRedoBoundary, CheckpointRootPosture, LogSequenceNumber, PageLsn,
    SharpCheckpointCertificationMode, WalLsnRange,
};

pub fn manifest(start: u64, end: u64, redo: u64) -> CheckpointManifest {
    CheckpointManifest::sharp(
        CheckpointRootPosture::root_present(root_record_reference()),
        frontier(redo),
        covered_range(start, end),
        redo_boundary(redo),
        SharpCheckpointCertificationMode::certified(),
    )
    .unwrap()
}

pub fn covered_range(start: u64, end: u64) -> CheckpointCoveredLsnRange {
    CheckpointCoveredLsnRange::new(lsn(start), lsn(end)).unwrap()
}

pub fn redo_boundary(value: u64) -> CheckpointRedoBoundary {
    CheckpointRedoBoundary::from_page_lsn(PageLsn::from_lsn(lsn(value)))
}

pub fn frontier(value: u64) -> CheckpointPageLsnFrontier {
    CheckpointPageLsnFrontier::from_pages([(page_cell(), PageLsn::from_lsn(lsn(value)))]).unwrap()
}

pub fn wal_range(start: u64, end: u64) -> WalLsnRange {
    WalLsnRange::new(lsn(start), lsn(end)).unwrap()
}

pub fn page_cell() -> worth_store_physical_format::PageGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
        )
        .with_page_generation(PhysicalGeneration::from_raw(1).unwrap())
}

pub fn root_reference() -> PhysicalRootReference {
    PhysicalRootReference::from_raw(1).unwrap()
}

pub fn root_record_reference() -> PhysicalReference {
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
