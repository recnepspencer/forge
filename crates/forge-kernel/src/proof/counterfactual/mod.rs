//! Counterfactual replay — generic decision override and analysis.
//!
//! DOMAIN: Replays any operation with forced classification overrides
//! to test "what would have happened if this decision went the other way?"
//!
//! Each operation provides its own `ReplayFn` closure (capturing its
//! input) and `ClassificationCodec` (teaching the engine how to parse
//! and flip its domain-specific classification labels).
//!
//! DEPENDENCIES: `forge-core`, `forge-topo`
//!
//! PUBLIC API:
//! - `replay_decision()` — override one decision and evaluate
//! - `replay_all_near_boundary()` — override all NearBoundary decisions
//! - `ClassificationCodec` — trait for parsing/flipping labels per operation
//! - `ReplayFn` — type alias for the operation replay closure
//! - `ReplayOutcome` — topology + decision log from a replay
//! - `CounterfactualResult` — original vs. counterfactual hashes + validation
//! - `DecisionOverride` — target decision + forced values
//! - `CounterfactualValidation` — Valid / TopologyBroken / DivergentButValid

mod eval;
mod schema;

pub use schema::{CounterfactualResult, CounterfactualValidation, DecisionOverride, EntityDelta};

pub use eval::{
    replay_all_near_boundary, replay_decision, ClassificationCodec, ReplayFn, ReplayOutcome,
};
