//! Shape builders for integration tests.
//!
//! DOMAIN: Creates real BSP-generated solids and returns either lightweight
//! handles (ModelingContext path — all tests use real sinks) or full context envelopes
//! (ModelingContext path for observability tests).
//!
//! When lineage or persistent naming lands, update these builders once —
//! all tests absorb the change automatically.

use crate::context::ModelingContext;
use forge_core::KernelError;
use forge_topo::transactions::{MutableDraft, TopologyState};
use forge_topo::handles::{BodyId, FaceId, HalfEdgeId, ShellId, VertexId};

use crate::configuration::facade::{resolve_config, KernelConfig, ResolvedConfig};
use crate::context::scope::OperationScope;
use crate::geometry::facade::GeometryStore;
use crate::operations::primitives;

/// Handles extracted from a cube solid.
#[derive(Debug, Clone)]
pub struct CubeHandles {
    pub body: BodyId,
    pub shell: ShellId,
    pub faces: Vec<FaceId>,
    pub vertices: Vec<VertexId>,
}

/// Handles extracted from a tetrahedron solid.
#[derive(Debug, Clone)]
pub struct TetraHandles {
    pub body: BodyId,
    pub shell: ShellId,
    pub faces: Vec<FaceId>,
    pub vertices: Vec<VertexId>,
}

/// Build a default test config.
pub fn test_config() -> ResolvedConfig {
    resolve_config(&KernelConfig::default(), None, None, None).unwrap()
}

/// Build a unit cube centered at origin.
///
/// Returns a committed TopologyState and extracted handles.
/// Call `state.into_mutation()` to get a MutableDraft for applying operators.
pub fn unit_cube() -> Result<(TopologyState, CubeHandles), KernelError> {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&config, &mut ctx);
    let result = primitives::make_cube([0.0, 0.0, 0.0], 1.0, &mut scope)?;
    let (topo, _geom) = result.into_parts();

    let arena = topo.arena();

    let bodies: Vec<BodyId> = arena.iter_bodies().map(|(id, _)| id).collect();
    let body = bodies[0];

    let shells: Vec<ShellId> = arena.iter_shells().map(|(id, _)| id).collect();
    let shell = shells[0];

    let faces: Vec<FaceId> = arena.iter_faces().map(|(id, _)| id).collect();
    let vertices: Vec<VertexId> = arena.iter_vertices().map(|(id, _)| id).collect();

    let handles = CubeHandles {
        body,
        shell,
        faces,
        vertices,
    };

    Ok((topo, handles))
}

/// Build a tetrahedron centered at origin.
///
/// Returns a committed TopologyState and extracted handles.
pub fn tetrahedron() -> Result<(TopologyState, TetraHandles), KernelError> {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&config, &mut ctx);
    let result = primitives::make_tetrahedron([0.0, 0.0, 0.0], 1.0, &mut scope)?;
    let (topo, _geom) = result.into_parts();

    let arena = topo.arena();

    let bodies: Vec<BodyId> = arena.iter_bodies().map(|(id, _)| id).collect();
    let body = bodies[0];

    let shells: Vec<ShellId> = arena.iter_shells().map(|(id, _)| id).collect();
    let shell = shells[0];

    let faces: Vec<FaceId> = arena.iter_faces().map(|(id, _)| id).collect();
    let vertices: Vec<VertexId> = arena.iter_vertices().map(|(id, _)| id).collect();

    let handles = TetraHandles {
        body,
        shell,
        faces,
        vertices,
    };

    Ok((topo, handles))
}

/// Find the first halfedge of a given face by traversal.
///
/// Traverses the face's halfedge list via the index to find a starting halfedge.
pub fn first_halfedge_of_face(
    arena: &forge_topo::b_rep::TopologyArena,
    face: FaceId,
) -> Result<HalfEdgeId, KernelError> {
    let hes = arena.halfedges_of_face(face);
    if hes.is_empty() {
        return Err(KernelError::InvalidInput {
            message: format!("Face {} has no halfedges", face.index()),
            context: None,
        });
    }
    Ok(hes[0])
}

/// Collect all halfedge IDs around a face loop by walking `next` pointers.
pub fn collect_face_loop(
    arena: &forge_topo::b_rep::TopologyArena,
    start_he: HalfEdgeId,
) -> Result<Vec<HalfEdgeId>, KernelError> {
    let mut result = vec![start_he];
    let mut current = arena.get_half_edge(start_he)?.next();

    let max_iterations = 1000;
    let mut i = 0;
    while current != start_he {
        result.push(current);
        current = arena.get_half_edge(current)?.next();
        i += 1;
        if i > max_iterations {
            return Err(KernelError::InvalidInput {
                message: "Loop walk exceeded 1000 iterations — broken loop".to_string(),
                context: None,
            });
        }
    }
    Ok(result)
}

// ── Traced builders (production-grade observability tests) ──────────────────

/// Full context envelope returned by traced shape builders.
///
/// Contains everything needed to assert on any dimension of the operation:
/// topology, geometry, decisions, and (eventually) lineage.
pub struct TracedResult {
    pub topology: TopologyState,
    pub geometry: GeometryStore,
    pub ctx: ModelingContext,
    pub handles: CubeHandles,
}

/// Build a unit cube with `ModelingContext` (real `DecisionSink`).
///
/// Returns the full context envelope so observability tests can assert
/// on `DecisionLog`, lineage, spans — everything the production path produces.
pub fn unit_cube_traced() -> Result<TracedResult, KernelError> {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&config, &mut ctx);
    let result = primitives::make_cube([0.0, 0.0, 0.0], 1.0, &mut scope)?;
    let (topo, geom) = result.into_parts();

    let arena = topo.arena();
    let bodies: Vec<BodyId> = arena.iter_bodies().map(|(id, _)| id).collect();
    let shells: Vec<ShellId> = arena.iter_shells().map(|(id, _)| id).collect();
    let faces: Vec<FaceId> = arena.iter_faces().map(|(id, _)| id).collect();
    let vertices: Vec<VertexId> = arena.iter_vertices().map(|(id, _)| id).collect();

    Ok(TracedResult {
        topology: topo,
        geometry: geom,
        ctx,
        handles: CubeHandles {
            body: bodies[0],
            shell: shells[0],
            faces,
            vertices,
        },
    })
}

/// Build an axis-aligned block with `ModelingContext` (real `DecisionSink`).
pub fn unit_block_traced(
    center: [f64; 3],
    half_extents: [f64; 3],
) -> Result<TracedResult, KernelError> {
    let config = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&config, &mut ctx);
    let result = primitives::make_block(center, half_extents, &mut scope)?;
    let (topo, geom) = result.into_parts();

    let arena = topo.arena();
    let bodies: Vec<BodyId> = arena.iter_bodies().map(|(id, _)| id).collect();
    let shells: Vec<ShellId> = arena.iter_shells().map(|(id, _)| id).collect();
    let faces: Vec<FaceId> = arena.iter_faces().map(|(id, _)| id).collect();
    let vertices: Vec<VertexId> = arena.iter_vertices().map(|(id, _)| id).collect();

    Ok(TracedResult {
        topology: topo,
        geometry: geom,
        ctx,
        handles: CubeHandles {
            body: bodies[0],
            shell: shells[0],
            faces,
            vertices,
        },
    })
}

/// Create a fresh `(ResolvedConfig, ModelingContext)` pair for manual pipeline tests.
///
/// The caller can destructure these into an `OperationScope` and drive
/// multi-step operations while keeping access to the `ModelingContext`
/// for post-operation assertions on `DecisionLog`.
pub fn traced_scope() -> (ResolvedConfig, ModelingContext) {
    (test_config(), ModelingContext::new())
}
