//! MB6 — The Unbounded Half-Space Storm
//!
//! DOMAIN: Boolean a closed solid against 200 infinite planar half-spaces
//! (approximated as very large boxes) arranged so 80 are exactly coplanar
//! in groups, 60 graze at 10⁻¹⁴, and 60 create sliver volumes thinner
//! than 10⁻¹². Convert final result back to closed manifold.
//!
//! RISK: Open-sheet trimming + coplanar storm + thin-feature +
//! unbounded classification failure.
//!
//! GOAL: Winding-number classifier + open-sheet handler + manifold repair
//! handles unbounded → bounded conversion flawlessly.
//!
//! KERNEL REQUIREMENTS TO PASS:
//! - Winding-number based classification (replaces ray-parity)
//! - Open-sheet / half-space boolean support
//! - Coplanar face groups resolved consistently across 80 planes
//! - Thin-feature preservation at 10⁻¹² thickness
//! - Manifold repair for unbounded → bounded conversion
//! - BVH handles 200 overlapping large-scale boxes efficiently

use super::super::test_helpers::{
    build_cube, execute_boolean_logged, euler_audit,
};
use super::super::schema::{BooleanInput, BooleanOp};

/// Approximate a half-space as a very large box.
///
/// The half-space has its face at `face_point` with outward normal `normal`.
/// The box extends `extent` units behind the face plane.
fn build_halfspace_approx(
    face_point: [f64; 3],
    normal: [f64; 3],
    extent: f64,
) -> (forge_topo::state::TopologyState, crate::geometry_store::GeometryStore) {
    let len = (normal[0]*normal[0] + normal[1]*normal[1] + normal[2]*normal[2]).sqrt();
    let n = [normal[0]/len, normal[1]/len, normal[2]/len];

    let tangent1 = if n[0].abs() < 0.9 {
        let t = [0.0, -n[2], n[1]];
        let tl = (t[0]*t[0] + t[1]*t[1] + t[2]*t[2]).sqrt();
        [t[0]/tl, t[1]/tl, t[2]/tl]
    } else {
        let t = [n[2], 0.0, -n[0]];
        let tl = (t[0]*t[0] + t[1]*t[1] + t[2]*t[2]).sqrt();
        [t[0]/tl, t[1]/tl, t[2]/tl]
    };
    let tangent2 = [
        n[1]*tangent1[2] - n[2]*tangent1[1],
        n[2]*tangent1[0] - n[0]*tangent1[2],
        n[0]*tangent1[1] - n[1]*tangent1[0],
    ];

    let center = [
        face_point[0] - n[0] * extent / 2.0,
        face_point[1] - n[1] * extent / 2.0,
        face_point[2] - n[2] * extent / 2.0,
    ];

    let planes = vec![
        forge_geom::Plane::from_point_normal(face_point, n).unwrap(),
        forge_geom::Plane::from_point_normal(
            [center[0] - n[0] * extent, center[1] - n[1] * extent, center[2] - n[2] * extent],
            [-n[0], -n[1], -n[2]],
        ).unwrap(),
        forge_geom::Plane::from_point_normal(
            [center[0] + tangent1[0] * extent, center[1] + tangent1[1] * extent, center[2] + tangent1[2] * extent],
            tangent1,
        ).unwrap(),
        forge_geom::Plane::from_point_normal(
            [center[0] - tangent1[0] * extent, center[1] - tangent1[1] * extent, center[2] - tangent1[2] * extent],
            [-tangent1[0], -tangent1[1], -tangent1[2]],
        ).unwrap(),
        forge_geom::Plane::from_point_normal(
            [center[0] + tangent2[0] * extent, center[1] + tangent2[1] * extent, center[2] + tangent2[2] * extent],
            tangent2,
        ).unwrap(),
        forge_geom::Plane::from_point_normal(
            [center[0] - tangent2[0] * extent, center[1] - tangent2[1] * extent, center[2] - tangent2[2] * extent],
            [-tangent2[0], -tangent2[1], -tangent2[2]],
        ).unwrap(),
    ];

    super::super::test_helpers::build_convex_solid(planes)
}

// ══════════════════════════════════════════════════════════════
// §MB6.1  80 COPLANAR HALF-SPACES
// ══════════════════════════════════════════════════════════════

/// MB6.1 — Intersect a cube with 80 coplanar half-spaces in groups.
///
/// The half-spaces are arranged in 10 coplanar groups of 8,
/// with faces aligned along the same plane. The cube must be
/// trimmed by all of them consistently.
#[test]
fn halfspace_storm_80_coplanar() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 10.0);
    let extent = 1e6;

    for group in 0..10 {
        let z = -5.0 + group as f64 * 1.0;
        for sub in 0..8 {
            let x_off = (sub as f64 - 3.5) * 0.1;
            let face_point = [x_off, 0.0, z];
            let normal = [0.0, 0.0, 1.0];

            let (topo_hs, geom_hs) = build_halfspace_approx(face_point, normal, extent);

            let input = BooleanInput::new(
                topo, geom,
                topo_hs, geom_hs,
                BooleanOp::Intersection,
            );

            let step_num = group * 8 + sub;
            match execute_boolean_logged(input) {
                Ok(envelope) => {
                    let r = envelope.into_value();
                    if step_num % 20 == 0 {
                        let (v, e, f, chi) = euler_audit(r.topology().arena());
                        eprintln!("MB6 coplanar HS {step_num}/80: V={v} E={e} F={f} χ={chi}");
                    }
                    let parts = r.into_parts();
                    topo = parts.0;
                    geom = parts.1;
                }
                Err(e) => {
                    panic!("MB6 coplanar half-space {step_num} failed: {e:?}");
                }
            }
        }
    }

    let (v, e, f, chi) = euler_audit(topo.arena());
    eprintln!("MB6 coplanar final: V={v} E={e} F={f} χ={chi}");
    assert_eq!(chi, 2, "MB6 coplanar Euler violation");
}

// ══════════════════════════════════════════════════════════════
// §MB6.2  60 GRAZE HALF-SPACES (10⁻¹⁴)
// ══════════════════════════════════════════════════════════════

/// MB6.2 — 60 half-spaces grazing the cube faces at 10⁻¹⁴ offset.
///
/// Each half-space nearly coincides with one of the cube's faces,
/// triggering near-coincident classification on every trimming step.
#[test]
fn halfspace_storm_60_graze() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 5.0);
    let epsilon = 1e-14;
    let extent = 1e6;

    for i in 0..60 {
        let axis = i % 3;
        let sign = if (i / 3) % 2 == 0 { 1.0 } else { -1.0 };
        let offset = 5.0 * sign + epsilon * (i as f64);

        let mut face_point = [0.0, 0.0, 0.0];
        face_point[axis] = offset;
        let mut normal = [0.0, 0.0, 0.0];
        normal[axis] = sign;

        let (topo_hs, geom_hs) = build_halfspace_approx(face_point, normal, extent);

        let input = BooleanInput::new(
            topo, geom,
            topo_hs, geom_hs,
            BooleanOp::Intersection,
        );

        match execute_boolean_logged(input) {
            Ok(envelope) => {
                let r = envelope.into_value();
                if i % 15 == 0 {
                    let (v, e, f, chi) = euler_audit(r.topology().arena());
                    eprintln!("MB6 graze HS {i}/60: V={v} E={e} F={f} χ={chi}");
                }
                let parts = r.into_parts();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                panic!("MB6 graze half-space {i} failed: {e:?}");
            }
        }
    }

    let (v, e, f, chi) = euler_audit(topo.arena());
    eprintln!("MB6 graze final: V={v} E={e} F={f} χ={chi}");
    assert!(f >= 6, "MB6 graze should produce a bounded solid");
}

// ══════════════════════════════════════════════════════════════
// §MB6.3  60 SLIVER HALF-SPACES (10⁻¹²)
// ══════════════════════════════════════════════════════════════

/// MB6.3 — 60 half-spaces creating sliver volumes thinner than 10⁻¹².
///
/// Pairs of nearly-parallel half-spaces sandwich the cube,
/// creating extremely thin volumes that must be preserved or
/// cleanly collapsed.
#[test]
fn halfspace_storm_60_sliver() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 5.0);
    let sliver_thickness = 1e-12;
    let extent = 1e6;

    for i in 0..30 {
        let z = -4.0 + (i as f64) * 0.25;

        let (topo_top, geom_top) = build_halfspace_approx(
            [0.0, 0.0, z + sliver_thickness],
            [0.0, 0.0, -1.0],
            extent,
        );

        let input = BooleanInput::new(
            topo, geom,
            topo_top, geom_top,
            BooleanOp::Intersection,
        );

        match execute_boolean_logged(input) {
            Ok(envelope) => {
                let r = envelope.into_value();
                let parts = r.into_parts();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                panic!("MB6 sliver half-space top {i} failed: {e:?}");
            }
        }

        let (topo_bot, geom_bot) = build_halfspace_approx(
            [0.0, 0.0, z],
            [0.0, 0.0, 1.0],
            extent,
        );

        let input2 = BooleanInput::new(
            topo, geom,
            topo_bot, geom_bot,
            BooleanOp::Intersection,
        );

        match execute_boolean_logged(input2) {
            Ok(envelope) => {
                let r = envelope.into_value();
                if i % 10 == 0 {
                    let (v, e, f, chi) = euler_audit(r.topology().arena());
                    eprintln!("MB6 sliver pair {i}/30: V={v} E={e} F={f} χ={chi}");
                }
                let parts = r.into_parts();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                panic!("MB6 sliver half-space bot {i} failed: {e:?}");
            }
        }
    }

    let (v, e, f, chi) = euler_audit(topo.arena());
    eprintln!("MB6 sliver final: V={v} E={e} F={f} χ={chi}");
}

// ══════════════════════════════════════════════════════════════
// §MB6.4  FULL SPEC: ALL 200 HALF-SPACES
// ══════════════════════════════════════════════════════════════

/// MB6.4 — Full 200 half-space storm: 80 coplanar + 60 graze + 60 sliver.
///
/// All 200 half-spaces intersected with a cube in sequence.
/// The final result must be a closed bounded manifold.
#[test]
fn halfspace_storm_full_200() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 10.0);
    let extent = 1e6;
    let epsilon = 1e-14;
    let sliver = 1e-12;

    let mut step = 0usize;

    for group in 0..10 {
        let z = -5.0 + group as f64 * 1.0;
        for sub in 0..8 {
            let (topo_hs, geom_hs) = build_halfspace_approx(
                [(sub as f64 - 3.5) * 0.1, 0.0, z],
                [0.0, 0.0, 1.0],
                extent,
            );
            let input = BooleanInput::new(topo, geom, topo_hs, geom_hs, BooleanOp::Intersection);
            match execute_boolean_logged(input) {
                Ok(env) => { let r = env.into_value(); let p = r.into_parts(); topo = p.0; geom = p.1; }
                Err(e) => panic!("MB6 full step {step} (coplanar) failed: {e:?}"),
            }
            step += 1;
        }
    }

    for i in 0..60 {
        let axis = i % 3;
        let sign = if (i / 3) % 2 == 0 { 1.0 } else { -1.0 };
        let mut fp = [0.0, 0.0, 0.0];
        fp[axis] = 10.0 * sign + epsilon * (i as f64);
        let mut n = [0.0, 0.0, 0.0];
        n[axis] = sign;
        let (topo_hs, geom_hs) = build_halfspace_approx(fp, n, extent);
        let input = BooleanInput::new(topo, geom, topo_hs, geom_hs, BooleanOp::Intersection);
        match execute_boolean_logged(input) {
            Ok(env) => { let r = env.into_value(); let p = r.into_parts(); topo = p.0; geom = p.1; }
            Err(e) => panic!("MB6 full step {step} (graze) failed: {e:?}"),
        }
        step += 1;
    }

    for i in 0..30 {
        let z = -4.0 + (i as f64) * 0.25;
        let (topo_t, geom_t) = build_halfspace_approx([0.0, 0.0, z + sliver], [0.0, 0.0, -1.0], extent);
        let input = BooleanInput::new(topo, geom, topo_t, geom_t, BooleanOp::Intersection);
        match execute_boolean_logged(input) {
            Ok(env) => { let r = env.into_value(); let p = r.into_parts(); topo = p.0; geom = p.1; }
            Err(e) => panic!("MB6 full step {step} (sliver-top) failed: {e:?}"),
        }
        step += 1;
        let (topo_b, geom_b) = build_halfspace_approx([0.0, 0.0, z], [0.0, 0.0, 1.0], extent);
        let input2 = BooleanInput::new(topo, geom, topo_b, geom_b, BooleanOp::Intersection);
        match execute_boolean_logged(input2) {
            Ok(env) => { let r = env.into_value(); let p = r.into_parts(); topo = p.0; geom = p.1; }
            Err(e) => panic!("MB6 full step {step} (sliver-bot) failed: {e:?}"),
        }
        step += 1;
    }

    let (v, e, f, chi) = euler_audit(topo.arena());
    eprintln!("MB6 full 200 final: V={v} E={e} F={f} χ={chi} ({step} ops total)");
    assert_eq!(chi, 2, "MB6 full storm Euler violation");
    assert!(f >= 6, "MB6 full storm must produce a bounded solid");
}
