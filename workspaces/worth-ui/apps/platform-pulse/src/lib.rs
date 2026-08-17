//! Stable cross-process observation meaning for the permanent Platform Pulse.

pub const PLATFORM_PULSE_STATUS_QUERY_VIEW: &str = "platform.pulse.status";

pub mod intent;
mod native_seed_application;
pub mod observation_contract;
pub mod visual_identity_pulse;

pub use native_seed_application::PlatformPulseNativeSeedApplication;
