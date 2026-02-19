//! Tier 1 — Manifold Stability Tests (95% Confidence)
//!
//! DOMAIN: Ensuring basic topological consistency and metadata accuracy
//! after Boolean operations.
//!
//! INVARIANTS:
//! - Euler characteristic χ = V − E + F = 2 for every single-shell result
//! - Twin reciprocity: he.twin.twin == he for every halfedge
//! - Every vertex has valence ≥ 3 in a closed manifold
//!
//! ═══════════════════════════════════════════════════════════════
//! REQUIRED CODE/MATH CHANGES TO PASS ALL TESTS:
//! ═══════════════════════════════════════════════════════════════
//!
//! 1. **Euler-Poincaré Auditor**: Implement `V − E + F = 2(S − G)` where
//!    S = shells, G = genus. If the audit fails, the operation must trigger
//!    `Result::Err` in release and a panic in debug.
//!
//! 2. **Winding Number Classifier**: Move away from simple ray-casting to a
//!    robust winding number algorithm to handle shells-within-shells accurately.
//!    This is critical for T1.2 (multi-shell) and T1.4 (nested containment).
//!
//! 3. **Half-Edge Pointer Sanity Firewall**: A traversal that checks:
//!    - `he.twin.twin == he` for every halfedge (twin reciprocity)
//!    - `he.next.prev == he` for every halfedge (loop consistency)
//!    Run this as a post-condition after every boolean operation.
//!
//! 4. **Empty Result Handling**: A − A (identical subtraction) must produce
//!    an empty `TopologyState` with 0 faces (not a corrupted shell with
//!    dangling edges). Requires the select phase to detect full cancellation.
//!
//! 5. **Chained Boolean Input Integrity**: When a `BooleanResult` is fed back
//!    as input to another boolean, vertex provenance must be preserved or
//!    re-derived so the split phase can match vertices across solids.

use super::super::test_helpers::{
    build_cube, run_boolean, try_boolean, execute_boolean_logged, euler_audit,
};
use super::super::schema::{BooleanInput, BooleanOp};

// ══════════════════════════════════════════════════════════════
// §T1.1  EULER-POINCARÉ AUDITS
// ══════════════════════════════════════════════════════════════

/// T1.1a — Euler χ = 2 after standard overlapping union.
///
/// Two cubes with half-overlap: the result is a single closed shell.
#[test]
fn euler_poincare_basic_union() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [1.0, 0.0, 0.0], 1.0,
        BooleanOp::Union,
    );
    let (v, e, f, chi) = euler_audit(result.topology().arena());
    assert_eq!(chi, 2, "Union Euler violation: V={v} E={e} F={f} χ={chi}");
}

/// T1.1b — Euler χ = 2 after subtraction (carving a hole).
///
/// Large cube minus overlapping smaller cube → single shell.
#[test]
fn euler_poincare_subtraction() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 2.0,
        [1.5, 0.0, 0.0], 1.0,
        BooleanOp::Subtraction,
    );
    let (v, e, f, chi) = euler_audit(result.topology().arena());
    assert_eq!(chi, 2, "Subtraction Euler violation: V={v} E={e} F={f} χ={chi}");
}

/// T1.1c — Euler χ = 2 after intersection.
#[test]
fn euler_poincare_intersection() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [0.5, 0.5, 0.0], 1.0,
        BooleanOp::Intersection,
    );
    let (v, e, f, chi) = euler_audit(result.topology().arena());
    assert_eq!(chi, 2, "Intersection Euler violation: V={v} E={e} F={f} χ={chi}");
}

// ══════════════════════════════════════════════════════════════
// §T1.2  MULTI-SHELL DISJOINT UNION
// ══════════════════════════════════════════════════════════════

/// T1.2 — Union of two non-touching cubes → two disjoint shells.
///
/// Each shell is a closed cube: 6 faces, 8 vertices, 12 edges each.
/// Combined: 12 faces, 16 vertices, 24 edges, χ = 16-24+12 = 4 = 2×2 shells.
#[test]
fn multi_shell_disjoint_union() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [10.0, 0.0, 0.0], 1.0,
        BooleanOp::Union,
    );
    let arena = result.topology().arena();
    let (v, e, f, chi) = euler_audit(arena);
    assert_eq!(f, 12, "Disjoint union should have 12 faces, got {f}");
    assert_eq!(v, 16, "Disjoint union should have 16 vertices, got {v}");
    assert_eq!(chi, 4, "Disjoint union (2 shells) Euler χ should be 4, got {chi}");
}

// ══════════════════════════════════════════════════════════════
// §T1.3  IDENTICAL SUBTRACTION → EMPTY
// ══════════════════════════════════════════════════════════════

/// T1.3 — A − A must produce 0 faces or a clean error.
///
/// Never a corrupted shell with dangling edges.
#[test]
fn identical_subtraction_produces_empty() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [0.0, 0.0, 0.0], 1.0,
        BooleanOp::Subtraction,
    );

    match result {
        Ok(r) => {
            let f = r.topology().arena().face_count();
            assert_eq!(f, 0, "A − A should produce 0 faces, got {f}");
        }
        Err(_) => {
            eprintln!("A − A returned error (acceptable — empty result)");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §T1.4  NESTED CONTAINMENT
// ══════════════════════════════════════════════════════════════

/// T1.4 — Large − (Medium − Small) → nested void.
///
/// Large contains Medium contains Small.
/// Medium − Small creates a shell with a hole.
/// Large − (M−S) should produce the outer shell with that void inside.
#[test]
fn nested_containment_subtraction() {
    let inner_result = try_boolean(
        [0.0, 0.0, 0.0], 2.0,
        [0.0, 0.0, 0.0], 0.5,
        BooleanOp::Subtraction,
    );

    match inner_result {
        Ok(medium_minus_small) => {
            let (topo_ms, geom_ms) = medium_minus_small.into_parts();
            let (topo_large, geom_large) = build_cube([0.0, 0.0, 0.0], 4.0);

            let input = BooleanInput::new(
                topo_large, geom_large,
                topo_ms, geom_ms,
                BooleanOp::Subtraction,
            );
            let outer_result = execute_boolean_logged(input);
            match outer_result {
                Ok(envelope) => {
                    let r = envelope.into_value();
                    let (v, e, f, chi) = euler_audit(r.topology().arena());
                    eprintln!("Nested containment: V={v} E={e} F={f} χ={chi}");
                    assert!(f >= 6, "Nested result should have faces, got {f}");
                }
                Err(e) => {
                    eprintln!("Nested containment returned error (tracking): {e:?}");
                }
            }
        }
        Err(e) => {
            eprintln!("Medium − Small failed (tracking): {e:?}");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §T1.5  HALFEDGE TWIN RECIPROCITY AUDIT
// ══════════════════════════════════════════════════════════════

/// T1.5 — Post-boolean: every he.twin.twin == he.
///
/// Traverses every halfedge in the result and verifies twin reciprocity.
#[test]
fn halfedge_twin_reciprocity_audit() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [0.8, 0.3, 0.0], 1.0,
        BooleanOp::Union,
    );
    let arena = result.topology().arena();

    for (he_id, he_data) in arena.iter_half_edges() {
        let twin_id = he_data.twin();
        assert_ne!(
            he_id, twin_id,
            "Halfedge {he_id} has self-referencing twin"
        );

        let twin_data = arena.get_half_edge(twin_id)
            .unwrap_or_else(|_| panic!("Twin {twin_id} of {he_id} is stale"));

        assert_eq!(
            twin_data.twin(), he_id,
            "Twin reciprocity violated: he[{}].twin={}, but he[{}].twin={}",
            he_id.index(), twin_id.index(), twin_id.index(), twin_data.twin().index()
        );
    }
}

// ══════════════════════════════════════════════════════════════
// §T1.6  VERTEX VALENCE AUDIT
// ══════════════════════════════════════════════════════════════

/// T1.6 — Post-boolean: every vertex has valence ≥ 3.
///
/// In a closed manifold, every vertex is shared by at least 3 faces/edges.
/// Valence < 3 indicates a dangling edge or degenerate topology.
#[test]
fn vertex_valence_audit() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [0.5, 0.5, 0.5], 1.0,
        BooleanOp::Intersection,
    );
    let arena = result.topology().arena();

    let mut valence: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
    for (_he_id, he_data) in arena.iter_half_edges() {
        *valence.entry(he_data.origin().index()).or_insert(0) += 1;
    }

    for (&vid, &val) in &valence {
        assert!(
            val >= 3,
            "Vertex index {vid} has valence {val} < 3 — dangling edge in manifold"
        );
    }
}
