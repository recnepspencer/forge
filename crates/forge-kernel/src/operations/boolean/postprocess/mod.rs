//! Post-processing of boolean results.
//!
//! DOMAIN: Simplification passes after boolean assembly:
//! - Merge coplanar faces (polygon extraction or legacy JoinFaces)
//! - Remove redundant collinear vertices
//! - Splice inner holes into outer boundaries
//!
//! DEPENDENCIES: forge_topo (Euler operators), GeometryStore.

mod coplanar;
pub mod polygon_extract;
mod vertex;
pub mod hole_splice;

use forge_core::KernelError;
use forge_topo::state::TopologyState;
use forge_topo::handles::HalfEdgeId;

use crate::geometry_store::GeometryStore;
use crate::core::ModelingContext;

pub use coplanar::merge_coplanar_faces;
pub use vertex::remove_redundant_vertices;
pub use polygon_extract::extract_coplanar_regions;
pub use hole_splice::splice_inner_holes;

/// Merge coplanar faces using the O(N) polygon extraction approach.
///
/// Falls back to the legacy iterative JoinFaces if extraction fails.
pub fn merge_coplanar_faces_extracted(
    topo: TopologyState,
    geom: &GeometryStore,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, usize), KernelError> {
    match extract_coplanar_regions(topo.clone(), geom, ctx) {
        Ok(result) => Ok(result),
        Err(_) => merge_coplanar_faces(topo, geom, ctx),
    }
}

/// Run an iterative pass until no more changes occur.
pub(crate) fn run_iterative_pass(
    mut topo: TopologyState,
    mut pass_fn: impl FnMut(TopologyState) -> Result<(TopologyState, usize), KernelError>,
) -> Result<(TopologyState, usize), KernelError> {
    let mut total = 0;
    let mut changed = 1;
    while changed > 0 {
        let (new_topo, count) = pass_fn(topo)?;
        topo = new_topo;
        changed = count;
        total += count;
    }
    Ok((topo, total))
}

/// Walk the half-edge ring around a vertex to compute degree.
pub(crate) fn compute_vertex_degree(
    arena: &forge_topo::arena::TopologyArena,
    he_first: HalfEdgeId,
) -> Option<(usize, Vec<HalfEdgeId>)> {
    let mut count = 0;
    let mut curr = he_first;
    let mut edges = Vec::new();

    loop {
        if count > 100 { return None; }
        count += 1;
        edges.push(curr);

        let curr_data = arena.get_half_edge(curr).ok()?;
        let twin_data = arena.get_half_edge(curr_data.radial_next()).ok()?;
        let next_outgoing = twin_data.next();
        if next_outgoing == he_first {
            return Some((count, edges));
        }
        curr = next_outgoing;
    }
}
