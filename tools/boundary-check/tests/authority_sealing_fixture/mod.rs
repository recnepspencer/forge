//! Aggregation-only fixture module for authority sealing production-binary tests.
//! Shared by sealing-surface and law-substrate contract binaries.

#![allow(dead_code)]
#![allow(unused_imports)]

mod facade_projection;
mod repository;
mod repository_cargo;
mod repository_config;
mod repository_execution;
mod repository_public_values;
mod source_cases;
mod source_cases_bypass;
mod source_cases_cargo;
mod source_cases_closure;
mod source_cases_graph;
mod source_cases_launder;
mod source_cases_resolve;
mod source_cases_value_gate;

pub use repository::AuthoritySealingTestRepository;
pub use source_cases::*;
pub use source_cases_bypass::*;
pub use source_cases_cargo::*;
pub use source_cases_closure::*;
pub use source_cases_graph::*;
pub use source_cases_launder::*;
pub use source_cases_resolve::*;
pub use source_cases_value_gate::*;
