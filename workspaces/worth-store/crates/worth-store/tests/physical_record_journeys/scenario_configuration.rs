use worth_store::physical_runtime::{
    ManifestEntryCapacity, PageFillPercent, PhysicalRecordAccessPolicy,
    PhysicalRecordPlacementPolicy, RecordByteLimit, RecordCountLimit, SegmentPageCount,
};

use super::configuration;

pub(super) fn dense_configuration(
    segment_pages: u32,
) -> (
    worth_store::physical_runtime::AdmittedPhysicalRecordFormat,
    worth_store::physical_runtime::AdmittedRecordPlacementPolicy,
    worth_store::physical_runtime::AdmittedRecordAccessPolicy,
) {
    let (format, _, access) = configuration();
    let placement = PhysicalRecordPlacementPolicy::builder()
        .segment_pages(SegmentPageCount::new(segment_pages).unwrap())
        .extent_threshold(RecordByteLimit::new(8_000).unwrap())
        .page_fill(PageFillPercent::new(50).unwrap())
        .manifest_capacity(ManifestEntryCapacity::new(128).unwrap())
        .admit(format)
        .unwrap();
    (format, placement, access)
}

pub(super) fn courtroom_configuration() -> (
    worth_store::physical_runtime::AdmittedPhysicalRecordFormat,
    worth_store::physical_runtime::AdmittedRecordPlacementPolicy,
    worth_store::physical_runtime::AdmittedRecordAccessPolicy,
) {
    let (format, _, _) = configuration();
    let placement = PhysicalRecordPlacementPolicy::builder()
        .segment_pages(SegmentPageCount::new(32).unwrap())
        .extent_threshold(RecordByteLimit::new(8_192).unwrap())
        .manifest_capacity(ManifestEntryCapacity::new(16).unwrap())
        .admit(format)
        .unwrap();
    let access = PhysicalRecordAccessPolicy::builder()
        .transfer_limit(RecordByteLimit::new(65_536).unwrap())
        .scratch_limit(RecordByteLimit::new(131_072).unwrap())
        .scan_record_limit(RecordCountLimit::new(17).unwrap())
        .admit(format)
        .unwrap();
    (format, placement, access)
}
