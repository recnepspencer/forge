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

pub mod eval;
pub mod schema;

pub use eval::{query_causal_chain, query_causal_summary};
pub use schema::{CausalChain, CausalStep, ChainSummary};
