//! Geometry binding validation — verify no dangling bindings exist.
//!
//! DOMAIN: Validates that all geometry bindings point to live topology entities.
//! This function remains in forge-kernel because it iterates over
//! `GeometryStore` internal key sets, which forge-spatial cannot access.
//!
//! For the REVERSE check (every topology entity has geometry assigned),
//! see `forge_spatial::integrity::completeness::validate_geometry_completeness`.
//!
//! DEPENDENCIES: `forge-core` (KernelError), `forge-topo` (TopologyArena)

use forge_core::KernelError;
use forge_topo::b_rep::TopologyArena;

/// Validate that no geometry bindings point to stale topology.
///
/// Iterates all binding keys in `GeometryStore` and verifies each
/// points to a live entity in the topology arena.
pub fn validate_bindings(
    store: &crate::geometry::data::store::GeometryStore,
    arena: &TopologyArena,
) -> Result<(), KernelError> {
    for &face in store.planes.keys() {
        if arena.get_face(face).is_err() {
            return Err(KernelError::InternalError {
                message: format!("Dangling plane binding for {}", face),
                context: None,
            });
        }
    }

    for &vertex in store.positions.keys() {
        if arena.get_vertex(vertex).is_err() {
            return Err(KernelError::InternalError {
                message: format!("Dangling position binding for {}", vertex),
                context: None,
            });
        }
    }

    for &face in store.surfaces.keys() {
        if arena.get_face(face).is_err() {
            return Err(KernelError::InternalError {
                message: format!("Dangling surface binding for {}", face),
                context: None,
            });
        }
    }

    for &edge in store.curves.keys() {
        if arena.get_edge(edge).is_err() {
            return Err(KernelError::InternalError {
                message: format!("Dangling curve binding for {}", edge),
                context: None,
            });
        }
    }

    for &he in store.coedges.keys() {
        if arena.get_halfedge(he).is_err() {
            return Err(KernelError::InternalError {
                message: format!("Dangling coedge binding for {}", he),
                context: None,
            });
        }
    }

    Ok(())
}
