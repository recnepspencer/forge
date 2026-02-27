//! Performance & Scalability Tests
//!
//! DOMAIN: Ensuring the boolean engine doesn't degrade catastrophically
//! under heavy load, tight spatial clusters, or repeated operations.
//!
//! INVARIANTS:
//! - No panics under load
//! - Deterministic results under repetition
//! - Reasonable completion time (not O(death))
//!
//! ═══════════════════════════════════════════════════════════════
//! REQUIRED CODE/MATH CHANGES TO PASS ALL TESTS:
//! ═══════════════════════════════════════════════════════════════
//!
//! 1. **AABB / BVH Optimization**: Already implemented in `forge-geom`.
//!    The split phase uses BVH to find overlapping face pairs. However,
//!    P.1 (cluster pressure) stresses this with 20 cubes in a tight region
//!    where almost ALL face pairs overlap — the BVH doesn't help much and
//!    the quadratic fallback dominates. Consider spatial hashing for dense
//!    clusters.
//!
//! 2. **Lazy Evaluation Signal Graph**: Use `forge-signal` to cache
//!    intermediate "split" results. If the user moves a cube, only
//!    re-calculate the faces whose bounding boxes are "dirty."
//!    Not yet wired into the boolean pipeline.
//!
//! 3. **Thread-Safe Topology Arena**: Ensure `VertexId` and `FaceId`
//!    generation is monotonic and independent of thread execution order.
//!    Currently uses sequential arena insertion (single-threaded), so P.2
//!    (determinism) passes trivially. Needs atomic counters or pre-allocation
//!    for parallel execution.
//!
//! 4. **Collection Ordering Determinism**: No `HashMap`/`HashSet` iteration
//!    order may affect results (per architecture.md §5.3). Use `BTreeMap`
//!    or sort-on-output. Already partially enforced but BVH pair ordering
//!    and split queue ordering need auditing.
//!
//! 5. **Chained Boolean Scalability**: P.1 and P.4 chain 8-20 boolean ops.
//!    Same root cause as deep_chains — vertex provenance loss and spatial
//!    weld tolerance. Fixing those unblocks these performance tests too.

use super::super::schema::{BooleanInput, BooleanOp};
use super::super::test_helpers::{build_cube, euler_audit, execute_boolean_logged, try_boolean};
use forge_topo::hashing::compute_arena_topology_hash;

// ══════════════════════════════════════════════════════════════
// §P.1  BOUNDING BOX CLUSTER PRESSURE
// ══════════════════════════════════════════════════════════════

/// P.1 — 20 overlapping cubes in a tight spatial cluster.
///
/// All cubes overlap in a small region. This pressures the BVH
/// and split phases with many face-face intersection candidates.
#[test]
fn bounding_box_cluster_pressure() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 1.0);

    for i in 1..20 {
        let offset = i as f64 * 0.15;
        let (topo_tool, geom_tool) = build_cube([offset, offset * 0.5, 0.0], 1.0);

        let input = BooleanInput::new(topo, geom, topo_tool, geom_tool, BooleanOp::Union);

        match execute_boolean_logged(input).into_result() {
            Ok(result) => {
                let r = result;
                let (v, e, f, chi) = euler_audit(r.topology().arena());
                eprintln!("Cluster step {i}: V={v} E={e} F={f} χ={chi}");
                assert_eq!(
                    chi, 2,
                    "Cluster step {i} Euler violation: V={v} E={e} F={f}"
                );
                let parts = r.into_states();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                panic!("Cluster step {i} failed: {e}");
            }
        }
    }

    let final_f = topo.arena().face_count();
    eprintln!("Cluster final: {final_f} faces");
    assert!(final_f >= 6, "Cluster result should have faces");
}

// ══════════════════════════════════════════════════════════════
// §P.2  REPLAY DETERMINISM STRESS
// ══════════════════════════════════════════════════════════════

/// P.2 — Same boolean operation 50×, all topology hashes must match.
///
/// Tests that no non-determinism creeps in from HashMap iteration,
/// thread scheduling, or floating-point non-associativity.
#[test]
fn replay_determinism_stress() {
    let first = try_boolean([0.0, 0.0, 0.0], 1.0, [0.7, 0.3, 0.2], 1.0, BooleanOp::Union)
        .expect("Reference boolean must succeed");

    let reference_hash = compute_arena_topology_hash(first.topology().arena());
    let reference_faces = first.topology().arena().face_count();

    for iter in 1..50 {
        let r = try_boolean([0.0, 0.0, 0.0], 1.0, [0.7, 0.3, 0.2], 1.0, BooleanOp::Union)
            .unwrap_or_else(|e| panic!("Replay iteration {iter} failed: {e}"));

        let hash = compute_arena_topology_hash(r.topology().arena());
        let faces = r.topology().arena().face_count();

        assert_eq!(
            hash, reference_hash,
            "Replay iteration {iter}: hash diverged! {hash:#x} vs {reference_hash:#x}"
        );
        assert_eq!(
            faces, reference_faces,
            "Replay iteration {iter}: face count diverged! {faces} vs {reference_faces}"
        );
    }
}

// ══════════════════════════════════════════════════════════════
// §P.3  LARGE FACE COUNT FROM MULTI-SPLIT
// ══════════════════════════════════════════════════════════════

/// P.3 — Operations that produce many split faces.
///
/// A cube intersected with a slightly rotated cube at an offset produces
/// many more faces than the original 6+6. Verifies the engine handles
/// the resulting complexity without degradation.
#[test]
fn large_face_count_union() {
    let result = try_boolean([0.0, 0.0, 0.0], 2.0, [1.0, 0.3, 0.7], 2.0, BooleanOp::Union);

    match result {
        Ok(r) => {
            let (v, e, f, chi) = euler_audit(r.topology().arena());
            eprintln!("Large face count: V={v} E={e} F={f} χ={chi}");
            assert_eq!(chi, 2, "Large face count Euler violation");
            assert!(f >= 6, "Should have at least 6 faces, got {f}");
        }
        Err(e) => {
            panic!("Large face count test failed: {e}");
        }
    }
}

/// P.3b — Subtraction version: high split count from asymmetric overlap.
#[test]
fn large_face_count_subtraction() {
    let result = try_boolean(
        [0.0, 0.0, 0.0],
        2.0,
        [1.0, 0.3, 0.7],
        2.0,
        BooleanOp::Subtraction,
    );

    match result {
        Ok(r) => {
            let (v, e, f, chi) = euler_audit(r.topology().arena());
            eprintln!("Large subtraction: V={v} E={e} F={f} χ={chi}");
            assert_eq!(chi, 2, "Large subtraction Euler violation");
        }
        Err(e) => {
            panic!("Large subtraction test failed: {e}");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §P.4  MULTI-DIRECTION CLUSTER
// ══════════════════════════════════════════════════════════════

/// P.4 — 8 cubes arranged in all octant directions.
///
/// Creates a "star" shape by unioning cubes at each corner.
/// Tests spatial indexing and classification with complex adjacency.
#[test]
fn octant_cluster_union() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 1.0);

    let offsets: &[[f64; 3]] = &[
        [0.8, 0.8, 0.8],
        [-0.8, 0.8, 0.8],
        [0.8, -0.8, 0.8],
        [0.8, 0.8, -0.8],
        [-0.8, -0.8, 0.8],
        [-0.8, 0.8, -0.8],
        [0.8, -0.8, -0.8],
        [-0.8, -0.8, -0.8],
    ];

    for (i, offset) in offsets.iter().enumerate() {
        let (topo_tool, geom_tool) = build_cube(*offset, 0.5);

        let input = BooleanInput::new(topo, geom, topo_tool, geom_tool, BooleanOp::Union);

        match execute_boolean_logged(input).into_result() {
            Ok(result) => {
                let r = result;
                let (v, e, f, chi) = euler_audit(r.topology().arena());
                eprintln!("Octant step {i}: V={v} E={e} F={f} χ={chi}");
                assert_eq!(chi, 2, "Octant step {i} Euler violation");
                let parts = r.into_states();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                panic!("Octant step {i} failed: {e}");
            }
        }
    }

    let final_f = topo.arena().face_count();
    eprintln!("Octant final: {final_f} faces");
    assert!(final_f >= 6, "Octant result should have faces");
}
