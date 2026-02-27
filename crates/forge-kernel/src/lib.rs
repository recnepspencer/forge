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
//! ├── core/          ← shared infrastructure (ModelingContext, policies)
//! ├── features/      ← feature-sliced domain modules (Bento Box)
//! ├── geometry_state/ ← data adapter
//! ├── mesh_builder/  ← construction service
//! ├── analysis/      ← queries & diagnostics
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

pub mod analysis;
pub mod brep;
pub mod core;
pub mod engine;
pub mod geom_facade;
pub mod geometry_state;
pub mod mesh_builder;
pub mod operations;
pub mod prelude;
pub mod primitives;
pub mod queries;
pub mod shared_ops;
pub mod shared_steps;
pub mod spatial;

// Re-exports for backward compatibility
pub use operations::boolean;
pub use operations::boolean::{execute_boolean, BooleanInput, BooleanOp, BooleanResult};

// Phase F — gap measurement
pub use analysis::gap::{measure_gap, GapReport, GapSampleDensity};

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(2 + 2, 4);
    }
}
