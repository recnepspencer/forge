//! BSP convex-cell construction phase.
//!
//! DOMAIN: Center the input geometry when needed, establish the bounding
//! extent and initial cube, then route each input plane through clipping.

use super::super::schema::{CellVertex, ConvexCell};
use super::clipping::clip_cell_by_plane;
use crate::primitives::plane::{intersect_three_planes, signed_distance, Plane};
use worth_math::MathError;

/// Threshold for triggering coordinate centering (to avoid numerical issues).
const CENTERING_THRESHOLD: f64 = 1e4;

/// Safety margin added to bounding extent.
const EXTENT_SAFETY_MARGIN: f64 = 10.0;

/// Configuration for BSP construction thresholds.
///
/// All numeric thresholds are explicit parameters — worth-geom
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
pub fn build_convex_polyhedron(
    input_planes: &[Plane],
    config: &BspConfig,
) -> Result<ConvexCell, MathError> {
    let centering = center_and_translate_planes(input_planes)?;
    let (mut all_planes, adjusted_config) = prepare_bounding_planes(&centering.planes, config)?;
    let initial_cell = build_initial_cube(&all_planes, &adjusted_config)?;
    let cell = clip_ordered_planes(
        &mut all_planes,
        &centering.planes,
        initial_cell,
        &adjusted_config,
    )?;

    if centering.needs_centering {
        Ok(translate_cell_back(
            cell,
            centering.center,
            &all_planes,
            input_planes,
        ))
    } else {
        Ok(ConvexCell::new(
            all_planes,
            cell.vertices().to_vec(),
            cell.faces().to_vec(),
        ))
    }
}

struct CenteringPreparation {
    center: [f64; 3],
    needs_centering: bool,
    planes: Vec<Plane>,
}

fn center_and_translate_planes(input_planes: &[Plane]) -> Result<CenteringPreparation, MathError> {
    let center = estimate_centroid(input_planes);
    let needs_centering = center[0].abs() > CENTERING_THRESHOLD
        || center[1].abs() > CENTERING_THRESHOLD
        || center[2].abs() > CENTERING_THRESHOLD;
    let planes = if needs_centering {
        input_planes
            .iter()
            .map(|plane| translate_plane(plane, &center))
            .collect::<Result<_, _>>()?
    } else {
        input_planes.to_vec()
    };
    Ok(CenteringPreparation {
        center,
        needs_centering,
        planes,
    })
}

fn prepare_bounding_planes(
    centered_planes: &[Plane],
    config: &BspConfig,
) -> Result<(Vec<Plane>, BspConfig), MathError> {
    let extent = compute_required_extent(centered_planes, config.bounding_extent);
    let bbox_planes = create_bounding_box(extent)?;
    let all_planes: Vec<Plane> = bbox_planes;
    let adjusted_config = if extent > config.bounding_extent {
        BspConfig {
            bounding_extent: extent,
            on_plane_eps: config.on_plane_eps * (extent / config.bounding_extent),
            ..config.clone()
        }
    } else {
        config.clone()
    };
    Ok((all_planes, adjusted_config))
}

fn clip_ordered_planes(
    all_planes: &mut Vec<Plane>,
    centered_planes: &[Plane],
    mut cell: ConvexCell,
    config: &BspConfig,
) -> Result<ConvexCell, MathError> {
    for plane in centered_planes {
        all_planes.push(plane.clone());
        let new_plane_idx = all_planes.len() - 1;
        cell = clip_cell_by_plane(all_planes, &cell, new_plane_idx, config)?;
        if cell.vertices().is_empty() {
            return Err(MathError::InvalidInput(
                "Plane intersection produced an empty cell".to_string(),
            ));
        }
    }
    Ok(cell)
}

fn translate_cell_back(
    cell: ConvexCell,
    center: [f64; 3],
    all_planes: &[Plane],
    input_planes: &[Plane],
) -> ConvexCell {
    let shifted_verts: Vec<CellVertex> = cell
        .vertices()
        .iter()
        .map(|v| {
            let pos = v.position();
            let shifted_pos = [pos[0] + center[0], pos[1] + center[1], pos[2] + center[2]];
            CellVertex::new(
                v.plane_indices()[0],
                v.plane_indices()[1],
                v.plane_indices()[2],
                shifted_pos,
            )
        })
        .collect();

    let original_and_bbox: Vec<Plane> = {
        let mut ps = Vec::with_capacity(all_planes.len());
        for i in 0..6 {
            ps.push(all_planes[i].clone());
        }
        for plane in input_planes {
            ps.push(plane.clone());
        }
        ps
    };

    ConvexCell::new(original_and_bbox, shifted_verts, cell.faces().to_vec())
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
        let face_verts: Vec<usize> = vertices
            .iter()
            .enumerate()
            .filter(|(_, v)| v.is_on_plane(plane_idx))
            .map(|(i, _)| i)
            .collect();

        if face_verts.len() >= 3 {
            let ordered = super::clipping::order_clip_face_vertices(
                planes,
                &vertices,
                &face_verts,
                plane_idx,
            );
            faces.push(super::super::schema::CellFace::new(plane_idx, ordered));
        }
    }

    Ok(ConvexCell::new(planes.to_vec(), vertices, faces))
}
