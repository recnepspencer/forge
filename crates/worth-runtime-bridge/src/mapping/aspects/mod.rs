mod frozen;
mod ids;
mod registration;
mod types;
mod validation;

pub use ids::BridgeAspectRegistrationId;
pub use registration::BridgeAspectRegistration;
pub use types::{
    BridgeAuthoritativeSourcePrecisionPolicy, SliceWideningPolicy, TruthDeltaSurfaceKind,
};

pub(crate) use frozen::{FrozenAspectMappingRegistry, FrozenAspectRegistration};
