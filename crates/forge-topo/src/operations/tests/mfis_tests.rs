#[cfg(test)]
mod tests {
    use crate::transactions::TopologyState;
    use crate::boundary_editing::make_face_in_shell_from_vertices::MakeFaceInShellFromVertices;
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::handles::{VertexId, HalfEdgeId};
    use crate::b_rep::VertexData;
    use forge_core::KernelError;
    use crate::b_rep::ShellKind;

    fn setup_vertices(draft: &mut crate::transactions::MutableDraft, count: usize) -> Vec<VertexId> {
        let mut verts = Vec::new();
        let placeholder_he = HalfEdgeId::DANGLING;
        for _ in 0..count {
            verts.push(draft.insert_vertex(VertexData::new(placeholder_he)));
        }
        verts
    }

    #[test]
    fn mfis_creates_face_in_existing_shell() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        
        let mvf = draft.execute(MakeVertexFace { shell_kind: ShellKind::Sheet }).unwrap().into_value();
        let shell_id = draft.arena().get_face(mvf.face).unwrap().shell();
        
        let verts = setup_vertices(&mut draft, 4);
        
        let result = draft.execute(
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
