use crate::support::{PolicyKind, PolicyQuery, PolicyResult};
use crate::surface::schema::SurfaceRelation;

use super::geometry::{frame_max_delta, norm};

fn ambiguous(relation: SurfaceRelation, margin: f64) -> PolicyResult<SurfaceRelation> {
    PolicyResult::Ambiguous {
        query: PolicyQuery {
            kind: PolicyKind::CoincidentGeometry,
            location: [0.0, 0.0, 0.0],
            margin,
            overridable: true,
        },
        potential_value: relation,
    }
}

/// Classify two planes: coincident (same or antiparallel normals, same offset),
/// disjoint (parallel but different offset), or general (intersecting).
pub(super) fn classify_plane_plane(
    n1: &[f64; 3],
    d1: f64,
    n2: &[f64; 3],
    d2: f64,
    tol: f64,
    af: f64,
) -> PolicyResult<SurfaceRelation> {
    let dot = n1[0] * n2[0] + n1[1] * n2[1] + n1[2] * n2[2];
    let angle_deviation = (dot.abs() - 1.0).abs();

    if angle_deviation < tol {
        let effective_d2 = if dot > 0.0 { d2 } else { -d2 };
        let offset_diff = (d1 - effective_d2).abs();

        if offset_diff < tol {
            PolicyResult::Success(SurfaceRelation::Coincident)
        } else if offset_diff < tol * af {
            ambiguous(SurfaceRelation::Coincident, offset_diff)
        } else {
            PolicyResult::Success(SurfaceRelation::Disjoint)
        }
    } else if angle_deviation < tol * af {
        ambiguous(SurfaceRelation::General, angle_deviation)
    } else {
        PolicyResult::Success(SurfaceRelation::General)
    }
}

/// Classify two spheres.
pub(super) fn classify_sphere_sphere(
    c1: &[f64; 3],
    r1: f64,
    c2: &[f64; 3],
    r2: f64,
    tol: f64,
    af: f64,
) -> PolicyResult<SurfaceRelation> {
    let dx = c1[0] - c2[0];
    let dy = c1[1] - c2[1];
    let dz = c1[2] - c2[2];
    let dist_sq = dx * dx + dy * dy + dz * dz;
    let radius_diff = (r1 - r2).abs();

    if dist_sq < tol * tol && radius_diff < tol {
        PolicyResult::Success(SurfaceRelation::Coincident)
    } else if dist_sq < (tol * af).powi(2) && radius_diff < tol * af {
        ambiguous(SurfaceRelation::Coincident, dist_sq.sqrt().max(radius_diff))
    } else {
        let dist = dist_sq.sqrt();
        if dist > r1 + r2 + tol * af {
            PolicyResult::Success(SurfaceRelation::Disjoint)
        } else if dist > r1 + r2 + tol {
            ambiguous(SurfaceRelation::Disjoint, dist - (r1 + r2))
        } else if dist + r1.min(r2) + tol * af < r1.max(r2) {
            PolicyResult::Success(SurfaceRelation::Disjoint)
        } else if dist + r1.min(r2) + tol < r1.max(r2) {
            let gap = r1.max(r2) - dist - r1.min(r2);
            ambiguous(SurfaceRelation::Disjoint, gap)
        } else {
            PolicyResult::Success(SurfaceRelation::General)
        }
    }
}

/// Classify two cylinders: coincident if coaxial with same radius.
pub(super) fn classify_cylinder_cylinder(
    o1: &[f64; 3],
    a1: &[f64; 3],
    r1: f64,
    o2: &[f64; 3],
    a2: &[f64; 3],
    r2: f64,
    tol: f64,
    af: f64,
) -> PolicyResult<SurfaceRelation> {
    let dot = a1[0] * a2[0] + a1[1] * a2[1] + a1[2] * a2[2];
    let angle_dev = (dot.abs() - 1.0).abs();

    if angle_dev > tol * af {
        return PolicyResult::Success(SurfaceRelation::General);
    }
    if angle_dev > tol {
        return ambiguous(SurfaceRelation::General, angle_dev);
    }

    let radius_diff = (r1 - r2).abs();
    if radius_diff > tol * af {
        return PolicyResult::Success(SurfaceRelation::General);
    }
    if radius_diff > tol {
        return ambiguous(SurfaceRelation::General, radius_diff);
    }

    let d = [o2[0] - o1[0], o2[1] - o1[1], o2[2] - o1[2]];
    let proj = d[0] * a1[0] + d[1] * a1[1] + d[2] * a1[2];
    let perp_sq = (d[0] - proj * a1[0]).powi(2)
        + (d[1] - proj * a1[1]).powi(2)
        + (d[2] - proj * a1[2]).powi(2);

    if perp_sq < tol * tol {
        PolicyResult::Success(SurfaceRelation::Coincident)
    } else if perp_sq < (tol * af).powi(2) {
        ambiguous(SurfaceRelation::Coincident, perp_sq.sqrt())
    } else {
        PolicyResult::Success(SurfaceRelation::General)
    }
}

/// Classify two cones: coincident if same apex, axis, half_angle.
pub(super) fn classify_cone_cone(
    a1: &[f64; 3],
    ax1: &[f64; 3],
    ha1: f64,
    a2: &[f64; 3],
    ax2: &[f64; 3],
    ha2: f64,
    tol: f64,
    af: f64,
) -> PolicyResult<SurfaceRelation> {
    let dot = ax1[0] * ax2[0] + ax1[1] * ax2[1] + ax1[2] * ax2[2];
    let angle_dev = (dot.abs() - 1.0).abs();

    if angle_dev > tol * af {
        return PolicyResult::Success(SurfaceRelation::General);
    }
    if angle_dev > tol {
        return ambiguous(SurfaceRelation::General, angle_dev);
    }

    let ha_diff = (ha1 - ha2).abs();
    if ha_diff > tol * af {
        return PolicyResult::Success(SurfaceRelation::General);
    }
    if ha_diff > tol {
        return ambiguous(SurfaceRelation::General, ha_diff);
    }

    let d = [a2[0] - a1[0], a2[1] - a1[1], a2[2] - a1[2]];
    let dist_sq = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];

    if dist_sq < tol * tol {
        PolicyResult::Success(SurfaceRelation::Coincident)
    } else if dist_sq < (tol * af).powi(2) {
        ambiguous(SurfaceRelation::Coincident, dist_sq.sqrt())
    } else {
        PolicyResult::Success(SurfaceRelation::General)
    }
}

/// Classify two tori: coincident if same center, axis, major/minor radius.
pub(super) fn classify_torus_torus(
    c1: &[f64; 3],
    ax1: &[f64; 3],
    major1: f64,
    minor1: f64,
    c2: &[f64; 3],
    ax2: &[f64; 3],
    major2: f64,
    minor2: f64,
    tol: f64,
    af: f64,
) -> PolicyResult<SurfaceRelation> {
    let dot = ax1[0] * ax2[0] + ax1[1] * ax2[1] + ax1[2] * ax2[2];
    let angle_dev = (dot.abs() - 1.0).abs();

    if angle_dev > tol * af {
        return PolicyResult::Success(SurfaceRelation::General);
    }
    if angle_dev > tol {
        return ambiguous(SurfaceRelation::General, angle_dev);
    }

    let major_diff = (major1 - major2).abs();
    let minor_diff = (minor1 - minor2).abs();
    let max_radius_diff = major_diff.max(minor_diff);

    if max_radius_diff > tol * af {
        return PolicyResult::Success(SurfaceRelation::General);
    }
    if max_radius_diff > tol {
        return ambiguous(SurfaceRelation::General, max_radius_diff);
    }

    let d = [c2[0] - c1[0], c2[1] - c1[1], c2[2] - c1[2]];
    let dist_sq = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];

    if dist_sq < tol * tol {
        PolicyResult::Success(SurfaceRelation::Coincident)
    } else if dist_sq < (tol * af).powi(2) {
        ambiguous(SurfaceRelation::Coincident, dist_sq.sqrt())
    } else {
        PolicyResult::Success(SurfaceRelation::General)
    }
}

pub(super) fn classify_triaxial_ellipsoid_triaxial_ellipsoid(
    c1: &[f64; 3],
    u1: &[f64; 3],
    v1: &[f64; 3],
    w1: &[f64; 3],
    a1: f64,
    b1: f64,
    c_radius1: f64,
    c2: &[f64; 3],
    u2: &[f64; 3],
    v2: &[f64; 3],
    w2: &[f64; 3],
    a2: f64,
    b2: f64,
    c_radius2: f64,
    tol: f64,
    af: f64,
) -> PolicyResult<SurfaceRelation> {
    let center_delta = norm([c1[0] - c2[0], c1[1] - c2[1], c1[2] - c2[2]]);
    let axis_delta = frame_max_delta(u1, v1, w1, u2, v2, w2);
    let radius_delta = (a1 - a2)
        .abs()
        .max((b1 - b2).abs())
        .max((c_radius1 - c_radius2).abs());
    let max_delta = center_delta.max(axis_delta).max(radius_delta);

    if max_delta < tol {
        PolicyResult::Success(SurfaceRelation::Coincident)
    } else if max_delta < tol * af {
        ambiguous(SurfaceRelation::Coincident, max_delta)
    } else {
        PolicyResult::Success(SurfaceRelation::General)
    }
}
