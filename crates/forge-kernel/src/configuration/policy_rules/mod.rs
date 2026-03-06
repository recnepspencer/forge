//! Policy rules vertical slice.
//!
//! DOMAIN: Default rules for handling policy ambiguity — section data
//! and overrides.

mod policy_override;
mod policy_section;

pub use policy_override::PolicyOverride;
pub use policy_section::PolicySection;
