//! Tolerance query interface — the Provider pattern for geometric uncertainty.
//!
//! DOMAIN: Defines the `ToleranceProvider` trait that lets topology-layer
//! functions (classification, validation) query per-entity tolerance bounds
//! without the topology layer owning any `f64` geometry values (Doctrine D3).
//!
//! DEPENDENCIES: `forge_topo` handles (via re-export in forge-core is not
//! possible due to layering — trait uses raw u32 index + generation pairs
//! equivalent to the handles, passed in by the kernel which owns both layers).
//!
//! # Design rationale
//!
//! Tolerance is a geometric property, not a topological one. `forge-topo`
//! stores only connectivity. But topology-layer functions like
//! `classify_point_on_face` and `validate_zero_length_edges` need to know
//! "how close is close enough?" for each specific entity.
//!
//! The Provider trait bridges this: `forge-topo` functions accept
//! `&dyn ToleranceProvider`, and `forge-kernel`'s `ModelingContext` (which
//! owns both topology and geometry) implements it by querying `GeometryStore`.
//!
//! # Safety and fallback
//!
//! `vertex_tolerance` and `edge_tolerance` return `f64` directly. When an
//! entity has not yet had geometry bound to it (e.g., a vertex created by a
//! Euler op but not yet decorated by the kernel), implementations MUST return
//! `global_default()` rather than panicking or returning `0.0`.
//! Returning `0.0` would cause every edge to appear degenerate.

/// Query interface for per-entity tolerance bounds.
///
/// Implemented by `forge-kernel`'s `ModelingContext` (Phase 2+) over the
/// existing `GeometryStore`. `forge-topo` functions accept `&dyn ToleranceProvider`
/// instead of a flat `tolerance: f64` so each entity is measured against its
/// own uncertainty sphere / tube rather than a single global threshold.
///
/// All implementations MUST satisfy the safety contract on the `global_default`
/// method — specifically, never return `0.0` or negative values.
pub trait ToleranceProvider: std::fmt::Debug {
    /// The certified uncertainty sphere radius for a vertex.
    ///
    /// For planar vertices (Phase 1–2) this is typically a small global default
    /// since planar positions are computed exactly. For curved vertices (Phase 4+)
    /// this is derived from the Surface-Surface Intersection solver's
    /// `PrecisionCertificate.error_bound`.
    ///
    /// Returns `global_default()` when no geometry has been bound to `vertex`
    /// (i.e., the entity was just created by a Euler op awaiting decoration).
    ///
    /// # Arguments
    /// `vertex_index` and `vertex_generation` are the raw fields of the `VertexId`
    /// handle. Using raw u32 pairs avoids a dependency on `forge-topo` handles
    /// from this crate.
    fn vertex_tolerance(&self, vertex_index: u32, vertex_generation: u32) -> f64;

    /// The certified uncertainty tube radius for an edge.
    ///
    /// For planar edges (Phase 1–2) this is always `global_default()` since
    /// planar edges are implicit plane-plane intersections with no numeric error.
    /// For curved edges (Phase 4+) this is stored in `forge-geom::CurveGeom`.
    ///
    /// Returns `global_default()` when no geometry has been bound to `edge`.
    fn edge_tolerance(&self, edge_index: u32, edge_generation: u32) -> f64;

    /// Safe conservative fallback tolerance.
    ///
    /// Used when an entity is not yet bound to geometry. Must be strictly
    /// positive and ≥ the kernel's configured minimum tolerance. Never `0.0`.
    ///
    /// Implementations derive this from `ToleranceConfig::global_default`.
    fn global_default(&self) -> f64;

    /// Tolerance for geometric identity checks (position matching,
    /// normal degeneracy, point-on-surface deviation).
    ///
    /// Defaults to `global_default()`. Override when geometric comparisons
    /// need a tighter tolerance than per-entity metric thresholds (edge
    /// length, face area).
    fn geometry_epsilon(&self) -> f64 {
        self.global_default()
    }
}

/// A flat, constant-tolerance provider for tests and the planar Phase 1–2 fast path.
///
/// Returns the same value for every entity. Safe for use when no per-entity
/// geometry is available — which is the correct model for exact planar geometry
/// where every planar vertex is an exact 3-plane intersection with zero error.
#[derive(Debug, Clone, Copy)]
pub struct FlatToleranceProvider {
    /// The single tolerance value returned for all entities.
    tolerance: f64,
}

impl FlatToleranceProvider {
    /// Create a flat provider with the given tolerance.
    ///
    /// # Panics
    /// Panics if `tolerance` is not strictly positive.
    pub fn new(tolerance: f64) -> Self {
        assert!(
            tolerance > 0.0,
            "FlatToleranceProvider tolerance must be > 0.0, got {}",
            tolerance
        );
        Self { tolerance }
    }
}

impl ToleranceProvider for FlatToleranceProvider {
    fn vertex_tolerance(&self, _vertex_index: u32, _vertex_generation: u32) -> f64 {
        self.tolerance
    }

    fn edge_tolerance(&self, _edge_index: u32, _edge_generation: u32) -> f64 {
        self.tolerance
    }

    fn global_default(&self) -> f64 {
        self.tolerance
    }

    fn geometry_epsilon(&self) -> f64 {
        self.tolerance
    }
}

// ── Comparison predicates ───────────────────────────────────────────────
//
// These are the ONLY sanctioned path for geometric comparisons.
// Direct `<`, `>`, `==` against float values with literal tolerances is
// banned (clippy::float_cmp + CI lint). Use these instead.

/// Two 3D points are the same location within `geometry_epsilon`.
///
/// Uses squared-distance to avoid sqrt.
#[inline]
pub fn positions_coincident(
    a: &[f64; 3],
    b: &[f64; 3],
    tol: &dyn ToleranceProvider,
) -> bool {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    let eps = tol.geometry_epsilon();
    dx * dx + dy * dy + dz * dz < eps * eps
}

/// A scalar magnitude is effectively zero (below `geometry_epsilon`).
#[inline]
pub fn is_effectively_zero(value: f64, tol: &dyn ToleranceProvider) -> bool {
    value.abs() < tol.geometry_epsilon()
}

/// A squared magnitude is degenerate (below `geometry_epsilon²`).
///
/// Use for checking if a normal vector or cross product is too small,
/// without taking a sqrt.
#[inline]
pub fn is_degenerate_magnitude_sq(mag_sq: f64, tol: &dyn ToleranceProvider) -> bool {
    let eps = tol.geometry_epsilon();
    mag_sq < eps * eps
}

/// Two scalars are approximately equal within `geometry_epsilon`.
#[inline]
pub fn approximately_equal(a: f64, b: f64, tol: &dyn ToleranceProvider) -> bool {
    (a - b).abs() < tol.geometry_epsilon()
}
