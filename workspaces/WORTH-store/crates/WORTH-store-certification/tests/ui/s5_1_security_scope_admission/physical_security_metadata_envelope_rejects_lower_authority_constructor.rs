use worth_store_physical_format::{PhysicalFrameHeader, PhysicalSecurityMetadataEnvelope};
use worth_store_security::StoreAuthenticityResult;

fn main() {
    let frame: PhysicalFrameHeader = todo!();
    let result: StoreAuthenticityResult = todo!();
    let _WORTHd = PhysicalSecurityMetadataEnvelope::frame_header(frame, result);
}
