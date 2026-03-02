//! Configuration policy layer.
//!
//! DOMAIN: Thin, read-only accessors that present focused slices of the
//! resolved configuration to lower-layer callers. Each struct groups
//! related tolerance thresholds into a domain-specific policy object.

mod tolerance;
mod tangency;
mod sliver;
mod gap_closure;
mod precision_escalation;
mod tolerance_config;

pub use tolerance::TolerancePolicy;
pub use tangency::TangencyPolicy;
pub use sliver::SliverPolicy;
pub use gap_closure::GapClosurePolicy;
pub use precision_escalation::PrecisionEscalationPolicy;
pub use tolerance_config::ToleranceConfig;
