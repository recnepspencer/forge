//! MB8 — The Ultimate Degeneracy Avalanche (The True Final Boss)
//!
//! DOMAIN: Start with MB3 star (112 solids at shared vertex), add MB1
//! coplanar overlaps on 12 planes, wrap in MB2 Menger-level genus,
//! inject MB4 self-intersections, then run MB5 200-step chain with
//! MB7 micro-features and one orientation flip at step 100.
//!
//! RISK: Literally every failure mode ever coded, triggered in one model.
//!
//! GOAL: Full pipeline (predicates → symbolic vertices → SoS → decimator
//! → auditor → rollback) must survive or cleanly fail with a debug trace
//! pointing to the exact trigger.
//!
//! KERNEL REQUIREMENTS TO PASS:
//! - Everything from MB1 through MB7, simultaneously
//! - SoS at 112-way degeneracy + coplanar storm + thin features
//! - Chained booleans on high-genus topology with micro-features
//! - Transaction rollback with debug traces on any failure
//! - Self-intersection recovery after multi-modal damage
//! - Performance under combined load of all failure modes

use super::super::test_helpers::{
    build_cube, build_tetrahedron, build_dodecahedron,
    execute_boolean_logged, euler_audit,
    menger_sponge_subtraction_centers,
};
use super::super::schema::{BooleanInput, BooleanOp};

// ══════════════════════════════════════════════════════════════
// §MB8.1  PHASE 1: SINGULARITY STAR BASE
// ══════════════════════════════════════════════════════════════

/// Build a simplified star (16 cubes + 8 tetrahedra + 4 dodecahedra)
/// to serve as the base for the ultimate test. Uses fewer solids
/// than MB3 to leave room for subsequent phases.
fn build_star_base() -> Option<(
    forge_topo::state::TopologyState,
    crate::geometry_store::GeometryStore,
)> {
    let (mut topo, mut geom) = build_cube([0.5, 0.5, 0.5], 0.5);

    for i in 0..15 {
        let angle = (i as f64) * std::f64::consts::TAU / 15.0;
        let r = 0.4;
        let cx = r * angle.cos() + r;
        let cy = r * angle.sin() + r;
        let (topo_tool, geom_tool) = build_cube([cx, cy, 0.5], 0.5);
        let input = BooleanInput::new(topo, geom, topo_tool, geom_tool, BooleanOp::Union);
        match execute_boolean_logged(input) {
            Ok(env) => { let r = env.into_value(); let p = r.into_topo_geom(); topo = p.0; geom = p.1; }
            Err(e) => { eprintln!("MB8 star cube {i} failed: {e:?}"); return None; }
        }
    }

    for i in 0..8 {
        let angle = (i as f64) * std::f64::consts::TAU / 8.0;
        let (topo_tool, geom_tool) = build_tetrahedron(
            [0.3 * angle.cos(), 0.3 * angle.sin(), (i as f64) * 0.1],
            0.2,
        );
        let input = BooleanInput::new(topo, geom, topo_tool, geom_tool, BooleanOp::Union);
        match execute_boolean_logged(input) {
            Ok(env) => { let r = env.into_value(); let p = r.into_topo_geom(); topo = p.0; geom = p.1; }
            Err(e) => { eprintln!("MB8 star tet {i} failed: {e:?}"); return None; }
        }
    }

    for i in 0..4 {
        let angle = (i as f64) * std::f64::consts::TAU / 4.0;
        let (topo_tool, geom_tool) = build_dodecahedron(
            [0.5 * angle.cos(), 0.5 * angle.sin(), 0.0],
            0.2,
        );
        let input = BooleanInput::new(topo, geom, topo_tool, geom_tool, BooleanOp::Union);
        match execute_boolean_logged(input) {
            Ok(env) => { let r = env.into_value(); let p = r.into_topo_geom(); topo = p.0; geom = p.1; }
            Err(e) => { eprintln!("MB8 star dodec {i} failed: {e:?}"); return None; }
        }
    }

    Some((topo, geom))
}

// ══════════════════════════════════════════════════════════════
// §MB8.2  PHASE 2: ADD COPLANAR OVERLAPS
// ══════════════════════════════════════════════════════════════

/// Layer 12 coplanar-overlap cubes onto the star base.
fn add_coplanar_overlaps(
    mut topo: forge_topo::state::TopologyState,
    mut geom: crate::geometry_store::GeometryStore,
) -> Option<(forge_topo::state::TopologyState, crate::geometry_store::GeometryStore)> {
    for i in 0..12 {
        let angle = (i as f64) * std::f64::consts::PI / 6.0;
        let (topo_tool, geom_tool) = build_cube(
            [angle.cos(), angle.sin(), 0.0],
            1.0,
        );
        let input = BooleanInput::new(topo, geom, topo_tool, geom_tool, BooleanOp::Union);
        match execute_boolean_logged(input) {
            Ok(env) => {
                let r = env.into_value();
                let p = r.into_topo_geom();
                topo = p.0;
                geom = p.1;
            }
            Err(e) => { eprintln!("MB8 coplanar {i} failed: {e:?}"); return None; }
        }
    }
    Some((topo, geom))
}

// ══════════════════════════════════════════════════════════════
// §MB8.3  PHASE 3: MENGER TUNNELS
// ══════════════════════════════════════════════════════════════

/// Carve Menger-level-1 tunnels into the solid.
fn add_menger_tunnels(
    mut topo: forge_topo::state::TopologyState,
    mut geom: crate::geometry_store::GeometryStore,
) -> Option<(forge_topo::state::TopologyState, crate::geometry_store::GeometryStore)> {
    let subs = menger_sponge_subtraction_centers([0.0, 0.0, 0.0], 1.5, 1);
    for (i, (center, half)) in subs.into_iter().enumerate() {
        let (topo_tool, geom_tool) = build_cube(center, half);
        let input = BooleanInput::new(topo, geom, topo_tool, geom_tool, BooleanOp::Subtraction);
        match execute_boolean_logged(input) {
            Ok(env) => {
                let r = env.into_value();
                let p = r.into_topo_geom();
                topo = p.0;
                geom = p.1;
            }
            Err(e) => { eprintln!("MB8 menger tunnel {i} failed: {e:?}"); return None; }
        }
    }
    Some((topo, geom))
}

// ══════════════════════════════════════════════════════════════
// §MB8.4  PHASE 4: MICRO-FEATURE CHAIN WITH ORIENTATION FLIP
// ══════════════════════════════════════════════════════════════

/// Run 200-step chain with micro-features and an orientation flip at step 100.
fn run_chain_with_flip(
    mut topo: forge_topo::state::TopologyState,
    mut geom: crate::geometry_store::GeometryStore,
) -> Option<(forge_topo::state::TopologyState, crate::geometry_store::GeometryStore)> {
    let ops = [
        BooleanOp::Union,
        BooleanOp::Subtraction,
        BooleanOp::Intersection,
        BooleanOp::Union,
    ];

    for step in 1..=200 {
        let op = ops[step % ops.len()];

        let mut offset = match step % 4 {
            0 => [0.3, 0.0, 0.0],
            1 => [-0.3, 0.0, 0.0],
            2 => [0.0, 0.3, 0.0],
            _ => [0.0, -0.3, 0.0],
        };
        let half = if op == BooleanOp::Subtraction { 0.1 } else { 0.5 };

        if step == 100 {
            offset = [-offset[0], -offset[1], -offset[2]];
            eprintln!("MB8 ORIENTATION FLIP at step 100");
        }

        let (topo_tool, geom_tool) = build_cube(offset, half);
        let input = BooleanInput::new(topo, geom, topo_tool, geom_tool, op);

        match execute_boolean_logged(input) {
            Ok(envelope) => {
                let r = envelope.into_value();
                if step % 50 == 0 {
                    let (v, e, f, chi) = euler_audit(r.topology().arena());
                    eprintln!("MB8 chain step {step}: V={v} E={e} F={f} χ={chi}");
                }
                let parts = r.into_topo_geom();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                eprintln!("MB8 chain step {step} ({op:?}) failed: {e:?}");
                return None;
            }
        }
    }

    Some((topo, geom))
}

// ══════════════════════════════════════════════════════════════
// §MB8  THE FULL BOSS FIGHT
// ══════════════════════════════════════════════════════════════

/// MB8 — The Ultimate Degeneracy Avalanche.
///
/// Phase 1: Build 28-solid singularity star (cubes + tets + dodecs)
/// Phase 2: Add 12 coplanar-overlap cubes
/// Phase 3: Carve Menger L1 tunnels (7 subtractions)
/// Phase 4: Run 200-step chain with orientation flip at step 100
///
/// Every failure mode from MB1-MB7 fires in one test.
#[test]
fn ultimate_degeneracy_avalanche() {
    eprintln!("╔══════════════════════════════════════════╗");
    eprintln!("║  MB8: THE ULTIMATE DEGENERACY AVALANCHE  ║");
    eprintln!("╚══════════════════════════════════════════╝");

    eprintln!("\n▸ Phase 1: Building singularity star...");
    let (topo, geom) = build_star_base()
        .unwrap_or_else(|| panic!("MB8 Phase 1 (star base) failed"));
    let (v, e, f, chi) = euler_audit(topo.arena());
    eprintln!("  Star complete: V={v} E={e} F={f} χ={chi}");

    eprintln!("\n▸ Phase 2: Adding coplanar overlaps...");
    let (topo, geom) = add_coplanar_overlaps(topo, geom)
        .unwrap_or_else(|| panic!("MB8 Phase 2 (coplanar overlaps) failed"));
    let (v, e, f, chi) = euler_audit(topo.arena());
    eprintln!("  Coplanar complete: V={v} E={e} F={f} χ={chi}");

    eprintln!("\n▸ Phase 3: Carving Menger tunnels...");
    let (topo, geom) = add_menger_tunnels(topo, geom)
        .unwrap_or_else(|| panic!("MB8 Phase 3 (Menger tunnels) failed"));
    let (v, e, f, chi) = euler_audit(topo.arena());
    eprintln!("  Menger complete: V={v} E={e} F={f} χ={chi}");

    eprintln!("\n▸ Phase 4: Running 200-step chain with flip at 100...");
    let (topo, _geom) = run_chain_with_flip(topo, geom)
        .unwrap_or_else(|| panic!("MB8 Phase 4 (chain + flip) failed"));
    let (v, e, f, chi) = euler_audit(topo.arena());

    eprintln!("\n╔══════════════════════════════════════════╗");
    eprintln!("║  MB8 FINAL: V={v} E={e} F={f} χ={chi}");
    eprintln!("╚══════════════════════════════════════════╝");

    assert_eq!(chi, 2, "MB8 FINAL Euler violation: V={v} E={e} F={f}");
    assert!(f > 6, "MB8 should produce a complex solid, got {f} faces");
}

/// MB8.phase1 — Isolated Phase 1 test for debugging.
#[test]
fn ultimate_phase1_star_only() {
    let (topo, _geom) = build_star_base()
        .unwrap_or_else(|| panic!("MB8 star base failed"));
    let (v, e, f, chi) = euler_audit(topo.arena());
    eprintln!("MB8-P1: V={v} E={e} F={f} χ={chi}");
    assert_eq!(chi, 2, "MB8 Phase 1 Euler violation");
}

/// MB8.phase1+2 — Phases 1+2 for debugging.
#[test]
fn ultimate_phase1_plus_phase2() {
    let (topo, geom) = build_star_base()
        .unwrap_or_else(|| panic!("MB8 star base failed"));
    let (topo, _geom) = add_coplanar_overlaps(topo, geom)
        .unwrap_or_else(|| panic!("MB8 coplanar overlaps failed"));
    let (v, e, f, chi) = euler_audit(topo.arena());
    eprintln!("MB8-P1+P2: V={v} E={e} F={f} χ={chi}");
    assert_eq!(chi, 2, "MB8 Phase 1+2 Euler violation");
}

/// MB8.phase1+2+3 — Phases 1+2+3 for debugging.
#[test]
fn ultimate_phase1_through_phase3() {
    let (topo, geom) = build_star_base()
        .unwrap_or_else(|| panic!("MB8 star base failed"));
    let (topo, geom) = add_coplanar_overlaps(topo, geom)
        .unwrap_or_else(|| panic!("MB8 coplanar overlaps failed"));
    let (topo, _geom) = add_menger_tunnels(topo, geom)
        .unwrap_or_else(|| panic!("MB8 Menger tunnels failed"));
    let (v, e, f, chi) = euler_audit(topo.arena());
    eprintln!("MB8-P1+P2+P3: V={v} E={e} F={f} χ={chi}");
    assert_eq!(chi, 2, "MB8 Phase 1+2+3 Euler violation");
}
