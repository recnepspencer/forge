//! Boundary Representation (B-Rep) subsystem.
//!
//! DOMAIN: Entity data shapes, arena storage, and graph operations
//! for the halfedge mesh topology.
//!
//! COMPONENTS:
//! - `data/`: Entity schemas (mesh, containment), arena storage infrastructure
//! - `logic/`: CRUD operations, index maintenance, topological helpers
//!
//! DEPENDENCIES: `handles` (typed IDs), `attributes` (semantic tags)

pub(crate) mod data;
pub(crate) mod logic;

pub mod facade;
pub use facade::*;

#[cfg(test)]
mod tests;
