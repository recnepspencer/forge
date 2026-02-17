//! # forge-view
//!
//! Representation traits for the Forge geometry kernel.
//!
//! This crate defines the contracts for converting kernel geometry
//! into visual representations. The kernel and geometry solvers are
//! NEVER aware of these traits — they are consumed by UI and export
//! layers only. This keeps the kernel pure while allowing multiple
//! concurrent representations (SDF, tessellation, ray-march).
//!
//! ## Architecture
//!
//! - [`Viewable`]: SDF distance evaluation for 60fps real-time preview
//! - [`Tessellatable`]: Triangle mesh generation for rendering/export
//! - [`TriangleMesh`]: Output container for tessellation results

#![forbid(unsafe_code)]

/// A triangle mesh produced by tessellation.
///
/// Stores vertices, normals, and triangle indices suitable for
/// rendering or STL/OBJ export.
#[derive(Debug, Clone, Default)]
pub struct TriangleMesh {
    /// Vertex positions in global modeling space.
    vertices: Vec<[f64; 3]>,
    /// Per-vertex normals (same length as `vertices`).
    normals: Vec<[f64; 3]>,
    /// Triangle indices — each `[u32; 3]` references three vertices.
    indices: Vec<[u32; 3]>,
}

impl TriangleMesh {
    /// Create an empty triangle mesh.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a mesh with pre-allocated capacity.
    pub fn with_capacity(vertex_count: usize, triangle_count: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(vertex_count),
            normals: Vec::with_capacity(vertex_count),
            indices: Vec::with_capacity(triangle_count),
        }
    }

    /// Add a vertex with its normal, returning the vertex index.
    pub fn add_vertex(&mut self, position: [f64; 3], normal: [f64; 3]) -> u32 {
        let idx = self.vertices.len() as u32;
        self.vertices.push(position);
        self.normals.push(normal);
        idx
    }

    /// Add a triangle from three vertex indices.
    pub fn add_triangle(&mut self, a: u32, b: u32, c: u32) {
        self.indices.push([a, b, c]);
    }

    /// Vertex positions.
    pub fn get_vertices(&self) -> &[[f64; 3]] {
        &self.vertices
    }

    /// Per-vertex normals.
    pub fn get_normals(&self) -> &[[f64; 3]] {
        &self.normals
    }

    /// Triangle indices.
    pub fn get_indices(&self) -> &[[u32; 3]] {
        &self.indices
    }

    /// Number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Number of triangles.
    pub fn triangle_count(&self) -> usize {
        self.indices.len()
    }
}

/// Trait for geometry that can provide an SDF (Signed Distance Field).
///
/// Implementors produce a distance value for any point in 3D space.
/// Negative = inside, positive = outside, zero = on surface.
///
/// This is consumed by the real-time preview engine for 60fps
/// ray-marched visualization of the model while B-Rep materializes
/// in the background (see Product Pitch, Pillar 4).
pub trait Viewable {
    /// Evaluate the signed distance from `point` to the surface.
    ///
    /// - Negative: point is inside the solid
    /// - Positive: point is outside the solid
    /// - Zero: point is on the surface boundary
    fn evaluate_sdf(&self, point: [f64; 3]) -> f64;

    /// Axis-aligned bounding box as `(min, max)` corners.
    fn bounding_box(&self) -> ([f64; 3], [f64; 3]);
}

/// Trait for geometry that can be tessellated into a triangle mesh.
///
/// Implementors produce a `TriangleMesh` at a given tolerance.
/// Smaller tolerance = more triangles, higher fidelity.
pub trait Tessellatable {
    /// Generate a triangle mesh approximation of this geometry.
    ///
    /// `tolerance` controls the maximum deviation from the true
    /// surface — smaller values produce denser, more accurate meshes.
    fn tessellate(&self, tolerance: f64) -> TriangleMesh;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_mesh_construction() {
        let mut mesh = TriangleMesh::new();
        let v0 = mesh.add_vertex([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let v1 = mesh.add_vertex([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let v2 = mesh.add_vertex([0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        mesh.add_triangle(v0, v1, v2);

        assert_eq!(mesh.vertex_count(), 3);
        assert_eq!(mesh.triangle_count(), 1);
    }

    #[test]
    fn viewable_trait_is_object_safe() {
        struct UnitSphere;
        impl Viewable for UnitSphere {
            fn evaluate_sdf(&self, point: [f64; 3]) -> f64 {
                let r = (point[0] * point[0]
                    + point[1] * point[1]
                    + point[2] * point[2])
                    .sqrt();
                r - 1.0
            }
            fn bounding_box(&self) -> ([f64; 3], [f64; 3]) {
                ([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0])
            }
        }

        let sphere = UnitSphere;
        let _: &dyn Viewable = &sphere;
        assert!(sphere.evaluate_sdf([0.0, 0.0, 0.0]) < 0.0);
        assert!(sphere.evaluate_sdf([2.0, 0.0, 0.0]) > 0.0);
    }

    #[test]
    fn tessellatable_trait_is_object_safe() {
        struct EmptyGeom;
        impl Tessellatable for EmptyGeom {
            fn tessellate(&self, _tolerance: f64) -> TriangleMesh {
                TriangleMesh::new()
            }
        }

        let geom = EmptyGeom;
        let _: &dyn Tessellatable = &geom;
        let mesh = geom.tessellate(0.1);
        assert_eq!(mesh.vertex_count(), 0);
    }
}
