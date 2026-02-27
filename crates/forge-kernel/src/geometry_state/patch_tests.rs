//! D8 Transactionality tests for `GeometryPatch` and `KernelDraft`.
//!
//! These tests directly verify the core D8 guarantee:
//! geometry mutations applied inside a draft are visible during the draft,
//! discarded on rollback, and permanently applied on commit.

#[cfg(test)]
mod tests {
    use crate::geom_facade::Plane;
    use forge_topo::handles::{FaceId, VertexId};

    use crate::core::{KernelDraft, KernelState};
    use crate::geometry_state::{ExactPosition, GeometryPatch, GeometryState};

    fn make_plane(nx: f64, ny: f64, nz: f64, d: f64) -> Plane {
        Plane::try_new([nx, ny, nz], d).unwrap()
    }

    fn face(idx: u32) -> FaceId {
        FaceId::from_raw_parts(idx, 0)
    }

    fn vertex(idx: u32) -> VertexId {
        VertexId::from_raw_parts(idx, 0)
    }

    // ─── GeometryPatch unit tests ───────────────────────────────────────────

    #[test]
    fn patch_insert_is_visible_before_commit() {
        let base = GeometryState::new();
        let mut patch = GeometryPatch::new(base);
        let f = face(0);
        patch.set_face_plane(f, make_plane(0.0, 0.0, 1.0, 2.0));
        assert!(
            patch.get_face_plane(f).is_some(),
            "inserted plane must be visible in patch"
        );
    }

    #[test]
    fn patch_rollback_discards_inserts() {
        let mut base = GeometryState::new();
        let f = face(0);
        base.set_face_plane(f, make_plane(0.0, 0.0, 1.0, 0.0));

        let mut patch = GeometryPatch::new(base);
        let new_plane = make_plane(0.0, 1.0, 0.0, 5.0);
        patch.set_face_plane(f, new_plane);

        // rollback: must restore original plane
        let recovered = patch.rollback();
        let retrieved = recovered.get_face_plane(f).unwrap();
        assert!(
            (retrieved.normal()[2] - 1.0).abs() < 1e-10,
            "rollback must restore original plane, got {:?}",
            retrieved.normal()
        );
    }

    #[test]
    fn patch_commit_applies_inserts() {
        let base = GeometryState::new();
        let mut patch = GeometryPatch::new(base);
        let f = face(1);
        patch.set_face_plane(f, make_plane(1.0, 0.0, 0.0, -0.5));

        let committed = patch.commit();
        assert!(
            committed.get_face_plane(f).is_some(),
            "committed state must contain inserted plane"
        );
    }

    #[test]
    fn patch_commit_applies_removes() {
        let mut base = GeometryState::new();
        let f = face(0);
        base.set_face_plane(f, make_plane(0.0, 0.0, 1.0, 0.0));

        let mut patch = GeometryPatch::new(base);
        patch.remove_face_plane(f);

        let committed = patch.commit();
        assert!(
            committed.get_face_plane(f).is_none(),
            "committed state must not contain removed plane"
        );
    }

    #[test]
    fn patch_vertex_position_rollback_restores_original() {
        let mut base = GeometryState::new();
        let v = vertex(0);
        base.set_vertex_position(v, [1.0, 2.0, 3.0]);

        let pos_in_base = *base.get_vertex_position(v).unwrap();

        let mut patch = GeometryPatch::new(base);
        patch.set_vertex_position(v, ExactPosition::from_f64([9.0, 9.0, 9.0]));

        assert_eq!(
            patch.get_vertex_position(v),
            Some(&[9.0, 9.0, 9.0]),
            "patch must show mutated position"
        );

        let recovered = patch.rollback();
        assert_eq!(
            recovered.get_vertex_position(v),
            Some(&pos_in_base),
            "rollback must restore original position"
        );
    }

    #[test]
    fn patch_vertex_position_commit_persists() {
        let base = GeometryState::new();
        let v = vertex(2);
        let mut patch = GeometryPatch::new(base);
        patch.set_vertex_position(v, ExactPosition::from_f64([4.0, 5.0, 6.0]));

        let committed = patch.commit();
        assert!(
            committed.get_vertex_position(v).is_some(),
            "committed state must contain inserted vertex"
        );
    }

    #[test]
    fn patch_base_unaffected_before_commit() {
        let base = GeometryState::new();
        let f = face(7);
        let mut patch = GeometryPatch::new(base);
        patch.set_face_plane(f, make_plane(0.0, 1.0, 0.0, 3.0));

        // base accessor should still see nothing (patch is diff-only)
        assert!(
            patch.base().get_face_plane(f).is_none(),
            "base must not see patch mutations before commit"
        );
    }

    // ─── KernelDraft integration tests ─────────────────────────────────────

    fn empty_kernel_state() -> KernelState {
        use forge_topo::state::TopologyState;
        KernelState::new(TopologyState::empty(), GeometryState::new())
    }

    #[test]
    fn kernel_draft_rollback_restores_geometry() {
        let mut base_geom = GeometryState::new();
        let f = face(0);
        base_geom.set_face_plane(f, make_plane(0.0, 0.0, 1.0, 0.0));

        let state = KernelState::new(forge_topo::state::TopologyState::empty(), base_geom);
        let mut draft = KernelDraft::new(state);

        // Mutate geometry inside draft
        draft
            .geometry_mut()
            .set_face_plane(f, make_plane(1.0, 0.0, 0.0, 99.0));
        assert!(
            (draft.geometry().get_face_plane(f).unwrap().normal()[0] - 1.0).abs() < 1e-10,
            "mutation visible inside draft"
        );

        // Rollback must discard the mutation
        let restored = draft.rollback();
        let plane = restored.geometry().get_face_plane(f).unwrap();
        assert!(
            (plane.normal()[2] - 1.0).abs() < 1e-10,
            "rollback must restore original plane, got {:?}",
            plane.normal()
        );
    }

    #[test]
    fn kernel_draft_commit_persists_geometry() {
        let state = empty_kernel_state();
        let mut draft = KernelDraft::new(state);
        let f = face(3);
        draft
            .geometry_mut()
            .set_face_plane(f, make_plane(0.0, 1.0, 0.0, -1.0));

        let committed = draft.commit().unwrap();
        assert!(
            committed.geometry().get_face_plane(f).is_some(),
            "committed state must contain the inserted plane"
        );
    }

    #[test]
    fn kernel_draft_rollback_uses_internal_snapshot_not_caller_provided() {
        let mut base_geom = GeometryState::new();
        let f = face(0);
        base_geom.set_face_plane(f, make_plane(0.0, 0.0, 1.0, 1.0));
        let original_d = base_geom.get_face_plane(f).unwrap().offset();

        let state = KernelState::new(forge_topo::state::TopologyState::empty(), base_geom);
        let mut draft = KernelDraft::new(state);
        draft
            .geometry_mut()
            .set_face_plane(f, make_plane(0.0, 0.0, 1.0, 999.0));

        let restored = draft.rollback();
        let d_after = restored.geometry().get_face_plane(f).unwrap().offset();
        assert_eq!(
            d_after, original_d,
            "rollback must use internally stored original, got d={} expected {}",
            d_after, original_d
        );
    }

    // ─── Adversarial: nonzero-generation handles ────────────────────────────

    /// Nonzero-generation FaceId must round-trip through patch get/set/remove.
    ///
    /// A gen=0 only test would miss `pack_handle` overflows or sign extension
    /// bugs that only manifest when the high-32-bit word is non-zero.
    #[test]
    fn patch_nonzero_generation_face_roundtrip() {
        let base = GeometryState::new();
        let mut patch = GeometryPatch::new(base);

        let f_gen1 = FaceId::from_raw_parts(7, 1);
        let f_gen2 = FaceId::from_raw_parts(7, 2);

        patch.set_face_plane(f_gen1, make_plane(0.0, 0.0, 1.0, 10.0));
        patch.set_face_plane(f_gen2, make_plane(1.0, 0.0, 0.0, 20.0));

        let p1 = patch
            .get_face_plane(f_gen1)
            .expect("gen1 face must be visible");
        let p2 = patch
            .get_face_plane(f_gen2)
            .expect("gen2 face must be visible");

        assert!(
            (p1.offset() - 10.0).abs() < 1e-10,
            "gen1 plane offset wrong: got {}",
            p1.offset(),
        );
        assert!(
            (p2.normal()[0] - 1.0).abs() < 1e-10,
            "gen2 plane normal wrong: got {:?}",
            p2.normal(),
        );

        patch.remove_face_plane(f_gen1);
        assert!(
            patch.get_face_plane(f_gen1).is_none(),
            "gen1 removed, must be invisible"
        );
        assert!(
            patch.get_face_plane(f_gen2).is_some(),
            "gen2 unaffected by gen1 remove"
        );
    }

    /// Nonzero-generation VertexId must round-trip through patch get/set/remove.
    #[test]
    fn patch_nonzero_generation_vertex_roundtrip() {
        let base = GeometryState::new();
        let mut patch = GeometryPatch::new(base);

        let v_gen0 = VertexId::from_raw_parts(3, 0);
        let v_gen1 = VertexId::from_raw_parts(3, 1);

        patch.set_vertex_position(v_gen0, ExactPosition::from_f64([1.0, 0.0, 0.0]));
        patch.set_vertex_position(v_gen1, ExactPosition::from_f64([0.0, 1.0, 0.0]));

        assert_eq!(patch.get_vertex_position(v_gen0), Some(&[1.0, 0.0, 0.0]));
        assert_eq!(patch.get_vertex_position(v_gen1), Some(&[0.0, 1.0, 0.0]));

        patch.remove_vertex_position(v_gen0);
        assert!(patch.get_vertex_position(v_gen0).is_none(), "gen0 removed");
        assert_eq!(
            patch.get_vertex_position(v_gen1),
            Some(&[0.0, 1.0, 0.0]),
            "gen1 unaffected"
        );
    }

    // ─── Adversarial: mixed base+patch ABA ambiguity ────────────────────────

    /// Inserting a face at gen=1 into a patch whose base already has the SAME
    /// face INDEX at gen=0 (not yet removed) must be detected as an ABA
    /// collision by `GeometrySource::get_plane`.
    #[test]
    fn get_plane_detects_mixed_base_patch_aba_collision() {
        use forge_math::GeometrySource;

        let mut base = GeometryState::new();
        let f_gen0 = FaceId::from_raw_parts(5, 0);
        base.set_face_plane(f_gen0, make_plane(0.0, 0.0, 1.0, 0.0));

        let mut patch = GeometryPatch::new(base);
        let f_gen1 = FaceId::from_raw_parts(5, 1); // Same index, different generation
        patch.set_face_plane(f_gen1, make_plane(1.0, 0.0, 0.0, 5.0));

        let result = patch.get_plane(5);
        assert!(
            result.is_err(),
            "mixed base+patch ABA collision at index 5 must return Err, got {:?}",
            result,
        );
        let err_msg = format!("{:?}", result.unwrap_err());
        assert!(
            err_msg.contains("ABA"),
            "error message should mention ABA collision, got: {}",
            err_msg,
        );
    }

    /// Removing the gen0 base entry resolves the collision — patch gen1 must
    /// then be returned correctly.
    #[test]
    fn get_plane_collision_resolved_after_removing_base_entry() {
        use forge_math::GeometrySource;

        let mut base = GeometryState::new();
        let f_gen0 = FaceId::from_raw_parts(5, 0);
        base.set_face_plane(f_gen0, make_plane(0.0, 0.0, 1.0, 0.0));

        let mut patch = GeometryPatch::new(base);
        let f_gen1 = FaceId::from_raw_parts(5, 1);
        patch.set_face_plane(f_gen1, make_plane(1.0, 0.0, 0.0, 5.0));

        // Resolve by removing the stale base entry through the patch
        patch.remove_face_plane(f_gen0);

        let result = patch.get_plane(5);
        assert!(
            result.is_ok(),
            "after removing stale base entry, index 5 must resolve to patch value, got {:?}",
            result,
        );
    }

    /// Same-layer patch ambiguity (two live generations in patch) must also error.
    #[test]
    fn get_plane_detects_dual_generation_in_patch_layer() {
        use forge_math::GeometrySource;

        let base = GeometryState::new();
        let mut patch = GeometryPatch::new(base);

        let f_gen0 = FaceId::from_raw_parts(9, 0);
        let f_gen1 = FaceId::from_raw_parts(9, 1);
        patch.set_face_plane(f_gen0, make_plane(0.0, 0.0, 1.0, 1.0));
        patch.set_face_plane(f_gen1, make_plane(0.0, 1.0, 0.0, 2.0));

        let result = patch.get_plane(9);
        assert!(
            result.is_err(),
            "two live patch-layer generations for index 9 must return Err, got {:?}",
            result,
        );
    }

    // ─── Adversarial: topo-failure-bleed ────────────────────────────────────

    /// Core D8 integration guarantee: geometry mutated inside `KernelDraft`
    /// and then rolled back must leave zero bleed in the returned `KernelState`.
    ///
    /// This is the user-facing PR3 claim: "Pre-commit geometry mutation resolved."
    #[test]
    fn topo_failure_does_not_bleed_geometry_mutations() {
        let mut base_geom = GeometryState::new();
        let f = face(0);
        base_geom.set_face_plane(f, make_plane(0.0, 0.0, 1.0, 1.0));
        let state = KernelState::new(forge_topo::state::TopologyState::empty(), base_geom);

        let mut draft = KernelDraft::new(state);
        let sentinel_face = face(99);
        draft
            .geometry_mut()
            .set_face_plane(sentinel_face, make_plane(1.0, 0.0, 0.0, 42.0));
        draft
            .geometry_mut()
            .set_face_plane(f, make_plane(0.0, 1.0, 0.0, 55.0));

        let rolled_back = draft.rollback();

        assert!(
            rolled_back
                .geometry()
                .get_face_plane(sentinel_face)
                .is_none(),
            "D3/PR3 regression: sentinel geometry mutation bled through rollback",
        );

        let restored_plane = rolled_back
            .geometry()
            .get_face_plane(f)
            .expect("original face plane must survive rollback");

        assert!(
            (restored_plane.offset() - 1.0).abs() < 1e-10,
            "D3/PR3: original plane offset must be 1.0 after rollback, got {}",
            restored_plane.offset(),
        );

        assert!(
            (restored_plane.offset() - 55.0).abs() > 1e-5,
            "D3/PR3: mutated plane offset (55.0) must not survive rollback",
        );
    }

    // ─── Production scenario: post-commit base ambiguity ────────────────────

    /// A patch containing TWO generations of the same face index, when committed,
    /// must produce a base state where `get_plane` detects the collision.
    ///
    /// Scenario: ABA reallocation happens in one patch session (kill face-5-gen0,
    /// create face-5-gen1), but the caller forgot to `remove_face_plane(f_gen0)`
    /// before committing. Both packed keys land in the HashMap. The fix:
    /// `GeometryState::get_plane` must detect this and error.
    #[test]
    fn commit_with_two_generations_produces_detectable_base_ambiguity() {
        use forge_math::GeometrySource;

        let base = GeometryState::new();
        let mut patch = GeometryPatch::new(base);

        let f_gen0 = FaceId::from_raw_parts(5, 0);
        let f_gen1 = FaceId::from_raw_parts(5, 1);
        patch.set_face_plane(f_gen0, make_plane(0.0, 0.0, 1.0, 1.0));
        patch.set_face_plane(f_gen1, make_plane(1.0, 0.0, 0.0, 2.0));

        let committed = patch.commit();

        let result = committed.get_plane(5);
        assert!(
            result.is_err(),
            "post-commit base with two generations for index 5 must error on get_plane, got {:?}",
            result,
        );
        let msg = format!("{:?}", result.unwrap_err());
        assert!(
            msg.contains("ABA") || msg.contains("Ambiguous"),
            "error must mention ABA or Ambiguous collision, got: {}",
            msg,
        );
    }

    // ─── Production scenario: pack_handle boundary ──────────────────────────

    /// `pack_handle(0, u32::MAX)` must not alias with `pack_handle(1, 0)` or
    /// `pack_handle(u32::MAX, 0)`. Without `<< 32` the generation overflows
    /// the index word and different handles produce the same key.
    #[test]
    fn pack_handle_no_alias_at_max_generation() {
        use crate::geometry_state::schema::pack_handle;

        let a = pack_handle(0, u32::MAX);
        let b = pack_handle(1, 0);
        let c = pack_handle(0, 0);
        let d = pack_handle(u32::MAX, 0);
        let e = pack_handle(u32::MAX, u32::MAX);

        let keys = [a, b, c, d, e];
        for i in 0..keys.len() {
            for j in (i + 1)..keys.len() {
                assert_ne!(
                    keys[i], keys[j],
                    "pack_handle collision between pair ({}, {}): both produce key {:#018x}",
                    i, j, keys[i],
                );
            }
        }

        assert_eq!(a & 0xFFFF_FFFF, 0, "index 0 should be in low 32 bits");
        assert_eq!(
            a >> 32,
            u32::MAX as u64,
            "gen u32::MAX should be in high 32 bits"
        );
    }

    /// Face index at gen=u32::MAX must roundtrip through patch get/set/remove.
    /// Extreme generation must not truncate or alias.
    #[test]
    fn patch_max_generation_face_roundtrip() {
        use forge_math::GeometrySource;

        let base = GeometryState::new();
        let mut patch = GeometryPatch::new(base);

        let f_max = FaceId::from_raw_parts(0, u32::MAX);
        let f_gen0 = FaceId::from_raw_parts(0, 0);

        patch.set_face_plane(f_gen0, make_plane(0.0, 1.0, 0.0, 10.0));
        patch.set_face_plane(f_max, make_plane(0.0, 0.0, 1.0, 99.0));

        // Two live entries for index 0 → ABA error
        let result = patch.get_plane(0);
        assert!(
            result.is_err(),
            "gen=0 and gen=u32::MAX both live for index 0 must produce ABA error, got {:?}",
            result,
        );

        // Remove gen=0; only max gen remains → must resolve
        patch.remove_face_plane(f_gen0);
        let result = patch.get_plane(0);
        assert!(
            result.is_ok(),
            "after removing gen=0, gen=u32::MAX must resolve correctly, got {:?}",
            result,
        );
        assert!(
            (result.unwrap().normal()[2] - 1.0).abs() < 1e-10,
            "gen=u32::MAX plane must have nz=1.0",
        );
    }

    // ─── Production scenario: remove-then-reinsert ordering ─────────────────

    /// Within a single patch: set, remove, re-set same generation.
    /// The final set must win — not the intermediate remove.
    ///
    /// Common during face surgery: kill old binding, apply new geometry.
    #[test]
    fn intra_patch_remove_then_reinsert_same_generation_last_write_wins() {
        let base = GeometryState::new();
        let mut patch = GeometryPatch::new(base);

        let f = FaceId::from_raw_parts(3, 0);
        patch.set_face_plane(f, make_plane(1.0, 0.0, 0.0, 5.0));
        patch.remove_face_plane(f);
        patch.set_face_plane(f, make_plane(0.0, 0.0, 1.0, 99.0));

        let plane = patch
            .get_face_plane(f)
            .expect("re-inserted face must be visible after remove+set");
        assert!(
            (plane.offset() - 99.0).abs() < 1e-10,
            "re-inserted plane must be the last-written value (offset=99), got {}",
            plane.offset(),
        );

        let committed = patch.commit();
        assert!(
            committed.get_face_plane(f).is_some(),
            "committed state must contain the re-inserted face plane, not the remove",
        );
    }

    // ─── Production scenario: sequential draft-commit-draft cycle ───────────

    /// Kill face-5-gen0 in draft 1, commit gen1.
    /// Draft 2 must see only gen1 — no gen0 ghost.
    ///
    /// If the base still holds a gen0 key after commit1 and draft2 inserts gen1,
    /// the base would have both → ABA ambiguity in the live state.
    #[test]
    fn sequential_draft_commit_draft_no_generation_ghost() {
        use forge_math::GeometrySource;

        let mut initial = GeometryState::new();
        let f_gen0 = FaceId::from_raw_parts(5, 0);
        initial.set_face_plane(f_gen0, make_plane(0.0, 0.0, 1.0, 1.0));

        // Draft 1: remove gen=0, insert gen=1 (ABA reallocation)
        let mut patch1 = GeometryPatch::new(initial);
        let f_gen1 = FaceId::from_raw_parts(5, 1);
        patch1.remove_face_plane(f_gen0);
        patch1.set_face_plane(f_gen1, make_plane(1.0, 0.0, 0.0, 2.0));
        let after_commit1 = patch1.commit();

        assert!(
            after_commit1.get_face_plane(f_gen0).is_none(),
            "gen0 gone after commit1"
        );
        assert!(
            after_commit1.get_face_plane(f_gen1).is_some(),
            "gen1 visible after commit1"
        );

        // get_plane must resolve unambiguously
        assert!(
            after_commit1.get_plane(5).is_ok(),
            "index 5 must resolve to gen1 unambiguously after commit1",
        );

        // Draft 2: read-only, must see clean base
        let patch2 = GeometryPatch::new(after_commit1);
        assert!(patch2.get_face_plane(f_gen1).is_some(), "draft2 sees gen1");
        assert!(
            patch2.get_face_plane(f_gen0).is_none(),
            "draft2 does not see gen0 ghost"
        );
    }

    // ─── Production scenario: commit of removes actually cleans the base ─────

    /// Commit of a patch containing removes must physically delete the keys from
    /// the underlying HashMap — not rely on the (now-consumed) remove set shadow.
    ///
    /// Regression: if commit() forgot to apply removes, the base state would still
    /// contain the removed key and get_plane would find it.
    #[test]
    fn commit_applies_removes_to_base_not_just_patch_shadow() {
        use forge_math::GeometrySource;

        let mut base = GeometryState::new();
        let f = FaceId::from_raw_parts(7, 0);
        base.set_face_plane(f, make_plane(0.0, 1.0, 0.0, 3.0));

        assert!(base.get_face_plane(f).is_some());
        assert!(base.get_plane(7).is_ok());

        let mut patch = GeometryPatch::new(base);
        patch.remove_face_plane(f);
        assert!(
            patch.get_face_plane(f).is_none(),
            "remove must shadow entry in patch view"
        );

        let committed = patch.commit();
        assert!(
            committed.get_face_plane(f).is_none(),
            "committed base must not contain the removed face plane",
        );
        assert!(
            committed.get_plane(7).is_err(),
            "get_plane on removed index must return Err after commit",
        );
    }
}
