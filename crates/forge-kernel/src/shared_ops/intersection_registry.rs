//! Canonical intersection position registry for multi-solid operations.
//!
//! DOMAIN: Ensures the same geometric intersection (keyed by its symbolic
//! plane triple) always resolves to the same f64 position. Prevents
//! floating-point divergence between two solids being split by the same
//! set of planes.
//!
//! CONSUMERS: Boolean parametric split (primary), future NURBS surface/surface
//! intersection, any solid-solid operation that computes shared vertices.
//!
//! INVARIANT: `canonical_position` is the sole write path — once a key is
//! registered its position is immutable.

use std::collections::BTreeMap;
use crate::shared_ops::vertex::identity::VertexMatchKey;

/// Canonical position store for multi-solid intersection points.
///
/// Keyed by `VertexMatchKey` (a sorted triple of plane indices).
/// The first call for a given key stores the provided position;
/// subsequent calls return the previously stored position unchanged —
/// guaranteeing zero floating-point divergence between operands.
pub struct IntersectionRegistry {
    positions: BTreeMap<VertexMatchKey, [f64; 3]>,
}

impl IntersectionRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            positions: BTreeMap::new(),
        }
    }

    /// Register a position for a symbolic intersection key.
    ///
    /// If the key already exists, returns the previously stored (canonical)
    /// position instead of the provided one. This guarantees that the same
    /// geometric intersection always resolves to the same coordinates.
    pub fn canonical_position(&mut self, key: &VertexMatchKey, computed: [f64; 3]) -> [f64; 3] {
        *self.positions.entry(key.clone()).or_insert(computed)
    }

    /// Retrieve the canonical position for a provenance key.
    ///
    /// Returns `None` if this key has not been registered yet.
    pub fn get_position(&self, key: &VertexMatchKey) -> Option<&[f64; 3]> {
        self.positions.get(key)
    }
}

impl Default for IntersectionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
