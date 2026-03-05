//! Container lifecycle operator regression tests.
//!
//! DOMAIN: Tests for Tier 1 hardening of body, lump, and shell
//! container operators — guards and validation fixes.
//!
//! Each test targets a specific bug found in the Gemini architectural review.

use crate::b_rep::ShellKind;
use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
use crate::entity_lifecycle::split_edge::SplitEdge;
use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;
use crate::lifecycle::solid::MakeSolid;
use crate::lifecycle::lump::MakeLumpRegion;
use crate::lifecycle::shell::MakeEmptyShell;
use crate::lifecycle::shell::DestroyShell;
use crate::lifecycle::shell_ops::{
    MergeShells, SplitShell, ExtractShell,
};
use crate::lifecycle::lump_ops::MergeLumps;
use crate::transactions::TopologyState;

// ── MergeLumps ──────────────────────────────────────────────────────

/// MergeLumps must reject source and target from different bodies.
///
/// BUG: Without this guard, topology silently migrates across bodies,
/// corrupting spatial indexes and ownership invariants.
#[test]
fn merge_lumps_rejects_cross_body() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let body_a = draft.execute(MakeSolid).unwrap().into_value();
    let body_b = draft.execute(MakeSolid).unwrap().into_value();

    // Add a second lump to body_a so the merge target isn't the last lump
    let extra = draft.execute(MakeLumpRegion { body: body_a.body }).unwrap().into_value();

    let result = draft.execute(MergeLumps {
        target: body_a.lump,
        source: body_b.lump,
    });

    assert!(
        result.is_err(),
        "MergeLumps must reject lumps from different bodies"
    );
    let _ = extra; // silence unused
}

/// MergeLumps succeeds when source and target belong to the same body.
#[test]
fn merge_lumps_succeeds_same_body() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let solid = draft.execute(MakeSolid).unwrap().into_value();
    let extra = draft.execute(MakeLumpRegion { body: solid.body }).unwrap().into_value();

    let result = draft.execute(MergeLumps {
        target: solid.lump,
        source: extra.lump,
    });

    assert!(
        result.is_ok(),
        "MergeLumps must succeed for lumps in the same body"
    );
}

// ── MergeShells ─────────────────────────────────────────────────────

/// MergeShells must reject source and target from different regions.
///
/// BUG: Without this guard, faces silently migrate across region
/// boundaries, breaking point-containment logic.
#[test]
fn merge_shells_rejects_cross_region() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let solid = draft.execute(MakeSolid).unwrap().into_value();

    // Create shells in the same region — first is outer (via add_shell auto-promotion)
    let shell_a = draft.execute(MakeEmptyShell {
        region: solid.region,
        kind: ShellKind::Sheet,
    }).unwrap().into_value();
    let shell_b = draft.execute(MakeEmptyShell {
        region: solid.region,
        kind: ShellKind::Sheet,
    }).unwrap().into_value();

    // Create a second lump+region
    let extra = draft.execute(MakeLumpRegion { body: solid.body }).unwrap().into_value();
    let shell_c = draft.execute(MakeEmptyShell {
        region: extra.region,
        kind: ShellKind::Sheet,
    }).unwrap().into_value();

    // Merging shells from different regions must fail
    let result = draft.execute(MergeShells {
        target: shell_a.shell,
        source: shell_c.shell,
    });

    assert!(
        result.is_err(),
        "MergeShells must reject shells from different regions"
    );

    let _ = shell_b;
}

// ── SplitShell — representative_face ────────────────────────────────

/// SplitShell must update the source shell's representative_face
/// when the representative face is moved to the new shell.
///
/// BUG: Without this fix, the source shell's representative_face
/// points to a face it no longer owns — a dangling reference.
#[test]
fn split_shell_updates_representative_face() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    // Build a two-face shell: MVF → SE → MEF
    let mvf = draft.execute(MakeVertexFace { shell_kind: ShellKind::Sheet }).unwrap().into_value();
    let se = draft.execute(SplitEdge { edge: mvf.half_edge }).unwrap().into_value();
    let mef = draft.execute(MakeEdgeFace {
        vertex_a: mvf.vertex,
        vertex_b: se.new_vertex,
        face: mvf.face,
    }).unwrap().into_value();

    let shell = draft.arena().get_face(mvf.face).unwrap().shell();
    let original_rep = draft.arena().get_shell(shell).unwrap().representative_face();

    // Determine which face to move — whichever is the representative
    let face_to_move = if original_rep == mvf.face {
        mvf.face
    } else {
        mef.new_face
    };

    let split_result = draft.execute(SplitShell {
        shell,
        faces_to_move: vec![face_to_move],
    }).unwrap().into_value();

    // The source shell's representative_face must NOT be the moved face
    let new_rep = draft.arena().get_shell(shell).unwrap().representative_face();
    assert_ne!(
        new_rep, face_to_move,
        "Source shell's representative_face must not point to a moved face"
    );

    // The new shell should own the moved face
    let moved_face_shell = draft.arena().get_face(face_to_move).unwrap().shell();
    assert_eq!(
        moved_face_shell, split_result.new_shell,
        "Moved face must belong to the new shell"
    );
}

// ── ExtractShell ────────────────────────────────────────────────────

/// ExtractShell must reject extracting the outer shell when no inner
/// shell can replace it, leaving the region with an undefined boundary.
///
/// BUG: Without this guard, extracting the outer shell leaves the
/// region with `None` for its outer shell — a mathematically infinite region.
#[test]
fn extract_shell_rejects_sole_outer() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let solid = draft.execute(MakeSolid).unwrap().into_value();
    let shell = draft.execute(MakeEmptyShell {
        region: solid.region,
        kind: ShellKind::Sheet,
    }).unwrap().into_value();

    // shell.shell is the outer shell (first one added auto-promotes to outer)
    let result = draft.execute(ExtractShell {
        shell: shell.shell,
    });

    assert!(
        result.is_err(),
        "ExtractShell must reject outer shell with no inner shells to replace it"
    );
}

// ── DestroyShell — wire guard ───────────────────────────────────────

/// DestroyShell must reject wire-kind shells to prevent memory leaks.
///
/// BUG: Wire shells have 0 faces, so they bypass the face_count > 0
/// safety check. Destroying them leaks all their edges and vertices.
#[test]
fn destroy_shell_rejects_wire() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let solid = draft.execute(MakeSolid).unwrap().into_value();
    let shell = draft.execute(MakeEmptyShell {
        region: solid.region,
        kind: ShellKind::Wire,
    }).unwrap().into_value();

    let result = draft.execute(DestroyShell {
        shell: shell.shell,
    });

    assert!(
        result.is_err(),
        "DestroyShell must reject wire shells"
    );
}
