#![forbid(unsafe_code)]

mod denial;
mod policy;
mod version;
mod window;
mod witness;

pub use denial::ArtifactCompatibilityDenial;
pub use policy::{
    CompatibilityManifestIndex, CompatibilityRegistrySnapshot, RestoreCompatibilityPlan,
    RestoreCompatibilityReceipt, RollingUpgradeAdmissionPlan, RollingUpgradePolicy,
    RollingWindowCompatibilityReceipt,
};
pub use version::{ArtifactFormatVersion, ArtifactSemanticVersion};
pub use window::ArtifactCompatibilityWindow;
pub use witness::{BackwardReadCompatibilityWitness, ForwardReadCompatibilityWitness};
