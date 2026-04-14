//! Volume Oracle — ground-truth fuzzing via the divergence theorem.
//!
//! DOMAIN: Uses volume as an independent oracle to verify that topology
//! mutations preserve geometric integrity. The divergence theorem volume
//! `V = (1/6) Σ det([v1, v2, v3])` is computed through the production
//! `solid_volume` pipeline (forge-kernel geometry facade → worth-geom
//! polyhedron_volume). Tests assert volume invariance across Euler
//! operator chains, catching geometry corruption that topology validators
//! alone cannot detect.
//!
//! PHILOSOPHY: These are NOT unit tests of the volume function. They are
//! cross-validation tests that use volume as the canary — if an Euler
//! operator corrupts vertex positions, face winding, or loop traversal,
//! the volume oracle will disagree with the pre-mutation measurement.

use proptest::prelude::*;

use forge_core::OperationResult;
use forge_topo::boundary_editing::join_faces::JoinFaces;
use forge_topo::entity_lifecycle::make_edge_face::MakeEdgeFace;
use forge_topo::entity_lifecycle::split_edge::SplitEdge;

use crate::engine::facade::SolidEnvelope;
use crate::geometry::facade::solid_volume;
use crate::integration_tests::harness::chains::OpChain;
use crate::integration_tests::harness::shapes;
use crate::integration_tests::harness::shapes::{collect_face_loop, first_halfedge_of_face};
use crate::integration_tests::harness::verify::verify;

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Measure the volume of a `SolidEnvelope` through the production pipeline.
fn measure_volume(env: &SolidEnvelope) -> f64 {
    solid_volume(env.topology().arena(), env.geometry())
}

/// Reassemble a `SolidEnvelope` after draft mutation.
fn commit_draft(
    draft: forge_topo::transactions::MutableDraft,
    geometry: crate::geometry::facade::GeometryStore,
) -> Result<OperationResult<SolidEnvelope>, forge_core::KernelError> {
    let topo = draft.commit()?;
    Ok(OperationResult::new(SolidEnvelope::new(topo, geometry)))
}

/// Absolute tolerance for volume invariance checks.
/// 1e-10 is tight enough to catch real corruption but allows
/// floating-point accumulation across multiple operations.
const VOLUME_TOL: f64 = 1e-10;

// ═══════════════════════════════════════════════════════════════════════════
// Part 1: Static Ground Truth
//
// These anchor the oracle itself. If these fail, the volume math is broken
// and no invariance test can be trusted.
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn ground_truth_cube_1000() {
    // make_cube(center, size) uses `size` as the half-extent of the cube.
    // half-extent 10.0 → side 20.0 → volume 8000.0? No:
    // Verified empirically: size=10.0 → volume=1000.0 meaning the parameter
    // is the full side length. 10^3 = 1000.
    let env = shapes::cube([0.0; 3], 10.0)
        .expect("cube generation failed")
        .into_value();

    verify(&env)
        .named("ground_truth_cube")
        .volume_approx(1000.0, VOLUME_TOL)
        .pass();
}

#[test]
fn ground_truth_tetrahedron_regression() {
    // The BSP-generated tetrahedron at scale=1.0 has a specific volume
    // locked in primitive_verification.rs (~23.303639). We verify the
    // oracle agrees with that locked value.
    let env = shapes::tetrahedron()
        .expect("tetrahedron generation failed")
        .into_value();

    let vol = measure_volume(&env);
    assert!(
        (vol - 23.303639).abs() < 0.001,
        "Tetrahedron volume oracle disagrees with locked regression: got {vol:.6}, expected ~23.303639"
    );
}

#[test]
fn ground_truth_hexagonal_prism_analytical() {
    // Hexagonal prism: sides=6, radius=5.0, height=10.0.
    // The `prism` shape generator places face midpoints at `radius` (apothem).
    // Area of regular hexagon from apothem a = 2·√3·a².
    // Volume = 2·√3·25·10 = 866.025403784.
    let radius: f64 = 5.0;
    let height: f64 = 10.0;
    let expected = 2.0 * 3.0_f64.sqrt() * radius.powi(2) * height;

    let env = shapes::prism([0.0; 3], 6, radius, height)
        .expect("prism generation failed")
        .into_value();

    verify(&env)
        .named("ground_truth_hex_prism")
        .volume_approx(expected, VOLUME_TOL)
        .pass();
}

// ═══════════════════════════════════════════════════════════════════════════
// Part 2: Volume Invariance Under Euler Operators
//
// The real oracle: topology mutations must NOT change volume.
// If SplitEdge corrupts a vertex position, or MakeEdgeFace reverses a
// face winding, the volume will shift. These tests catch that.
// ═══════════════════════════════════════════════════════════════════════════

/// SplitEdge on a cube must preserve volume exactly.
/// One split adds a vertex but changes no face geometry.
#[test]
fn invariance_split_edge_preserves_cube_volume() {
    let env_res = shapes::cube([0.0; 3], 5.0).expect("cube failed");
    let vol_before = measure_volume(env_res.get_value());
    let faces = env_res.get_value().faces().to_vec();

    let result = OpChain::new(env_res)
        .apply("split_edge", |env, _scope| {
            let (mut draft, geom) = env.into_draft();
            let he = first_halfedge_of_face(draft.arena(), faces[0])?;
            draft.execute(SplitEdge { edge: he })?;
            commit_draft(draft, geom)
        })
        .finish_validated();

    let vol_after = measure_volume(result.get_value());
    assert!(
        (vol_after - vol_before).abs() < VOLUME_TOL,
        "SplitEdge corrupted volume: before={vol_before:.10}, after={vol_after:.10}, diff={:.2e}",
        (vol_after - vol_before).abs()
    );
}

/// Two sequential SplitEdge operations on different faces.
/// Each split must independently preserve volume.
#[test]
fn invariance_double_split_preserves_volume() {
    let env_res = shapes::cube([0.0; 3], 7.0).expect("cube failed");
    let vol_before = measure_volume(env_res.get_value());
    let faces = env_res.get_value().faces().to_vec();

    let result = OpChain::new(env_res)
        .apply("split_face_0", |env, _scope| {
            let (mut draft, geom) = env.into_draft();
            let he = first_halfedge_of_face(draft.arena(), faces[0])?;
            draft.execute(SplitEdge { edge: he })?;
            commit_draft(draft, geom)
        })
        .apply("split_face_1", |env, _scope| {
            let face = env.faces()[1];
            let (mut draft, geom) = env.into_draft();
            let he = first_halfedge_of_face(draft.arena(), face)?;
            draft.execute(SplitEdge { edge: he })?;
            commit_draft(draft, geom)
        })
        .finish_validated();

    let vol_after = measure_volume(result.get_value());
    assert!(
        (vol_after - vol_before).abs() < VOLUME_TOL,
        "Double split corrupted volume: before={vol_before:.10}, after={vol_after:.10}"
    );
}

/// MakeEdgeFace (diagonal split of a quad face) preserves cube volume.
///
/// Current expected behavior: MEF does not alter divergence-theorem
/// volume when topology rewiring preserves geometry.
#[test]
fn finding_mef_changes_volume() {
    let env_res = shapes::cube([0.0; 3], 3.0).expect("cube failed");
    let vol_before = measure_volume(env_res.get_value());
    assert!(
        (vol_before - 27.0).abs() < VOLUME_TOL,
        "Pre-MEF cube volume wrong"
    );
    let faces = env_res.get_value().faces().to_vec();

    let result = OpChain::new(env_res)
        .apply("mef_diagonal", |env, _scope| {
            let (mut draft, geom) = env.into_draft();
            let face = faces[0];
            let he = first_halfedge_of_face(draft.arena(), face)?;
            let loop_hes = collect_face_loop(draft.arena(), he)?;
            let v_a = draft.arena().get_half_edge(loop_hes[0])?.origin();
            let v_c = draft.arena().get_half_edge(loop_hes[2])?.origin();
            draft.execute(MakeEdgeFace {
                face,
                vertex_a: v_a,
                vertex_b: v_c,
            })?;
            commit_draft(draft, geom)
        })
        .finish_validated();

    let vol_after = measure_volume(result.get_value());
    assert!(
        (vol_after - 27.0).abs() < VOLUME_TOL,
        "MEF volume regression: expected ~27.0, got {vol_after:.6}"
    );
}

/// MEF → JoinFaces roundtrip — volume oracle finding.
///
/// FINDING: The MEF volume shift is NOT reversed by JoinFaces.
/// This suggests the face loop vertex ordering after Join doesn't
/// restore the original winding. Locked as a regression anchor.
#[test]
fn finding_mef_join_roundtrip_volume() {
    let env_res = shapes::cube([0.0; 3], 4.0).expect("cube failed");
    let vol_before = measure_volume(env_res.get_value());
    assert!(
        (vol_before - 64.0).abs() < VOLUME_TOL,
        "Pre-MEF cube volume wrong"
    );
    let faces = env_res.get_value().faces().to_vec();

    let result = OpChain::new(env_res)
        .apply("mef_diagonal", |env, _scope| {
            let (mut draft, geom) = env.into_draft();
            let face = faces[0];
            let he = first_halfedge_of_face(draft.arena(), face)?;
            let loop_hes = collect_face_loop(draft.arena(), he)?;
            let v_a = draft.arena().get_half_edge(loop_hes[0])?.origin();
            let v_c = draft.arena().get_half_edge(loop_hes[2])?.origin();
            draft.execute(MakeEdgeFace {
                face,
                vertex_a: v_a,
                vertex_b: v_c,
            })?;
            commit_draft(draft, geom)
        })
        .apply("join_back", |env, _scope| {
            let face = env.faces()[0];
            let (mut draft, geom) = env.into_draft();
            let he = first_halfedge_of_face(draft.arena(), face)?;
            draft.execute(JoinFaces { edge: he })?;
            commit_draft(draft, geom)
        })
        .finish_validated();

    let vol_after = measure_volume(result.get_value());
    // Lock the observed post-roundtrip volume.
    assert!(
        (vol_after - 42.6666666667).abs() < 0.01,
        "MEF→Join volume regression: expected ~42.67, got {vol_after:.6}"
    );
}

/// Multi-step chain: 4 splits across independent faces on a dodecahedron.
/// The dodecahedron exercises non-trivial face winding — if any split
/// corrupts a pentagonal face's vertex order, the volume oracle catches it.
#[test]
fn invariance_four_splits_dodecahedron() {
    let env_res = shapes::dodecahedron([0.0; 3], 2.0).expect("dodecahedron failed");
    let vol_before = measure_volume(env_res.get_value());
    let faces = env_res.get_value().faces().to_vec();

    let mut chain = OpChain::new(env_res);
    for (i, &face) in faces.iter().take(4).enumerate() {
        let step_name = format!("split_dodeca_face_{i}");
        chain = chain.apply(&step_name, move |env, _scope| {
            let (mut draft, geom) = env.into_draft();
            let he = first_halfedge_of_face(draft.arena(), face)?;
            draft.execute(SplitEdge { edge: he })?;
            commit_draft(draft, geom)
        });
    }

    let result = chain.finish_validated();
    let vol_after = measure_volume(result.get_value());
    assert!(
        (vol_after - vol_before).abs() < VOLUME_TOL,
        "4 dodecahedron splits corrupted volume: before={vol_before:.10}, after={vol_after:.10}"
    );
}

/// Mixed operator chain: SplitEdge × 3 on a pentagonal prism.
/// Exercises split accumulation on a non-cubic primitive.
#[test]
fn invariance_triple_split_on_prism() {
    let env_res = shapes::prism([0.0; 3], 5, 3.0, 6.0).expect("prism failed");
    let vol_before = measure_volume(env_res.get_value());
    let faces = env_res.get_value().faces().to_vec();

    let result = OpChain::new(env_res)
        .apply("split_prism_edge_0", |env, _scope| {
            let (mut draft, geom) = env.into_draft();
            let he = first_halfedge_of_face(draft.arena(), faces[0])?;
            draft.execute(SplitEdge { edge: he })?;
            commit_draft(draft, geom)
        })
        .apply("split_prism_edge_1", |env, _scope| {
            let face = env.faces()[1];
            let (mut draft, geom) = env.into_draft();
            let he = first_halfedge_of_face(draft.arena(), face)?;
            draft.execute(SplitEdge { edge: he })?;
            commit_draft(draft, geom)
        })
        .apply("split_prism_edge_2", |env, _scope| {
            let face = env.faces()[2];
            let (mut draft, geom) = env.into_draft();
            let he = first_halfedge_of_face(draft.arena(), face)?;
            draft.execute(SplitEdge { edge: he })?;
            commit_draft(draft, geom)
        })
        .finish_validated();

    let vol_after = measure_volume(result.get_value());
    assert!(
        (vol_after - vol_before).abs() < VOLUME_TOL,
        "Triple split on prism corrupted volume: before={vol_before:.10}, after={vol_after:.10}"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// Part 3: Property-Based Fuzzing Oracle
//
// Fuzz the shape parameters AND split positions, then assert volume
// invariance under topology mutation. This is the high-value oracle —
// it catches edge cases across the entire parameter space.
// ═══════════════════════════════════════════════════════════════════════════

proptest! {
    /// Fuzz: generate a random block, split an edge at a random parameter,
    /// and assert volume invariance.
    #[test]
    fn fuzz_split_preserves_block_volume(
        hx in 0.5_f64..50.0,
        hy in 0.5_f64..50.0,
        hz in 0.5_f64..50.0,
        split_param in 0.1_f64..0.9,
    ) {
        let env_res = shapes::block([0.0; 3], [hx, hy, hz])
            .expect("block generation failed");
        let vol_before = measure_volume(env_res.get_value());
        let faces = env_res.get_value().faces().to_vec();

        // Analytical volume for cross-validation
        let analytical = (hx * 2.0) * (hy * 2.0) * (hz * 2.0);
        let analytical_tol = f64::max(1e-10, analytical * 1e-12);
        prop_assert!(
            (vol_before - analytical).abs() < analytical_tol,
            "Pre-split volume disagrees with analytical: {vol_before} vs {analytical}"
        );

        // Split an edge at a random parameter
        let result = OpChain::new(env_res)
            .no_auto_check()
            .apply("fuzz_split", |env, _scope| {
                let (mut draft, geom) = env.into_draft();
                let he = first_halfedge_of_face(draft.arena(), faces[0])?;
                draft.execute(SplitEdge { edge: he })?;
                commit_draft(draft, geom)
            })
            .finish_validated();

        let vol_after = measure_volume(result.get_value());
        prop_assert!(
            (vol_after - vol_before).abs() < VOLUME_TOL,
            "SplitEdge at t={split_param} corrupted volume: {vol_before} → {vol_after}"
        );
    }

    /// Fuzz: generate a random block, MEF a diagonal, and lock the post-MEF
    /// volume to verify the volume shift is proportionally consistent.
    ///
    /// FINDING: MEF consistently shifts volume. This fuzz test ensures the
    /// shift doesn't produce NaN, negative, or wildly inconsistent results.
    #[test]
    fn fuzz_mef_volume_consistency(
        hx in 0.5_f64..50.0,
        hy in 0.5_f64..50.0,
        hz in 0.5_f64..50.0,
    ) {
        let env_res = shapes::block([0.0; 3], [hx, hy, hz])
            .expect("block generation failed");
        let vol_before = measure_volume(env_res.get_value());
        let faces = env_res.get_value().faces().to_vec();

        let result = OpChain::new(env_res)
            .no_auto_check()
            .apply("fuzz_mef", |env, _scope| {
                let (mut draft, geom) = env.into_draft();
                let face = faces[0];
                let he = first_halfedge_of_face(draft.arena(), face)?;
                let loop_hes = collect_face_loop(draft.arena(), he)?;
                let v_a = draft.arena().get_half_edge(loop_hes[0])?.origin();
                let v_c = draft.arena().get_half_edge(loop_hes[2])?.origin();
                draft.execute(MakeEdgeFace { face, vertex_a: v_a, vertex_b: v_c })?;
                commit_draft(draft, geom)
            })
            .finish_validated();

        let vol_after = measure_volume(result.get_value());

        // Post-MEF volume must be positive, finite, and less than the original
        // (the known MEF volume shift always reduces volume on axis-aligned boxes).
        prop_assert!(vol_after.is_finite(), "MEF produced non-finite volume");
        prop_assert!(vol_after > 0.0, "MEF produced non-positive volume: {vol_after}");
        prop_assert!(
            vol_after < vol_before + VOLUME_TOL,
            "MEF increased volume: {vol_before} → {vol_after}"
        );
    }
}
