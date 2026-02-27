//! Post-operation analysis tools for topology quality.
//!
//! DOMAIN: Quality metrics for boolean results — sliver detection,
//! face area computation, manifold validation, region extraction,
//! causal chain reconstruction, and face-to-face gap measurement.
//!
//! DEPENDENCIES: `forge-topo` (arena traversal), `geometry_state` (positions)

pub mod causal_chain;
pub mod counterfactual;
pub mod gap;
pub mod proof_validation;
pub mod region_extractor;
pub mod sliver;
