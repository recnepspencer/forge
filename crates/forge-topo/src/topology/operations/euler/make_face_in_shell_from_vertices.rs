//! MakeFaceInShellFromVertices — build a face in an existing shell from an existing sequence of vertices.
//!
//! DOMAIN: Create a new face by connecting a pre-existing ordered sequence
//! of isolated `VertexId`s and adding it to an existing `Shell`.
//!
//! INVARIANTS:
//! - The vertices must exist in the arena.
//! - The shell must exist in the arena.
//! - Creates exactly N half-edges, N edges, 1 face, 1 loop.
//! - Does NOT create a new solid, lump, region, or shell.
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::arena::{EdgeData, FaceData, HalfEdgeData, LoopData};
use crate::handles::{EdgeId, FaceId, HalfEdgeId, LoopId, ShellId, VertexId};
use crate::lineage::{Lineage, OpSignature};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::state::MutableDraft;
use crate::EulerOperator;

/// Creates a new face in an existing shell by connecting a sequence of existing vertices.
#[derive(Debug)]
pub struct MakeFaceInShellFromVertices {
    /// The shell to add the new face to.
    pub shell: ShellId,
    /// Ordered list of existing vertices to connect into a face.
    pub vertices: Vec<VertexId>,
}

/// Output of the MakeFaceInShellFromVertices operator.
pub struct MfisOutput {
    /// The created face.
    pub face: FaceId,
    /// The created halfedges (in same order as vertices).
    pub half_edges: Vec<HalfEdgeId>,
    /// The created loop.
    pub loop_id: LoopId,
    /// The created edges (in same order as halfedges).
    pub edges: Vec<EdgeId>,
}

impl EulerOperator for MakeFaceInShellFromVertices {
    type Output = MfisOutput;

    fn execute(
        &self,
        draft: &mut MutableDraft,
        sig: &OpSignature,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let n = self.vertices.len();
        if n < 3 {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "MakeFaceInShellFromVertices: at least 3 vertices required, got {}",
                    n
                ),
                context: None,
            });
        }

        // Ensure the shell exists
        if draft.arena().get_shell(self.shell).is_err() {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "MakeFaceInShellFromVertices: shell {} not found",
                    self.shell
                ),
                context: None,
            });
        }

        // Ensure all vertices exist
        for &v in &self.vertices {
            if draft.arena().get_vertex(v).is_err() {
                return Err(KernelError::InvalidInput {
                    message: format!("MakeFaceInShellFromVertices: vertex {} not found", v),
                    context: None,
                });
            }
        }

        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let placeholder_loop = LoopId::new(u32::MAX, 0);

        let face_lineage = Lineage::root(0, sig.clone());
        let loop_lineage = Lineage::root(1, sig.clone());

        let face = draft.insert_face(FaceData::with_lineage(
            placeholder_loop,
            self.shell,
            Some(face_lineage),
        ));

        let loop_id = draft.insert_loop(LoopData::new(placeholder_he, face));

        draft
            .arena_mut()
            .get_face_mut(face)?
            .set_outer_loop(loop_id);

        let mut half_edges = Vec::with_capacity(n);
        let mut edges = Vec::with_capacity(n);

        for _ in 0..n {
            let edge_lineage = Lineage::root(6, sig.clone());
            let he_lineage = Lineage::root(7, sig.clone());

            let edge =
                draft.insert_edge(EdgeData::with_lineage(placeholder_he, Some(edge_lineage)));
            let he = draft.insert_half_edge(HalfEdgeData::with_lineage(
                placeholder_he,
                placeholder_he,
                placeholder_he,
                face,
                VertexId::new(u32::MAX, 0),
                edge,
                Some(he_lineage),
            ));

            draft.arena_mut().get_edge_mut(edge)?.set_half_edge(he);
            half_edges.push(he);
            edges.push(edge);
        }

        // Wire them up
        for i in 0..n {
            let next_i = (i + 1) % n;
            let prev_i = if i == 0 { n - 1 } else { i - 1 };

            let he = half_edges[i];
            let next_he = half_edges[next_i];
            let prev_he = half_edges[prev_i];
            let v = self.vertices[i];

            let arena = draft.arena_mut();
            arena.get_half_edge_mut(he)?.set_origin(v);
            arena.get_half_edge_mut(he)?.set_radial_next(he); // Boundaries are self-radial
            arena.get_half_edge_mut(he)?.set_next(next_he);
            arena.get_half_edge_mut(he)?.set_prev(prev_he);

            // Standard topological assignment: if vertex outgoing is max, set it.
            let orig_out = arena.get_vertex(v)?.outgoing();
            if orig_out == HalfEdgeId::new(u32::MAX, 0) {
                arena.get_vertex_mut(v)?.set_outgoing(he);
            }
        }

        draft
            .arena_mut()
            .get_loop_mut(loop_id)?
            .set_half_edge(half_edges[0]);

        Ok(ExecutionResult {
            value: MfisOutput {
                face,
                half_edges,
                loop_id,
                edges,
            },
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: n as i32,
                faces: 1,
                loops: 1,
                edges: n as i32,
                shells: 0,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("make_face_in_shell_from_vertices")
    }
}
