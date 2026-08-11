use crate::support::PolicyResult;
use crate::surface::schema::{SurfaceData, SurfaceKind, SurfaceRelation};

use super::analytic_pair_classifiers::{
    classify_cone_cone, classify_cylinder_cylinder, classify_plane_plane, classify_sphere_sphere,
    classify_torus_torus, classify_triaxial_ellipsoid_triaxial_ellipsoid,
};

/// Analytic surface-surface relation classification.
///
/// Compares abstract surface definitions to determine if two surfaces are
/// coincident (same surface), disjoint (cannot intersect), or general
/// (require SSI computation). This is the "analytic arbitration" step in the
/// hybrid Boolean pipeline (Architecture §4.3.4).
///
/// Returns `PolicyResult<SurfaceRelation>` per Doctrine D2: when a measure
/// falls within the ambiguity band (`tol..ambiguity_factor*tol`), the solver
/// returns `Ambiguous` and lets the kernel decide via `ModelingContext` policy.
///
/// `ambiguity_factor` is passed from `ToleranceConfig::get_ambiguity_band_factor()`.
pub fn classify_surface_pair(
    a: &SurfaceData,
    b: &SurfaceData,
    tol: f64,
    ambiguity_factor: f64,
) -> PolicyResult<SurfaceRelation> {
    classify_pair_by_kind(&a.kind, &b.kind, tol, ambiguity_factor)
}

fn classify_pair_by_kind(
    a: &SurfaceKind,
    b: &SurfaceKind,
    tol: f64,
    ambiguity_factor: f64,
) -> PolicyResult<SurfaceRelation> {
    classify_plane_pair(a, b, tol, ambiguity_factor)
        .or_else(|| classify_sphere_pair(a, b, tol, ambiguity_factor))
        .or_else(|| classify_cylinder_pair(a, b, tol, ambiguity_factor))
        .or_else(|| classify_cone_pair(a, b, tol, ambiguity_factor))
        .or_else(|| classify_triaxial_ellipsoid_pair(a, b, tol, ambiguity_factor))
        .or_else(|| classify_torus_pair(a, b, tol, ambiguity_factor))
        .unwrap_or(PolicyResult::Success(SurfaceRelation::General))
}

fn classify_plane_pair(
    a: &SurfaceKind,
    b: &SurfaceKind,
    tol: f64,
    ambiguity_factor: f64,
) -> Option<PolicyResult<SurfaceRelation>> {
    match (a, b) {
        (
            SurfaceKind::Plane {
                normal: n1,
                offset: d1,
            },
            SurfaceKind::Plane {
                normal: n2,
                offset: d2,
            },
        ) => Some(classify_plane_plane(
            n1,
            *d1,
            n2,
            *d2,
            tol,
            ambiguity_factor,
        )),
        _ => None,
    }
}

fn classify_sphere_pair(
    a: &SurfaceKind,
    b: &SurfaceKind,
    tol: f64,
    ambiguity_factor: f64,
) -> Option<PolicyResult<SurfaceRelation>> {
    match (a, b) {
        (
            SurfaceKind::Sphere {
                center: c1,
                radius: r1,
            },
            SurfaceKind::Sphere {
                center: c2,
                radius: r2,
            },
        ) => Some(classify_sphere_sphere(
            c1,
            *r1,
            c2,
            *r2,
            tol,
            ambiguity_factor,
        )),
        _ => None,
    }
}

fn classify_cylinder_pair(
    a: &SurfaceKind,
    b: &SurfaceKind,
    tol: f64,
    ambiguity_factor: f64,
) -> Option<PolicyResult<SurfaceRelation>> {
    match (a, b) {
        (
            SurfaceKind::Cylinder {
                origin: o1,
                axis: a1,
                radius: r1,
            },
            SurfaceKind::Cylinder {
                origin: o2,
                axis: a2,
                radius: r2,
            },
        ) => Some(classify_cylinder_cylinder(
            o1,
            a1,
            *r1,
            o2,
            a2,
            *r2,
            tol,
            ambiguity_factor,
        )),
        _ => None,
    }
}

fn classify_cone_pair(
    a: &SurfaceKind,
    b: &SurfaceKind,
    tol: f64,
    ambiguity_factor: f64,
) -> Option<PolicyResult<SurfaceRelation>> {
    match (a, b) {
        (
            SurfaceKind::Cone {
                apex: a1,
                axis: ax1,
                half_angle: ha1,
            },
            SurfaceKind::Cone {
                apex: a2,
                axis: ax2,
                half_angle: ha2,
            },
        ) => Some(classify_cone_cone(
            a1,
            ax1,
            *ha1,
            a2,
            ax2,
            *ha2,
            tol,
            ambiguity_factor,
        )),
        _ => None,
    }
}

fn classify_triaxial_ellipsoid_pair(
    a: &SurfaceKind,
    b: &SurfaceKind,
    tol: f64,
    ambiguity_factor: f64,
) -> Option<PolicyResult<SurfaceRelation>> {
    match (a, b) {
        (
            SurfaceKind::TriaxialEllipsoid {
                center: c1,
                axis_u: u1,
                axis_v: v1,
                axis_w: w1,
                radius_a: a1,
                radius_b: b1,
                radius_c: c_radius1,
            },
            SurfaceKind::TriaxialEllipsoid {
                center: c2,
                axis_u: u2,
                axis_v: v2,
                axis_w: w2,
                radius_a: a2,
                radius_b: b2,
                radius_c: c_radius2,
            },
        ) => Some(classify_triaxial_ellipsoid_triaxial_ellipsoid(
            c1,
            u1,
            v1,
            w1,
            *a1,
            *b1,
            *c_radius1,
            c2,
            u2,
            v2,
            w2,
            *a2,
            *b2,
            *c_radius2,
            tol,
            ambiguity_factor,
        )),
        _ => None,
    }
}

fn classify_torus_pair(
    a: &SurfaceKind,
    b: &SurfaceKind,
    tol: f64,
    ambiguity_factor: f64,
) -> Option<PolicyResult<SurfaceRelation>> {
    match (a, b) {
        (
            SurfaceKind::Torus {
                center: c1,
                axis: ax1,
                major_r: mj1,
                minor_r: mn1,
            },
            SurfaceKind::Torus {
                center: c2,
                axis: ax2,
                major_r: mj2,
                minor_r: mn2,
            },
        ) => Some(classify_torus_torus(
            c1,
            ax1,
            *mj1,
            *mn1,
            c2,
            ax2,
            *mj2,
            *mn2,
            tol,
            ambiguity_factor,
        )),
        _ => None,
    }
}
