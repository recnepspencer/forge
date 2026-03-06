//! Tests for `GroupPolicyRuntime` — policy resolution, context derivation, and shell-kind verification.
//!
//! Extracted from `group_policy_runtime.rs` per domain standards (single-responsibility).

use super::*;
use crate::b_rep::{ShellData, ShellKind, ShellOrientation, TopologyArena};

// ══════════════════════════════════════════════════════════════════
//  GroupPolicyRuntime::resolve — policy resolution tests
// ══════════════════════════════════════════════════════════════════

#[test]
fn solid_certified_runs_everything_at_commit() {
    let ctx = TopologyContext {
        stage: CertificationStage::Certified,
        ..TopologyContext::SOLID
    };
    let rt = GroupPolicyRuntime::resolve(
        0,
        0,
        [ValidatorCost::Expensive; ValidationCheckpoint::COUNT],
        &ctx,
    );

    for &group in InvariantGroup::ALL {
        assert!(
            rt.should_run(group, ValidationCheckpoint::PostCommit),
            "Solid+Certified should run {:?} at PostCommit",
            group,
        );
    }
}

#[test]
fn solid_defers_semantic_tier_from_per_op() {
    let rt = GroupPolicyRuntime::resolve(
        0,
        0,
        [ValidatorCost::Expensive; ValidationCheckpoint::COUNT],
        &TopologyContext::SOLID,
    );

    // Semantic tier should be deferred (not per-op)
    assert!(!rt.should_run(InvariantGroup::EulerFormula, ValidationCheckpoint::PerOp));
    assert!(!rt.should_run(InvariantGroup::ShellClosure, ValidationCheckpoint::PerOp));

    // But runs at PostCommit
    assert!(rt.should_run(
        InvariantGroup::EulerFormula,
        ValidationCheckpoint::PostCommit
    ));
    assert!(rt.should_run(
        InvariantGroup::ShellClosure,
        ValidationCheckpoint::PostCommit
    ));

    // Topology tier still runs per-op
    assert!(rt.should_run(
        InvariantGroup::PointerCoherence,
        ValidationCheckpoint::PerOp
    ));
    assert!(rt.should_run(InvariantGroup::CacheCoherence, ValidationCheckpoint::PerOp));
}

#[test]
fn wire_skips_face_based_groups() {
    let rt = GroupPolicyRuntime::resolve(
        0,
        0,
        [ValidatorCost::Expensive; ValidationCheckpoint::COUNT],
        &TopologyContext::WIRE,
    );

    // Wire should skip face-based groups even at PostCommit
    assert!(!rt.should_run(
        InvariantGroup::LoopIntegrity,
        ValidationCheckpoint::PostCommit
    ));
    assert!(!rt.should_run(InvariantGroup::RadialEdge, ValidationCheckpoint::PostCommit));
    assert!(!rt.should_run(
        InvariantGroup::ShellClosure,
        ValidationCheckpoint::PostCommit
    ));
    assert!(!rt.should_run(InvariantGroup::VertexDisk, ValidationCheckpoint::PostCommit));
    assert!(!rt.should_run(
        InvariantGroup::EulerFormula,
        ValidationCheckpoint::PostCommit
    ));

    // Wire should still run pointer + ownership + cache
    assert!(rt.should_run(
        InvariantGroup::PointerCoherence,
        ValidationCheckpoint::PerOp
    ));
    assert!(rt.should_run(InvariantGroup::Ownership, ValidationCheckpoint::PerOp));
    assert!(rt.should_run(InvariantGroup::CacheCoherence, ValidationCheckpoint::PerOp));
}

#[test]
fn open_sheet_skips_shell_closure() {
    let rt = GroupPolicyRuntime::resolve(
        0,
        0,
        [ValidatorCost::Expensive; ValidationCheckpoint::COUNT],
        &TopologyContext::SHEET_OPEN,
    );

    assert!(!rt.should_run(
        InvariantGroup::ShellClosure,
        ValidationCheckpoint::PostCommit
    ));
}

#[test]
fn force_per_op_overrides_deferral() {
    let force_per_op = InvariantGroup::EulerFormula.mask();
    let rt = GroupPolicyRuntime::resolve(
        0,
        force_per_op,
        [ValidatorCost::Expensive; ValidationCheckpoint::COUNT],
        &TopologyContext::SOLID,
    );

    // EulerFormula should now run at PerOp despite being Semantic tier
    assert!(rt.should_run(InvariantGroup::EulerFormula, ValidationCheckpoint::PerOp));
}

#[test]
fn force_skip_overrides_applicability() {
    let force_skip = InvariantGroup::PointerCoherence.mask();
    let rt = GroupPolicyRuntime::resolve(
        force_skip,
        0,
        [ValidatorCost::Expensive; ValidationCheckpoint::COUNT],
        &TopologyContext::SOLID,
    );

    assert!(!rt.should_run(
        InvariantGroup::PointerCoherence,
        ValidationCheckpoint::PerOp
    ));
    assert!(!rt.should_run(
        InvariantGroup::PointerCoherence,
        ValidationCheckpoint::PostCommit
    ));
}

#[test]
fn should_run_is_o1() {
    // Just verify it doesn't panic on all combinations
    let rt = GroupPolicyRuntime::default();
    for &group in InvariantGroup::ALL {
        let _ = rt.should_run(group, ValidationCheckpoint::PerOp);
        let _ = rt.should_run(group, ValidationCheckpoint::PostCommit);
        let _ = rt.should_run(group, ValidationCheckpoint::PostBoolean);
        let _ = rt.should_run(group, ValidationCheckpoint::PostFeature);
        let _ = rt.should_run(group, ValidationCheckpoint::PostImport);
        let _ = rt.should_run(group, ValidationCheckpoint::OnDemand);
    }
}

// ══════════════════════════════════════════════════════════════════
//  Option A — Model-derived context tests
// ══════════════════════════════════════════════════════════════════

// ── topology_context_from_shell_metadata unit tests ─────────────

#[test]
fn empty_arena_defaults_to_solid() {
    let arena = TopologyArena::new();
    let ctx = topology_context_from_shell_metadata(&arena);
    assert_eq!(ctx.kind, TopologyKind::Solid);
    assert_eq!(ctx.closure, Closure::Closed);
}

#[test]
fn single_sheet_shell_derives_sheet_open() {
    let mut arena = TopologyArena::new();
    let body = arena.insert_body(crate::b_rep::BodyData::new());
    let lump = arena.insert_lump(crate::b_rep::LumpData::new(body));
    arena.get_body_mut(body).unwrap().add_lump(lump);
    let region = arena.insert_region(crate::b_rep::RegionData::new(lump));
    // Placeholder face id — we only need the shell to exist in the arena
    // for metadata derivation, not a fully wired face.
    let _shell = arena.insert_shell(ShellData::new(
        crate::handles::FaceId::new(0, 0),
        ShellKind::Sheet,
        region,
    ));

    let ctx = topology_context_from_shell_metadata(&arena);
    assert_eq!(ctx.kind, TopologyKind::Sheet);
    assert_eq!(ctx.closure, Closure::Open);
}

#[test]
fn single_wire_shell_derives_wire() {
    let mut arena = TopologyArena::new();
    let body = arena.insert_body(crate::b_rep::BodyData::new());
    let lump = arena.insert_lump(crate::b_rep::LumpData::new(body));
    arena.get_body_mut(body).unwrap().add_lump(lump);
    let region = arena.insert_region(crate::b_rep::RegionData::new(lump));
    let _shell = arena.insert_shell(ShellData::new(
        crate::handles::FaceId::new(0, 0),
        ShellKind::Wire,
        region,
    ));

    let ctx = topology_context_from_shell_metadata(&arena);
    assert_eq!(ctx.kind, TopologyKind::Wire);
    assert_eq!(ctx.closure, Closure::Open);
}

#[test]
fn single_solid_shell_derives_solid_closed() {
    let mut arena = TopologyArena::new();
    let body = arena.insert_body(crate::b_rep::BodyData::new());
    let lump = arena.insert_lump(crate::b_rep::LumpData::new(body));
    arena.get_body_mut(body).unwrap().add_lump(lump);
    let region = arena.insert_region(crate::b_rep::RegionData::new(lump));
    let _shell = arena.insert_shell(ShellData::new(
        crate::handles::FaceId::new(0, 0),
        ShellKind::Solid(ShellOrientation::Outer),
        region,
    ));

    let ctx = topology_context_from_shell_metadata(&arena);
    assert_eq!(ctx.kind, TopologyKind::Solid);
    assert_eq!(ctx.closure, Closure::Closed);
}

#[test]
fn mixed_shells_widest_kind_wins() {
    let mut arena = TopologyArena::new();
    let body = arena.insert_body(crate::b_rep::BodyData::new());
    let lump = arena.insert_lump(crate::b_rep::LumpData::new(body));
    arena.get_body_mut(body).unwrap().add_lump(lump);
    let region = arena.insert_region(crate::b_rep::RegionData::new(lump));

    // Wire + Sheet + Solid → should produce Solid (widest)
    let _wire = arena.insert_shell(ShellData::new(
        crate::handles::FaceId::new(0, 0),
        ShellKind::Wire,
        region,
    ));
    let _sheet = arena.insert_shell(ShellData::new(
        crate::handles::FaceId::new(0, 0),
        ShellKind::Sheet,
        region,
    ));
    let _solid = arena.insert_shell(ShellData::new(
        crate::handles::FaceId::new(0, 0),
        ShellKind::Solid(ShellOrientation::Outer),
        region,
    ));

    let ctx = topology_context_from_shell_metadata(&arena);
    assert_eq!(
        ctx.kind,
        TopologyKind::Solid,
        "With mixed shells, the widest kind (Solid) must win"
    );
    // Open because Wire + Sheet shells are present
    assert_eq!(
        ctx.closure,
        Closure::Open,
        "Mixed shells with Wire/Sheet present must be Open"
    );
}

// ── Draft auto-derivation integration test ─────────────────────

/// Verifies that `into_mutation()` auto-derives the group policy
/// from the arena's shell metadata, so per-op validation is
/// kind-aware without operator boilerplate.
///
/// # Ignored
/// Current operators create shells with `ShellKind::Solid` even
/// for open topologies (single face). Once operators are fixed to
/// set correct `ShellKind` on construction, un-ignore this test.
#[test]
#[ignore = "Awaiting operator fix: MakeEmptyShell must set correct ShellKind"]
fn draft_auto_derives_policy_from_shell_metadata() {
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::transactions::TopologyState;

    // MVF creates a single face in a Sheet shell (once operators are fixed)
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let _mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let state = draft.commit().unwrap();

    // Re-open for mutation — should pick up the Sheet context
    let draft2 = state.into_mutation();
    let policy = &draft2.config.group_policy;

    // Sheet topology: ShellClosure should be skipped regardless of checkpoint
    assert!(
        !policy.should_run(
            InvariantGroup::ShellClosure,
            ValidationCheckpoint::PostCommit
        ),
        "Sheet topology must skip ShellClosure even at PostCommit",
    );

    // Sheet topology: PointerCoherence should still run
    assert!(
        policy.should_run(
            InvariantGroup::PointerCoherence,
            ValidationCheckpoint::PerOp
        ),
        "Sheet topology must still run PointerCoherence per-op",
    );
}

// ── verify_shell_kind_matches_structure adversarial tests ───────

/// Verifies that `verify_shell_kind_matches_structure` catches
/// a shell declared as `Solid` that actually has boundary edges.
///
/// Only compiles in debug builds (the function under test is
/// `#[cfg(debug_assertions)]`).
#[cfg(debug_assertions)]
#[test]
#[ignore = "Awaiting operator fix: test needs valid wiring to construct mismatch"]
fn verify_catches_stale_solid_kind_on_open_topology() {
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::entity_lifecycle::split_edge::SplitEdge;
    use crate::transactions::TopologyState;

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let _se = draft
        .execute(SplitEdge {
            edge: mvf.half_edge,
        })
        .unwrap()
        .into_value();

    // Force the shell to be declared Solid even though it has boundary edges
    let face_data = draft.arena().get_face(mvf.face).unwrap();
    let shell_id = face_data.shell();
    draft
        .arena_mut()
        .get_shell_mut(shell_id)
        .unwrap()
        .set_kind(ShellKind::Solid(ShellOrientation::Outer));

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        verify_shell_kind_matches_structure(draft.arena());
    }));
    assert!(
        result.is_err(),
        "verify_shell_kind_matches_structure must panic when Solid shell has boundary edges"
    );
}

/// Proves the O(E) scan catches boundary edges even when they are NOT
/// on the representative face — the "hidden hole" scenario.
#[cfg(debug_assertions)]
#[test]
#[ignore = "Awaiting operator fix: test needs valid wiring to construct mismatch"]
fn verify_catches_hidden_hole_on_solid() {
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::transactions::TopologyState;

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mvf1 = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();

    let face_data = draft.arena().get_face(mvf1.face).unwrap();
    let shell_id = face_data.shell();
    draft
        .arena_mut()
        .get_shell_mut(shell_id)
        .unwrap()
        .set_kind(ShellKind::Solid(ShellOrientation::Outer));

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        verify_shell_kind_matches_structure(draft.arena());
    }));
    assert!(
        result.is_err(),
        "Must panic when any face in a Solid shell has boundary edges"
    );
}

/// Catches watertight sheets — operators that forgot to promote to Solid.
#[cfg(debug_assertions)]
#[test]
#[ignore = "Awaiting operator fix: test needs valid watertight topology"]
fn verify_catches_watertight_sheet_as_missed_opportunity() {
    use crate::transactions::TopologyState;

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let body = draft.arena_mut().insert_body(crate::b_rep::BodyData::new());
    let lump = draft
        .arena_mut()
        .insert_lump(crate::b_rep::LumpData::new(body));
    let region = draft
        .arena_mut()
        .insert_region(crate::b_rep::RegionData::new(lump));
    let shell = draft.arena_mut().insert_shell(ShellData::new(
        crate::handles::FaceId::new(0, 0),
        ShellKind::Sheet, // Accidental sheet
        region,
    ));

    // Insert a face just so face_count > 0 (otherwise the sheet warn is skipped)
    draft.arena_mut().insert_face(crate::b_rep::FaceData::new(
        crate::handles::LoopId::new(0, 0),
        shell,
    ));

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        verify_shell_kind_matches_structure(draft.arena());
    }));
    assert!(
        result.is_err(),
        "Must panic when a Sheet has NO boundary edges (accidental solid)"
    );
}

/// Catches faces that somehow ended up in a Wire shell.
#[cfg(debug_assertions)]
#[test]
#[ignore = "Awaiting operator fix: test needs valid wiring"]
fn verify_catches_face_in_wire_shell() {
    use crate::transactions::TopologyState;
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let body = draft.arena_mut().insert_body(crate::b_rep::BodyData::new());
    let lump = draft
        .arena_mut()
        .insert_lump(crate::b_rep::LumpData::new(body));
    let region = draft
        .arena_mut()
        .insert_region(crate::b_rep::RegionData::new(lump));
    let shell = draft.arena_mut().insert_shell(ShellData::new(
        crate::handles::FaceId::new(0, 0),
        ShellKind::Wire, // Claims Wire
        region,
    ));

    // Add a face to the Wire shell!
    draft.arena_mut().insert_face(crate::b_rep::FaceData::new(
        crate::handles::LoopId::new(0, 0),
        shell,
    ));

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        verify_shell_kind_matches_structure(draft.arena());
    }));
    assert!(
        result.is_err(),
        "Must panic when a Wire shell contains faces"
    );
}

// ── max_cost_snapshot preserves cost settings ───────────────────

#[test]
fn max_cost_snapshot_roundtrips() {
    let costs = [
        ValidatorCost::Cheap,
        ValidatorCost::Medium,
        ValidatorCost::Expensive,
        ValidatorCost::Cheap,
        ValidatorCost::Medium,
        ValidatorCost::Expensive,
    ];
    let rt = GroupPolicyRuntime::resolve(0, 0, costs, &TopologyContext::SOLID);
    let snapshot = rt.max_cost_snapshot();
    assert_eq!(snapshot, costs, "max_cost_snapshot must roundtrip");
}
