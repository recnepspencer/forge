mod compatibility_window;
mod identity;
mod unsupported_version;
mod version;

pub use compatibility_window::{
    BoundaryProtocolCompatibilityWindow, BoundaryProtocolCompatibilityWindowDenial,
};
pub use identity::{BoundaryProtocolIdentity, BoundaryProtocolIdentityDenial};
pub use unsupported_version::{
    BoundaryProtocolUnsupportedVersion, BoundaryProtocolUnsupportedVersionPosture,
};
pub use version::{BoundaryProtocolVersion, BoundaryProtocolVersionDenial};
