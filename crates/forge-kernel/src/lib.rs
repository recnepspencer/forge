//! # forge-kernel
//!
//! The application layer of the Forge geometry kernel.
//! Features, booleans, fillets, chamfers, and the modeling pipeline.
//!
//! ## Architecture
//!
//! ```text
//! forge-kernel/src/
//! ├── lib.rs           ← this file (Table of Contents only)
//! ├── prelude.rs       ← convenient imports
//! ├── configuration/   ← config schema + resolution + overrides
//! ├── engine/          ← feature tree, pipeline, contracts
//! ├── registry/        ← feature catalog, command dispatch, handlers
//! ├── operations/      ← modeling operations (boolean, fillet, etc.)
//! ├── primitives/      ← parameterized shape generation
//! ├── geometry/        ← unified geometry store (PropertyLayer pattern)
//! ├── geometry_state/  ← [DEPRECATED] old side-car geometry storage
//! ├── mesh_builder/    ← BSP → halfedge construction
//! ├── proof/           ← proof infrastructure
//! ├── brep/            ← [DEPRECATED] old B-rep state
//! └── observability/   ← tracing/span telemetry
//! ```
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
pub mod diff;
pub mod engine;
pub mod registry;
pub mod geometry;
pub mod geometry_state;
pub mod mesh_builder;
pub mod observability;
pub mod operations;
pub mod prelude;
pub mod primitives;

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
