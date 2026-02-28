#[cfg(test)]
mod tests {
    use crate::state::TopologyState;
    use crate::boundary_editing::make_loop_in_face_from_vertices::MakeLoopInFaceFromVertices;
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
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
    fn mlifv_creates_inner_loop_in_face() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        
        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let verts = setup_vertices(&mut draft, 3);
        
        let initial_loops = draft.arena().get_face(mvf.face).unwrap().inner_loop_count();
        assert_eq!(initial_loops, 0);
        
        let result = draft.execute(
            MakeLoopInFaceFromVertices {
                face: mvf.face,
                vertices: verts.clone(),
            },
        )
        .expect("MLIFV must succeed")
        .into_value();
        
        let arena = draft.arena();
        let face = arena.get_face(mvf.face).unwrap();
        
        assert_eq!(face.inner_loop_count(), 1);
        
        let inserted_loop = *face.inner_loops().first().unwrap();
        assert_eq!(inserted_loop, result.loop_id);
        
        let lp = arena.get_loop(result.loop_id).unwrap();
        assert_eq!(lp.face(), mvf.face);
    }
}
