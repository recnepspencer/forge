//! Tolerance vertical slice.
//!
//! DOMAIN: Spatial, angular, and tolerance settings — section data,
//! default constants, overrides, and policies.

mod gap_closure_policy;
mod sliver_policy;
mod tangency_policy;
mod tolerance_config;
mod tolerance_override;
mod tolerance_policy;
pub mod tolerance_section;

pub use gap_closure_policy::GapClosurePolicy;
pub use sliver_policy::SliverPolicy;
pub use tangency_policy::TangencyPolicy;
pub use tolerance_config::ToleranceConfig;
pub use tolerance_override::ToleranceOverride;
pub use tolerance_policy::TolerancePolicy;
pub use tolerance_section::{ToleranceSection, UnitSystem};
