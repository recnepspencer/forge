//! Consolidated integration suite for authority sealing and forgery resistance.

mod authority_sealing_fixture;

#[path = "authority_sealing_contracts/cargo.rs"]
mod cargo;
#[path = "authority_sealing_contracts/closure.rs"]
mod closure;
#[path = "authority_sealing_contracts/forgery.rs"]
mod forgery;
#[path = "authority_sealing_contracts/graph.rs"]
mod graph;
#[path = "authority_sealing_contracts/laundering.rs"]
mod laundering;
#[path = "authority_sealing_contracts/reachability_preservation.rs"]
mod reachability_preservation;
#[path = "authority_sealing_contracts/resolution.rs"]
mod resolution;
#[path = "authority_sealing_contracts/surface.rs"]
mod surface;
#[path = "authority_sealing_contracts/value_gate.rs"]
mod value_gate;
#[path = "authority_sealing_contracts/value_gate_forgery.rs"]
mod value_gate_forgery;
