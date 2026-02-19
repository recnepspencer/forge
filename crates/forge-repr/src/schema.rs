//! Representation data shapes.
//!
//! DOMAIN: Output containers for visual representation.
//! DEPENDENCIES: None.

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
