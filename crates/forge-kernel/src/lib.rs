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
//! ├── core/          ← shared infrastructure (ModelingContext, policies)
//! └── features/      ← feature-sliced domain modules (Bento Box)
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

pub mod core;
pub mod features;
pub mod geometry_store;
pub mod mesh_builder;
pub mod boolean;
pub mod analysis;

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(2 + 2, 4);
    }
}
