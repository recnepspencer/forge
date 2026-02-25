//! Evaluation logic for the geometry store.

use forge_core::KernelError;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::VertexId;
use super::schema::GeometryState;

/// Create a position lookup function for use with `classify_point_in_solid`.
///
/// Returns a closure that maps raw vertex slot indices to 3D positions
/// by resolving the generation from the arena and looking up the position
/// in the geometry store.
///
/// This bridges the `GeometryState` (kernel layer) to the `classify`
/// module (topo layer) without upward dependency (Architecture Rule 6).
pub fn build_position_lookup<'a>(
    store: &'a GeometryState,
    arena: &'a TopologyArena,
) -> impl Fn(u32) -> Result<[f64; 3], KernelError> + 'a {
    move |index: u32| {
        let gen = arena.vertex_generation(index as usize).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No active vertex at slot index {}", index),
                context: None,
            }
        })?;
        let vertex_id = VertexId::from_raw_parts(index, gen);
        store.get_vertex_position(vertex_id).copied().ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No position found for vertex index {}", index),
                context: None,
            }
        })
    }
}
