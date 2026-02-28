#[cfg(test)]
mod tests {
    use crate::state::TopologyState;
    use crate::euler::make_face_in_shell_from_vertices::MakeFaceInShellFromVertices;
    use crate::euler::make_vertex_face::MakeVertexFace;
    use crate::operator::apply_op;
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
    fn mfis_creates_face_in_existing_shell() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let shell_id = draft.arena().get_face(mvf.face).unwrap().shell();
        
        let verts = setup_vertices(&mut draft, 4);
        
        let result = apply_op(
            &mut draft,
            MakeFaceInShellFromVertices {
                shell: shell_id,
                vertices: verts.clone(),
            },
        )
        .expect("MFIS must succeed")
        .into_value();
        
        let arena = draft.arena();
        let face = arena.get_face(result.face).unwrap();
        assert_eq!(face.shell(), shell_id);
        
        assert_eq!(arena.face_count(), 2);
        assert_eq!(arena.edge_count(), 5); // 1 from MVF, 4 from MFIS
        assert_eq!(arena.half_edge_count(), 5); // 1 from MVF, 4 from MFIS
    }
}
