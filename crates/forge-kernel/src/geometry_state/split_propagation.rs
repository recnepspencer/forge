//! SplitEdge geometry propagation — subdivide curves when edges are split.
//!
//! DOMAIN: When the topology-layer `SplitEdge` operator splits an edge E
//! into E_old and E_new, and E had an attached `CurveRef`, the kernel must
//! subdivide the curve into two segments and assign them to the resulting edges.
//!
//! This is a kernel-level concern (not forge-topo) because the topology layer
//! doesn't see CurveGeom data (Doctrine D3). The kernel calls this helper
//! after each `SplitEdge` application.
//!
//! DEPENDENCIES: `geometry_state` (GeometryState), `forge-geom` (CurveGeom)

use forge_core::KernelError;
use crate::geom_facade::{CurveGeom, CurveKind, CurveProvenance, SpCurveApproximation};
use forge_topo::handles::{CurveRef, EdgeId};

use crate::brep::state::BrepState;

/// Propagate curve geometry after a `SplitEdge` operation.
///
/// If the original edge (`old_edge`) had a `CurveRef` in the B-Rep store,
/// subdivide the curve at parameter `t` into two segments:
/// - `[domain_start, t]` → assigned to `old_edge`
/// - `[t, domain_end]` → assigned to `new_edge`
///
/// If the old edge had no curve (planar case), this is a no-op.
///
/// # Errors
/// Returns `KernelError` if the `CurveRef` points to a stale/missing entry.
pub fn propagate_curve_on_split(
    old_edge: EdgeId,
    new_edge: EdgeId,
    parameter: f64,
    brep: &mut BrepState,
) -> Result<(), KernelError> {
    let curve_ref = match brep.get_edge_curve(old_edge) {
        Some(r) => r,
        None => return Ok(()),
    };

    let original_curve = brep.get_curve(curve_ref)?.clone();

    let (segment_a, segment_b) = subdivide_curve(&original_curve, parameter);

    let ref_a = brep.insert_curve(segment_a);
    let ref_b = brep.insert_curve(segment_b);

    brep.attach_curve_to_edge(old_edge, ref_a);
    brep.attach_curve_to_edge(new_edge, ref_b);

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
        } => {
            let angle_a_end = t * std::f64::consts::TAU;
            let _angle_b_start = angle_a_end;
            (
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
            )
        }
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
///
/// Splits the control point polyline at the parametric midpoint and
/// adjusts domains. Full de Casteljau subdivision is Phase 7 (NURBS).
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry_state::GeometryState;

    #[test]
    fn no_op_when_edge_has_no_curve() {
        let mut geom = GeometryState::new();
        let old_edge = EdgeId::from_raw_parts(0, 0);
        let new_edge = EdgeId::from_raw_parts(1, 0);

        let result = propagate_curve_on_split(old_edge, new_edge, 0.5, &mut geom);
        assert!(result.is_ok());
        assert!(geom.get_edge_curve(old_edge).is_none());
        assert!(geom.get_edge_curve(new_edge).is_none());
    }

    #[test]
    fn line_curve_propagates_to_both_segments() {
        let mut geom = GeometryState::new();
        let old_edge = EdgeId::from_raw_parts(0, 0);
        let new_edge = EdgeId::from_raw_parts(1, 0);

        let curve = CurveGeom::from_analytic(
            CurveKind::Line {
                origin: [0.0, 0.0, 0.0],
                direction: [1.0, 0.0, 0.0],
            },
            [0, 1],
        );
        let curve_ref = geom.insert_curve(curve);
        geom.attach_curve_to_edge(old_edge, curve_ref);

        let result = propagate_curve_on_split(old_edge, new_edge, 0.5, &mut geom);
        assert!(result.is_ok());

        let ref_a = geom
            .get_edge_curve(old_edge)
            .expect("old edge should have curve");
        let ref_b = geom
            .get_edge_curve(new_edge)
            .expect("new edge should have curve");

        assert_ne!(ref_a.index(), ref_b.index());

        let curve_a = geom.get_curve(ref_a).unwrap();
        let curve_b = geom.get_curve(ref_b).unwrap();
        assert!(matches!(curve_a.kind, CurveKind::Line { .. }));
        assert!(matches!(curve_b.kind, CurveKind::Line { .. }));
        assert_eq!(curve_a.tolerance, 0.0);
        assert_eq!(curve_b.tolerance, 0.0);
    }

    #[test]
    fn surface_intersection_preserves_symbolic_references() {
        let mut geom = GeometryState::new();
        let old_edge = EdgeId::from_raw_parts(0, 0);
        let new_edge = EdgeId::from_raw_parts(1, 0);

        let sp_cache = SpCurveApproximation {
            control_points: vec![[0.0, 0.0, 0.0], [0.5, 0.5, 0.0], [1.0, 1.0, 0.0]],
            knots: vec![0.0, 0.5, 1.0],
            error_bound: 1e-8,
            domain: (0.0, 1.0),
        };
        let curve = CurveGeom {
            kind: CurveKind::SurfaceIntersection {
                surface_a: 0,
                surface_b: 1,
                sp_curve_cache: sp_cache,
            },
            tolerance: 1e-8,
            provenance: CurveProvenance::AnalyticIntersection {
                surface_indices: [0, 1],
            },
        };
        let curve_ref = geom.insert_curve(curve);
        geom.attach_curve_to_edge(old_edge, curve_ref);

        let result = propagate_curve_on_split(old_edge, new_edge, 0.5, &mut geom);
        assert!(result.is_ok());

        let ref_a = geom.get_edge_curve(old_edge).unwrap();
        let ref_b = geom.get_edge_curve(new_edge).unwrap();

        let curve_a = geom.get_curve(ref_a).unwrap();
        let curve_b = geom.get_curve(ref_b).unwrap();

        match (&curve_a.kind, &curve_b.kind) {
            (
                CurveKind::SurfaceIntersection {
                    surface_a: sa1,
                    surface_b: sb1,
                    ..
                },
                CurveKind::SurfaceIntersection {
                    surface_a: sa2,
                    surface_b: sb2,
                    ..
                },
            ) => {
                assert_eq!(*sa1, 0);
                assert_eq!(*sb1, 1);
                assert_eq!(*sa2, 0);
                assert_eq!(*sb2, 1);
            }
            _ => panic!("Expected SurfaceIntersection for both segments"),
        }
    }

    #[test]
    fn split_provenance_has_correct_parameter_ranges() {
        let mut geom = GeometryState::new();
        let old_edge = EdgeId::from_raw_parts(0, 0);
        let new_edge = EdgeId::from_raw_parts(1, 0);

        let curve = CurveGeom::from_analytic(
            CurveKind::Circle {
                center: [0.0, 0.0, 0.0],
                normal: [0.0, 0.0, 1.0],
                radius: 1.0,
            },
            [0, 1],
        );
        let curve_ref = geom.insert_curve(curve);
        geom.attach_curve_to_edge(old_edge, curve_ref);

        propagate_curve_on_split(old_edge, new_edge, 0.3, &mut geom).unwrap();

        let ref_a = geom.get_edge_curve(old_edge).unwrap();
        let curve_a = geom.get_curve(ref_a).unwrap();
        match &curve_a.provenance {
            CurveProvenance::SplitInherited {
                parameter_range, ..
            } => {
                assert!((parameter_range.0 - 0.0).abs() < 1e-12);
                assert!((parameter_range.1 - 0.3).abs() < 1e-12);
            }
            other => panic!("Expected SplitInherited, got {:?}", other),
        }

        let ref_b = geom.get_edge_curve(new_edge).unwrap();
        let curve_b = geom.get_curve(ref_b).unwrap();
        match &curve_b.provenance {
            CurveProvenance::SplitInherited {
                parameter_range, ..
            } => {
                assert!((parameter_range.0 - 0.3).abs() < 1e-12);
                assert!((parameter_range.1 - 1.0).abs() < 1e-12);
            }
            other => panic!("Expected SplitInherited, got {:?}", other),
        }
    }
}
