use crate::PhysicalSecurityMetadataEnvelope;

use super::support::{decoded_frame_header, decoded_page_header};

#[test]
fn page_metadata_preserves_physical_header_identity() {
    let page = decoded_page_header(7);
    let secured_page = PhysicalSecurityMetadataEnvelope::page_header(page, "page metadata");

    assert_eq!(secured_page.header(), page);
    assert_eq!(secured_page.security_metadata(), "page metadata");
}

#[test]
fn frame_metadata_preserves_physical_header_identity() {
    let frame = decoded_frame_header(7);
    let secured_frame = PhysicalSecurityMetadataEnvelope::frame_header(frame, "frame metadata");

    assert_eq!(secured_frame.header(), frame);
    assert_eq!(secured_frame.security_metadata(), "frame metadata");
}
