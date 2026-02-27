//! Counterfactual replay — witness-based decision override and analysis.
//!
//! DOMAIN: Replays a specific decision with mutated inputs to test
//! "what would have happened if this decision went the other way?"
//!
//! DEPENDENCIES: `forge-core`, `forge-topo`
//!
//! PUBLIC API:
//! - `replay_decision()` — override one decision and evaluate
//! - `replay_all_near_boundary()` — override all NearBoundary decisions
//! - `CounterfactualResult` — original vs. counterfactual hashes + validation
//! - `DecisionOverride` — target decision + forced values
//! - `CounterfactualValidation` — Valid / TopologyBroken / DivergentButValid

mod eval;
mod schema;

pub use schema::{CounterfactualResult, CounterfactualValidation, DecisionOverride, EntityDelta};

pub use eval::{replay_all_near_boundary, replay_decision};
