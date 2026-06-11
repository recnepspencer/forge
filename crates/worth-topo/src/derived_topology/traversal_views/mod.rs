mod facade;
#[cfg(test)]
mod tests;
pub(crate) mod types;

pub use facade::{build_topology_read_artifact, certify_topology_view, interpret_topology_view};
pub use types::InterpretedTopologyView;
