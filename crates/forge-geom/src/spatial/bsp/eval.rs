//! BSP convex cell construction algorithms.
//!
//! Builds convex polyhedra by iteratively clipping a bounding box with planes.

use forge_math::MathError;
use crate::primitives::plane::{Plane, signed_distance, intersect_three_planes};
use super::schema::{ConvexCell, CellFace, CellVertex};

/// Threshold for triggering coordinate centering (to avoid numerical issues).
const CENTERING_THRESHOLD: f64 = 1e4;

/// Safety margin added to bounding extent.
const EXTENT_SAFETY_MARGIN: f64 = 10.0;

/// Configuration for BSP construction thresholds.
///
/// All numeric thresholds are explicit parameters — forge-geom
/// never hardcodes tolerance constants (Architecture Rule 4.1).
#[derive(Debug, Clone)]
pub struct BspConfig {
    /// Degeneracy threshold for plane intersection determinant.
    pub plane_degeneracy: f64,
    /// Signed-distance threshold for on-plane classification.
    pub on_plane_eps: f64,
    /// Half-extent of the initial axis-aligned bounding box.
    pub bounding_extent: f64,
}

impl Default for BspConfig {
    fn default() -> Self {
        Self {
            plane_degeneracy: 1e-12,
            on_plane_eps: 1e-10,
            bounding_extent: 1e6,
        }
    }
}

/// Build a bounded convex polyhedron from a set of half-space planes.
///
/// Each plane defines a half-space: points with `n·x + d < 0` are inside.
/// The result is the intersection of all half-spaces, bounded by a
/// large axis-aligned bounding box.
pub fn build_convex_polyhedron(input_planes: &[Plane], config: &BspConfig) -> Result<ConvexCell, MathError> {
    let center = estimate_centroid(input_planes);
    let needs_centering = center[0].abs() > CENTERING_THRESHOLD 
        || center[1].abs() > CENTERING_THRESHOLD 
        || center[2].abs() > CENTERING_THRESHOLD;

    let centered_planes: Vec<Plane> = if needs_centering {
        input_planes.iter().map(|p| translate_plane(p, &center)).collect::<Result<_, _>>()?
    } else {
        input_planes.to_vec()
    };

    let extent = compute_required_extent(&centered_planes, config.bounding_extent);
    let bbox_planes = create_bounding_box(extent)?;

    let mut all_planes: Vec<Plane> = bbox_planes;

    let adjusted_config = if extent > config.bounding_extent {
        BspConfig {
            bounding_extent: extent,
            on_plane_eps: config.on_plane_eps * (extent / config.bounding_extent),
            ..config.clone()
        }
    } else {
        config.clone()
    };

    let mut cell = build_initial_cube(&all_planes, &adjusted_config)?;

    for plane in &centered_planes {
        all_planes.push(plane.clone());
        let new_plane_idx = all_planes.len() - 1;
        cell = clip_cell_by_plane(&all_planes, &cell, new_plane_idx, &adjusted_config)?;

        if cell.vertices().is_empty() {
            return Err(MathError::InvalidInput(
                "Plane intersection produced an empty cell".to_string(),
            ));
        }
    }

    if needs_centering {
        let shifted_verts: Vec<CellVertex> = cell.vertices().iter().map(|v| {
            let pos = v.position();
            let shifted_pos = [pos[0] + center[0], pos[1] + center[1], pos[2] + center[2]];
            CellVertex::new(v.plane_indices()[0], v.plane_indices()[1], v.plane_indices()[2], shifted_pos)
        }).collect();

        let original_and_bbox: Vec<Plane> = {
            let mut ps = Vec::with_capacity(all_planes.len());
            for i in 0..6 {
                ps.push(all_planes[i].clone());
            }
            for p in input_planes {
                ps.push(p.clone());
            }
            ps
        };

        Ok(ConvexCell::new(original_and_bbox, shifted_verts, cell.faces().to_vec()))
    } else {
        Ok(ConvexCell::new(all_planes, cell.vertices().to_vec(), cell.faces().to_vec()))
    }
}

/// Estimate the centroid of geometry defined by input planes.
fn estimate_centroid(planes: &[Plane]) -> [f64; 3] {
    if planes.is_empty() {
        return [0.0, 0.0, 0.0];
    }

    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut cz = 0.0;

    for p in planes {
        let n = p.normal();
        let d = p.offset();
        cx += -n[0] * d;
        cy += -n[1] * d;
        cz += -n[2] * d;
    }

    let n = planes.len() as f64;
    [cx / n, cy / n, cz / n]
}

/// Translate a plane by subtracting `center` from its reference frame.
fn translate_plane(plane: &Plane, center: &[f64; 3]) -> Result<Plane, MathError> {
    let n = plane.raw_normal();
    let d = plane.raw_offset();
    let new_offset = d + n[0] * center[0] + n[1] * center[1] + n[2] * center[2];
    Plane::try_new(n, new_offset)
}

/// Compute the minimum bounding extent needed to contain the geometry.
fn compute_required_extent(input_planes: &[Plane], default_extent: f64) -> f64 {
    let mut max_dist = 0.0_f64;
    for plane in input_planes {
        let offset_magnitude = plane.offset().abs();
        max_dist = max_dist.max(offset_magnitude);
    }

    let required = max_dist * 2.0 + EXTENT_SAFETY_MARGIN;
    required.max(default_extent)
}

/// Clip a convex cell by a new plane, keeping the negative half-space.
pub fn clip_cell_by_plane(
    planes: &[Plane],
    cell: &ConvexCell,
    clip_plane_idx: usize,
    config: &BspConfig,
) -> Result<ConvexCell, MathError> {
    let clip_plane = &planes[clip_plane_idx];

    let distances: Vec<f64> = cell.vertices().iter()
        .map(|v| signed_distance(clip_plane, v.position()))
        .collect();

    let all_inside = distances.iter().all(|d| *d < config.on_plane_eps);
    if all_inside {
        return Ok(cell.clone());
    }

    let all_outside = distances.iter().all(|d| *d > -config.on_plane_eps);
    if all_outside {
        return Ok(ConvexCell::new(planes.to_vec(), Vec::new(), Vec::new()));
    }

    let mut new_vertices: Vec<CellVertex> = Vec::new();
    let mut old_to_new: Vec<Option<usize>> = vec![None; cell.vertices().len()];

    for (i, v) in cell.vertices().iter().enumerate() {
        if distances[i] < config.on_plane_eps {
            old_to_new[i] = Some(new_vertices.len());
            new_vertices.push(v.clone());
        }
    }

    let mut clip_face_vertices: Vec<usize> = Vec::new();

    let mut new_faces: Vec<CellFace> = Vec::new();

    for face in cell.faces() {
        let face_verts = face.vertices();
        let vert_count = face_verts.len();

        let mut clipped_face_verts: Vec<usize> = Vec::new();

        for edge_idx in 0..vert_count {
            let vi = face_verts[edge_idx];
            let vj = face_verts[(edge_idx + 1) % vert_count];
            let di = distances[vi];
            let dj = distances[vj];

            let vi_inside = di < config.on_plane_eps;
            let vj_inside = dj < config.on_plane_eps;

            if vi_inside {
                let mapped = old_to_new[vi].ok_or_else(|| MathError::InternalError(
                    "BSP clip: inside vertex missing from old_to_new map".to_string(),
                ))?;
                clipped_face_verts.push(mapped);
            }

            if vi_inside != vj_inside {
                let new_vert_idx = find_or_create_edge_vertex(
                    planes,
                    cell,
                    &mut new_vertices,
                    vi,
                    vj,
                    face.plane_idx(),
                    clip_plane_idx,
                    config,
                )?;
                clipped_face_verts.push(new_vert_idx);
                clip_face_vertices.push(new_vert_idx);
            }
        }

        if clipped_face_verts.len() >= 3 {
            new_faces.push(CellFace::new(face.plane_idx(), clipped_face_verts));
        }
    }

    clip_face_vertices.sort();
    clip_face_vertices.dedup();

    if clip_face_vertices.len() >= 3 {
        let ordered = order_clip_face_vertices(planes, &new_vertices, &clip_face_vertices, clip_plane_idx);
        new_faces.push(CellFace::new(clip_plane_idx, ordered));
    }

    Ok(ConvexCell::new(planes.to_vec(), new_vertices, new_faces))
}

/// Find or create a vertex at the intersection of an edge with the clip plane.
fn find_or_create_edge_vertex(
    planes: &[Plane],
    cell: &ConvexCell,
    new_vertices: &mut Vec<CellVertex>,
    vi: usize,
    vj: usize,
    face_plane_idx: usize,
    clip_plane_idx: usize,
    config: &BspConfig,
) -> Result<usize, MathError> {
    let v_i = &cell.vertices()[vi];
    let v_j = &cell.vertices()[vj];

    let shared_plane = find_shared_plane(v_i, v_j, face_plane_idx);

    let p0 = face_plane_idx;
    let p1 = shared_plane.unwrap_or(clip_plane_idx);

    let mut tri = [p0, p1, clip_plane_idx];
    tri.sort();

    for (idx, v) in new_vertices.iter().enumerate() {
        let mut existing = v.plane_indices();
        existing.sort();
        if existing == tri {
            return Ok(idx);
        }
    }

    let position = intersect_three_planes(
        &planes[tri[0]],
        &planes[tri[1]],
        &planes[tri[2]],
        config.plane_degeneracy,
    )?;

    let new_idx = new_vertices.len();
    new_vertices.push(CellVertex::new(tri[0], tri[1], tri[2], position));
    Ok(new_idx)
}

/// Find a plane that both vertices share, other than `exclude_plane`.
fn find_shared_plane(v_i: &CellVertex, v_j: &CellVertex, exclude_plane: usize) -> Option<usize> {
    let planes_i = v_i.plane_indices();
    let planes_j = v_j.plane_indices();

    planes_i.iter()
        .filter(|pi| **pi != exclude_plane)
        .find(|pi| planes_j.contains(pi))
        .copied()
}

/// Order clip face vertices in a consistent winding around the clip plane normal.
fn order_clip_face_vertices(
    planes: &[Plane],
    vertices: &[CellVertex],
    vert_indices: &[usize],
    clip_plane_idx: usize,
) -> Vec<usize> {
    if vert_indices.len() <= 2 {
        return vert_indices.to_vec();
    }

    let clip_normal = planes[clip_plane_idx].normal();

    let centroid = compute_centroid(vertices, vert_indices);

    let first_pos = vertices[vert_indices[0]].position();
    let ref_dir = [
        first_pos[0] - centroid[0],
        first_pos[1] - centroid[1],
        first_pos[2] - centroid[2],
    ];

    let perp = cross(&clip_normal, &ref_dir);

    let mut indexed: Vec<(usize, f64)> = vert_indices.iter().map(|&idx| {
        let pos = vertices[idx].position();
        let dx = pos[0] - centroid[0];
        let dy = pos[1] - centroid[1];
        let dz = pos[2] - centroid[2];
        let cos_component = dx * ref_dir[0] + dy * ref_dir[1] + dz * ref_dir[2];
        let sin_component = dx * perp[0] + dy * perp[1] + dz * perp[2];
        let angle = sin_component.atan2(cos_component);
        (idx, angle)
    }).collect();

    indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    indexed.into_iter().map(|(idx, _)| idx).collect()
}

/// Compute the centroid of a set of vertices.
fn compute_centroid(vertices: &[CellVertex], indices: &[usize]) -> [f64; 3] {
    let n = indices.len() as f64;
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut cz = 0.0;
    for &idx in indices {
        let p = vertices[idx].position();
        cx += p[0];
        cy += p[1];
        cz += p[2];
    }
    [cx / n, cy / n, cz / n]
}

/// Cross product of two 3D vectors.
fn cross(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Create 6 bounding-box planes at ±bound on each axis.
fn create_bounding_box(bound: f64) -> Result<Vec<Plane>, MathError> {
    Ok(vec![
        Plane::try_new([1.0, 0.0, 0.0], -bound)?,
        Plane::try_new([-1.0, 0.0, 0.0], -bound)?,
        Plane::try_new([0.0, 1.0, 0.0], -bound)?,
        Plane::try_new([0.0, -1.0, 0.0], -bound)?,
        Plane::try_new([0.0, 0.0, 1.0], -bound)?,
        Plane::try_new([0.0, 0.0, -1.0], -bound)?,
    ])
}

/// Build the initial cube from the 6 bounding-box planes.
fn build_initial_cube(planes: &[Plane], config: &BspConfig) -> Result<ConvexCell, MathError> {
    let mut vertices = Vec::new();
    let n = planes.len();

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if let Ok(pos) = intersect_three_planes(
                    &planes[i],
                    &planes[j],
                    &planes[k],
                    config.plane_degeneracy,
                ) {
                    let is_defining_plane = |idx: usize| idx == i || idx == j || idx == k;
                    let inside = planes.iter().enumerate().all(|(idx, p)| {
                        is_defining_plane(idx) || signed_distance(p, &pos) < config.on_plane_eps
                    });
                    if inside {
                        vertices.push(CellVertex::new(i, j, k, pos));
                    }
                }
            }
        }
    }

    let mut faces = Vec::new();
    for plane_idx in 0..n {
        let face_verts: Vec<usize> = vertices.iter().enumerate()
            .filter(|(_, v)| v.is_on_plane(plane_idx))
            .map(|(i, _)| i)
            .collect();

        if face_verts.len() >= 3 {
            let ordered = order_clip_face_vertices(planes, &vertices, &face_verts, plane_idx);
            faces.push(CellFace::new(plane_idx, ordered));
        }
    }

    Ok(ConvexCell::new(planes.to_vec(), vertices, faces))
}
