//! Configuration data layer.
//!
//! DOMAIN: Pure data shapes for the kernel configuration subsystem.

mod kernel_config;
mod overrides;
mod tolerance_section;
mod solver_section;
mod validation_section;
mod policy_section;
mod precision_section;

pub use kernel_config::{ConfigSection, KernelConfig};
pub use overrides::{
    ConfigOverride, PolicyOverride, PrecisionOverride, SolverOverride, ToleranceOverride,
    ValidationOverride,
};
pub use tolerance_section::{ToleranceSection, UnitSystem};
pub use solver_section::SolverSection;
pub use validation_section::ValidationSection;
pub use policy_section::PolicySection;
pub use precision_section::PrecisionSection;

pub mod defaults;
