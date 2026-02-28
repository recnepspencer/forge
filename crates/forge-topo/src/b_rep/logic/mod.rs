//! Logic layer for the B-Rep subsystem.
//!
//! DOMAIN: CRUD operations, index maintenance, and topological helpers
//! that operate on the TopologyArena.

mod graph_ops;
mod topo_ops;

pub use graph_ops::membership_tracker::EntityBitset;
