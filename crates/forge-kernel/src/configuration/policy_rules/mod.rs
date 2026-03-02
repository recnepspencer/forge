//! Policy rules vertical slice.
//!
//! DOMAIN: Default rules for handling policy ambiguity — section data
//! and overrides.

mod policy_section;
mod policy_override;

pub use policy_section::PolicySection;
pub use policy_override::PolicyOverride;
