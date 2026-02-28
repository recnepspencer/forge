//! MakeIsolatedVertex — creates a standalone vertex.
//!
//! DOMAIN: Create a single vertex not attached to any half-edge.

use forge_core::KernelError;

use crate::arena::VertexData;
use crate::handles::{HalfEdgeId, VertexId};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::state::MutableDraft;
use crate::operator::TopoOperator;


/// Creates an isolated vertex not attached to any half-edge.
#[derive(Debug)]
pub struct MakeIsolatedVertex;

/// Output of the MakeIsolatedVertex operator.
pub struct MakeIsolatedVertexOutput {
    /// The created vertex.
    pub vertex: VertexId,
}

impl TopoOperator for MakeIsolatedVertex {
    type Output = MakeIsolatedVertexOutput;

    const NAME: &'static str = "make_isolated_vertex";

    fn execute(
        &self,
        draft: &mut MutableDraft,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {

        let vertex = draft.insert_vertex(VertexData::new(
            HalfEdgeId::new(u32::MAX, 0),
        ));

        Ok(ExecutionResult {
            value: MakeIsolatedVertexOutput { vertex },
            declared_delta: EulerDelta {
                vertices: 1,
                half_edges: 0,
                faces: 0,
                loops: 0,
                edges: 0,
                shells: 0,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }


}
