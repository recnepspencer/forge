//! Solid envelope — the complete, queryable result of any solid-producing operation.
//!
//! DOMAIN: Unifies `MeshBuildResult`, `FeatureOutput`, and test handle types
//! (`CubeHandles`, `TetraHandles`) into a single type with lazy handle extraction.
//!
//! Audit metadata (decisions, metrics, lineage) lives in the
//! `OperationResult<SolidEnvelope>` envelope, not here.

use std::cell::OnceCell;

use serde::{Deserialize, Serialize};

use forge_topo::handles::{BodyId, EdgeId, FaceId, ShellId, VertexId};
use forge_topo::transactions::TopologyState;

use crate::geometry::facade::{GeometryStore, GeometryView};

/// The complete, queryable result of any solid-producing operation.
///
/// Owns topology and geometry. Handle lists (bodies, shells, faces, vertices,
/// edges) are extracted lazily on first access via `OnceCell` — zero allocation
/// at construction time.
///
/// Audit metadata (decisions, metrics, lineage) lives in the
/// `OperationResult<SolidEnvelope>` envelope that wraps this, not here.
///
/// # Cache Invalidation Safety
///
/// The `OnceCell` handle caches derive from `topology`, which is immutable
/// after construction. There is no `topology_mut()` accessor — only
/// `geometry_mut()` exists (for pipeline coordinate conditioning). Since
/// geometry changes don't affect handle lists (which depend only on arena
/// adjacency), the caches can never become stale.
///
/// If topology mutation were ever added, these caches would need to be
/// invalidated or replaced with a versioned lookup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolidEnvelope {
    /// Committed topology snapshot.
    topology: TopologyState,
    /// Unified geometry store (plane + vertex positions).
    geometry: GeometryStore,

    // ── Lazily-extracted handle caches ─────────────────────────────
    // Derived from `topology`; recomputed on deserialization.
    #[serde(skip)]
    bodies: OnceCell<Vec<BodyId>>,
    #[serde(skip)]
    shells: OnceCell<Vec<ShellId>>,
    #[serde(skip)]
    faces: OnceCell<Vec<FaceId>>,
    #[serde(skip)]
    vertices: OnceCell<Vec<VertexId>>,
    #[serde(skip)]
    edges: OnceCell<Vec<EdgeId>>,
}

impl SolidEnvelope {
    // ── Construction ──────────────────────────────────────────────────

    /// Create a new `SolidEnvelope` from a committed topology and geometry.
    ///
    /// Handle lists are NOT extracted eagerly — they are computed lazily
    /// on first access. This means construction is zero-allocation beyond
    /// the topology and geometry themselves.
    pub fn new(topology: TopologyState, geometry: GeometryStore) -> Self {
        Self {
            topology,
            geometry,
            bodies: OnceCell::new(),
            shells: OnceCell::new(),
            faces: OnceCell::new(),
            vertices: OnceCell::new(),
            edges: OnceCell::new(),
        }
    }

    // ── Core accessors ────────────────────────────────────────────────

    /// The committed topology snapshot.
    pub fn topology(&self) -> &TopologyState {
        &self.topology
    }

    /// The unified geometry store.
    pub fn geometry(&self) -> &GeometryStore {
        &self.geometry
    }

    /// Mutable access to the geometry store (e.g., for coordinate restoration).
    pub fn geometry_mut(&mut self) -> &mut GeometryStore {
        &mut self.geometry
    }

    // ── Lazy handle accessors ─────────────────────────────────────────

    /// All body handles in this solid. Computed lazily on first access.
    pub fn bodies(&self) -> &[BodyId] {
        self.bodies.get_or_init(|| {
            self.topology
                .arena()
                .iter_bodies()
                .map(|(id, _)| id)
                .collect()
        })
    }

    /// All shell handles in this solid. Computed lazily on first access.
    pub fn shells(&self) -> &[ShellId] {
        self.shells.get_or_init(|| {
            self.topology
                .arena()
                .iter_shells()
                .map(|(id, _)| id)
                .collect()
        })
    }

    /// All face handles in this solid. Computed lazily on first access.
    pub fn faces(&self) -> &[FaceId] {
        self.faces.get_or_init(|| {
            self.topology
                .arena()
                .iter_faces()
                .map(|(id, _)| id)
                .collect()
        })
    }

    /// All vertex handles in this solid. Computed lazily on first access.
    pub fn vertices(&self) -> &[VertexId] {
        self.vertices.get_or_init(|| {
            self.topology
                .arena()
                .iter_vertices()
                .map(|(id, _)| id)
                .collect()
        })
    }

    /// All edge handles in this solid. Computed lazily on first access.
    pub fn edges(&self) -> &[EdgeId] {
        self.edges.get_or_init(|| {
            self.topology
                .arena()
                .iter_edges()
                .map(|(id, _)| id)
                .collect()
        })
    }

    // ── Convenience counts ────────────────────────────────────────────

    /// Number of faces.
    pub fn face_count(&self) -> usize {
        self.faces().len()
    }

    /// Number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.vertices().len()
    }

    /// Number of edges.
    pub fn edge_count(&self) -> usize {
        self.edges().len()
    }

    // ── Convenience: single-body results ──────────────────────────────

    /// The single body in this solid.
    ///
    /// # Panics
    /// Panics if the solid contains zero or more than one body.
    pub fn body(&self) -> BodyId {
        let bs = self.bodies();
        assert_eq!(
            bs.len(),
            1,
            "SolidEnvelope::body() requires exactly 1 body, found {}",
            bs.len()
        );
        bs[0]
    }

    /// The single shell in this solid.
    ///
    /// # Panics
    /// Panics if the solid contains zero or more than one shell.
    pub fn shell(&self) -> ShellId {
        let ss = self.shells();
        assert_eq!(
            ss.len(),
            1,
            "SolidEnvelope::shell() requires exactly 1 shell, found {}",
            ss.len()
        );
        ss[0]
    }

    // ── Fingerprinting ────────────────────────────────────────────────

    /// Topology-only fingerprint (structural hash).
    ///
    /// Hashes the arena adjacency structure. Fast, but won't detect
    /// geometry-only changes (moved vertices, rotated planes).
    pub fn topology_fingerprint(&self) -> u128 {
        forge_topo::transactions::compute_arena_topology_hash(self.topology.arena())
    }

    /// Geometry-only fingerprint.
    ///
    /// Hashes vertex positions and face planes without incorporating topology.
    /// This is the semantic geometry version boundary for reactive evaluation.
    pub fn geometry_fingerprint(&self) -> u128 {
        let mut hash = 0_u128;

        for (v_id, _) in self.topology.arena().iter_vertices() {
            if let Some(pos) = self.geometry.get_vertex_position(v_id) {
                for coord in pos {
                    hash = hash.wrapping_mul(31).wrapping_add(coord.to_bits() as u128);
                }
            }
        }

        for (f_id, _) in self.topology.arena().iter_faces() {
            if let Some(plane) = self.geometry.get_face_plane(f_id) {
                for coord in plane.normal() {
                    hash = hash.wrapping_mul(31).wrapping_add(coord.to_bits() as u128);
                }
                hash = hash
                    .wrapping_mul(31)
                    .wrapping_add(plane.offset().to_bits() as u128);
            }
        }

        hash
    }

    /// Full deterministic fingerprint (topology + geometry).
    ///
    /// Hashes topology arena, all vertex positions (f64 bit-exact),
    /// and all face plane normals + offsets. This is the single source
    /// of truth for envelope-level equality.
    ///
    /// # Performance
    ///
    /// O(V + F) — iterates all vertices and faces. Use for:
    /// - Determinism verification in test harnesses
    /// - Serialization checksums
    /// - Undo/redo diffing
    /// - Full-detail pipeline fingerprinting
    ///
    /// For hot-path change detection, use `topology_fingerprint()` instead.
    pub fn full_fingerprint(&self) -> u128 {
        self.topology_fingerprint()
            .wrapping_mul(31)
            .wrapping_add(self.geometry_fingerprint())
    }

    // ── Lifecycle / decomposition ─────────────────────────────────────

    /// Consume into topology and geometry parts.
    pub fn into_parts(self) -> (TopologyState, GeometryStore) {
        (self.topology, self.geometry)
    }

    /// Consume into a mutable draft and geometry for further mutation.
    pub fn into_draft(self) -> (forge_topo::transactions::MutableDraft, GeometryStore) {
        (self.topology.into_mutation(), self.geometry)
    }
}
