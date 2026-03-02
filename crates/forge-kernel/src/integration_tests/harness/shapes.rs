//! Shape builders for integration tests.
//!
//! DOMAIN: Creates real BSP-generated solids and returns a MutableDraft
//! with pre-extracted handles. When lineage or persistent naming lands,
//! update these builders once — all tests absorb the change automatically.

use forge_core::KernelError;
use forge_topo::transactions::{MutableDraft, TopologyState};
use forge_topo::handles::{BodyId, FaceId, HalfEdgeId, ShellId, VertexId};

use crate::configuration::facade::{resolve_config, KernelConfig, ResolvedConfig};
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
    let result = mesh_builder::make_cube([0.0, 0.0, 0.0], 1.0, &config)?;
    let (topo, _geom, _brep) = result.into_parts();

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
    let result = mesh_builder::make_tetrahedron([0.0, 0.0, 0.0], 1.0, &config)?;
    let (topo, _geom, _brep) = result.into_parts();

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
