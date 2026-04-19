//! Data shapes for BSP convex cell construction.

use serde::{Deserialize, Serialize};
use worth_math::{GeometrySource, MathError, PlaneCoefficients};

use crate::primitives::plane::Plane;

/// A vertex of a convex cell, defined by the intersection of three planes.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
        Self {
            plane_a,
            plane_b,
            plane_c,
            position,
        }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellFace {
    /// Index of the plane this face lies on.
    plane_idx: usize,
    /// Ordered vertex indices forming the convex polygon boundary.
    vertices: Vec<usize>,
}

impl CellFace {
    /// Create a new cell face.
    pub fn new(plane_idx: usize, vertices: Vec<usize>) -> Self {
        Self {
            plane_idx,
            vertices,
        }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
        Self {
            planes,
            vertices,
            faces,
        }
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
    pub fn edge_count(&self) -> usize {
        let total_edges: usize = self.faces.iter().map(|f| f.vertices().len()).sum();
        total_edges / 2
    }
}

/// Boolean operation type for BSP merge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BspOp {
    /// A ∪ B — regions in either solid.
    Union,
    /// A ∩ B — regions in both solids.
    Intersection,
    /// A \ B — regions in A but not B.
    Subtraction,
}

/// A BSP tree node — either a leaf (in/out) or an internal splitting plane.
///
/// Internal nodes partition space by a plane index into the owning
/// `BspSolid`'s plane set. Leaf nodes classify their region as solid
/// (inside the object) or empty (outside).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BspNode {
    /// This region is entirely inside (`solid = true`) or outside the solid.
    Leaf { solid: bool },
    /// Space is split by the plane at `plane_idx`.
    Internal {
        /// Index into the owning `BspSolid`'s plane set.
        plane_idx: usize,
        /// Negative half-space (behind the plane: n·x + d < 0).
        neg: Box<BspNode>,
        /// Positive half-space (in front of the plane: n·x + d > 0).
        pos: Box<BspNode>,
    },
}

impl BspNode {
    /// Create a solid leaf.
    pub fn solid() -> Self {
        BspNode::Leaf { solid: true }
    }

    /// Create an empty leaf.
    pub fn empty() -> Self {
        BspNode::Leaf { solid: false }
    }

    /// Create an internal splitting node.
    pub fn split(plane_idx: usize, neg: BspNode, pos: BspNode) -> Self {
        BspNode::Internal {
            plane_idx,
            neg: Box::new(neg),
            pos: Box::new(pos),
        }
    }

    /// Whether this node is a leaf.
    pub fn is_leaf(&self) -> bool {
        matches!(self, BspNode::Leaf { .. })
    }

    /// Whether this node is a solid leaf.
    pub fn is_solid(&self) -> bool {
        matches!(self, BspNode::Leaf { solid: true })
    }

    /// Whether this node is an empty leaf.
    pub fn is_empty(&self) -> bool {
        matches!(self, BspNode::Leaf { solid: false })
    }

    /// Complement this tree (swap solid ↔ empty at every leaf).
    pub fn complement(&self) -> BspNode {
        match self {
            BspNode::Leaf { solid } => BspNode::Leaf { solid: !solid },
            BspNode::Internal {
                plane_idx,
                neg,
                pos,
            } => BspNode::Internal {
                plane_idx: *plane_idx,
                neg: Box::new(neg.complement()),
                pos: Box::new(pos.complement()),
            },
        }
    }

    /// Simplify this tree by collapsing internal nodes whose children
    /// are both leaves with the same label.
    pub fn simplify(self) -> BspNode {
        match self {
            BspNode::Leaf { .. } => self,
            BspNode::Internal {
                plane_idx,
                neg,
                pos,
            } => {
                let neg_s = neg.simplify();
                let pos_s = pos.simplify();
                match (&neg_s, &pos_s) {
                    (BspNode::Leaf { solid: a }, BspNode::Leaf { solid: b }) if a == b => {
                        BspNode::Leaf { solid: *a }
                    }
                    _ => BspNode::Internal {
                        plane_idx,
                        neg: Box::new(neg_s),
                        pos: Box::new(pos_s),
                    },
                }
            }
        }
    }

    /// Count the total number of nodes (leaves + internal).
    pub fn node_count(&self) -> usize {
        match self {
            BspNode::Leaf { .. } => 1,
            BspNode::Internal { neg, pos, .. } => 1 + neg.node_count() + pos.node_count(),
        }
    }

    /// Count the number of leaf nodes.
    pub fn leaf_count(&self) -> usize {
        match self {
            BspNode::Leaf { .. } => 1,
            BspNode::Internal { neg, pos, .. } => neg.leaf_count() + pos.leaf_count(),
        }
    }

    /// Count the number of solid leaves.
    pub fn solid_leaf_count(&self) -> usize {
        match self {
            BspNode::Leaf { solid } => {
                if *solid {
                    1
                } else {
                    0
                }
            }
            BspNode::Internal { neg, pos, .. } => neg.solid_leaf_count() + pos.solid_leaf_count(),
        }
    }

    /// Maximum depth of the tree.
    pub fn depth(&self) -> usize {
        match self {
            BspNode::Leaf { .. } => 0,
            BspNode::Internal { neg, pos, .. } => 1 + neg.depth().max(pos.depth()),
        }
    }
}

/// A solid represented as a BSP tree.
///
/// The solid is defined by its plane set and tree structure.
/// Vertices are implicit (3-plane intersections) — never stored as
/// coordinates until halfedge conversion. This is what makes chained
/// boolean operations robust: no coordinate drift, no tolerance matching.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BspSolid {
    /// All planes referenced by the tree.
    planes: Vec<Plane>,
    /// Root of the BSP tree.
    root: BspNode,
}

impl BspSolid {
    /// Create a new BSP solid from planes and a root node.
    pub fn new(planes: Vec<Plane>, root: BspNode) -> Self {
        Self { planes, root }
    }

    /// The planes defining this solid.
    pub fn planes(&self) -> &[Plane] {
        &self.planes
    }

    /// The root node of the BSP tree.
    pub fn root(&self) -> &BspNode {
        &self.root
    }

    /// Consume and return parts.
    pub fn into_parts(self) -> (Vec<Plane>, BspNode) {
        (self.planes, self.root)
    }

    /// Number of planes in the plane set.
    pub fn plane_count(&self) -> usize {
        self.planes.len()
    }

    /// Total number of nodes in the tree.
    pub fn node_count(&self) -> usize {
        self.root.node_count()
    }

    /// Simplify the tree in place.
    pub fn simplify(&mut self) {
        let root = std::mem::replace(&mut self.root, BspNode::empty());
        self.root = root.simplify();
    }

    /// Classify a point as inside (true) or outside (false) the solid.
    ///
    /// Walks the BSP tree, evaluating the point against each splitting
    /// plane. At internal nodes, the point goes to the neg child if the
    /// plane evaluates negative, or the pos child if positive. At a leaf,
    /// returns the leaf's solid flag.
    pub fn classify_point(&self, point: [f64; 3]) -> bool {
        classify_node(&self.root, &self.planes, point)
    }
}

/// Recursively classify a point against a BSP tree.
fn classify_node(node: &BspNode, planes: &[Plane], point: [f64; 3]) -> bool {
    match node {
        BspNode::Leaf { solid } => *solid,
        BspNode::Internal {
            plane_idx,
            neg,
            pos,
        } => {
            let plane = &planes[*plane_idx];
            let n = plane.raw_normal();
            let d = plane.raw_offset();
            let val = n[0] * point[0] + n[1] * point[1] + n[2] * point[2] + d;
            if val < 0.0 {
                classify_node(neg, planes, point)
            } else {
                classify_node(pos, planes, point)
            }
        }
    }
}

/// A collection of planes that implements `GeometrySource`.
pub struct PlaneSet(pub Vec<Plane>);

impl PlaneSet {
    /// Create a new plane set from a vector of planes.
    pub fn new(planes: Vec<Plane>) -> Self {
        Self(planes)
    }

    /// The planes in this set.
    pub fn planes(&self) -> &[Plane] {
        &self.0
    }
}

impl GeometrySource for PlaneSet {
    fn get_plane(&self, index: usize) -> Result<PlaneCoefficients, MathError> {
        let p = self.planes().get(index).ok_or_else(|| {
            MathError::InvalidInput(format!("Plane index {} out of bounds", index))
        })?;
        let n = p.raw_normal();
        PlaneCoefficients::try_new(n[0], n[1], n[2], p.raw_offset())
    }
}
