//! Signed volume computation for closed polyhedra.
//!
//! DOMAIN: Uses the divergence theorem to compute the volume of a
//! closed polyhedron from its face vertices. Each face is fan-
//! triangulated and accumulated as signed tetrahedra from the origin.
//! No topology dependency.

/// Compute the signed volume of a closed polyhedron.
///
/// Each entry in `face_vertices` is the ordered vertex list of one face.
/// Faces must have outward-pointing normals (right-hand rule) for a
/// positive result.
///
/// # Example
/// ```
/// use forge_geom::algorithms::measurement::volume::polyhedron_volume;
///
/// // Unit cube faces (outward normals)
/// let faces = vec![
///     // -Z face
///     vec![[0.,0.,0.], [1.,0.,0.], [1.,1.,0.], [0.,1.,0.]],
///     // +Z face
///     vec![[0.,0.,1.], [0.,1.,1.], [1.,1.,1.], [1.,0.,1.]],
///     // -Y face
///     vec![[0.,0.,0.], [0.,0.,1.], [1.,0.,1.], [1.,0.,0.]],
///     // +Y face
///     vec![[0.,1.,0.], [1.,1.,0.], [1.,1.,1.], [0.,1.,1.]],
///     // -X face
///     vec![[0.,0.,0.], [0.,1.,0.], [0.,1.,1.], [0.,0.,1.]],
///     // +X face
///     vec![[1.,0.,0.], [1.,0.,1.], [1.,1.,1.], [1.,1.,0.]],
/// ];
/// let vol = polyhedron_volume(&faces);
/// assert!((vol - 1.0).abs() < 1e-10, "got {vol}");
/// ```
pub fn polyhedron_volume(face_vertices: &[Vec<[f64; 3]>]) -> f64 {
    let mut volume = 0.0;

    for face_verts in face_vertices {
        if face_verts.len() < 3 {
            continue;
        }

        let v0 = &face_verts[0];
        for i in 1..face_verts.len() - 1 {
            let v1 = &face_verts[i];
            let v2 = &face_verts[i + 1];
            volume += signed_tetra_volume_6x(v0, v1, v2);
        }
    }

    volume / 6.0
}

/// Compute 6× the signed volume of the tetrahedron (origin, v0, v1, v2).
///
/// This is the scalar triple product of v0, v1, v2. Positive when
/// the triangle (v0, v1, v2) faces away from the origin (outward
/// normal convention).
pub fn signed_tetra_volume_6x(v0: &[f64; 3], v1: &[f64; 3], v2: &[f64; 3]) -> f64 {
    v0[0] * (v1[1] * v2[2] - v2[1] * v1[2])
        - v1[0] * (v0[1] * v2[2] - v2[1] * v0[2])
        + v2[0] * (v0[1] * v1[2] - v1[1] * v0[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_tetrahedron_volume() {
        // Regular tetrahedron with one vertex at origin
        let faces = vec![
            vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
            vec![[0., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            vec![[0., 0., 0.], [0., 0., 1.], [1., 0., 0.]],
            vec![[1., 0., 0.], [0., 0., 1.], [0., 1., 0.]],
        ];
        let vol = polyhedron_volume(&faces);
        // V = 1/6 for this tetrahedron
        assert!((vol.abs() - 1.0 / 6.0).abs() < 1e-10, "got {vol}");
    }

    #[test]
    fn empty_returns_zero() {
        assert_eq!(polyhedron_volume(&[]), 0.0);
    }
}
