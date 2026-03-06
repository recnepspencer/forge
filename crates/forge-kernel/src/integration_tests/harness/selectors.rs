//! Entity selector DSL for querying solid topology and geometry.
//!
//! DOMAIN: The geometry kernel's equivalent of Laravel's Eloquent queries.
//! Instead of manually iterating handles, selectors let tests find entities
//! by geometric properties.
//!
//! ```rust,ignore
//! let top_face = select(&envelope)
//!     .faces()
//!     .where_normal_near([0.0, 0.0, 1.0], 0.1)
//!     .one()?;
//! ```
//!
//! All geometry computation delegates to production algorithms in
//! `geometry::logic::measurements`.

use forge_core::KernelError;
use forge_topo::handles::{EdgeId, FaceId, VertexId};

use crate::engine::facade::SolidEnvelope;
use crate::geometry::facade::{edge_length, GeometryView};

/// Create a selector for querying entities in a `SolidEnvelope`.
pub fn select(env: &SolidEnvelope) -> Selector<'_> {
    Selector { env }
}

/// Entry point for entity queries.
pub struct Selector<'a> {
    env: &'a SolidEnvelope,
}

impl<'a> Selector<'a> {
    /// Start a face query.
    pub fn faces(&self) -> FaceQuery<'a> {
        let all: Vec<FaceId> = self
            .env
            .topology()
            .arena()
            .iter_faces()
            .map(|(fid, _)| fid)
            .collect();
        FaceQuery {
            env: self.env,
            candidates: all,
        }
    }

    /// Start an edge query.
    pub fn edges(&self) -> EdgeQuery<'a> {
        let all: Vec<EdgeId> = self
            .env
            .topology()
            .arena()
            .iter_edges()
            .map(|(eid, _)| eid)
            .collect();
        EdgeQuery {
            env: self.env,
            candidates: all,
        }
    }

    /// Start a vertex query.
    pub fn vertices(&self) -> VertexQuery<'a> {
        let all: Vec<VertexId> = self
            .env
            .topology()
            .arena()
            .iter_vertices()
            .map(|(vid, _)| vid)
            .collect();
        VertexQuery {
            env: self.env,
            candidates: all,
        }
    }

    /// Query edges of a specific face.
    pub fn edges_of(&self, face: FaceId) -> EdgeQuery<'a> {
        let arena = self.env.topology().arena();
        let hes = arena.halfedges_of_face(face);
        let edges: Vec<EdgeId> = hes
            .iter()
            .filter_map(|he_id| arena.get_half_edge(*he_id).ok().map(|he| he.edge()))
            .collect();
        EdgeQuery {
            env: self.env,
            candidates: edges,
        }
    }

    /// Query vertices of a specific face.
    pub fn vertices_of(&self, face: FaceId) -> VertexQuery<'a> {
        let arena = self.env.topology().arena();
        let hes = arena.halfedges_of_face(face);
        let verts: Vec<VertexId> = hes
            .iter()
            .filter_map(|he_id| arena.get_half_edge(*he_id).ok().map(|he| he.origin()))
            .collect();
        VertexQuery {
            env: self.env,
            candidates: verts,
        }
    }
}

// ── Face queries ─────────────────────────────────────────────────────────────

/// Chainable face filter.
pub struct FaceQuery<'a> {
    env: &'a SolidEnvelope,
    candidates: Vec<FaceId>,
}

impl<'a> FaceQuery<'a> {
    /// Keep faces whose normal is within `tolerance` of `target` (dot product > 1 - tol).
    pub fn where_normal_near(mut self, target: [f64; 3], tolerance: f64) -> Self {
        let geom = self.env.geometry();
        self.candidates.retain(|&fid| {
            if let Some(plane) = geom.get_face_plane(fid) {
                let n = plane.normal();
                let dot = n[0] * target[0] + n[1] * target[1] + n[2] * target[2];
                (dot - 1.0).abs() < tolerance
            } else {
                false
            }
        });
        self
    }

    /// Return all matching faces.
    pub fn all(self) -> Vec<FaceId> {
        self.candidates
    }

    /// Return exactly one matching face, or error.
    pub fn one(self) -> Result<FaceId, KernelError> {
        match self.candidates.len() {
            0 => Err(KernelError::InvalidInput {
                message: "Face selector matched 0 faces".to_string(),
                context: None,
            }),
            1 => Ok(self.candidates[0]),
            n => Err(KernelError::InvalidInput {
                message: format!("Face selector matched {n} faces, expected 1"),
                context: None,
            }),
        }
    }

    /// Return the first match, or error if none.
    pub fn first(self) -> Result<FaceId, KernelError> {
        self.candidates
            .first()
            .copied()
            .ok_or_else(|| KernelError::InvalidInput {
                message: "Face selector matched 0 faces".to_string(),
                context: None,
            })
    }

    /// Return the count of matching faces.
    pub fn count(&self) -> usize {
        self.candidates.len()
    }
}

// ── Edge queries ─────────────────────────────────────────────────────────────

/// Chainable edge filter.
pub struct EdgeQuery<'a> {
    env: &'a SolidEnvelope,
    candidates: Vec<EdgeId>,
}

impl<'a> EdgeQuery<'a> {
    /// Keep edges longer than `min_length`.
    pub fn where_length_above(mut self, min_length: f64) -> Self {
        let arena = self.env.topology().arena();
        let geom = self.env.geometry();
        self.candidates
            .retain(|&eid| edge_length(arena, geom, eid).map_or(false, |len| len > min_length));
        self
    }

    /// Keep edges with length between min and max.
    pub fn where_length_between(mut self, min: f64, max: f64) -> Self {
        let arena = self.env.topology().arena();
        let geom = self.env.geometry();
        self.candidates.retain(|&eid| {
            edge_length(arena, geom, eid).map_or(false, |len| len >= min && len <= max)
        });
        self
    }

    /// Return all matching edges.
    pub fn all(self) -> Vec<EdgeId> {
        self.candidates
    }

    /// Return exactly one matching edge, or error.
    pub fn one(self) -> Result<EdgeId, KernelError> {
        match self.candidates.len() {
            0 => Err(KernelError::InvalidInput {
                message: "Edge selector matched 0 edges".to_string(),
                context: None,
            }),
            1 => Ok(self.candidates[0]),
            n => Err(KernelError::InvalidInput {
                message: format!("Edge selector matched {n} edges, expected 1"),
                context: None,
            }),
        }
    }

    /// Return the count of matching edges.
    pub fn count(&self) -> usize {
        self.candidates.len()
    }
}

// ── Vertex queries ───────────────────────────────────────────────────────────

/// Chainable vertex filter.
pub struct VertexQuery<'a> {
    env: &'a SolidEnvelope,
    candidates: Vec<VertexId>,
}

impl<'a> VertexQuery<'a> {
    /// Keep vertices within `radius` of a target position.
    pub fn where_near(mut self, target: [f64; 3], radius: f64) -> Self {
        let geom = self.env.geometry();
        self.candidates.retain(|&vid| {
            if let Some(pos) = geom.get_vertex_position(vid) {
                let dist = forge_geom::facade::distance(pos, &target);
                dist <= radius
            } else {
                false
            }
        });
        self
    }

    /// Return all matching vertices.
    pub fn all(self) -> Vec<VertexId> {
        self.candidates
    }

    /// Return exactly one matching vertex, or error.
    pub fn one(self) -> Result<VertexId, KernelError> {
        match self.candidates.len() {
            0 => Err(KernelError::InvalidInput {
                message: "Vertex selector matched 0 vertices".to_string(),
                context: None,
            }),
            1 => Ok(self.candidates[0]),
            n => Err(KernelError::InvalidInput {
                message: format!("Vertex selector matched {n} vertices, expected 1"),
                context: None,
            }),
        }
    }

    /// Return the count of matching vertices.
    pub fn count(&self) -> usize {
        self.candidates.len()
    }
}
