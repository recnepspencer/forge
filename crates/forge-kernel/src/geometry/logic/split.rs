//! SplitEdge geometry propagation — subdivide curves when edges are split.
//!
//! DOMAIN: When the topology-layer `SplitEdge` operator splits an edge E
//! into E_old and E_new, and E had an attached 3D curve, the kernel must
//! subdivide the curve into two segments and assign them to the resulting edges.
//!
//! DEPENDENCIES: `geometry` (GeometryStore), `worth-geom` (CurveGeom)

use std::sync::Arc;

use forge_core::KernelError;
use worth_geom::facade::{CurveGeom, CurveKind, CurveProvenance, SpCurveApproximation};
use forge_topo::handles::EdgeId;

use super::super::data::store::GeometryStore;

/// Propagate curve geometry after a `SplitEdge` operation.
///
/// If the original edge (`old_edge`) had a curve in the geometry store,
/// subdivide the curve at parameter `t` into two segments:
/// - `[domain_start, t]` → assigned to `old_edge`
/// - `[t, domain_end]` → assigned to `new_edge`
///
/// If the old edge had no curve (planar case), this is a no-op.
pub fn propagate_curve_on_split(
    old_edge: EdgeId,
    new_edge: EdgeId,
    parameter: f64,
    store: &mut GeometryStore,
) -> Result<(), KernelError> {
    let original_curve = match store.curves.get(old_edge) {
        Some(arc) => (**arc).clone(),
        None => return Ok(()),
    };

    let (segment_a, segment_b) = subdivide_curve(&original_curve, parameter);

    store.curves.set(old_edge, Arc::new(segment_a));
    store.curves.set(new_edge, Arc::new(segment_b));

    Ok(())
}

/// Subdivide a CurveGeom at parameter `t`, producing two segments.
fn subdivide_curve(curve: &CurveGeom, t: f64) -> (CurveGeom, CurveGeom) {
    let (kind_a, kind_b) = subdivide_curve_kind(&curve.kind, t);

    let provenance_a = CurveProvenance::SplitInherited {
        parent_edge_index: 0,
        parameter_range: (0.0, t),
    };
    let provenance_b = CurveProvenance::SplitInherited {
        parent_edge_index: 0,
        parameter_range: (t, 1.0),
    };

    let seg_a = CurveGeom {
        kind: kind_a,
        tolerance: curve.tolerance,
        provenance: provenance_a,
    };
    let seg_b = CurveGeom {
        kind: kind_b,
        tolerance: curve.tolerance,
        provenance: provenance_b,
    };

    (seg_a, seg_b)
}

/// Subdivide a CurveKind at parameter `t`.
fn subdivide_curve_kind(kind: &CurveKind, t: f64) -> (CurveKind, CurveKind) {
    match kind {
        CurveKind::Line { origin, direction } => {
            let mid = [
                origin[0] + t * direction[0],
                origin[1] + t * direction[1],
                origin[2] + t * direction[2],
            ];
            (
                CurveKind::Line {
                    origin: *origin,
                    direction: *direction,
                },
                CurveKind::Line {
                    origin: mid,
                    direction: *direction,
                },
            )
        }
        CurveKind::Circle {
            center,
            normal,
            radius,
        } => (
            CurveKind::Circle {
                center: *center,
                normal: *normal,
                radius: *radius,
            },
            CurveKind::Circle {
                center: *center,
                normal: *normal,
                radius: *radius,
            },
        ),
        CurveKind::Ellipse {
            center,
            major,
            minor,
        } => (
            CurveKind::Ellipse {
                center: *center,
                major: *major,
                minor: *minor,
            },
            CurveKind::Ellipse {
                center: *center,
                major: *major,
                minor: *minor,
            },
        ),
        CurveKind::SurfaceIntersection {
            surface_a,
            surface_b,
            sp_curve_cache,
        } => {
            let (cache_a, cache_b) = subdivide_sp_curve(sp_curve_cache, t);
            (
                CurveKind::SurfaceIntersection {
                    surface_a: *surface_a,
                    surface_b: *surface_b,
                    sp_curve_cache: cache_a,
                },
                CurveKind::SurfaceIntersection {
                    surface_a: *surface_a,
                    surface_b: *surface_b,
                    sp_curve_cache: cache_b,
                },
            )
        }
    }
}

/// Subdivide a SpCurveApproximation at parameter `t`.
fn subdivide_sp_curve(
    cache: &SpCurveApproximation,
    t: f64,
) -> (SpCurveApproximation, SpCurveApproximation) {
    let t_global = cache.domain.0 + t * (cache.domain.1 - cache.domain.0);

    let n = cache.control_points.len();
    if n <= 1 {
        return (cache.clone(), cache.clone());
    }

    let split_frac = t.clamp(0.0, 1.0);
    let split_idx = ((n - 1) as f64 * split_frac).round() as usize;
    let split_idx = split_idx.clamp(1, n - 1);

    let pts_a: Vec<[f64; 3]> = cache.control_points[..=split_idx].to_vec();
    let pts_b: Vec<[f64; 3]> = cache.control_points[split_idx..].to_vec();

    let knots_a: Vec<f64> = cache
        .knots
        .iter()
        .filter(|&&k| k <= t_global)
        .cloned()
        .collect();
    let knots_b: Vec<f64> = cache
        .knots
        .iter()
        .filter(|&&k| k >= t_global)
        .cloned()
        .collect();

    (
        SpCurveApproximation {
            control_points: pts_a,
            knots: if knots_a.is_empty() {
                vec![cache.domain.0, t_global]
            } else {
                knots_a
            },
            error_bound: cache.error_bound,
            domain: (cache.domain.0, t_global),
        },
        SpCurveApproximation {
            control_points: pts_b,
            knots: if knots_b.is_empty() {
                vec![t_global, cache.domain.1]
            } else {
                knots_b
            },
            error_bound: cache.error_bound,
            domain: (t_global, cache.domain.1),
        },
    )
}
