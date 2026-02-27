//! Harness sub-module — self-consistency testing for Boolean operations.
//!
//! DOMAIN: Test infrastructure — validates Boolean results against
//! topological invariants and point-in-solid consistency.
//! DEPENDENCIES: `forge-kernel` (execute_boolean), `forge-topo` (classify, validate)
//!
//! ## Contents
//!
//! - `boolean` — `FuzzOutcome`, `FuzzReport`, `run_single_case`, `run_fuzz_corpus`

mod boolean;

pub use boolean::{run_fuzz_corpus, run_single_case, FuzzOutcome, FuzzReport};
