//! Data definitions for the ImplicitVertex primitive.

use serde::{Deserialize, Serialize};

/// An implicit vertex defined by the intersection of 3 or more planes.
///
/// The vertex position is not stored — it is derived on demand by
/// solving the linear system of plane equations. This ensures the
/// vertex always agrees with its defining geometry.
///
/// # Overconstrained Vertices
///
/// When 4+ planes define a vertex (e.g., a pyramid apex), the solver
/// selects the best-conditioned triple and verifies against all others.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ImplicitVertex {
    /// Indices into the plane table. Must contain at least 3 entries.
    defining_planes: Vec<PlaneRef>,
}

/// Lightweight reference to a plane in a plane table.
///
/// This is a simple index — typed for clarity, not to be confused
/// with topology handles from `forge-topo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PlaneRef {
    /// Index into the plane table.
    index: usize,
}

impl PlaneRef {
    /// Create a new plane reference.
    pub fn new(index: usize) -> Self {
        Self { index }
    }

    /// The index into the plane table.
    pub fn index(self) -> usize {
        self.index
    }
}

impl ImplicitVertex {
    /// Create a new implicit vertex from 3+ plane references.
    ///
    /// Returns `None` if fewer than 3 planes are provided.
    pub fn try_new(planes: Vec<PlaneRef>) -> Option<Self> {
        if planes.len() < 3 {
            return None;
        }
        Some(Self {
            defining_planes: planes,
        })
    }

    /// The plane references defining this vertex.
    pub fn defining_planes(&self) -> &[PlaneRef] {
        &self.defining_planes
    }

    /// The number of defining planes.
    pub fn plane_count(&self) -> usize {
        self.defining_planes.len()
    }

    /// Whether this vertex is overconstrained (4+ planes).
    pub fn is_overconstrained(&self) -> bool {
        self.defining_planes.len() > 3
    }
}
