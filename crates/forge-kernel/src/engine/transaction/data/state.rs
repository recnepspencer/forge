//! Owned topology + geometry state bundle.
//!
//! DOMAIN: The resting-state representation of a kernel solid. Bundles
//! topology and unified geometry into a single ownership unit.
//!
//! INVARIANTS: Owns only derived model state (`TopologyState`, `GeometryStore`).
//! No tracing, no lineage duplication, no cached hashes.

use forge_topo::transactions::TopologyState;

use crate::geometry::facade::GeometryStore;

/// Owned topology + geometry state bundle.
///
/// The single unit of ownership for kernel phases that operate on an
/// assembled solid. NOT for pre-assembly phases (split, classify, select)
/// which operate on two separate solids.
///
/// INVARIANT: Owns only derived model state (`TopologyState`, `GeometryStore`).
/// No tracing, no lineage duplication, no cached hashes.
#[derive(Debug, Clone)]
pub struct KernelState {
    topo: TopologyState,
    geom: GeometryStore,
}

impl KernelState {
    /// Create a new `KernelState` from its parts.
    pub fn new(topo: TopologyState, geom: GeometryStore) -> Self {
        Self { topo, geom }
    }

    /// Read-only access to the topology state.
    pub fn topology(&self) -> &TopologyState {
        &self.topo
    }

    /// Read-only access to the geometry store.
    pub fn geometry(&self) -> &GeometryStore {
        &self.geom
    }

    /// Mutable access to the geometry store.
    pub fn geometry_mut(&mut self) -> &mut GeometryStore {
        &mut self.geom
    }

    /// Consume the state into its constituent parts.
    pub fn into_parts(self) -> (TopologyState, GeometryStore) {
        (self.topo, self.geom)
    }

    /// Borrow parts simultaneously.
    pub fn as_parts(&self) -> (&TopologyState, &GeometryStore) {
        (&self.topo, &self.geom)
    }
}
