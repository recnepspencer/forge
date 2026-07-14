mod frozen;
mod ids;
mod registration;
mod types;
mod validation;

pub use ids::BridgeAspectRegistrationId;
pub use registration::BridgeAspectRegistration;
pub use types::{SliceWideningPolicy, TruthDeltaSurfaceKind};

pub(crate) use frozen::{FrozenAspectMappingRegistry, FrozenAspectRegistration};
