mod error;
pub(crate) mod facade;
mod loop_wiring;
mod naming;
mod ownership;
mod radial_rings;
pub mod reference_integrity;
mod registered_report;
mod rule_identity;
mod rule_registry;
mod shared;
mod shell_closure;
mod tests;
mod vertex_disks;

pub use error::TopologyValidationError;
pub(crate) use facade::TopologyValidator;
pub use facade::{
    validate_interpreted_topology, validate_named_topology_truth, DerivedTopologyValidationReport,
    TopologyValidationPhase, TopologyValidationReport,
};
#[cfg(test)]
pub(crate) use loop_wiring::validate as loop_wiring_validation_oracle;
pub(crate) use registered_report::RegisteredTopologyValidationReport;
pub(crate) use rule_identity::loop_wiring_rule;
#[cfg(test)]
pub(crate) use rule_identity::ownership_rule;
pub use rule_identity::TopologyValidationRuleIdentity;
pub(crate) use rule_registry::derived_topology_rule_specs;
