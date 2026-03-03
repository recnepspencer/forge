//! ConvexCell → halfedge mesh conversion — pure orchestration.
//!
//! Builds a complete halfedge mesh from a BSP ConvexCell by orchestrating:
//! 1. Input validation (via `shared_validators::facade`)
//! 2. Vertex insertion (via `shared_operations::facade`)
//! 3. Face/loop/halfedge construction (via `shared_operations::facade`)
//! 4. Twin stitching (via `shared_operations::facade`)
//! 5. Geometry binding validation (via `forge_spatial`)
//!
//! This file contains NO inline math, data structures, or validation logic.

use forge_core::KernelError;
use forge_geom::{build_convex_polyhedron, BspConfig, ConvexCell, Plane};
use forge_topo::handles::VertexId;
use forge_topo::provenance::OpSignature;
use forge_topo::transactions::{MutableDraft, TopologyState};

use crate::engine::facade::SolidEnvelope;

use crate::context::scope::OperationScope;
use crate::geometry::facade::GeometryStore;
use crate::operations::shared_operations::facade::{
    insert_faces_and_loops, make_solid_hierarchy, place_vertex_exact,
    stitch_twins, PlacementRegistry,
};
use crate::operations::shared_validators::facade::{
    validate_cell, validate_center_and_size, validate_coordinate, validate_dimension,
};


// ── Core orchestrator ────────────────────────────────────────────────────

/// Build a halfedge mesh from a BSP ConvexCell.
///
/// Pure orchestration: validates → inserts vertices → builds faces →
/// stitches twins → validates geometry bindings.
pub fn build_halfedge_mesh(
    cell: &ConvexCell,
    scope: &mut OperationScope<'_>,
) -> Result<SolidEnvelope, KernelError> {
    validate_cell(cell)?;

    let span = scope.sink.start_span("build_halfedge_mesh");
    let start = std::time::Instant::now();

    let tolerance = scope.config.scaled_vertex_tolerance();
    let sig = OpSignature::new("build_halfedge_mesh");
    let mut ordinal: u64 = 0;

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut geometry = GeometryStore::default();

    // 1. Insert vertices (dedup + exact positions)
    let vertex_ids = insert_vertices(
        &mut draft, &mut geometry, cell, tolerance, &sig, &mut ordinal, scope.sink,
    )?;

    // 2. Create containment hierarchy
    let hierarchy = make_solid_hierarchy(&mut draft, &mut ordinal)?;

    // 3. Build faces, loops, halfedges
    let edge_map = insert_faces_and_loops(
        &mut draft, &mut geometry, cell, &vertex_ids, hierarchy.shell, &sig, &mut ordinal,
    )?;

    // 4. Stitch twin pointers
    stitch_twins(&mut draft, &edge_map, &sig, &mut ordinal)?;

    // 5. Set representative face on shell
    let first_face = draft.arena().iter_faces().next().map(|(fid, _)| fid);
    if let Some(fid) = first_face {
        draft.arena_mut().get_shell_mut(hierarchy.shell)?.set_representative_face(fid);
    }

    let topology = draft.commit()?;

    // 6. Validate geometry bindings
    crate::geometry::facade::validate_bindings(&geometry, topology.arena())?;
    forge_spatial::validate_geometry_completeness(
        topology.arena(),
        &|f| geometry.planes.contains(f),
        &|v| geometry.positions.contains(v),
    )?;

    scope.sink.end_span(span, start.elapsed().as_micros() as u64);

    Ok(SolidEnvelope::new(topology, geometry))
}

// ── Vertex insertion adapter ─────────────────────────────────────────────

/// Thin adapter: iterates ConvexCell vertices and delegates each placement
/// to `shared_operations::facade::place_vertex_exact`.
fn insert_vertices(
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    cell: &ConvexCell,
    tolerance: f64,
    _sig: &OpSignature,
    ordinal: &mut u64,
    sink: &mut dyn forge_core::tracing::DecisionSink,
) -> Result<Vec<VertexId>, KernelError> {
    let mut vertex_ids = Vec::with_capacity(cell.vertex_count());
    let mut registry = PlacementRegistry::with_capacity(cell.vertex_count());
    let planes = cell.planes();

    for vert in cell.vertices() {
        let pos = *vert.position();
        let plane_indices = vert.plane_indices();
        let vid = place_vertex_exact(
            draft, geometry, &mut registry, pos, plane_indices, planes, tolerance, sink,
        )?;
        *ordinal += 1;
        vertex_ids.push(vid);
    }

    Ok(vertex_ids)
}

// ── Primitive constructors (pure orchestration) ──────────────────────────

/// Build a convex solid from arbitrary planes.
pub fn make_convex_solid(
    planes: Vec<Plane>,
    scope: &mut OperationScope<'_>,
) -> Result<SolidEnvelope, KernelError> {
    let cell = build_convex_polyhedron(&planes, &BspConfig::default())?;
    build_halfedge_mesh(&cell, scope)
}

/// Create a cube centered at `center` with side length `size`.
pub fn make_cube(
    center: [f64; 3], size: f64, scope: &mut OperationScope<'_>,
) -> Result<SolidEnvelope, KernelError> {
    validate_center_and_size(center, size, scope.config)?;
    make_convex_solid(forge_geom::cube(center, size / 2.0)?, scope)
}

/// Create a regular tetrahedron centered at `center` with the given `scale`.
pub fn make_tetrahedron(
    center: [f64; 3], scale: f64, scope: &mut OperationScope<'_>,
) -> Result<SolidEnvelope, KernelError> {
    validate_center_and_size(center, scale, scope.config)?;
    make_convex_solid(forge_geom::tetrahedron(center, scale)?, scope)
}

/// Create a regular dodecahedron centered at `center` with the given `scale`.
pub fn make_dodecahedron(
    center: [f64; 3], scale: f64, scope: &mut OperationScope<'_>,
) -> Result<SolidEnvelope, KernelError> {
    validate_center_and_size(center, scale, scope.config)?;
    make_convex_solid(forge_geom::dodecahedron(center, scale)?, scope)
}

/// Create an axis-aligned block with independent half-extents.
pub fn make_block(
    center: [f64; 3], half_extents: [f64; 3], scope: &mut OperationScope<'_>,
) -> Result<SolidEnvelope, KernelError> {
    for (i, &v) in center.iter().enumerate() { validate_coordinate(v, &format!("center[{i}]"))?; }
    for (i, &v) in half_extents.iter().enumerate() { validate_dimension(v, &format!("half_extents[{i}]"), scope.config)?; }
    make_convex_solid(forge_geom::block(center, half_extents)?, scope)
}

/// Create a regular prism (n-gon extrusion) centered at `center`.
pub fn make_prism(
    center: [f64; 3], sides: u32, radius: f64, height: f64, scope: &mut OperationScope<'_>,
) -> Result<SolidEnvelope, KernelError> {
    for (i, &v) in center.iter().enumerate() { validate_coordinate(v, &format!("center[{i}]"))?; }
    validate_dimension(radius, "radius", scope.config)?;
    validate_dimension(height, "height", scope.config)?;
    validate_minimum_sides(sides, 3, "prism")?;
    make_convex_solid(forge_geom::prism(center, sides, radius, height)?, scope)
}

/// Create a regular pyramid (n-gon base with apex) centered at `center`.
pub fn make_pyramid(
    center: [f64; 3], sides: u32, radius: f64, height: f64, scope: &mut OperationScope<'_>,
) -> Result<SolidEnvelope, KernelError> {
    for (i, &v) in center.iter().enumerate() { validate_coordinate(v, &format!("center[{i}]"))?; }
    validate_dimension(radius, "radius", scope.config)?;
    validate_dimension(height, "height", scope.config)?;
    validate_minimum_sides(sides, 3, "pyramid")?;
    make_convex_solid(forge_geom::pyramid(center, sides, radius, height)?, scope)
}

/// Create a wedge (triangular cross-section extrusion) centered at `center`.
pub fn make_wedge(
    center: [f64; 3], dimensions: [f64; 3], scope: &mut OperationScope<'_>,
) -> Result<SolidEnvelope, KernelError> {
    for (i, &v) in center.iter().enumerate() { validate_coordinate(v, &format!("center[{i}]"))?; }
    let names = ["width", "depth", "height"];
    for (i, &v) in dimensions.iter().enumerate() { validate_dimension(v, names[i], scope.config)?; }
    make_convex_solid(forge_geom::wedge(center, dimensions)?, scope)
}

/// Validate that a polygon primitive has enough sides.
fn validate_minimum_sides(sides: u32, min: u32, name: &str) -> Result<(), KernelError> {
    if sides < min {
        return Err(KernelError::InvalidInput {
            message: format!("{name} needs at least {min} sides, got {sides}"),
            context: None,
        });
    }
    Ok(())
}
