//! Tolerance computation — derives tolerances from geometry data.
//!
//! DOMAIN: Computes model-scale-based tolerances from vertex positions.
//! Provides a `GeometryToleranceProvider` adapter that implements
//! `ToleranceProvider` without coupling the data store to the trait.
//!
//! DEPENDENCIES: `forge-core` (ToleranceProvider)

use forge_core::ToleranceProvider;

use crate::geometry::contracts::GeometryView;

/// Model scale constant — minimum tolerance for planar vertices.
const PLANAR_VERTEX_TOLERANCE: f64 = 1e-7;

/// Compute model scale from the bounding box of all vertex positions.
///
/// Returns the diagonal length of the axis-aligned bounding box of all
/// visible vertex positions. Returns 0.0 if no positions exist.
///
/// Accepts `&impl GeometryView` — works on both `GeometryStore` and `GeometryDraft`.
pub fn compute_model_scale(view: &impl GeometryView) -> f64 {
    let mut min = [f64::MAX; 3];
    let mut max = [f64::MIN; 3];
    let mut any = false;

    for pos in view.vertex_positions_approx() {
        any = true;
        for i in 0..3 {
            if pos[i] < min[i] {
                min[i] = pos[i];
            }
            if pos[i] > max[i] {
                max[i] = pos[i];
            }
        }
    }

    if !any {
        return 0.0;
    }
    forge_geom::facade::distance(&min, &max)
}

/// Compute a sensible global default tolerance from model scale.
pub fn compute_global_tolerance(view: &impl GeometryView) -> f64 {
    let scale = compute_model_scale(view);
    PLANAR_VERTEX_TOLERANCE * scale.max(1.0)
}

/// Adapter: implements `ToleranceProvider` with a cached tolerance value.
///
/// The tolerance is computed ONCE at construction time from any `GeometryView`.
/// This avoids O(n) position iteration on every `global_default()` call.
///
/// ```ignore
/// let provider = GeometryToleranceProvider::new(&store);
/// let tol = provider.global_default();
/// ```
#[derive(Debug)]
pub struct GeometryToleranceProvider {
    cached_tolerance: f64,
}

impl GeometryToleranceProvider {
    /// Create an adapter, caching the tolerance computed from any geometry view.
    pub fn new(view: &impl GeometryView) -> Self {
        Self {
            cached_tolerance: compute_global_tolerance(view),
        }
    }
}

impl ToleranceProvider for GeometryToleranceProvider {
    fn vertex_tolerance(&self, _vertex_index: u32, _vertex_generation: u32) -> f64 {
        self.cached_tolerance
    }

    fn edge_tolerance(&self, _edge_index: u32, _edge_generation: u32) -> f64 {
        self.cached_tolerance
    }

    fn global_default(&self) -> f64 {
        self.cached_tolerance
    }
}
