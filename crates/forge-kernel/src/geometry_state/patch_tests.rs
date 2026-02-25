//! D8 Transactionality tests for `GeometryPatch` and `KernelDraft`.
//!
//! These tests directly verify the core D8 guarantee:
//! geometry mutations applied inside a draft are visible during the draft,
//! discarded on rollback, and permanently applied on commit.

#[cfg(test)]
mod tests {
    use forge_geom::Plane;
    use forge_topo::handles::{FaceId, VertexId};

    use crate::core::{KernelState, KernelDraft};
    use crate::geometry_state::{GeometryState, GeometryPatch, ExactPosition};

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
        assert!(patch.get_face_plane(f).is_some(), "inserted plane must be visible in patch");
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
        assert!(committed.get_face_plane(f).is_some(), "committed state must contain inserted plane");
    }

    #[test]
    fn patch_commit_applies_removes() {
        let mut base = GeometryState::new();
        let f = face(0);
        base.set_face_plane(f, make_plane(0.0, 0.0, 1.0, 0.0));

        let mut patch = GeometryPatch::new(base);
        patch.remove_face_plane(f);

        let committed = patch.commit();
        assert!(committed.get_face_plane(f).is_none(), "committed state must not contain removed plane");
    }

    #[test]
    fn patch_vertex_position_rollback_restores_original() {
        let mut base = GeometryState::new();
        let v = vertex(0);
        base.set_vertex_position(v, [1.0, 2.0, 3.0]);

        let pos_in_base = *base.get_vertex_position(v).unwrap();

        let mut patch = GeometryPatch::new(base);
        patch.set_vertex_position(v, ExactPosition::from_f64([9.0, 9.0, 9.0]));

        assert_eq!(patch.get_vertex_position(v), Some(&[9.0, 9.0, 9.0]), "patch must show mutated position");

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
        assert!(committed.get_vertex_position(v).is_some(), "committed state must contain inserted vertex");
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
        draft.geometry_mut().set_face_plane(f, make_plane(1.0, 0.0, 0.0, 99.0));
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
        draft.geometry_mut().set_face_plane(f, make_plane(0.0, 1.0, 0.0, -1.0));

        let committed = draft.commit().unwrap();
        assert!(
            committed.geometry().get_face_plane(f).is_some(),
            "committed state must contain the inserted plane"
        );
    }

    #[test]
    fn kernel_draft_rollback_uses_internal_snapshot_not_caller_provided() {
        // Build state with a known plane
        let mut base_geom = GeometryState::new();
        let f = face(0);
        base_geom.set_face_plane(f, make_plane(0.0, 0.0, 1.0, 1.0));
        let original_d = base_geom.get_face_plane(f).unwrap().offset();

        let state = KernelState::new(forge_topo::state::TopologyState::empty(), base_geom);
        let mut draft = KernelDraft::new(state);
        draft.geometry_mut().set_face_plane(f, make_plane(0.0, 0.0, 1.0, 999.0));

        // Rollback — the stored internal snapshot determines the result, no caller input
        let restored = draft.rollback();
        let d_after = restored.geometry().get_face_plane(f).unwrap().offset();
        assert_eq!(
            d_after, original_d,
            "rollback must use internally stored original, got d={} expected {}",
            d_after, original_d
        );
    }
}
