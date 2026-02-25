use forge_topo::state::TopologyState;

use crate::geometry_state::GeometryState;

/// Owned topology + geometry state bundle.
///
/// The single unit of ownership for kernel phases that operate on an
/// assembled solid. NOT for pre-assembly phases (split, classify, select)
/// which operate on two separate solids.
///
/// INVARIANT: Owns only derived model state (`TopologyState`, `GeometryState`).
/// No tracing, no lineage duplication, no cached hashes.
#[derive(Debug, Clone)]
pub struct KernelState {
    topo: TopologyState,
    geom: GeometryState,
}

impl KernelState {
    /// Create a new `KernelState` from its parts.
    pub fn new(topo: TopologyState, geom: GeometryState) -> Self {
        Self { topo, geom }
    }

    /// Read-only access to the topology state.
    pub fn topology(&self) -> &TopologyState {
        &self.topo
    }

    /// Read-only access to the geometry store.
    pub fn geometry(&self) -> &GeometryState {
        &self.geom
    }

    /// Mutable access to the geometry store.
    pub fn geometry_mut(&mut self) -> &mut GeometryState {
        &mut self.geom
    }

    /// Consume the state into its constituent parts.
    pub fn into_parts(self) -> (TopologyState, GeometryState) {
        (self.topo, self.geom)
    }

    /// Borrow both parts simultaneously.
    pub fn as_parts(&self) -> (&TopologyState, &GeometryState) {
        (&self.topo, &self.geom)
    }
}
