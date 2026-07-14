use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalRootManifestRebuildSource, PhysicalSegmentId, PhysicalStoreIdentity,
    PlatformPhysicalAppendRequest,
};

/// Execute ordinary physical Store operations and reopen their current root as
/// a rebuild source. The helper prepares bytes; authority remains physical-format owned.
pub fn execute_root_manifest_rebuild_source(
    store: &PhysicalStoreIdentity,
    segment: u64,
    first_page: u64,
    row_count: u64,
) -> PhysicalRootManifestRebuildSource {
    let segment = PhysicalSegmentId::from_raw(segment).expect("fixture segment is nonzero");
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let mut runtime = super::open_layout_physical_facade_for_store(store);

    for page in first_page..first_page + row_count {
        let slot = generations
            .slot_cell(
                segment,
                PhysicalPageId::from_raw(page).expect("fixture page is nonzero"),
                PhysicalRecordSlot::from_raw(1).expect("fixture slot is nonzero"),
            )
            .with_slot_generation(
                PhysicalGeneration::from_raw(7).expect("fixture generation is nonzero"),
            );
        runtime
            .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
                slot,
                format!("rebuild-row-{}-{page}", segment.get()).as_bytes(),
            ))
            .expect("fixture row must append through the physical Store facade");
    }

    runtime
        .publish_physical_root()
        .expect("fixture root must publish through the physical Store facade");
    runtime
        .root_manifest_rebuild_source()
        .expect("the opened Store must issue its root-manifest rebuild source")
}
