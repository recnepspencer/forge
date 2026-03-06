//! MakeIsolatedVertex — create a standalone vertex with no topological container.
//!
//! DOMAIN: Creates a raw vertex that is not part of any shell, region,
//! lump, or body hierarchy.
//!
//! TODO(Acorn Shell): A production kernel needs a minimal container
//! structure (e.g., "Point Lump" or "Acorn Shell") to host isolated
//! vertices. Without one, the vertex is invisible to spatial indexing,
//! serialization, and bounding-box traversal. This operator is
//! intentionally minimal for construction use — the containment
//! gap must be addressed when idle/floater vertex support is added.
//!
//! INVARIANTS:
//! - ΔV=+1, no other entities created
//! - The vertex has DANGLING primary_disk
//!
//! DEPENDENCIES: `arena` (entity storage)
use forge_core::KernelError;

use crate::b_rep::VertexData;
use crate::handles::{HalfEdgeId, VertexId};
use crate::operator::TopoOperator;
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::validators::invariant_id::InvariantContract;

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

    const INVARIANT_CONTRACT: InvariantContract =
        crate::validators::contract_registry::ISOLATED_VERTEX;

    fn semantic_summary(&self) -> String {
        "Create isolated vertex (no face, no shell)".into()
    }

    fn execute(
        &self,
        draft: &mut MutableDraft,
        _recorder: &mut crate::provenance::LineageRecorder,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let vertex = draft.insert_vertex(VertexData::new(HalfEdgeId::DANGLING));

        // ── Provenance Stamping (Root — no parent) ─────────────────────
        use forge_core::{EntityKind, EntityRef};
        let store = draft.lineage_store_mut();
        _recorder.stamp(
            store,
            EntityRef::new(EntityKind::Vertex, vertex.index(), vertex.generation()),
        );

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
