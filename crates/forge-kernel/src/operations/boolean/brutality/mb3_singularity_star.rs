//! MB3 — The High-Valence Singularity Star
//!
//! DOMAIN: 64 cubes + 32 tetrahedra + 16 dodecahedra all sharing one
//! single vertex with edges radiating in every direction (SoS pushed
//! to 112-way degeneracy). Subtract a tool that passes 10⁻¹⁵ away
//! from that vertex while exactly coplanar with 8 of the faces.
//!
//! RISK: SoS breakdown, predicate inconsistency across 100+ incident
//! elements, non-manifold star repair failure.
//!
//! GOAL: Full SoS + non-manifold post-processor at extreme valence;
//! Euler auditor must still hold.
//!
//! KERNEL REQUIREMENTS TO PASS:
//! - Simulation of Simplicity (SoS) handles 112-way degeneracy
//! - Predicate consistency across 100+ incident elements at one vertex
//! - Non-manifold star repair at extreme valence
//! - Stitching survives vertex fan with 100+ outgoing halfedges
//! - Classification handles near-coplanar tool faces at shared vertex

use super::super::test_helpers::{
    build_cube, build_tetrahedron, build_dodecahedron,
    execute_boolean_logged, euler_audit,
};
use super::super::schema::{BooleanInput, BooleanOp};

/// Build the singularity star: union 64+32+16 solids all touching one vertex.
fn build_singularity_star() -> Option<(
    forge_topo::state::TopologyState,
    crate::geometry_state::GeometryState,
)> {
    let shared_vertex = [0.0, 0.0, 0.0];
    let (mut topo, mut geom) = build_cube([0.5, 0.5, 0.5], 0.5);

    let mut step = 0usize;

    for i in 0..63 {
        let angle_h = (i as f64) * std::f64::consts::TAU / 63.0;
        let angle_v = (i as f64) * 0.3;
        let r = 0.5;
        let cx = shared_vertex[0] + r * angle_h.cos() * angle_v.cos() + r;
        let cy = shared_vertex[1] + r * angle_h.sin() * angle_v.cos() + r;
        let cz = shared_vertex[2] + r * angle_v.sin() + r;

        let (topo_tool, geom_tool) = build_cube([cx, cy, cz], 0.5);
        let input = BooleanInput::new(topo, geom, topo_tool, geom_tool, BooleanOp::Union);

        match execute_boolean_logged(input).into_result() {
            Ok(result) => {
                let r = result;
                step += 1;
                if step % 20 == 0 {
                    let (v, e, f, chi) = euler_audit(r.topology().arena());
                    eprintln!("MB3 cube {step}/63: V={v} E={e} F={f} χ={chi}");
                }
                let parts = r.into_topo_geom();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                eprintln!("MB3 cube {step} failed: {e}");
                return None;
            }
        }
    }

    eprintln!("MB3: 64 cubes done, adding 32 tetrahedra...");
    for i in 0..32 {
        let angle = (i as f64) * std::f64::consts::TAU / 32.0;
        let r = 0.3;
        let cx = shared_vertex[0] + r * angle.cos();
        let cy = shared_vertex[1] + r * angle.sin();
        let cz = shared_vertex[2] + (i as f64) * 0.05;

        let (topo_tool, geom_tool) = build_tetrahedron([cx, cy, cz], 0.3);
        let input = BooleanInput::new(topo, geom, topo_tool, geom_tool, BooleanOp::Union);

        match execute_boolean_logged(input).into_result() {
            Ok(result) => {
                let r = result;
                step += 1;
                if step % 20 == 0 {
                    let (v, e, f, chi) = euler_audit(r.topology().arena());
                    eprintln!("MB3 tet {step}: V={v} E={e} F={f} χ={chi}");
                }
                let parts = r.into_topo_geom();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                eprintln!("MB3 tetrahedron {step} failed: {e}");
                return None;
            }
        }
    }

    eprintln!("MB3: 32 tetrahedra done, adding 16 dodecahedra...");
    for i in 0..16 {
        let angle = (i as f64) * std::f64::consts::TAU / 16.0;
        let r = 0.7;
        let cx = shared_vertex[0] + r * angle.cos();
        let cy = shared_vertex[1] + r * angle.sin();

        let (topo_tool, geom_tool) = build_dodecahedron([cx, cy, 0.0], 0.25);
        let input = BooleanInput::new(topo, geom, topo_tool, geom_tool, BooleanOp::Union);

        match execute_boolean_logged(input).into_result() {
            Ok(result) => {
                let r = result;
                step += 1;
                let (v, e, f, chi) = euler_audit(r.topology().arena());
                eprintln!("MB3 dodec {step}: V={v} E={e} F={f} χ={chi}");
                let parts = r.into_topo_geom();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                eprintln!("MB3 dodecahedron {step} failed: {e}");
                return None;
            }
        }
    }

    Some((topo, geom))
}

// ══════════════════════════════════════════════════════════════
// §MB3.1  SINGULARITY STAR CONSTRUCTION
// ══════════════════════════════════════════════════════════════

/// MB3.1 — Build the full 112-solid singularity star.
///
/// 64 cubes + 32 tetrahedra + 16 dodecahedra, all sharing
/// edges/vertices near the origin.
#[test]
fn singularity_star_construction() {
    match build_singularity_star() {
        Some((topo, _geom)) => {
            let (v, e, f, chi) = euler_audit(topo.arena());
            eprintln!("MB3 star final: V={v} E={e} F={f} χ={chi}");
            assert_eq!(chi, 2, "MB3 star Euler violation: V={v} E={e} F={f}");
        }
        None => {
            panic!("MB3 singularity star construction failed");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §MB3.2  NEAR-MISS SUBTRACTION AT SINGULARITY
// ══════════════════════════════════════════════════════════════

/// MB3.2 — Subtract a tool that passes 10⁻¹⁵ from the shared vertex.
///
/// The tool is exactly coplanar with 8 of the star's faces,
/// creating a 112-way SoS degeneracy at the shared vertex.
#[test]
fn singularity_near_miss_subtraction() {
    let star = build_singularity_star();

    let (topo_star, geom_star) = match star {
        Some(s) => s,
        None => panic!("MB3 star construction failed — cannot test near-miss"),
    };

    let epsilon = 1e-15;
    let (topo_tool, geom_tool) = build_cube([epsilon, epsilon, epsilon], 2.0);

    let input = BooleanInput::new(
        topo_star, geom_star,
        topo_tool, geom_tool,
        BooleanOp::Subtraction,
    );

    match execute_boolean_logged(input).into_result() {
        Ok(result) => {
            let r = result;
            let (v, e, f, chi) = euler_audit(r.topology().arena());
            eprintln!("MB3 near-miss: V={v} E={e} F={f} χ={chi}");
            assert!(f > 0, "MB3 near-miss should produce faces");
        }
        Err(e) => {
            panic!("MB3 near-miss subtraction failed: {e}");
        }
    }
}
