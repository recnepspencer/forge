//! Transient vertex match key for cross-solid deduplication.
//!
//! DOMAIN: Exact rational position key used during boolean assembly to
//! identify vertices at the same physical location across solids.
//! This key is NOT the vertex's permanent identity — that remains
//! the `VertexId` + `Lineage`.
//!
//! ## Future: Euler Operator Formalization
//!
//! Currently, vertex dedup during assembly is a construction-time optimization
//! in `copy.rs::resolve_vertex` — it reuses an existing `VertexId` instead of
//! creating a duplicate. The merge decision is not recorded as a `TracedDecision`
//! and lineage is merged via a manual `Lineage::merge` call.
//!
//! This should eventually be formalized as a proper Euler operator
//! (`IdentifyVertices(V_a, V_b)`) that:
//! - Does NOT require a shared edge (unlike `KillEdgeVertex`)
//! - Records a `TracedDecision` for every merge
//! - Emits `LineageEvent::EntityMerged` automatically
//! - Is testable in isolation
//!
//! See `forge-topo::euler::kill_edge_vertex` for the closest existing operator.
//! The key difference: KEV requires an edge between A and B, whereas vertex
//! identity resolution happens *before* cross-solid edges exist.

use forge_math::arithmetic::Rational;

/// Transient geometric match key for cross-solid vertex deduplication.
///
/// Keyed by the exact rational `[x, y, z]` position of the vertex.
/// Because `intersect_three_planes_exact` always reduces its result
/// to canonical form (GCD-reduced numerator/denominator), two vertices
/// at the same physical point produce bitwise-identical `Rational` values
/// and therefore identical `VertexMatchKey`s — regardless of which solid
/// they came from, which PlaneTable they used, or how many planes meet
/// at that corner.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VertexMatchKey {
    /// Exact rational coordinates.
    pos: [Rational; 3],
}

impl VertexMatchKey {
    /// Create a match key from an exact rational vertex position.
    ///
    /// `BigRational` is always stored in canonical (GCD-reduced) form,
    /// so two keys are equal iff they represent the same point.
    pub fn from_exact_position(x: Rational, y: Rational, z: Rational) -> Self {
        Self { pos: [x, y, z] }
    }

    /// Access the exact rational coordinates.
    pub fn position(&self) -> &[Rational; 3] {
        &self.pos
    }
}
