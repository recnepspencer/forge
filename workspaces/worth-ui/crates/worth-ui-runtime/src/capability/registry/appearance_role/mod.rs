mod descriptor;
mod frozen_entry;
mod identity;
mod registration;
mod registry;
mod semantic_digest;

pub use frozen_entry::FrozenAppearanceRoleCapabilities;
pub(crate) use registration::AppearanceRoleAcceptedRegistrationProof;
pub(crate) use registry::{AppearanceRoleRegistrationDenial, AppearanceRoleRegistry};
