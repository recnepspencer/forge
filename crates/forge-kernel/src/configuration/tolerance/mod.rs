//! Tolerance vertical slice.
//!
//! DOMAIN: Spatial, angular, and tolerance settings — section data,
//! default constants, overrides, and policies.

pub mod tolerance_section;
mod tolerance_override;
mod tolerance_policy;
mod tangency_policy;
mod sliver_policy;
mod gap_closure_policy;
mod tolerance_config;

pub use tolerance_section::{ToleranceSection, UnitSystem};
pub use tolerance_override::ToleranceOverride;
pub use tolerance_policy::TolerancePolicy;
pub use tangency_policy::TangencyPolicy;
pub use sliver_policy::SliverPolicy;
pub use gap_closure_policy::GapClosurePolicy;
pub use tolerance_config::ToleranceConfig;
