//! Data shapes for BSP convex cell construction.

use crate::plane::Plane;

/// A vertex of a convex cell, defined by the intersection of three planes.
#[derive(Debug, Clone)]
pub struct CellVertex {
    /// Index of the first defining plane.
    plane_a: usize,
    /// Index of the second defining plane.
    plane_b: usize,
    /// Index of the third defining plane.
    plane_c: usize,
    /// Resolved 3D position (cached from `intersect_three_planes`).
    position: [f64; 3],
}

impl CellVertex {
    /// Create a new cell vertex from three plane indices and resolved position.
    pub fn new(plane_a: usize, plane_b: usize, plane_c: usize, position: [f64; 3]) -> Self {
        Self { plane_a, plane_b, plane_c, position }
    }

    /// The resolved 3D position.
    pub fn position(&self) -> &[f64; 3] {
        &self.position
    }

    /// The three plane indices that define this vertex.
    pub fn plane_indices(&self) -> [usize; 3] {
        [self.plane_a, self.plane_b, self.plane_c]
    }

    /// Whether this vertex is defined by the given plane index.
    pub fn is_on_plane(&self, plane_idx: usize) -> bool {
        self.plane_a == plane_idx || self.plane_b == plane_idx || self.plane_c == plane_idx
    }
}

/// A face of a convex cell — a convex polygon lying on one plane.
#[derive(Debug, Clone)]
pub struct CellFace {
    /// Index of the plane this face lies on.
    plane_idx: usize,
    /// Ordered vertex indices forming the convex polygon boundary.
    vertices: Vec<usize>,
}

impl CellFace {
    /// Create a new cell face.
    pub fn new(plane_idx: usize, vertices: Vec<usize>) -> Self {
        Self { plane_idx, vertices }
    }

    /// The plane index this face lies on.
    pub fn plane_idx(&self) -> usize {
        self.plane_idx
    }

    /// The ordered vertex indices.
    pub fn vertices(&self) -> &[usize] {
        &self.vertices
    }
}

/// A bounded convex polyhedron represented as a face-vertex mesh.
///
/// Built by repeatedly clipping a large bounding cell with input planes.
/// Each vertex is the intersection of exactly three planes.
#[derive(Debug, Clone)]
pub struct ConvexCell {
    /// All planes (input + bounding box).
    planes: Vec<Plane>,
    /// Vertices of the cell.
    vertices: Vec<CellVertex>,
    /// Faces of the cell (convex polygons).
    faces: Vec<CellFace>,
}

impl ConvexCell {
    /// Create a new convex cell.
    pub fn new(planes: Vec<Plane>, vertices: Vec<CellVertex>, faces: Vec<CellFace>) -> Self {
        Self { planes, vertices, faces }
    }

    /// The planes defining this cell.
    pub fn planes(&self) -> &[Plane] {
        &self.planes
    }

    /// The vertices of this cell.
    pub fn vertices(&self) -> &[CellVertex] {
        &self.vertices
    }

    /// The faces of this cell.
    pub fn faces(&self) -> &[CellFace] {
        &self.faces
    }

    /// Number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Number of faces.
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    /// Number of edges (each edge is shared by exactly 2 faces in a convex polyhedron).
    /// Sum of face vertex counts / 2 (since each edge appears in two faces).
    pub fn edge_count(&self) -> usize {
        let total_edges: usize = self.faces.iter().map(|f| f.vertices().len()).sum();
        total_edges / 2
    }
}
