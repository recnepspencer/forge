use super::*;
use crate::lineage::{LineageEvent, OpSignature};
// =====================================================================
// SECTION 4: Defect regression tests (D1, D3, D4, D5)
// =====================================================================

/// D1 regression: execute_sheet_region_merge must return committed KernelState.
///
/// Verify the returned state reflects actual merge mutations:
/// - Face count decreased (killed face removed)
/// - Killed face's plane binding removed from GeometryState
/// - Surviving face's plane binding preserved
#[test]
fn merge_returns_committed_kernel_state() {
    let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();
    let face_count_before = state.topology().arena().face_count();

    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(face_a.index()).unwrap();
    selected.insert(face_b.index()).unwrap();

    let mut protected = EntityBitset::with_capacity(cap);
    protected.insert(face_extra.index()).unwrap();

    let selection = MergeRegionSelection::new(selected, protected, face_a);
    let mut ctx = ModelingContext::new();
    let result =
        execute_sheet_region_merge(state, &selection, &mut ctx).expect("merge must succeed");

    let output = result.into_value();
    let new_state = output.get_state();

    let face_count_after = new_state.topology().arena().face_count();
    assert!(
        face_count_after < face_count_before,
        "D1 regression: returned KernelState must have fewer faces after merge. \
        Before: {}, after: {}",
        face_count_before,
        face_count_after,
    );

    assert!(
        new_state.geometry().get_face_plane(face_b).is_none(),
        "D1 regression: killed face_b's plane binding must be removed in returned state",
    );

    assert!(
        new_state.geometry().get_face_plane(face_a).is_some(),
        "D1 regression: surviving face_a's plane binding must be preserved",
    );
}

/// D3 regression: overlap between selected_faces and protected_faces
/// must be rejected with ProtectedUseConflict.
#[test]
fn protected_face_in_selected_set_rejected() {
    let (state, _, face_a, face_b, _face_extra) = build_cube_with_valence_3_edge();

    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(face_a.index()).unwrap();
    selected.insert(face_b.index()).unwrap();

    // Overlap: face_b is both selected AND protected.
    let mut protected = EntityBitset::with_capacity(cap);
    protected.insert(face_b.index()).unwrap();

    let selection = MergeRegionSelection::new(selected, protected, face_a);
    let mut ctx = ModelingContext::new();
    let err = execute_sheet_region_merge(state, &selection, &mut ctx)
        .expect_err("Must reject when selected ∩ protected != ∅");

    assert!(
        matches!(
            err,
            KernelError::MergeFailure(MergeError::ProtectedUseConflict { .. })
        ),
        "D3 regression: expected ProtectedUseConflict, got: {:?}",
        err,
    );
}

/// D5/P2-2 regression: merge traces must survive finalization exactly once.
///
/// Under the OperationFinalizer contract, decisions accumulate in `ModelingContext`
/// during execution and are drained into the returned `OperationResult` at the
/// operation boundary. The context must be empty after successful finalization to
/// avoid double-counting on reuse.
#[test]
fn ctx_receives_traced_decisions_after_merge() {
    let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();

    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(face_a.index()).unwrap();
    selected.insert(face_b.index()).unwrap();

    let mut protected = EntityBitset::with_capacity(cap);
    protected.insert(face_extra.index()).unwrap();

    let selection = MergeRegionSelection::new(selected, protected, face_a);
    let mut ctx = ModelingContext::new();
    let result =
        execute_sheet_region_merge(state, &selection, &mut ctx).expect("merge must succeed");

    assert!(
        !result.get_decision_log().is_empty(),
        "D5 regression: OperationResult decision log must not be empty",
    );

    assert!(
        ctx.get_decision_log_mut().is_empty(),
        "P2-2 regression: ModelingContext decision log must be drained after finalization",
    );
}

/// Epic A gate regression: if boundary certification rejects before draft creation,
/// the returned error must preserve witness/reason and the ctx trace must still
/// contain the certifier decision.
#[test]
fn boundary_cert_gate_rejection_preserves_witness_reason_and_ctx_trace() {
    // Selecting all three faces from this synthetic valence-3 fixture is known to
    // produce a degenerate/rejected boundary under the certifier.
    let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();

    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(face_a.index()).unwrap();
    selected.insert(face_b.index()).unwrap();
    selected.insert(face_extra.index()).unwrap();
    let protected = EntityBitset::with_capacity(cap);
    let selection = MergeRegionSelection::new(selected, protected, face_a);

    let mut ctx = ModelingContext::new();
    let err = execute_sheet_region_merge(state, &selection, &mut ctx)
        .expect_err("rejected boundary must fail before merge execution");

    match err {
        KernelError::MergeFailure(MergeError::BoundaryCertificationFailed {
            reason,
            witness,
        }) => {
            assert!(
                !reason.is_empty(),
                "gate rejection must preserve certifier reason text",
            );
            assert!(
                reason.contains("Boundary") || reason.contains("Degenerate"),
                "expected certifier rejection detail in reason, got: {}",
                reason,
            );
            assert!(
                witness.is_some(),
                "gate rejection must preserve certifier witness when provided",
            );
        }
        other => panic!(
            "expected BoundaryCertificationFailed from gate rejection, got {:?}",
            other
        ),
    }

    let decisions: Vec<_> = ctx.get_decision_log_mut().decisions().collect();
    assert!(
        !decisions.is_empty(),
        "gate rejection must still propagate certifier decision trace into ctx",
    );
    assert_eq!(
        decisions.len(), 1,
        "gate rejection should stop before merge-step execution; expected only certifier decision",
    );

    let d = decisions[0];
    assert_eq!(
        d.get_tier(),
        forge_core::DecisionTier::Escalated,
        "rejected certificate should trace as an escalated decision",
    );
    assert!(
        matches!(d.get_kind(), forge_core::DecisionKind::Forced { .. }),
        "rejected certificate should trace as Forced, got {:?}",
        d.get_kind(),
    );
    match d.get_context() {
        forge_core::DecisionContext::Degeneracy { description } => {
            assert!(
                description.contains("Boundary rejected"),
                "expected rejection context text, got: {}",
                description,
            );
            assert!(
                !description.contains("MergeStep"),
                "gate rejection must occur before any merge-step decisions",
            );
        }
        other => panic!(
            "expected Degeneracy context on cert rejection, got {:?}",
            other
        ),
    }
}

/// Gate ordering regression: boundary certification must run before later
/// input validation (e.g. selected/protected overlap), so a rejected boundary
/// fails with BoundaryCertificationFailed rather than ProtectedUseConflict.
#[test]
fn boundary_cert_gate_precedes_protected_overlap_validation() {
    let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();

    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(face_a.index()).unwrap();
    selected.insert(face_b.index()).unwrap();
    selected.insert(face_extra.index()).unwrap();

    // Deliberately invalid: overlap with selected set.
    let mut protected = EntityBitset::with_capacity(cap);
    protected.insert(face_b.index()).unwrap();

    let selection = MergeRegionSelection::new(selected, protected, face_a);
    let mut ctx = ModelingContext::new();
    let err = execute_sheet_region_merge(state, &selection, &mut ctx)
        .expect_err("gate should reject before protected-face overlap validation");

    assert!(
        matches!(err, KernelError::MergeFailure(MergeError::BoundaryCertificationFailed { .. })),
        "gate ordering regression: expected BoundaryCertificationFailed before ProtectedUseConflict, got {:?}",
        err,
    );

    let decisions: Vec<_> = ctx.get_decision_log_mut().decisions().collect();
    assert_eq!(
        decisions.len(),
        1,
        "gate ordering regression: expected only the certifier decision before early return",
    );
    assert!(
        !matches!(decisions[0].get_context(), forge_core::DecisionContext::Degeneracy { description } if description.contains("MergeStep")),
        "gate ordering regression: merge-step decisions must not appear before boundary gate passes",
    );
}

/// D4 regression: RadialUseSelector uses face indices (not halfedge indices).
///
/// Passing valid face indices succeeds.
#[test]
fn selector_with_valid_face_indices_succeeds() {
    let (state, edge_idx, face_a, face_b, _face_extra) = build_cube_with_valence_3_edge();

    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(face_a.index()).unwrap();
    selected.insert(face_b.index()).unwrap();

    let protected = EntityBitset::with_capacity(cap);

    let selectors = vec![RadialUseSelector::new(
        edge_idx,
        face_a.index(),
        face_b.index(),
    )];

    let selection =
        MergeRegionSelection::with_radial_selectors(selected, protected, face_a, selectors);

    let mut ctx = ModelingContext::new();
    let result = execute_sheet_region_merge(state, &selection, &mut ctx);
    assert!(
        result.is_ok(),
        "D4 regression: valid face-index selectors must succeed: {:?}",
        result.err(),
    );
}

/// D4 regression: passing halfedge indices (old incorrect semantics)
/// into RadialUseSelector must fail because the plan builder looks up
/// by face index and won't find matching faces in the radial ring.
#[test]
fn selector_with_halfedge_indices_fails() {
    let (state, edge_idx, face_a, face_b, _face_extra) = build_cube_with_valence_3_edge();

    // Find actual halfedge indices — these differ from face indices.
    let arena = state.topology().arena();
    let target_edge = {
        let mut found = None;
        for (eid, _) in arena.iter_edges() {
            if eid.index() == edge_idx {
                found = Some(eid);
                break;
            }
        }
        found.expect("target edge must exist")
    };
    let entry_he = arena.get_edge(target_edge).unwrap().half_edge();
    let he_idx_a = entry_he.index();
    let twin_he = arena.get_half_edge(entry_he).unwrap().radial_next();
    let he_idx_b = twin_he.index();

    // Only proceed if halfedge indices differ from face indices.
    if he_idx_a == face_a.index() && he_idx_b == face_b.index() {
        return;
    }

    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(face_a.index()).unwrap();
    selected.insert(face_b.index()).unwrap();

    let protected = EntityBitset::with_capacity(cap);

    let selectors = vec![RadialUseSelector::new(
        edge_idx, he_idx_a, // halfedge index, not face index
        he_idx_b,
    )];

    let selection =
        MergeRegionSelection::with_radial_selectors(selected, protected, face_a, selectors);

    let mut ctx = ModelingContext::new();
    let result = execute_sheet_region_merge(state, &selection, &mut ctx);

    assert!(
        result.is_err(),
        "D4 regression: halfedge indices as face selectors must not succeed silently",
    );
}

/// D6 adversarial: out-of-range EntityBitset::contains must propagate as
/// KernelError, not be silently swallowed via unwrap_or(false).
///
/// Constructs a bitset with capacity derived from the fixture's actual
/// face indices, guaranteeing an out-of-range contains() hit.
#[test]
fn out_of_range_bitset_propagates_error() {
    let (state, _, face_a, _face_b, _face_extra) = build_cube_with_valence_3_edge();

    // Find the maximum face index in the arena.
    let max_face_idx = state
        .topology()
        .arena()
        .iter_faces()
        .map(|(fid, _)| fid.index())
        .max()
        .expect("cube must have faces");

    // Capacity = max_face_idx, so contains(max_face_idx) is out-of-range
    // (bitset is [0, capacity), so capacity itself is OOB).
    let cap = max_face_idx;
    assert!(
        cap > 0,
        "Precondition: max face index must be > 0 for OOB test",
    );

    let mut selected = EntityBitset::with_capacity(cap);
    // Insert face_a if it fits (it likely does since cap = max_face_idx).
    if face_a.index() < cap {
        selected.insert(face_a.index()).unwrap();
    }

    let protected = EntityBitset::with_capacity(cap);
    let selection = MergeRegionSelection::new(selected, protected, face_a);

    let mut ctx = ModelingContext::new();
    let result = execute_sheet_region_merge(state, &selection, &mut ctx);

    // Connectivity validation walks all faces in the arena. When it hits
    // a face with index == max_face_idx, selected.contains(max_face_idx)
    // returns Err (out of bounds). With fail-closed `?`, this propagates.
    assert!(
        result.is_err(),
        "D6 regression: out-of-range bitset must propagate error, not silently ignore. \
        max_face_idx={}, bitset_cap={}",
        max_face_idx,
        cap,
    );
}

/// Integration: the runtime validator runs at emission without false-positives.
/// Uses the same live-descendant fixture as `lineage_fallback_resolves_live_descendant_and_traces_lineage_route`.
/// On the happy path the validator must pass (no `InternalError`) and the
/// reidentification adjunct must be emitted with matching outcome/compatibility.
#[test]
fn reidentification_trace_payload_drift_causes_internal_error_before_adjunct_push_no_false_positive(
) {
    use forge_core::{
        ReidentificationCompatibilitySummary, ReidentificationOutcome,
        ReidentificationTraceConsistencyError,
    };

    // ── Build the live-descendant fixture ──────────────────────────────
    let (topo, _geom, group) = build_two_face_coplanar_sheet_fixture();
    let target_face = topo
        .arena()
        .iter_faces()
        .find_map(|(fid, _)| {
            group
                .contains(fid.index())
                .ok()
                .and_then(|in_group| in_group.then_some(fid))
        })
        .expect("fixture must have at least one selected face");

    let synthetic_root = forge_topo::lineage::Lineage::root(
        99,
        forge_topo::lineage::OpSignature::with_id("synthetic_root_face", 1),
    );
    let child = forge_topo::lineage::Lineage::derive(
        &synthetic_root,
        forge_topo::lineage::OpSignature::with_id("synthetic_split_face", 2),
    );

    let mut draft = topo.into_mutation();
    draft
        .arena_mut()
        .get_face_mut(target_face)
        .expect("target face exists")
        .set_lineage(Some(child.clone()));
    draft.log_lineage_event(forge_topo::lineage::LineageEvent::EntityCreated {
        entity: forge_core::EntityRef::new(forge_core::EntityKind::Face, target_face.index()),
        entity_snapshot: Some(target_face.into()),
        lineage: child,
    });
    let topo = draft
        .commit()
        .expect("synthetic lineage descendant fixture commit");

    let missing_parent_name = PersistentName::new(
        synthetic_root.get_ancestry_hash(),
        forge_core::EntityKind::Face,
        0,
    );

    // ── Happy path: validator must pass and adjunct must be emitted ───
    let persistent = MergeRegionSelectionPersistent::new(
        vec![missing_parent_name.clone()],
        Vec::new(),
        missing_parent_name,
    );
    let state = crate::core::KernelState::new(topo, GeometryState::new());
    let mut ctx = ModelingContext::new();
    let resolved = resolve_merge_region_selection_persistent(&state, &persistent, &mut ctx)
        .expect("happy-path lineage resolved — validator must not false-positive");
    assert_eq!(resolved.get_selected_faces().iter_ones().count(), 1);

    let reid_adjunct = ctx
        .get_trace_adjuncts()
        .records()
        .iter()
        .find_map(|r| r.as_reidentification_payload())
        .expect("happy-path must emit a reidentification adjunct")
        .expect("adjunct decoded");
    assert_eq!(
        reid_adjunct.outcome,
        ReidentificationOutcome::Resolved,
        "happy-path adjunct outcome must be Resolved",
    );
    assert_eq!(
        reid_adjunct.compatibility,
        ReidentificationCompatibilitySummary::Available,
        "happy-path adjunct compatibility must be Available",
    );

    // ── Adversarial: artificially drift payload and confirm validator ─
    // Build a standalone drifted payload to test the validator API directly.
    // (We can't inject a tampered payload into resolve_single_face_ref without
    // refactoring the emission site; instead this covers the validator contract
    // at the API level, complementing the unit tests in reidentification_trace.rs.)
    let mut drifted = reid_adjunct;
    drifted.outcome = ReidentificationOutcome::Incompatible;
    // Leave compatibility = Available — this is the exact drift the validator guards against.
    let fake_decision = forge_core::tracing::TracedDecision::new(
        drifted.decision_id,
        forge_core::DecisionKind::Forced {
            reason: "ReidentificationIncompatible".into(),
        },
        forge_core::DecisionTier::Escalated,
        0.0,
        forge_core::DecisionContext::Degeneracy {
            description: "adversarial_drift_test".into(),
        },
    );
    assert_eq!(
        drifted.validate_against_decision(&fake_decision),
        Err(ReidentificationTraceConsistencyError::OutcomeCompatibilityMismatch),
        "Incompatible outcome with Available compatibility must be flagged as OutcomeCompatibilityMismatch",
    );
}

#[test]
fn generation_reuse_does_not_cause_stale_snapshot_leakage() {
    // P2-4A adversarial test: exercises the FULL re-identification pipeline.
    //
    // Scenario:
    //   1. Face at slot 0, gen=1 is created with lineage hash H (from parent P).
    //   2. Face at slot 0, gen=1 is deleted.
    //   3. Face at slot 0, gen=2 is created with a DIFFERENT child lineage hash H2
    //      (from the same parent P — it's a descendant).
    //   4. Both events produce link records in the index.
    //   5. resolve_reidentification_query_v1 is called looking for
    //      descendants of P.
    //
    // Expected: The resolver must find only gen=2 (the live candidate).
    //   gen=1's link record has the right parent hash, but arena.get_face(gen=1)
    //   returns StaleHandle — the ABA guard in link_record_to_live_candidate
    //   filters it out. This is the critical generational safety gate.

    use forge_topo::provenance::{Lineage, LineageEntityRef, OpSignature};
    use forge_topo::provenance::{
        PersistentNameRef, ReidentificationCandidateState, ReidentificationLinkIndex,
        ReidentificationMode, ReidentificationQuery, ReidentificationQueryResult,
    };

    let parent = Lineage::root(1, OpSignature::with_id("parent_face", 1));
    let parent_hash = parent.get_ancestry_hash();

    // Two children from the same parent — different ops produce different ancestry hashes
    let child_gen1 = Lineage::derive(&parent, OpSignature::with_id("split_a", 2));
    let child_gen2 = Lineage::derive(&parent, OpSignature::with_id("split_b", 3));
    let child_gen1_hash = child_gen1.get_ancestry_hash();
    let child_gen2_hash = child_gen2.get_ancestry_hash();

    // Both events land in the link index — same slot, different generations
    let events = vec![
        forge_topo::lineage::LineageEvent::EntityCreated {
            entity: forge_core::EntityRef::new(forge_core::EntityKind::Face, 0),
            entity_snapshot: Some(LineageEntityRef::new(forge_core::EntityKind::Face, 0, 1)),
            lineage: child_gen1.clone(),
        },
        forge_topo::lineage::LineageEvent::EntityCreated {
            entity: forge_core::EntityRef::new(forge_core::EntityKind::Face, 0),
            entity_snapshot: Some(LineageEntityRef::new(forge_core::EntityKind::Face, 0, 2)),
            lineage: child_gen2.clone(),
        },
    ];

    let index = ReidentificationLinkIndex::from_lineage_events(1, &events);

    // Build an arena that has ONLY gen=2 alive at slot 0 (gen=1 is dead)
    let mut arena = forge_topo::b_rep::TopologyArena::new();
    let ph_loop = forge_topo::handles::LoopId::new(u32::MAX, 0);
    let ph_shell = forge_topo::handles::ShellId::new(u32::MAX, 0);

    // First insert → slot 0, gen=0 (arena starts at 0)
    let f_first = arena.insert_face(forge_topo::b_rep::FaceData::new(ph_loop, ph_shell));
    // Remove to free the slot
    arena.remove_face(f_first, None).unwrap();
    // Second insert → slot 0, gen=1 (the "stale" face)
    let f_stale = arena.insert_face(forge_topo::b_rep::FaceData::new(ph_loop, ph_shell));
    arena
        .get_face_mut(f_stale)
        .unwrap()
        .set_lineage(Some(child_gen1));
    // Remove again
    arena.remove_face(f_stale, None).unwrap();
    // Third insert → slot 0, gen=2 (the "live" face)
    let f_live = arena.insert_face(forge_topo::b_rep::FaceData::new(ph_loop, ph_shell));
    arena
        .get_face_mut(f_live)
        .unwrap()
        .set_lineage(Some(child_gen2));

    // Verify arena state: slot 0 is at gen=2, gen=1 is unreachable
    assert_eq!(f_live.index(), 0, "must reuse slot 0");
    assert!(
        f_live.generation() >= 2,
        "generation must have advanced past stale"
    );

    // Query: find descendants of parent_hash
    let query = ReidentificationQuery {
        target: PersistentNameRef {
            ancestry_hash: parent_hash,
            kind: forge_core::EntityKind::Face,
            ordinal: 0,
        },
        mode: ReidentificationMode::Descendants,
    };

    let result = forge_topo::provenance::resolve_reidentification_query_v1(
        &arena, &events, &index, &query,
    );

    match result {
        ReidentificationQueryResult::Resolved {
            candidate,
            evidence,
        } => {
            // Only the gen=2 live candidate should resolve
            assert_eq!(
                candidate.snapshot_ref.generation,
                f_live.generation(),
                "resolved candidate must be the live gen={} face, not the stale gen=1",
                f_live.generation()
            );
            assert_eq!(
                candidate.candidate_state,
                ReidentificationCandidateState::Live,
            );
            assert_eq!(
                candidate.link_evidence.child_ancestry_hash, child_gen2_hash,
                "resolved candidate must carry gen=2's ancestry hash"
            );
            // Evidence must show the index had 2 records but only 1 survived live filter
            assert_eq!(
                evidence.records_scanned, 2,
                "index should contain both gen=1 and gen=2 link records"
            );
            assert_eq!(
                evidence.candidates_post_filter, 1,
                "only gen=2 should survive live-arena validation"
            );
        }
        other => panic!(
            "Expected Resolved with gen=2 candidate, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
}
