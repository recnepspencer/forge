use crate::{
    StoreKeyVersionPosture, StoreLegacySecurityPosture, StorePhysicalSecurityMetadataCarrier,
    StorePhysicalSecurityMetadataEnvelope,
};

use super::support::{
    admitted_scope, assert_platform_metadata, current_authority, decoded_frame_header,
    decoded_page_header,
};

#[test]
fn page_metadata_preserves_physical_header_identity() {
    let authority = current_authority("s51.phase3.physical.page", "page");
    let witnesses = admitted_scope(&authority);
    let page_metadata = StorePhysicalSecurityMetadataCarrier::for_page_header(
        &witnesses,
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let page = decoded_page_header(7);

    let secured_page = StorePhysicalSecurityMetadataEnvelope::page_header(page, page_metadata);

    assert_eq!(secured_page.header(), page);
    assert_platform_metadata(
        secured_page.security_metadata(),
        StoreLegacySecurityPosture::NativeScoped,
        StoreKeyVersionPosture::Current,
    );
}

#[test]
fn frame_metadata_preserves_physical_header_identity() {
    let authority = current_authority("s51.phase3.physical.frame", "frame");
    let witnesses = admitted_scope(&authority);
    let frame_metadata = StorePhysicalSecurityMetadataCarrier::for_frame_header(
        &witnesses,
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let frame = decoded_frame_header(7);

    let secured_frame = StorePhysicalSecurityMetadataEnvelope::frame_header(frame, frame_metadata);

    assert_eq!(secured_frame.header(), frame);
    assert_platform_metadata(
        secured_frame.security_metadata(),
        StoreLegacySecurityPosture::NativeScoped,
        StoreKeyVersionPosture::Current,
    );
}
