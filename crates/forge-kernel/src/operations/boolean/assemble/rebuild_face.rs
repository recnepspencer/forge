//! Rebuild a topological face from an ordered list of vertices.
//!
//! DOMAIN: Create a new face by converting a sequence of VertexIds into
//! a closed loop using exclusively Euler operators (MVF + MEV... + MEF). 

use forge_core::KernelError;
use forge_topo::handles::{FaceId, VertexId, HalfEdgeId};
use forge_topo::state::MutableDraft;
use forge_topo::operator::apply_op;
use forge_topo::euler::make_face_in_shell_from_vertices::MakeFaceInShellFromVertices;
use forge_topo::euler::make_loop_in_face_from_vertices::MakeLoopInFaceFromVertices;
use forge_topo::lineage::OpSignature;
use forge_topo::handles::ShellId;

/// Output of a rebuilt face.
pub struct RebuildFaceOutput {
    /// The newly created face.
    pub face: FaceId,
    /// The halfedges defining the outer loop of the face.
    /// Ordered sequentially matching the input vertices.
    pub outer_loop_halfedges: Vec<HalfEdgeId>,
    /// The shell containing the face.
    pub shell: ShellId,
}

/// Output of a rebuilt inner loop.
pub struct RebuildLoopOutput {
    /// The newly created loop.
    pub loop_id: forge_topo::handles::LoopId,
    /// The halfedges defining the loop in sequence.
    pub loop_halfedges: Vec<HalfEdgeId>,
}

/// Rebuild a closed face from a sequence of vertices using Euler operators.
///
/// Converts a list of pre-existing `VertexId`s into a fully closed 
/// topological face. This correctly establishes the `Face`, `Loop`, 
/// `HalfEdge`, and `Edge` entities without raw arena insertions.
///
/// # Returns
/// The new `FaceId` and the ordered list of `HalfEdgeId`s forming its boundary.
pub fn rebuild_face_from_vertices(
    draft: &mut MutableDraft,
    vertices: &[VertexId],
    shell: ShellId,
    sig: OpSignature,
) -> Result<RebuildFaceOutput, KernelError> {
    if vertices.len() < 3 {
        return Err(KernelError::InvalidInput { 
            message: format!("Cannot rebuild face from {} vertices (minimum 3 required)", vertices.len()),
            context: None 
        });
    }

    let op = MakeFaceInShellFromVertices { shell, vertices: vertices.to_vec() };
    match apply_op(draft, op) {
        Ok(res) => {
            let val = res.into_value();
            Ok(RebuildFaceOutput {
                face: val.face,
                outer_loop_halfedges: val.half_edges,
                shell: shell,
            })
        }
        Err(e) => Err(e),
    }
}

/// Rebuild an inner loop on an existing face from a sequence of vertices.
pub fn rebuild_inner_loop_from_vertices(
    draft: &mut MutableDraft,
    face: FaceId,
    vertices: &[VertexId],
    sig: OpSignature,
) -> Result<RebuildLoopOutput, KernelError> {
    if vertices.len() < 3 {
        return Err(KernelError::InvalidInput {
            message: format!("Cannot rebuild loop from {} vertices (minimum 3 required)", vertices.len()),
            context: None,
        });
    }

    let op = MakeLoopInFaceFromVertices { face, vertices: vertices.to_vec() };
    match apply_op(draft, op) {
        Ok(res) => {
            let val = res.into_value();
            Ok(RebuildLoopOutput {
                loop_id: val.loop_id,
                loop_halfedges: val.half_edges,
            })
        }
        Err(e) => Err(e),
    }
}
