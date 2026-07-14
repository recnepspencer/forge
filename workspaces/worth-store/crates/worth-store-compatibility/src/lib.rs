#![forbid(unsafe_code)]

mod admission;
mod denial;
mod policy;
mod version;
mod window;
mod witness;

pub use admission::{
    compatibility_admission, CompatibilityAdmission, RestoreCompatibilityAdmissionOutcome,
    RestoreCompatibilityAdmissionView, RollingCompatibilityAdmissionOutcome,
    RollingCompatibilityAdmissionView,
};
pub use denial::ArtifactCompatibilityDenial;
pub use policy::{
    CompatibilityManifestIndex, CompatibilityRegistrySnapshot, RestoreCompatibilityPlan,
    RestoreCompatibilityReceipt, RollingUpgradeAdmissionPlan, RollingUpgradePolicy,
    RollingWindowCompatibilityReceipt,
};
pub use version::{ArtifactFormatVersion, ArtifactSemanticVersion};
pub use window::ArtifactCompatibilityWindow;
pub use witness::{BackwardReadCompatibilityWitness, ForwardReadCompatibilityWitness};
