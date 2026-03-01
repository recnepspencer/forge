//! # forge-kernel
//!
//! The application layer of the Forge geometry kernel.
//! Features, booleans, fillets, chamfers, and the modeling pipeline.
//!
//! ## Architecture
//!
//! ```text
//! forge-kernel/src/
//! ├── lib.rs         ← this file (Table of Contents only)
//! ├── prelude.rs     ← convenient imports
//! ├── configuration/ ← config schema + resolution + overrides
//! ├── observability/ ← tracing/span telemetry
//! ├── features/      ← feature-sliced domain modules (Bento Box)
//! ├── geometry_state/ ← data adapter
//! ├── mesh_builder/  ← construction service
//! ├── proof/          ← proof infrastructure (causal chain, counterfactual, region extractor)
//! ├── operations/    ← all modeling operations
//! │   └── boolean/   ← Boolean operations
//! └── brep/          ← future B-rep abstractions
//! ```
//!
//! See `CONVENTIONS.md` for coding rules and `docs/FILE_NAMING.md` for naming.
//!
//! ## Doctrine Enforcement
//!
//! - **D2 (Explicit Policy)**: Use [`check_tolerance!`] for ambiguous decisions
//! - **D6 (Atomic Transactionality)**: Operations return `Result<TopologyState, KernelError>`
//! - **D3 (Topology-Geometry Firewall)**: Accept only `CertifiedTriSign` for topology decisions

#![forbid(unsafe_code)]

pub mod proof;
pub mod brep;
pub mod configuration;
pub mod context;
pub mod engine;
pub mod finalization;
pub mod geom_facade;
pub mod geometry_state;
pub mod mesh_builder;
pub mod observability;
pub mod operations;
pub mod naming;
pub mod prelude;
pub mod primitives;
pub mod queries;
pub mod shared_ops;
pub mod spatial;
pub mod tolerance;

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(2 + 2, 4);
    }
}

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod architecture_guard_tests;
