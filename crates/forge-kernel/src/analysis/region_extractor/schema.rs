//! Data shapes for the extracted region.
//!
//! DOMAIN: Self-contained, serializable topological sub-region for
//! standalone test case reproduction and causal debugging.
//!
//! DEPENDENCIES: `forge-topo` (handles, arena), `forge-geom` (Plane), serde

use std::collections::BTreeSet;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use forge_core::KernelError;
use forge_geom::Plane;
use forge_topo::arena::{TopologyArena, FaceData, HalfEdgeData, VertexData, LoopData};
use forge_topo::bitset::EntityBitset;
use forge_topo::handles::{FaceId, HalfEdgeId, LoopId, VertexId, ShellId, EdgeId};

/// A self-contained topological sub-region extracted from an arena.
///
/// Contains the face set, their boundary halfedges and vertices,
/// plus the associated geometry (planes for faces, positions for vertices).
/// Fully serializable for standalone test-case reproduction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedRegion {
    /// The face that seeded the extraction.
    seed_face: FaceId,
    /// How many rings were extracted.
    ring_depth: usize,
    /// Extracted face IDs (deterministically ordered).
    faces: EntityBitset,
    /// Halfedges bounding the extracted faces.
    half_edges: EntityBitset,
    /// Vertices referenced by the extracted halfedges.
    vertices: EntityBitset,
    /// Halfedge connectivity (keyed by halfedge index).
    half_edge_connectivity: BTreeMap<u32, SerializedHalfEdge>,
    /// Plane geometry for each extracted face (keyed by face index).
    face_planes: BTreeMap<u32, SerializedPlane>,
    /// Vertex positions for each extracted vertex (keyed by vertex index).
    vertex_positions: BTreeMap<u32, [f64; 3]>,
}

/// Serializable representation of a plane (serde-friendly).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedPlane {
    /// Normal vector.
    normal: [f64; 3],
    /// Signed offset from origin.
    offset: f64,
}

impl SerializedPlane {
    /// Create from a `Plane`.
    pub fn from_plane(plane: &Plane) -> Self {
        Self {
            normal: plane.normal(),
            offset: plane.offset(),
        }
    }

    /// Normal vector.
    pub fn get_normal(&self) -> [f64; 3] {
        self.normal
    }

    /// Signed offset.
    pub fn get_offset(&self) -> f64 {
        self.offset
    }
}

/// Serializable halfedge connectivity (twin, next, prev, face, origin indices).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedHalfEdge {
    /// Twin halfedge index.
    twin: u32,
    /// Next halfedge in face loop.
    next: u32,
    /// Previous halfedge in face loop.
    prev: u32,
    /// Owning face index.
    face: u32,
    /// Origin vertex index.
    origin: u32,
}

impl SerializedHalfEdge {
    /// Construct from arena halfedge data.
    pub fn from_half_edge_data(he: &forge_topo::arena::HalfEdgeData) -> Self {
        Self {
            twin: he.radial_next().index(),
            next: he.next().index(),
            prev: he.prev().index(),
            face: he.face().index(),
            origin: he.origin().index(),
        }
    }
}

impl ExtractedRegion {
    /// Construct a new extracted region.
    pub fn new(
        seed_face: FaceId,
        ring_depth: usize,
        faces: EntityBitset,
        half_edges: EntityBitset,
        vertices: EntityBitset,
        half_edge_connectivity: BTreeMap<u32, SerializedHalfEdge>,
        face_planes: BTreeMap<u32, SerializedPlane>,
        vertex_positions: BTreeMap<u32, [f64; 3]>,
    ) -> Self {
        Self {
            seed_face,
            ring_depth,
            faces,
            half_edges,
            vertices,
            half_edge_connectivity,
            face_planes,
            vertex_positions,
        }
    }

    /// The seed face that started the extraction.
    pub fn get_seed_face(&self) -> FaceId {
        self.seed_face
    }

    /// Number of rings extracted.
    pub fn get_ring_depth(&self) -> usize {
        self.ring_depth
    }

    /// The extracted face set.
    pub fn get_faces(&self) -> &EntityBitset {
        &self.faces
    }

    /// The extracted halfedge set.
    pub fn get_half_edges(&self) -> &EntityBitset {
        &self.half_edges
    }

    /// The extracted vertex set.
    pub fn get_vertices(&self) -> &EntityBitset {
        &self.vertices
    }

    /// Plane geometry for extracted faces.
    pub fn get_face_planes(&self) -> &BTreeMap<u32, SerializedPlane> {
        &self.face_planes
    }

    /// Vertex positions for extracted vertices.
    pub fn get_vertex_positions(&self) -> &BTreeMap<u32, [f64; 3]> {
        &self.vertex_positions
    }

    /// Number of faces in the extracted region.
    pub fn face_count(&self) -> usize {
        self.faces.count() as usize
    }

    /// Number of vertices in the extracted region.
    pub fn vertex_count(&self) -> usize {
        self.vertices.count() as usize
    }

    /// Number of halfedges in the extracted region.
    pub fn half_edge_count(&self) -> usize {
        self.half_edges.count() as usize
    }

    /// Whether the region is empty.
    pub fn is_empty(&self) -> bool {
        self.faces.is_empty()
    }

    /// Serialize to a JSON string for standalone test-case reproduction.
    ///
    /// Used by the fuzzer (P4.4), disagreement protocol (P1.3), and
    /// token-budget workflows (DZ-3) to persist minimal regions to disk.
    pub fn to_json(&self) -> Result<String, KernelError> {
        serde_json::to_string_pretty(self).map_err(|e| KernelError::InternalError {
            message: format!("Failed to serialize ExtractedRegion: {}", e),
            context: None,
        })
    }

    /// Deserialize from a JSON string previously produced by `to_json`.
    pub fn from_json(s: &str) -> Result<Self, KernelError> {
        serde_json::from_str(s).map_err(|e| KernelError::InternalError {
            message: format!("Failed to deserialize ExtractedRegion: {}", e),
            context: None,
        })
    }

    /// Reconstruct a `TopologyArena` from the serialized region data.
    ///
    /// Rebuilds vertices, faces, loops, and halfedges with their original
    /// connectivity. Uses index-based slot pre-allocation so that handles
    /// from the original arena remain valid in the reconstructed one.
    pub fn to_arena(&self) -> Result<TopologyArena, KernelError> {
        let mut arena = TopologyArena::new();

        let max_vtx = self.vertices.iter_ones().max().unwrap_or(0);
        let max_face = self.faces.iter_ones().max().unwrap_or(0);
        let max_he = self.half_edges.iter_ones().max().unwrap_or(0);

        let placeholder_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);
        let placeholder_loop = LoopId::from_raw_parts(u32::MAX, 0);
        let placeholder_shell = ShellId::from_raw_parts(u32::MAX, 0);
        let placeholder_edge = EdgeId::from_raw_parts(u32::MAX, 0);

        for _ in 0..=max_vtx {
            arena.insert_vertex(VertexData::new(placeholder_he), None);
        }
        for _ in 0..=max_face {
            let fid = arena.insert_face(FaceData::new(placeholder_loop, placeholder_shell), None);
            arena.insert_loop(LoopData::new(placeholder_he, fid), None);
        }
        for _ in 0..=max_he {
            arena.insert_half_edge(HalfEdgeData::new(
                placeholder_he,
                placeholder_he,
                placeholder_he,
                FaceId::from_raw_parts(0, 0),
                VertexId::from_raw_parts(0, 0),
                placeholder_edge,
            ), None);
        }

        for (&he_idx, conn) in &self.half_edge_connectivity {
            let he_id = HalfEdgeId::from_raw_parts(he_idx, 0);
            let he_mut = arena.get_half_edge_mut(he_id)?;
            he_mut.set_radial_next(HalfEdgeId::from_raw_parts(conn.twin, 0));
            he_mut.set_next(HalfEdgeId::from_raw_parts(conn.next, 0));
            he_mut.set_prev(HalfEdgeId::from_raw_parts(conn.prev, 0));
            he_mut.set_face(FaceId::from_raw_parts(conn.face, 0));
            he_mut.set_origin(VertexId::from_raw_parts(conn.origin, 0));
        }

        for vtx_id in self.vertices.iter_ones() {
            let first_outgoing = self.half_edge_connectivity.iter()
                .find(|(_, conn)| conn.origin == vtx_id);
            if let Some((&he_idx, _)) = first_outgoing {
                arena.get_vertex_mut(VertexId::from_raw_parts(vtx_id, 0))?
                    .set_outgoing(HalfEdgeId::from_raw_parts(he_idx, 0));
            }
        }

        for face_id in self.faces.iter_ones() {
            let first_he = self.half_edge_connectivity.iter()
                .find(|(_, conn)| conn.face == face_id);
            if let Some((&he_idx, _)) = first_he {
                let loop_id = LoopId::from_raw_parts(face_id, 0);
                arena.get_loop_mut(loop_id)?
                    .set_half_edge(HalfEdgeId::from_raw_parts(he_idx, 0));
                arena.get_face_mut(FaceId::from_raw_parts(face_id, 0))?
                    .set_outer_loop(loop_id);
            }
        }

        Ok(arena)
    }
}
