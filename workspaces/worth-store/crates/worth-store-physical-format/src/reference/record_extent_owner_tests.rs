use crate::{
    PhysicalCellReuseDomain, PhysicalExtentId, PhysicalGeneration, PhysicalGenerationAuthority,
    PhysicalSegmentId,
};

#[test]
fn top_level_record_extent_owner_never_fabricates_segment_ownership() {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let extent = PhysicalExtentId::from_raw(44).expect("extent");
    let generation = PhysicalGeneration::from_raw(1).expect("generation");
    let record_extent_owner = generations
        .record_extent_cell(extent)
        .with_extent_generation(generation)
        .owner();
    let segment_extent_owner = generations
        .extent_cell(PhysicalSegmentId::from_raw(7).expect("segment"), extent)
        .with_extent_generation(generation)
        .owner();

    assert_eq!(
        record_extent_owner.domain(),
        PhysicalCellReuseDomain::RecordExtentAllocation
    );
    assert_eq!(record_extent_owner.segment_id(), None);
    assert_eq!(record_extent_owner.extent_id(), Some(extent));
    assert_ne!(record_extent_owner, segment_extent_owner);
}
