//! # forge-repr
//!
//! Representation traits for the Forge geometry kernel.
//!
//! This crate defines the contracts for converting kernel geometry
//! into visual representations. The kernel and geometry solvers are
//! NEVER aware of these traits — they are consumed by UI and export
//! layers only.
//!
//! ## Domains
//!
//! - **schema** — `TriangleMesh` output container for tessellation
//! - **traits** — `Viewable` (SDF) and `Tessellatable` (mesh generation)

#![forbid(unsafe_code)]

mod schema;
mod traits;

#[cfg(test)]
mod tests;

pub use schema::TriangleMesh;
pub use traits::{Tessellatable, Viewable};
