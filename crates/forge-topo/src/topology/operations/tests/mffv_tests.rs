#[cfg(test)]
mod tests {
    use crate::state::TopologyState;
    use crate::boundary_editing::make_face_from_vertices::MakeFaceFromVertices;
    use crate::handles::{VertexId, HalfEdgeId};
    use crate::arena::VertexData;
    use forge_core::KernelError;

    fn setup_vertices(draft: &mut crate::state::MutableDraft, count: usize) -> Vec<VertexId> {
        let mut verts = Vec::new();
        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        for _ in 0..count {
            verts.push(draft.insert_vertex(VertexData::new(placeholder_he)));
        }
        verts
    }

    #[test]
    fn mffv_creates_new_face_and_loop() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        
        let verts = setup_vertices(&mut draft, 3);
        
        let result = draft.execute(
            MakeFaceFromVertices {
                vertices: verts.clone(),
            },
        )
        .expect("MFFV must succeed with 3 vertices")
        .into_value();
        
        let arena = draft.arena();
        assert_eq!(arena.face_count(), 1);
        assert_eq!(arena.edge_count(), 3);
        assert_eq!(arena.half_edge_count(), 3);
        assert_eq!(arena.loop_count(), 1);
        
        let face = arena.get_face(result.face).unwrap();
        assert_eq!(face.outer_loop(), result.loop_id);
        
        let lp = arena.get_loop(result.loop_id).unwrap();
        assert_eq!(lp.face(), result.face);
    }

    #[test]
    fn mffv_rejects_insufficient_vertices() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let verts = setup_vertices(&mut draft, 2);
        
        let result = draft.execute(MakeFaceFromVertices { vertices: verts });
        assert!(matches!(result, Err(KernelError::InvalidInput { .. })));
    }
}
