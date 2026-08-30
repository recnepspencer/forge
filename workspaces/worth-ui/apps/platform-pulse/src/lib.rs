//! Stable cross-process observation meaning for the permanent Platform Pulse.

mod application_readiness;

pub const PLATFORM_PULSE_STATUS_QUERY_VIEW: &str = "platform.pulse.status";

pub mod intent;
mod native_seed_application;
pub mod observation_contract;
#[doc(hidden)]
pub mod product_world;
pub mod visual_identity_pulse;

#[doc(hidden)]
pub use application_readiness::PlatformPulseApplicationReadinessSignal;
pub use native_seed_application::PlatformPulseNativeSeedApplication;
