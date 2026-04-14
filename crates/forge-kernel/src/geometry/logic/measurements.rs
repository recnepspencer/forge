//! Geometric measurement adapters for B-Rep solids.
//!
//! DOMAIN: Thin adapter layer that walks topology (arena + handles)
//! to collect vertex data, then delegates all computation to pure
//! math algorithms in `worth_geom`.
//!
//! These functions need both `TopologyArena` and `GeometryView`,
//! which is why they live in `forge-kernel` rather than `worth-geom`.

use worth_geom::facade::{compute_polygon_area, distance, polyhedron_centroid, polyhedron_volume};
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{EdgeId, FaceId};

use super::super::contracts::GeometryView;

// ── Face area ────────────────────────────────────────────────────────────────

/// Compute the area of a face by collecting its vertices and delegating
/// to `worth_geom::compute_polygon_area`.
pub fn face_area(arena: &TopologyArena, geom: &impl GeometryView, face: FaceId) -> f64 {
    let verts = collect_face_positions(arena, geom, face);
    compute_polygon_area(&verts)
}

/// Compute the areas of all faces in an arena.
pub fn all_face_areas(arena: &TopologyArena, geom: &impl GeometryView) -> Vec<(FaceId, f64)> {
    arena
        .iter_faces()
        .map(|(fid, _)| (fid, face_area(arena, geom, fid)))
        .collect()
}

// ── Edge length ──────────────────────────────────────────────────────────────

/// Compute the length of an edge by looking up its endpoint positions
/// and delegating to `worth_geom::distance`.
pub fn edge_length(arena: &TopologyArena, geom: &impl GeometryView, edge: EdgeId) -> Option<f64> {
    let edata = arena.get_edge(edge).ok()?;
    let he = edata.half_edge();
    let hd = arena.get_half_edge(he).ok()?;
    let twin = arena.get_half_edge(hd.radial_next()).ok()?;

    let p0 = geom.get_vertex_position(hd.origin())?;
    let p1 = geom.get_vertex_position(twin.origin())?;

    Some(distance(p0, p1))
}

// ── Solid volume ─────────────────────────────────────────────────────────────

/// Compute the signed volume of a closed solid by collecting all face
/// vertex lists and delegating to `worth_geom::polyhedron_volume`.
pub fn solid_volume(arena: &TopologyArena, geom: &impl GeometryView) -> f64 {
    let face_verts: Vec<Vec<[f64; 3]>> = arena
        .iter_faces()
        .map(|(fid, _)| collect_face_positions(arena, geom, fid))
        .collect();

    polyhedron_volume(&face_verts)
}

// ── Solid centroid ───────────────────────────────────────────────────────────

/// Compute the volumetric centroid of a closed solid by collecting all face
/// vertex lists and delegating to `worth_geom::polyhedron_centroid`.
///
/// Returns `None` if the solid has near-zero volume (degenerate).
pub fn solid_centroid(arena: &TopologyArena, geom: &impl GeometryView) -> Option<[f64; 3]> {
    let face_verts: Vec<Vec<[f64; 3]>> = arena
        .iter_faces()
        .map(|(fid, _)| collect_face_positions(arena, geom, fid))
        .collect();

    polyhedron_centroid(&face_verts)
}

// ── Bounding box ─────────────────────────────────────────────────────────────

/// Axis-aligned bounding box.
#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

/// Compute the AABB of all vertex positions in the arena.
pub fn bounding_box(arena: &TopologyArena, geom: &impl GeometryView) -> Option<Aabb> {
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    let mut found = false;

    for (vid, _) in arena.iter_vertices() {
        if let Some(pos) = geom.get_vertex_position(vid) {
            found = true;
            for axis in 0..3 {
                min[axis] = min[axis].min(pos[axis]);
                max[axis] = max[axis].max(pos[axis]);
            }
        }
    }

    if found {
        Some(Aabb { min, max })
    } else {
        None
    }
}

// ── Internal ─────────────────────────────────────────────────────────────────

/// Collect ordered vertex positions around a face loop.
pub fn collect_face_positions(
    arena: &TopologyArena,
    geom: &impl GeometryView,
    face: FaceId,
) -> Vec<[f64; 3]> {
    arena
        .halfedges_of_face(face)
        .iter()
        .filter_map(|he_id| {
            let he = arena.get_half_edge(*he_id).ok()?;
            geom.get_vertex_position(he.origin()).copied()
        })
        .collect()
}
