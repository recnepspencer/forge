//! TopologyArena struct definition and constructor.
//!
//! DOMAIN: The central entity storage container. Holds slot vectors
//! and free-lists for all topology entity types.

use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use smallvec::SmallVec;
use std::collections::HashMap;

use crate::semantic_attributes::AttributeStore;
use crate::b_rep::data::storage::slot::Slot;
use crate::b_rep::data::mesh::{FaceData, HalfEdgeData, VertexData, LoopData, EdgeData, CoedgeInfo};
use crate::b_rep::data::containment::{BodyData, LumpData, RegionData, ShellData};
use crate::handles::{FaceId, HalfEdgeId, ShellId, VertexId, CurveRef};

/// Entity storage for the halfedge mesh.
///
/// Holds faces, halfedges, vertices, loops, edges, shells, bodies,
/// lumps, and regions in arena-allocated vectors.
/// Each slot tracks its generation counter for stale-handle detection.
/// This struct is `Clone`-able and lives inside `Arc` for structural sharing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyArena {
    // ── Mesh entity slots ───────────────────────────────────────
    pub(crate) face_slots: Vec<Slot<FaceData>>,
    pub(crate) half_edge_slots: Vec<Slot<HalfEdgeData>>,
    pub(crate) vertex_slots: Vec<Slot<VertexData>>,
    pub(crate) loop_slots: Vec<Slot<LoopData>>,
    pub(crate) edge_slots: Vec<Slot<EdgeData>>,

    // ── Containment entity slots ────────────────────────────────
    pub(crate) shell_slots: Vec<Slot<ShellData>>,
    pub(crate) body_slots: Vec<Slot<BodyData>>,
    pub(crate) lump_slots: Vec<Slot<LumpData>>,
    pub(crate) region_slots: Vec<Slot<RegionData>>,

    // ── Free-lists ──────────────────────────────────────────────
    #[serde(default)]
    pub(crate) free_face_head: Option<u32>,
    #[serde(default)]
    pub(crate) free_half_edge_head: Option<u32>,
    #[serde(default)]
    pub(crate) free_vertex_head: Option<u32>,
    #[serde(default)]
    pub(crate) free_loop_head: Option<u32>,
    #[serde(default)]
    pub(crate) free_edge_head: Option<u32>,
    #[serde(default)]
    pub(crate) free_shell_head: Option<u32>,
    #[serde(default)]
    pub(crate) free_body_head: Option<u32>,
    #[serde(default)]
    pub(crate) free_lump_head: Option<u32>,
    #[serde(default)]
    pub(crate) free_region_head: Option<u32>,

    // ── Attribute side-car ──────────────────────────────────────
    pub(crate) attribute_store: AttributeStore,

    // ── O(1) Active Counts ──────────────────────────────────────
    pub(crate) active_face_count: usize,
    pub(crate) active_half_edge_count: usize,
    pub(crate) active_vertex_count: usize,
    pub(crate) active_loop_count: usize,
    pub(crate) active_shell_count: usize,
    pub(crate) active_body_count: usize,
    pub(crate) active_lump_count: usize,
    pub(crate) active_region_count: usize,
    pub(crate) active_edge_count: usize,

    // ── Side-car vectors (parallel to entity slot vectors) ───────
    // Metadata stripped from entity structs to keep them pure connectivity.
    // Grow in lockstep with entity slots via insert_remove.rs hooks.

    /// Bridge flag per halfedge slot (synthetic zero-width bridge from BridgeEdge).
    #[serde(default)]
    pub(crate) bridge_flags: Vec<bool>,
    /// Coedge metadata per halfedge slot (UV trim curve + direction sense).
    #[serde(default)]
    pub(crate) coedge_data: Vec<Option<CoedgeInfo>>,
    /// 3D curve reference per edge slot.
    #[serde(default)]
    pub(crate) edge_curves: Vec<Option<CurveRef>>,
    /// 3-plane intersection provenance per vertex slot.
    #[serde(default)]
    pub(crate) vertex_provenance: Vec<Option<[usize; 3]>>,
    /// Extra disk entries for non-manifold vertices (sparse side-car).
    #[serde(default)]
    pub(crate) nmt_extra_disks: HashMap<VertexId, SmallVec<[HalfEdgeId; 2]>>,
    /// O(1) NMT membership bitset parallel to `vertex_slots`.
    #[serde(default)]
    pub(crate) vertex_is_nmt: Vec<bool>,

    // ── O(1) Reverse Indexes (derived, not serialized) ──────────
    // SmallVec inline storage avoids heap allocation for typical valence.
    #[serde(skip)]
    pub(crate) shell_faces: IndexMap<ShellId, SmallVec<[FaceId; 8]>>,
    #[serde(skip)]
    pub(crate) face_halfedges: IndexMap<FaceId, SmallVec<[HalfEdgeId; 6]>>,
    #[serde(skip)]
    pub(crate) vertex_halfedges: IndexMap<VertexId, SmallVec<[HalfEdgeId; 6]>>,
}

impl TopologyArena {
    /// Create an empty arena with no entities.
    pub fn new() -> Self {
        Self {
            face_slots: Vec::new(),
            half_edge_slots: Vec::new(),
            vertex_slots: Vec::new(),
            loop_slots: Vec::new(),
            edge_slots: Vec::new(),
            shell_slots: Vec::new(),
            body_slots: Vec::new(),
            lump_slots: Vec::new(),
            region_slots: Vec::new(),
            free_face_head: None,
            free_half_edge_head: None,
            free_vertex_head: None,
            free_loop_head: None,
            free_edge_head: None,
            free_shell_head: None,
            free_body_head: None,
            free_lump_head: None,
            free_region_head: None,
            attribute_store: AttributeStore::new(),
            active_face_count: 0,
            active_half_edge_count: 0,
            active_vertex_count: 0,
            active_loop_count: 0,
            active_shell_count: 0,
            active_body_count: 0,
            active_lump_count: 0,
            active_region_count: 0,
            active_edge_count: 0,
            bridge_flags: Vec::new(),
            coedge_data: Vec::new(),
            edge_curves: Vec::new(),
            vertex_provenance: Vec::new(),
            nmt_extra_disks: HashMap::new(),
            vertex_is_nmt: Vec::new(),
            shell_faces: IndexMap::new(),
            face_halfedges: IndexMap::new(),
            vertex_halfedges: IndexMap::new(),
        }
    }

    /// Occupy a recycled slot if available, otherwise append a new slot.
    pub(crate) fn insert_slot<T: Clone>(
        slots: &mut Vec<Slot<T>>,
        free_head: &mut Option<u32>,
        data: T,
    ) -> (u32, u32) {
        if let Some(index) = *free_head {
            let slot = &mut slots[index as usize];
            *free_head = slot.next_free;
            let generation = slot.occupy(data);
            return (index, generation);
        }
        let index = slots.len() as u32;
        let mut slot = Slot::empty();
        let generation = slot.occupy(data);
        slots.push(slot);
        (index, generation)
    }

    /// Reserve a slot without data. Returns `(index, generation)`.
    ///
    /// The slot exists but has `data = None` until `populate_slot` fills it.
    /// This allows circular references to be wired before any data is stored.
    pub(crate) fn reserve_slot<T: Clone>(
        slots: &mut Vec<Slot<T>>,
        free_head: &mut Option<u32>,
    ) -> (u32, u32) {
        if let Some(index) = *free_head {
            let slot = &mut slots[index as usize];
            *free_head = slot.next_free;
            slot.next_free = None;
            return (index, slot.generation);
        }
        let index = slots.len() as u32;
        slots.push(Slot::empty());
        (index, 0)
    }

    /// Populate a previously reserved slot with data.
    ///
    /// # Panics
    /// Panics if the slot is already occupied.
    pub(crate) fn populate_slot<T: Clone>(
        slots: &mut Vec<Slot<T>>,
        index: u32,
        data: T,
    ) {
        let slot = &mut slots[index as usize];
        debug_assert!(slot.data.is_none(), "populate_slot called on occupied slot {index}");
        slot.data = Some(data);
        slot.version = 0;
    }

    /// Read-only access to the attribute store.
    pub fn get_attribute_store(&self) -> &AttributeStore {
        &self.attribute_store
    }

    /// Mutable access to the attribute store.
    pub fn get_attribute_store_mut(&mut self) -> &mut AttributeStore {
        &mut self.attribute_store
    }
}

impl Default for TopologyArena {
    fn default() -> Self {
        Self::new()
    }
}
