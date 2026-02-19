//! Causal decision chain reconstruction — module manifest.
//!
//! DOMAIN: Given a topological entity in a result, reconstructs the
//! complete chain of operations and decisions that produced it.
//!
//! DEPENDENCIES:
//! - `forge-core` (TracedDecision, EntityRef, DecisionLog)
//! - `forge-topo` (OpSignature, LineageEvent, ReplayLog)
//!
//! EXPORTS:
//! - `CausalChain`, `CausalStep`, `ChainSummary` — data shapes
//! - `query_causal_chain`, `query_causal_summary` — agent API

pub mod schema;
pub mod eval;

pub use schema::{CausalChain, CausalStep, ChainSummary};
pub use eval::{query_causal_chain, query_causal_summary};
