//! MakeShellFace — creates a new, disjoint shell within an existing solid.
//!
//! DOMAIN: Takes an existing BodyId and creates a new Shell belonging to it,
//! populated with an initial Face, Vertex, Loop, Edge, and self-radial HalfEdge.
//! This allows a single solid body to contain multiple disjoint shells (e.g., voids).
//!
//! INVARIANTS:
//! - ΔV=+1, ΔHE=+1, ΔF=+1, ΔL=+1, ΔE=+1, ΔS=+1, ΔSo=0
//! - The solid must already exist in the arena.
//! - Creates exactly 1 shell, 1 face, 1 vertex, 1 loop, 1 halfedge, 1 edge.
//! - The new halfedge is a degenerate self-loop: `twin == next == prev == self`.
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::arena::{EdgeData, FaceData, HalfEdgeData, LoopData, ShellData, ShellKind, VertexData};
use crate::handles::{EdgeId, HalfEdgeId, LoopId, RegionId, ShellId};
use crate::lineage::{Lineage, OpSignature};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::state::MutableDraft;
use crate::EulerOperator;

/// Creates a new, disjoint shell within an existing solid.
#[derive(Debug)]
pub struct MakeShellFace {
    /// The parent region that will own the new shell.
    pub region: RegionId,
}

/// Output of the MakeShellFace operator.
pub struct MsfOutput {
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
    /// The created edge (self-loop edge).
    pub edge: EdgeId,
}

impl EulerOperator for MakeShellFace {
    type Output = MsfOutput;

    fn execute(
        &self,
        draft: &mut MutableDraft,
        sig: &OpSignature,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let placeholder_loop = LoopId::new(u32::MAX, 0);

        let vertex_lineage = Lineage::root(0, sig.clone());
        let face_lineage = Lineage::root(1, sig.clone());
        let he_lineage = Lineage::root(2, sig.clone());
        let shell_lineage = Lineage::root(3, sig.clone());
        let edge_lineage = Lineage::root(4, sig.clone());

        let vertex = draft.insert_vertex(VertexData::with_lineage(
            placeholder_he,
            Some(vertex_lineage),
        ));

        let shell = draft.insert_shell(ShellData::with_lineage(
            crate::handles::FaceId::new(u32::MAX, 0),
            ShellKind::Sheet,
            self.region,
            Some(shell_lineage),
        ));

        let face = draft.insert_face(FaceData::with_lineage(
            placeholder_loop,
            shell,
            Some(face_lineage),
        ));

        let loop_id = draft.insert_loop(LoopData::new(placeholder_he, face));

        let edge = draft.insert_edge(EdgeData::with_lineage(placeholder_he, Some(edge_lineage)));

        let he = draft.insert_half_edge(HalfEdgeData::with_lineage(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            face,
            vertex,
            edge,
            Some(he_lineage),
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
        draft
            .arena_mut()
            .get_region_mut(self.region)?
            .add_shell(shell);
        draft.arena_mut().get_edge_mut(edge)?.set_half_edge(he);

        Ok(ExecutionResult {
            value: MsfOutput {
                face,
                vertex,
                half_edge: he,
                loop_id,
                shell,
                edge,
            },
            declared_delta: EulerDelta {
                vertices: 1,
                half_edges: 1,
                faces: 1,
                loops: 1,
                edges: 1,
                shells: 1,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("make_shell_face")
    }
}

#[cfg(test)]
mod tests {
    use super::MakeShellFace;
    use crate::operator::apply_op;
    use crate::state::TopologyState;
    use crate::topology::operations::euler::make_vertex_face::MakeVertexFace;
    use crate::EulerOperator;

    #[test]
    fn make_shell_face_creates_new_shell_in_solid() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

        assert_eq!(draft.arena().face_count(), 1);
        assert_eq!(draft.arena().shell_count(), 1);
        assert_eq!(draft.arena().body_count(), 1);

        let region = draft.arena().get_shell(mvf.shell).unwrap().region();
        let msf = apply_op(&mut draft, MakeShellFace { region })
            .unwrap()
            .into_value();

        assert_eq!(draft.arena().face_count(), 2);
        assert_eq!(draft.arena().shell_count(), 2);
        assert_eq!(draft.arena().body_count(), 1);

        let region_data = draft.arena().get_region(region).unwrap();
        assert_eq!(region_data.shell_count(), 2);
        assert!(region_data.shells().contains(&mvf.shell));
        assert!(region_data.shells().contains(&msf.shell));
    }
}
