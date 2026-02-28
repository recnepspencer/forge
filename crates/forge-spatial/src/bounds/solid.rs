//! Shell, region, lump, and solid AABB queries.
//!
//! DOMAIN: Hierarchical AABB aggregation — unions face bounds up through
//!         the full topology hierarchy to the solid body level.

use forge_core::KernelError;
use forge_geom::Aabb;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{BodyId, LumpId, RegionId, ShellId, VertexId};
use forge_topo::queries::hierarchy::shell_faces;

use super::face::face_bounds;

/// Compute an AABB for a shell by unioning bounds of all member faces.
pub fn shell_bounds(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    shell: ShellId,
) -> Result<Option<Aabb>, KernelError> {
    let mut result: Option<Aabb> = None;
    for face_id in shell_faces(arena, shell)? {
        let Some(face_box) = face_bounds(arena, position_fn, face_id)? else {
            continue;
        };
        result = match result {
            Some(bounds) => Some(bounds.union(&face_box)),
            None => Some(face_box),
        };
    }
    Ok(result)
}

/// Compute an AABB for a region by unioning its outer and inner shell bounds.
pub fn region_bounds(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    region: RegionId,
) -> Result<Option<Aabb>, KernelError> {
    let region_data = arena.get_region(region)?;
    let mut result: Option<Aabb> = None;

    if let Some(outer_shell) = region_data.outer_shell() {
        if let Some(shell_box) = shell_bounds(arena, position_fn, outer_shell)? {
            result = Some(shell_box);
        }
    }
    for &inner_shell in region_data.inner_shells() {
        let Some(shell_box) = shell_bounds(arena, position_fn, inner_shell)? else {
            continue;
        };
        result = match result {
            Some(bounds) => Some(bounds.union(&shell_box)),
            None => Some(shell_box),
        };
    }
    Ok(result)
}

/// Compute an AABB for a lump by unioning bounds of all regions.
pub fn lump_bounds(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    lump: LumpId,
) -> Result<Option<Aabb>, KernelError> {
    let lump_data = arena.get_lump(lump)?;
    let mut result: Option<Aabb> = None;
    for &region_id in lump_data.regions() {
        let Some(region_box) = region_bounds(arena, position_fn, region_id)? else {
            continue;
        };
        result = match result {
            Some(bounds) => Some(bounds.union(&region_box)),
            None => Some(region_box),
        };
    }
    Ok(result)
}

/// Compute an AABB for a solid body traversing body → lump → region → shell.
pub fn solid_bounds(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    body: BodyId,
) -> Result<Option<Aabb>, KernelError> {
    let body_data = arena.get_body(body)?;
    let mut result: Option<Aabb> = None;
    for &lump_id in body_data.lumps() {
        let Some(lump_box) = lump_bounds(arena, position_fn, lump_id)? else {
            continue;
        };
        result = match result {
            Some(bounds) => Some(bounds.union(&lump_box)),
            None => Some(lump_box),
        };
    }
    Ok(result)
}
