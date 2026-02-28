//! MakeVertexFace — create the topological seed.
//!
//! DOMAIN: Creates the initial vertex + face + loop + degenerate halfedge
//! + shell + edge from which all topology is grown.
//!
//! INVARIANTS:
//! - Creates exactly 1 vertex, 1 face, 1 loop, 1 halfedge (self-loop), 1 shell, 1 edge
//! - The halfedge is its own twin, next, and prev
//! - All entities carry root lineage from the provided `OpSignature`
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::b_rep::{
    BodyData, EdgeData, FaceData, HalfEdgeData, LoopData, LumpData, RegionData, ShellData,
    ShellKind, VertexData,
};
use crate::handles::{EdgeId, HalfEdgeId, LoopId, LumpId, RegionId, ShellId};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::operator::TopoOperator;


/// Creates the topological seed: one vertex, one face, one loop, one selfloop halfedge,
/// one shell, and one edge.
///
/// This is always the first operator applied to an empty draft.
/// The halfedge is a degenerate self-loop: `twin == next == prev == self`.
#[derive(Debug)]
pub struct MakeVertexFace;

/// Output of the MakeVertexFace operator.
pub struct MvfOutput {
    /// The created vertex.
    pub vertex: crate::handles::VertexId,
    /// The created face.
    pub face: crate::handles::FaceId,
    /// The created halfedge (self-loop).
    pub half_edge: HalfEdgeId,
    /// The created loop.
    pub loop_id: crate::handles::LoopId,
    /// The created shell.
    pub shell: ShellId,
    /// The created region.
    pub region: RegionId,
    /// The created lump.
    pub lump: LumpId,
    /// The created solid.
    pub solid: crate::handles::BodyId,
    /// The created edge (self-loop edge).
    pub edge: EdgeId,
}

impl TopoOperator for MakeVertexFace {
    type Output = MvfOutput;

    const NAME: &'static str = "make_vertex_face";

    fn execute(
        &self,
        draft: &mut MutableDraft,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let placeholder_loop = LoopId::new(u32::MAX, 0);








        let vertex = draft.insert_vertex(VertexData::new(
            placeholder_he,
        ));

        let solid = draft.insert_body(BodyData::new());

        let lump = draft.insert_lump(LumpData::new(solid));

        let region = draft.insert_region(RegionData::new(lump));

        let shell = draft.insert_shell(ShellData::new(
            crate::handles::FaceId::new(u32::MAX, 0),
            ShellKind::Sheet,
            region,
        ));

        let face = draft.insert_face(FaceData::new(
            placeholder_loop,
            shell,
        ));

        let loop_id = draft.insert_loop(LoopData::new(placeholder_he, face));

        let edge = draft.insert_edge(EdgeData::new(placeholder_he));

        let he = draft.insert_half_edge(HalfEdgeData::new(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            face,
            vertex,
            edge,
        ));

        draft.arena_mut().get_half_edge_mut(he)?.set_radial_next(he);
        draft.arena_mut().get_half_edge_mut(he)?.set_next(he);
        draft.arena_mut().get_half_edge_mut(he)?.set_prev(he);
        draft.arena_mut().get_vertex_mut(vertex)?.set_outgoing(he);
        draft
            .arena_mut()
            .get_face_mut(face)?
            .set_outer_loop(loop_id);
        draft.arena_mut().get_loop_mut(loop_id)?.set_half_edge(he);
        draft
            .arena_mut()
            .get_shell_mut(shell)?
            .set_representative_face(face);
        draft.arena_mut().get_body_mut(solid)?.add_lump(lump);
        draft.arena_mut().get_lump_mut(lump)?.add_region(region);
        draft.arena_mut().get_region_mut(region)?.add_shell(shell);
        draft.arena_mut().get_edge_mut(edge)?.set_half_edge(he);

        Ok(ExecutionResult {
            value: MvfOutput {
                face,
                vertex,
                half_edge: he,
                loop_id,
                shell,
                region,
                lump,
                solid,
                edge,
            },
            declared_delta: EulerDelta {
                vertices: 1,
                half_edges: 1,
                faces: 1,
                loops: 1,
                edges: 1,
                shells: 1,
                solids: 1,
                lumps: 1,
                regions: 1,
            },
        })
    }


}
