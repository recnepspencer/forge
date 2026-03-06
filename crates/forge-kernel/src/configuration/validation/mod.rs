//! Validation vertical slice.
//!
//! DOMAIN: Invariant validation checkpoints and settings — section data
//! and overrides.

mod group_policy;
mod validation_override;
mod validation_section;

pub use group_policy::GroupPolicyConfig;
pub use validation_override::ValidationOverride;
pub use validation_section::ValidationSection;
