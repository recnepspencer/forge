//! Aspect and versioning types for the reactive signal graph.

use serde::{Deserialize, Serialize};

/// Which aspect of a feature output a downstream node subscribes to.
///
/// This enables the topology change firewall: a geometry-only change
/// (e.g., dragging an extrude depth) won't trigger re-evaluation of
/// nodes that only subscribe to the topology aspect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Aspect {
    /// Subscribes to topology changes (connectivity, face count, etc.).
    Topology,
    /// Subscribes to geometry changes (positions, dimensions, etc.).
    Geometry,
}

/// Per-aspect version counters carried by each signal node.
///
/// When a feature evaluates, it reports new aspect versions.
/// Downstream nodes compare these against their cached versions
/// to determine if they actually need to recompute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectVersion {
    topology: u64,
    geometry: u64,
}

impl AspectVersion {
    /// Create a new aspect version with both counters at zero.
    pub fn zero() -> Self {
        Self {
            topology: 0,
            geometry: 0,
        }
    }

    /// Create a new aspect version with explicit values.
    pub fn new(topology: u64, geometry: u64) -> Self {
        Self { topology, geometry }
    }

    /// The topology version counter.
    pub fn topology(self) -> u64 {
        self.topology
    }

    /// The geometry version counter.
    pub fn geometry(self) -> u64 {
        self.geometry
    }

    /// Read the version for a specific aspect.
    pub fn get(self, aspect: Aspect) -> u64 {
        match aspect {
            Aspect::Topology => self.topology,
            Aspect::Geometry => self.geometry,
        }
    }

    /// Bump the topology version by one.
    pub fn bump_topology(self) -> Self {
        Self {
            topology: self.topology + 1,
            geometry: self.geometry,
        }
    }

    /// Bump the geometry version by one.
    pub fn bump_geometry(self) -> Self {
        Self {
            topology: self.topology,
            geometry: self.geometry + 1,
        }
    }

    /// Bump both versions by one.
    pub fn bump_all(self) -> Self {
        Self {
            topology: self.topology + 1,
            geometry: self.geometry + 1,
        }
    }
}
