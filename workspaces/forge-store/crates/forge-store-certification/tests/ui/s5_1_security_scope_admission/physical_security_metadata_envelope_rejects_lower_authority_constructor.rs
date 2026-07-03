use forge_store_physical_format::{PhysicalFrameHeader, PhysicalSecurityMetadataEnvelope};
use forge_store_security::StoreAuthenticityResult;

fn main() {
    let frame: PhysicalFrameHeader = todo!();
    let result: StoreAuthenticityResult = todo!();
    let _forged = PhysicalSecurityMetadataEnvelope::frame_header(frame, result);
}
