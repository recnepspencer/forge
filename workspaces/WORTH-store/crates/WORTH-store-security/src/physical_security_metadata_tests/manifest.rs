use crate::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreKeyVersionPosture,
    StoreLegacySecurityPosture, StorePhysicalSecurityMetadataCarrier,
    StorePhysicalSecurityMetadataEnvelope,
};

use super::support::{
    admitted_scope, assert_platform_metadata, current_authority, root_manifest_with_all_entry_kinds,
};

#[test]
fn root_manifest_metadata_preserves_root_identity() {
    let authority = current_authority("s51.phase3.manifest.root", "root");
    let witnesses = admitted_scope(&authority);
    let metadata = StorePhysicalSecurityMetadataCarrier::for_manifest(
        &witnesses,
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let root = root_manifest_with_all_entry_kinds();
    let root_publication = root.root_publication();

    let secured_root = StorePhysicalSecurityMetadataEnvelope::root_manifest(root, metadata);

    assert_eq!(secured_root.manifest().root_publication(), root_publication);
    assert_eq!(
        secured_root.security_metadata().authenticity_requirement(),
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        )
    );
    assert_platform_metadata(
        secured_root.security_metadata(),
        StoreLegacySecurityPosture::NativeScoped,
        StoreKeyVersionPosture::Current,
    );
}

#[test]
fn manifest_entry_metadata_preserves_entry_identity() {
    let authority = current_authority("s51.phase3.manifest.entries", "entries");
    let witnesses = admitted_scope(&authority);
    let metadata = StorePhysicalSecurityMetadataCarrier::for_manifest(
        &witnesses,
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let root = root_manifest_with_all_entry_kinds();
    let segment = root.segments()[0];
    let page_slot = root.page_slots()[0];
    let extent = root.extents()[0];
    let allocation_class = root.allocation_classes()[0];
    let free_space = root.free_space()[0];

    let secured_segment =
        StorePhysicalSecurityMetadataEnvelope::segment_manifest_entry(segment, metadata);
    let secured_page =
        StorePhysicalSecurityMetadataEnvelope::segment_page_manifest_entry(page_slot, metadata);
    let secured_extent =
        StorePhysicalSecurityMetadataEnvelope::extent_manifest_entry(extent, metadata);
    let secured_allocation = StorePhysicalSecurityMetadataEnvelope::allocation_class_manifest_entry(
        allocation_class,
        metadata,
    );
    let secured_free_space =
        StorePhysicalSecurityMetadataEnvelope::free_space_manifest_entry(free_space, metadata);

    assert_eq!(*secured_segment.artifact(), segment);
    assert_eq!(*secured_page.artifact(), page_slot);
    assert_eq!(*secured_extent.artifact(), extent);
    assert_eq!(*secured_allocation.artifact(), allocation_class);
    assert_eq!(*secured_free_space.artifact(), free_space);
    assert_platform_metadata(
        secured_segment.security_metadata(),
        StoreLegacySecurityPosture::NativeScoped,
        StoreKeyVersionPosture::Current,
    );
    assert_platform_metadata(
        secured_page.security_metadata(),
        StoreLegacySecurityPosture::NativeScoped,
        StoreKeyVersionPosture::Current,
    );
    assert_platform_metadata(
        secured_extent.security_metadata(),
        StoreLegacySecurityPosture::NativeScoped,
        StoreKeyVersionPosture::Current,
    );
    assert_platform_metadata(
        secured_allocation.security_metadata(),
        StoreLegacySecurityPosture::NativeScoped,
        StoreKeyVersionPosture::Current,
    );
    assert_platform_metadata(
        secured_free_space.security_metadata(),
        StoreLegacySecurityPosture::NativeScoped,
        StoreKeyVersionPosture::Current,
    );
}
