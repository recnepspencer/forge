//! Post-operation analysis tools for topology quality.
//!
//! DOMAIN: Quality metrics for boolean results — sliver detection,
//! face area computation, manifold validation, region extraction,
//! causal chain reconstruction.
//!
//! DEPENDENCIES: `forge-topo` (arena traversal), `geometry_store` (positions)

pub mod sliver;
pub mod proof_validation;
pub mod region_extractor;
pub mod causal_chain;
pub mod counterfactual;

