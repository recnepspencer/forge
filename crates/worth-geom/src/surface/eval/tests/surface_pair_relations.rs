use crate::surface::eval::classify_surface_pair;
use crate::surface::schema::{SurfaceData, SurfaceRelation};
use std::f64::consts::PI;

#[test]
fn axis_swapped_triaxial_ellipsoids_are_general_not_coincident() {
    let canonical = SurfaceData::triaxial_ellipsoid(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        5.0,
        3.0,
        2.0,
    )
    .expect("canonical");
    let swapped = SurfaceData::triaxial_ellipsoid(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0],
        5.0,
        2.0,
        3.0,
    )
    .expect("swapped");
    assert_eq!(
        classify_surface_pair(&canonical, &swapped, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::General
    );
}

#[test]
fn coincident_planes_detected() {
    let a = SurfaceData::plane([0.0, 0.0, 1.0], 5.0);
    let b = SurfaceData::plane([0.0, 0.0, 1.0], 5.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::Coincident
    );
}

#[test]
fn antiparallel_coincident_planes_detected() {
    let a = SurfaceData::plane([0.0, 0.0, 1.0], 5.0);
    let b = SurfaceData::plane([0.0, 0.0, -1.0], -5.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::Coincident
    );
}

#[test]
fn parallel_disjoint_planes() {
    let a = SurfaceData::plane([0.0, 0.0, 1.0], 3.0);
    let b = SurfaceData::plane([0.0, 0.0, 1.0], 7.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::Disjoint
    );
}

#[test]
fn intersecting_planes_are_general() {
    let a = SurfaceData::plane([1.0, 0.0, 0.0], 0.0);
    let b = SurfaceData::plane([0.0, 1.0, 0.0], 0.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::General
    );
}

#[test]
fn coincident_spheres() {
    let a = SurfaceData::sphere([0.0, 0.0, 0.0], 5.0);
    let b = SurfaceData::sphere([0.0, 0.0, 0.0], 5.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::Coincident
    );
}

#[test]
fn disjoint_spheres() {
    let a = SurfaceData::sphere([0.0, 0.0, 0.0], 1.0);
    let b = SurfaceData::sphere([10.0, 0.0, 0.0], 1.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::Disjoint
    );
}

#[test]
fn contained_sphere_is_disjoint() {
    let a = SurfaceData::sphere([0.0, 0.0, 0.0], 10.0);
    let b = SurfaceData::sphere([0.0, 0.0, 0.0], 1.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::Disjoint
    );
}

#[test]
fn coincident_cylinders() {
    let a = SurfaceData::cylinder([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0);
    let b = SurfaceData::cylinder([0.0, 0.0, 5.0], [0.0, 0.0, 1.0], 3.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::Coincident
    );
}

// ── Cone/Torus Classification Tests ─────────────────────────────────────

#[test]
fn same_cone_detected_as_coincident() {
    let a = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI / 4.0);
    let b = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI / 4.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::Coincident
    );
}

#[test]
fn same_cone_antiparallel_axis_detected() {
    let a = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI / 4.0);
    let b = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], PI / 4.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::Coincident
    );
}

#[test]
fn different_cone_angle_is_general() {
    let a = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI / 4.0);
    let b = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI / 3.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::General
    );
}

#[test]
fn same_torus_detected_as_coincident() {
    let a = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 1.0);
    let b = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 1.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::Coincident
    );
}

#[test]
fn same_torus_antiparallel_axis_detected() {
    let a = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 1.0);
    let b = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], 5.0, 1.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::Coincident
    );
}

#[test]
fn different_torus_major_radius_is_general() {
    let a = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 1.0);
    let b = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 6.0, 1.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::General
    );
}
