//! Data shapes for the geometry store.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use forge_math::{MathError, GeometrySource, PlaneCoefficients};
use forge_geom::Plane;
use forge_topo::handles::{FaceId, VertexId};

/// Side-car geometry storage mapping topology handles to geometric data.
///
/// The topology arena (Architecture Rule 2.3) stores only structural
/// connectivity. This store holds the geometric meaning: which plane
/// each face lies on, and where each vertex sits in 3D space.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeometryStore {
    /// Map from face handle to the plane the face lies on.
    face_planes: HashMap<u64, Plane>,
    /// Map from vertex handle to its 3D position.
    vertex_positions: HashMap<u64, [f64; 3]>,
}

impl GeometryStore {
    /// Create an empty geometry store.
    pub fn new() -> Self {
        Self {
            face_planes: HashMap::new(),
            vertex_positions: HashMap::new(),
        }
    }

    /// Associate a face with a plane.
    pub fn set_face_plane(&mut self, face: FaceId, plane: Plane) {
        self.face_planes.insert(pack_handle(face.index(), face.generation()), plane);
    }

    /// Retrieve the plane for a face.
    pub fn get_face_plane(&self, face: FaceId) -> Option<&Plane> {
        self.face_planes.get(&pack_handle(face.index(), face.generation()))
    }

    /// Associate a vertex with a 3D position.
    pub fn set_vertex_position(&mut self, vertex: VertexId, position: [f64; 3]) {
        self.vertex_positions.insert(
            pack_handle(vertex.index(), vertex.generation()),
            position,
        );
    }

    /// Retrieve the position for a vertex.
    pub fn get_vertex_position(&self, vertex: VertexId) -> Option<&[f64; 3]> {
        self.vertex_positions.get(&pack_handle(vertex.index(), vertex.generation()))
    }

    /// Number of face-plane associations.
    pub fn face_plane_count(&self) -> usize {
        self.face_planes.len()
    }

    /// Number of vertex-position associations.
    pub fn vertex_position_count(&self) -> usize {
        self.vertex_positions.len()
    }
}

impl Default for GeometryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl GeometrySource for GeometryStore {
    fn get_plane(&self, index: usize) -> Result<PlaneCoefficients, MathError> {
        for (&key, plane) in &self.face_planes {
            let stored_index = (key & 0xFFFF_FFFF) as usize;
            if stored_index == index {
                let n = plane.normal();
                return PlaneCoefficients::try_new(n[0], n[1], n[2], plane.offset());
            }
        }
        Err(MathError::InvalidInput(
            format!("No plane found for face index {}", index),
        ))
    }
}

/// Pack a (index, generation) pair into a single u64 key for HashMap lookup.
fn pack_handle(index: u32, generation: u32) -> u64 {
    (u64::from(generation) << 32) | u64::from(index)
}
