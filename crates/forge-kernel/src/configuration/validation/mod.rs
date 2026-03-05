//! Validation vertical slice.
//!
//! DOMAIN: Invariant validation checkpoints and settings — section data
//! and overrides.

mod group_policy;
mod validation_section;
mod validation_override;

pub use group_policy::GroupPolicyConfig;
pub use validation_section::ValidationSection;
pub use validation_override::ValidationOverride;
