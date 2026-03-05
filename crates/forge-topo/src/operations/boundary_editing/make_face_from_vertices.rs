//! MakeFaceFromVertices — build a face from an existing sequence of vertices.
//!
//! DOMAIN: Create a new face by connecting a pre-existing ordered sequence
//! of isolated `VertexId`s.
//!
//! INVARIANTS:
//! - The vertices must exist in the arena.
//! - Creates exactly N half-edges, N edges, 1 face, 1 loop, 1 shell, 1 region, 1 lump, 1 solid.
//! - Operates purely as a compound raw insertion, effectively a multi-vertex
//!   analog to `MakeVertexFace`.
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::b_rep::{
    BodyData, EdgeData, FaceData, HalfEdgeData, LoopData, LumpData, RegionData, ShellData,
    ShellKind,
};
use crate::handles::{
    BodyId, EdgeId, FaceId, HalfEdgeId, LoopId, LumpId, RegionId, ShellId, VertexId,
};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::operator::TopoOperator;
use crate::validators::invariant_id::InvariantContract;


/// Creates a new face by connecting a sequence of existing vertices.
///
/// Converts a list of pre-existing `VertexId`s into a fully closed
/// topological face. This correctly establishes the `Face`, `Loop`,
/// `HalfEdge`, and `Edge` entities.
#[derive(Debug)]
pub struct MakeFaceFromVertices {
    /// Ordered list of existing vertices to connect into a face.
    pub vertices: Vec<VertexId>,
}

/// Output of the MakeFaceFromVertices operator.
pub struct MffvOutput {
    /// The created face.
    pub face: FaceId,
    /// The created halfedges (in same order as vertices).
    pub half_edges: Vec<HalfEdgeId>,
    /// The created loop.
    pub loop_id: LoopId,
    /// The created shell.
    pub shell: ShellId,
    /// The created region.
    pub region: RegionId,
    /// The created lump.
    pub lump: LumpId,
    /// The created solid.
    pub solid: BodyId,
    /// The created edges (in same order as halfedges).
    pub edges: Vec<EdgeId>,
}

impl TopoOperator for MakeFaceFromVertices {
    type Output = MffvOutput;

    const NAME: &'static str = "make_face_from_vertices";

    const INVARIANT_CONTRACT: InvariantContract = crate::conservative_contract!();

    fn semantic_summary(&self) -> String {
        format!("Create {}-sided face from existing vertices", self.vertices.len())
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let n = self.vertices.len();
        if n < 3 {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "MakeFaceFromVertices: at least 3 vertices required, got {}",
                    n
                ),
                context: None,
            });
        }

        let sentinel_he = HalfEdgeId::DANGLING;
        let sentinel_loop = LoopId::DANGLING;





        let solid = draft.insert_body(BodyData::new());

        let lump = draft.insert_lump(LumpData::new(solid));

        let region = draft.insert_region(RegionData::new(lump));

        let shell = draft.insert_shell(ShellData::new(
            FaceId::DANGLING,
            ShellKind::Sheet,
            region,
        ));

        let face = draft.insert_face(FaceData::new(
            sentinel_loop,
            shell,
        ));

        let loop_id = draft.insert_loop(LoopData::new(sentinel_he, face));

        draft
            .arena_mut()
            .get_shell_mut(shell)?
            .set_representative_face(face);
        draft
            .arena_mut()
            .get_face_mut(face)?
            .set_outer_loop(loop_id);
        draft.arena_mut().get_body_mut(solid)?.add_lump(lump);
        draft.arena_mut().get_lump_mut(lump)?.add_region(region);
        draft.arena_mut().get_region_mut(region)?.add_shell(shell);

        // Ensure all vertices exist
        for &v in &self.vertices {
            if draft.arena().get_vertex(v).is_err() {
                return Err(KernelError::InvalidInput {
                    message: format!("MakeFaceFromVertices: vertex {} not found", v),
                    context: None,
                });
            }
        }

        let mut half_edges = Vec::with_capacity(n);
        let mut edges = Vec::with_capacity(n);

        for _i in 0..n {


            let edge = draft.insert_edge(EdgeData::new(sentinel_he));
            let he = draft.insert_half_edge(HalfEdgeData::new(
                sentinel_he,
                sentinel_he,
                sentinel_he,
                face,
                VertexId::DANGLING,
                edge,
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

            // Note: Does not overwrite vertex outgoing if it has one!
            // Wait, we need to set the outgoing if it's currently a sentinel or if
            // the operator is claiming isolated vertices.
            // In boolean copy, vertices DO come independently. But wait!
            // If the vertex is already used by another face in the schema, we shouldn't overwrite
            // `outgoing` unless we are linking it. Actually in disjoint copy, vertices are brand new
            // and isolated before we construct the faces.

            // Standard topological assignment: if vertex outgoing is max, set it.
            let orig_out = arena.get_vertex(v)?.primary_disk();
            if orig_out == HalfEdgeId::DANGLING {
                arena.get_vertex_mut(v)?.set_primary_disk(he);
            }
        }

        draft
            .arena_mut()
            .get_loop_mut(loop_id)?
            .set_half_edge(half_edges[0]);

        Ok(ExecutionResult {
            value: MffvOutput {
                face,
                half_edges,
                loop_id,
                shell,
                region,
                lump,
                solid,
                edges,
            },
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: n as i32,
                faces: 1,
                loops: 1,
                edges: n as i32,
                shells: 1,
                solids: 1,
                lumps: 1,
                regions: 1,
            },
        })
    }


}
