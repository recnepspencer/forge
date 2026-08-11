//! BSP convex-cell clipping phase.
//!
//! DOMAIN: Clip an existing convex cell by one half-space, preserving exact
//! plane provenance, face winding, shared-edge vertices, and configured
//! numeric thresholds.

use super::super::schema::{CellFace, CellVertex, ConvexCell};
use super::construction::BspConfig;
use crate::primitives::plane::{intersect_three_planes, signed_distance, Plane};
use worth_math::MathError;

/// Clip a convex cell by a new plane, keeping the negative half-space.
pub fn clip_cell_by_plane(
    planes: &[Plane],
    cell: &ConvexCell,
    clip_plane_idx: usize,
    config: &BspConfig,
) -> Result<ConvexCell, MathError> {
    let clip_plane = &planes[clip_plane_idx];
    match classify_clip_vertices(cell, clip_plane, config) {
        ClipVertexClassification::AllInside => Ok(cell.clone()),
        ClipVertexClassification::AllOutside => {
            Ok(ConvexCell::new(planes.to_vec(), Vec::new(), Vec::new()))
        }
        ClipVertexClassification::Partial {
            distances,
            old_to_new,
            mut new_vertices,
        } => {
            let (mut new_faces, mut clip_face_vertices) = construct_clipped_faces(
                planes,
                cell,
                clip_plane_idx,
                config,
                &distances,
                &old_to_new,
                &mut new_vertices,
            )?;
            if let Some(cut_face) = construct_cut_face(
                planes,
                &new_vertices,
                &mut clip_face_vertices,
                clip_plane_idx,
            ) {
                new_faces.push(cut_face);
            }
            Ok(ConvexCell::new(planes.to_vec(), new_vertices, new_faces))
        }
    }
}

enum ClipVertexClassification {
    AllInside,
    AllOutside,
    Partial {
        distances: Vec<f64>,
        old_to_new: Vec<Option<usize>>,
        new_vertices: Vec<CellVertex>,
    },
}

fn classify_clip_vertices(
    cell: &ConvexCell,
    clip_plane: &Plane,
    config: &BspConfig,
) -> ClipVertexClassification {
    let distances: Vec<f64> = cell
        .vertices()
        .iter()
        .map(|v| signed_distance(clip_plane, v.position()))
        .collect();

    let all_inside = distances.iter().all(|d| *d < config.on_plane_eps);
    if all_inside {
        return ClipVertexClassification::AllInside;
    }

    let all_outside = distances.iter().all(|d| *d > -config.on_plane_eps);
    if all_outside {
        return ClipVertexClassification::AllOutside;
    }

    let mut new_vertices: Vec<CellVertex> = Vec::new();
    let mut old_to_new: Vec<Option<usize>> = vec![None; cell.vertices().len()];
    for (i, vertex) in cell.vertices().iter().enumerate() {
        if distances[i] < config.on_plane_eps {
            old_to_new[i] = Some(new_vertices.len());
            new_vertices.push(vertex.clone());
        }
    }
    ClipVertexClassification::Partial {
        distances,
        old_to_new,
        new_vertices,
    }
}

fn construct_clipped_faces(
    planes: &[Plane],
    cell: &ConvexCell,
    clip_plane_idx: usize,
    config: &BspConfig,
    distances: &[f64],
    old_to_new: &[Option<usize>],
    new_vertices: &mut Vec<CellVertex>,
) -> Result<(Vec<CellFace>, Vec<usize>), MathError> {
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
                let mapped = old_to_new[vi].ok_or_else(|| {
                    MathError::InternalError(
                        "BSP clip: inside vertex missing from old_to_new map".to_string(),
                    )
                })?;
                clipped_face_verts.push(mapped);
            }

            if vi_inside != vj_inside {
                let new_vert_idx = find_or_create_edge_vertex(
                    planes,
                    cell,
                    new_vertices,
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
    Ok((new_faces, clip_face_vertices))
}

fn construct_cut_face(
    planes: &[Plane],
    new_vertices: &[CellVertex],
    clip_face_vertices: &mut Vec<usize>,
    clip_plane_idx: usize,
) -> Option<CellFace> {
    clip_face_vertices.sort();
    clip_face_vertices.dedup();
    if clip_face_vertices.len() >= 3 {
        let ordered =
            order_clip_face_vertices(planes, new_vertices, clip_face_vertices, clip_plane_idx);
        Some(CellFace::new(clip_plane_idx, ordered))
    } else {
        None
    }
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

    planes_i
        .iter()
        .filter(|pi| **pi != exclude_plane)
        .find(|pi| planes_j.contains(pi))
        .copied()
}

/// Order clip face vertices in a consistent winding around the clip plane normal.
pub(super) fn order_clip_face_vertices(
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

    let mut indexed: Vec<(usize, f64)> = vert_indices
        .iter()
        .map(|&idx| {
            let pos = vertices[idx].position();
            let dx = pos[0] - centroid[0];
            let dy = pos[1] - centroid[1];
            let dz = pos[2] - centroid[2];
            let cos_component = dx * ref_dir[0] + dy * ref_dir[1] + dz * ref_dir[2];
            let sin_component = dx * perp[0] + dy * perp[1] + dz * perp[2];
            let angle = sin_component.atan2(cos_component);
            (idx, angle)
        })
        .collect();

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
