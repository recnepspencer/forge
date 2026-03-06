//! Precision vertical slice.
//!
//! DOMAIN: Settings for automatic scaling to arbitrary precision —
//! section data, default constants, overrides, and escalation policy.

mod precision_escalation_policy;
mod precision_override;
mod precision_section;

pub use precision_escalation_policy::PrecisionEscalationPolicy;
pub use precision_override::PrecisionOverride;
pub use precision_section::PrecisionSection;
