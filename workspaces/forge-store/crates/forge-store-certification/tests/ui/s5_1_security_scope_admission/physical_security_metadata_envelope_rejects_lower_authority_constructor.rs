use forge_store_physical_format::{
    PhysicalAuthenticityIdentity, PhysicalFrameHeader, PhysicalSecurityMetadataEnvelope,
};
use forge_store_security::StoreAuthenticityResult;

fn main() {
    let frame: PhysicalFrameHeader = todo!();
    let result: StoreAuthenticityResult<PhysicalAuthenticityIdentity> = todo!();
    let _forged = PhysicalSecurityMetadataEnvelope::frame_header(frame, result);
}
