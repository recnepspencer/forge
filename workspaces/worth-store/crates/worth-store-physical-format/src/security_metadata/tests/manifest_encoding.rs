use crate::PhysicalSecurityMetadataEnvelope;

use super::support::root_manifest_with_all_entry_kinds;

#[test]
fn root_manifest_metadata_preserves_root_identity() {
    let root = root_manifest_with_all_entry_kinds();
    let root_publication = root.root_publication();
    let secured_root = PhysicalSecurityMetadataEnvelope::root_manifest(root, "root metadata");

    assert_eq!(secured_root.manifest().root_publication(), root_publication);
    assert_eq!(secured_root.security_metadata(), "root metadata");
}

#[test]
fn manifest_entry_metadata_preserves_entry_identity() {
    let root = root_manifest_with_all_entry_kinds();
    let segment = root.segments()[0];
    let page_slot = root.page_slots()[0];
    let extent = root.extents()[0];
    let allocation_class = root.allocation_classes()[0];
    let free_space = root.free_space()[0];

    let secured_segment =
        PhysicalSecurityMetadataEnvelope::segment_manifest_entry(segment, "segment metadata");
    let secured_page =
        PhysicalSecurityMetadataEnvelope::segment_page_manifest_entry(page_slot, "page metadata");
    let secured_extent =
        PhysicalSecurityMetadataEnvelope::extent_manifest_entry(extent, "extent metadata");
    let secured_allocation = PhysicalSecurityMetadataEnvelope::allocation_class_manifest_entry(
        allocation_class,
        "allocation metadata",
    );
    let secured_free_space = PhysicalSecurityMetadataEnvelope::free_space_manifest_entry(
        free_space,
        "free-space metadata",
    );

    assert_eq!(*secured_segment.artifact(), segment);
    assert_eq!(*secured_page.artifact(), page_slot);
    assert_eq!(*secured_extent.artifact(), extent);
    assert_eq!(*secured_allocation.artifact(), allocation_class);
    assert_eq!(*secured_free_space.artifact(), free_space);
}
