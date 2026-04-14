//! Signed centroid computation for closed polyhedra.
//!
//! DOMAIN: Uses the divergence theorem (tetrahedralization against origin)
//! to compute the volumetric centroid of a closed polyhedron. Each face is
//! fan-triangulated into signed tetrahedra, and the centroid is the
//! volume-weighted average of tetrahedra centroids. No topology dependency.
//!
//! The centroid of a tetrahedron (origin, v0, v1, v2) is (v0+v1+v2)/4.
//! The volumetric centroid of the polyhedron is:
//!   C = Σ(signed_vol_6x_i * (v0_i + v1_i + v2_i)) / (4 * Σ(signed_vol_6x_i))

use super::volume::signed_tetra_volume_6x;

/// Compute the volumetric centroid of a closed polyhedron.
///
/// Each entry in `face_vertices` is the ordered vertex list of one face.
/// Faces must have consistent orientation (outward normals for positive
/// volume). Returns `None` if total volume is near-zero (degenerate).
///
/// # Algorithm
///
/// Fan-triangulate each face from vertex 0, form signed tetrahedra
/// against the origin, and accumulate weighted centroid contributions.
/// This gives the true mass centroid independent of tessellation density.
///
/// # Assumptions
///
/// - All faces are planar and convex (fan-from-v0 is valid).
///   For non-convex faces, use ear-clipping triangulation first.
///
/// # Example
/// ```
/// use worth_geom::polyhedron_centroid;
///
/// // Unit cube centered at origin: half-extent 0.5
/// let faces = vec![
///     vec![[-0.5,-0.5,-0.5], [0.5,-0.5,-0.5], [0.5,0.5,-0.5], [-0.5,0.5,-0.5]],
///     vec![[-0.5,-0.5,0.5], [-0.5,0.5,0.5], [0.5,0.5,0.5], [0.5,-0.5,0.5]],
///     vec![[-0.5,-0.5,-0.5], [-0.5,-0.5,0.5], [0.5,-0.5,0.5], [0.5,-0.5,-0.5]],
///     vec![[-0.5,0.5,-0.5], [0.5,0.5,-0.5], [0.5,0.5,0.5], [-0.5,0.5,0.5]],
///     vec![[-0.5,-0.5,-0.5], [-0.5,0.5,-0.5], [-0.5,0.5,0.5], [-0.5,-0.5,0.5]],
///     vec![[0.5,-0.5,-0.5], [0.5,-0.5,0.5], [0.5,0.5,0.5], [0.5,0.5,-0.5]],
/// ];
/// let c = polyhedron_centroid(&faces).unwrap();
/// assert!((c[0]).abs() < 1e-10);
/// assert!((c[1]).abs() < 1e-10);
/// assert!((c[2]).abs() < 1e-10);
/// ```
pub fn polyhedron_centroid(face_vertices: &[Vec<[f64; 3]>]) -> Option<[f64; 3]> {
    let mut cx: f64 = 0.0;
    let mut cy: f64 = 0.0;
    let mut cz: f64 = 0.0;
    let mut total_vol_6x: f64 = 0.0;

    for face_verts in face_vertices {
        if face_verts.len() < 3 {
            continue;
        }

        let v0 = &face_verts[0];
        for i in 1..face_verts.len() - 1 {
            let v1 = &face_verts[i];
            let v2 = &face_verts[i + 1];

            let sv6 = signed_tetra_volume_6x(v0, v1, v2);
            total_vol_6x += sv6;

            // Centroid of tetrahedron (origin, v0, v1, v2) = (v0+v1+v2)/4
            // (origin contributes [0,0,0])
            // Weighted by signed volume:
            cx += sv6 * (v0[0] + v1[0] + v2[0]);
            cy += sv6 * (v0[1] + v1[1] + v2[1]);
            cz += sv6 * (v0[2] + v1[2] + v2[2]);
        }
    }

    if total_vol_6x.abs() < 1e-30 {
        return None;
    }

    // C = Σ(sv6 * (v0+v1+v2)) / (4 * total_vol_6x)
    let denom = 4.0 * total_vol_6x;
    Some([cx / denom, cy / denom, cz / denom])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centered_cube_centroid_at_origin() {
        let h = 1.0;
        let faces = vec![
            vec![[-h, -h, -h], [h, -h, -h], [h, h, -h], [-h, h, -h]],
            vec![[-h, -h, h], [-h, h, h], [h, h, h], [h, -h, h]],
            vec![[-h, -h, -h], [-h, -h, h], [h, -h, h], [h, -h, -h]],
            vec![[-h, h, -h], [h, h, -h], [h, h, h], [-h, h, h]],
            vec![[-h, -h, -h], [-h, h, -h], [-h, h, h], [-h, -h, h]],
            vec![[h, -h, -h], [h, -h, h], [h, h, h], [h, h, -h]],
        ];
        let c = polyhedron_centroid(&faces).expect("should not be degenerate");
        for axis in 0..3 {
            assert!(c[axis].abs() < 1e-14, "axis {axis}: {}", c[axis]);
        }
    }

    #[test]
    fn offset_cube_centroid_at_center() {
        let cx = 3.0;
        let cy = 5.0;
        let cz = 7.0;
        let h = 1.0;
        let faces = vec![
            vec![
                [cx - h, cy - h, cz - h],
                [cx + h, cy - h, cz - h],
                [cx + h, cy + h, cz - h],
                [cx - h, cy + h, cz - h],
            ],
            vec![
                [cx - h, cy - h, cz + h],
                [cx - h, cy + h, cz + h],
                [cx + h, cy + h, cz + h],
                [cx + h, cy - h, cz + h],
            ],
            vec![
                [cx - h, cy - h, cz - h],
                [cx - h, cy - h, cz + h],
                [cx + h, cy - h, cz + h],
                [cx + h, cy - h, cz - h],
            ],
            vec![
                [cx - h, cy + h, cz - h],
                [cx + h, cy + h, cz - h],
                [cx + h, cy + h, cz + h],
                [cx - h, cy + h, cz + h],
            ],
            vec![
                [cx - h, cy - h, cz - h],
                [cx - h, cy + h, cz - h],
                [cx - h, cy + h, cz + h],
                [cx - h, cy - h, cz + h],
            ],
            vec![
                [cx + h, cy - h, cz - h],
                [cx + h, cy - h, cz + h],
                [cx + h, cy + h, cz + h],
                [cx + h, cy + h, cz - h],
            ],
        ];
        let c = polyhedron_centroid(&faces).expect("should not be degenerate");
        assert!((c[0] - cx).abs() < 1e-14, "x: {}", c[0]);
        assert!((c[1] - cy).abs() < 1e-14, "y: {}", c[1]);
        assert!((c[2] - cz).abs() < 1e-14, "z: {}", c[2]);
    }

    #[test]
    fn degenerate_returns_none() {
        let faces: Vec<Vec<[f64; 3]>> = vec![vec![[0.0; 3], [0.0; 3], [0.0; 3]]];
        assert!(polyhedron_centroid(&faces).is_none());
    }
}
