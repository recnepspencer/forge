//! Macro-generated CRUD for all topology entity types.
//!
//! DOMAIN: The `define_topology_entities!` macro generates insert, get,
//! get_mut, remove, iter, count, active_indices, generation, and version
//! methods for each registered entity type on `TopologyArena`.
//! Also generates proxy insert/remove methods on `MutableDraft`.
//!
//! The `loop` keyword requires special handling — its methods are generated
//! via a dedicated arm that spells out each method name explicitly.

use forge_core::KernelError;
use crate::arena::slot::{validate_generation, cold_err_bounds, cold_err_deleted};
use crate::arena::core::TopologyArena;

/// Generate accessor methods (get, get_mut, iter, count, indices, generation, version).
///
/// The `loop` entry uses explicit method names to avoid keyword issues.
macro_rules! define_entity_accessors {
    // Standard arm: method names derived from `$m` via paste
    (@standard $m:ident, $pl:ident, $label:expr, $id:ty, $data:ty, $slots:ident, $count:ident) => {
        paste::paste! {
            impl TopologyArena {
                #[doc = concat!("Get a ", $label, " by handle, validating the generation.")]
                #[inline]
                pub fn [<get_ $m>](&self, id: $id) -> Result<&$data, KernelError> {
                    let slot = self.$slots.get(id.index() as usize)
                        .ok_or_else(|| cold_err_bounds($label, id.index(), id.generation()))?;
                    validate_generation(slot.generation, id.generation(), $label, id.index())?;
                    slot.data.as_ref()
                        .ok_or_else(|| cold_err_deleted($label, id.index(), id.generation(), slot.generation))
                }

                #[doc = concat!("Get a mutable reference to a ", $label, " by handle.")]
                #[inline]
                pub fn [<get_ $m _mut>](&mut self, id: $id) -> Result<&mut $data, KernelError> {
                    let slot = self.$slots.get_mut(id.index() as usize)
                        .ok_or_else(|| cold_err_bounds($label, id.index(), id.generation()))?;
                    validate_generation(slot.generation, id.generation(), $label, id.index())?;
                    slot.version += 1;
                    slot.data.as_mut()
                        .ok_or_else(|| cold_err_deleted($label, id.index(), id.generation(), slot.generation))
                }

                #[doc = concat!("Iterate over all active ", $label, "s.")]
                pub fn [<iter_ $pl>](&self) -> impl Iterator<Item = ($id, &$data)> {
                    self.$slots.iter().enumerate().filter_map(|(i, slot)| {
                        let data = slot.data.as_ref()?;
                        Some((<$id>::new(i as u32, slot.generation), data))
                    })
                }

                #[doc = concat!("Count of active ", $label, "s.")]
                pub fn [<$m _count>](&self) -> usize { self.$count }

                #[doc = concat!("Indices of all active ", $label, " slots.")]
                pub fn [<active_ $m _indices>](&self) -> impl Iterator<Item = usize> + '_ {
                    self.$slots.iter().enumerate()
                        .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
                }

                #[doc = concat!("Generation of ", $label, " at slot index.")]
                pub fn [<$m _generation>](&self, index: usize) -> Option<u32> {
                    self.$slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
                }

                #[doc = concat!("Version of ", $label, " at slot index.")]
                pub fn [<$m _version>](&self, index: usize) -> Option<u32> {
                    self.$slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
                }
            }
        }
    };
}

/// Generate insert/remove for entities WITHOUT index hooks.
macro_rules! define_plain_crud {
    (@standard $m:ident, $label:expr, $id:ty, $data:ty, $slots:ident, $free_head:ident, $count:ident) => {
        paste::paste! {
            impl TopologyArena {
                #[doc = concat!("Insert a new ", $label, ", returning its handle.")]
                pub(crate) fn [<insert_ $m>](&mut self, data: $data) -> $id {
                    let (index, gen) = Self::insert_slot(&mut self.$slots, &mut self.$free_head, data);
                    self.$count += 1;
                    <$id>::new(index, gen)
                }

                #[doc = concat!("Remove a ", $label, ", bumping the slot generation.")]
                pub(crate) fn [<remove_ $m>](&mut self, id: $id) -> Result<$data, KernelError> {
                    let slot = self.$slots.get_mut(id.index() as usize)
                        .ok_or_else(|| cold_err_bounds($label, id.index(), id.generation()))?;
                    validate_generation(slot.generation, id.generation(), $label, id.index())?;
                    let data = slot.data.take()
                        .ok_or_else(|| cold_err_deleted($label, id.index(), id.generation(), slot.generation))?;
                    slot.generation += 1;
                    slot.next_free = self.$free_head;
                    self.$free_head = Some(id.index());
                    self.$count -= 1;
                    Ok(data)
                }
            }
        }
    };
}

/// Generate MutableDraft proxy methods.
macro_rules! define_draft_proxies {
    (@standard $m:ident, $id:ty, $data:ty) => {
        paste::paste! {
            impl crate::state::MutableDraft {
                #[doc = concat!("Insert a new ", stringify!($m), ".")]
                pub fn [<insert_ $m>](&mut self, data: $data) -> $id {
                    self.arena.[<insert_ $m>](data)
                }

                #[doc = concat!("Remove a ", stringify!($m), ".")]
                pub fn [<remove_ $m>](&mut self, id: $id) -> Result<$data, KernelError> {
                    self.arena.[<remove_ $m>](id)
                }
            }
        }
    };
}

// ═══════════════════════════════════════════════════════════════════
// Entity Registration Table
// Add a new entity? Add one invocation below + one in each section.
// ═══════════════════════════════════════════════════════════════════

// ── Accessor methods (get, get_mut, iter, count, indices, generation, version) ──

define_entity_accessors!(@standard face,      faces,      "Face",      FaceId,     FaceData,     face_slots,      active_face_count);
define_entity_accessors!(@standard half_edge, half_edges, "HalfEdge",  HalfEdgeId, HalfEdgeData, half_edge_slots, active_half_edge_count);
define_entity_accessors!(@standard vertex,    vertices,   "Vertex",    VertexId,   VertexData,   vertex_slots,    active_vertex_count);
define_entity_accessors!(@standard edge,      edges,      "Edge",      EdgeId,     EdgeData,     edge_slots,      active_edge_count);
define_entity_accessors!(@standard shell,     shells,     "Shell",     ShellId,    ShellData,    shell_slots,     active_shell_count);
define_entity_accessors!(@standard region,    regions,    "Region",    RegionId,   RegionData,   region_slots,    active_region_count);
define_entity_accessors!(@standard lump,      lumps,      "Lump",      LumpId,     LumpData,     lump_slots,      active_lump_count);
define_entity_accessors!(@standard body,      bodies,     "Body",      BodyId,     BodyData,     body_slots,      active_body_count);

// Loop — keyword-safe explicit methods
impl TopologyArena {
    /// Get a loop by handle, validating the generation.
    #[inline]
    pub fn get_loop(&self, id: LoopId) -> Result<&LoopData, KernelError> {
        let slot = self.loop_slots.get(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Loop", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Loop", id.index())?;
        slot.data.as_ref()
            .ok_or_else(|| cold_err_deleted("Loop", id.index(), id.generation(), slot.generation))
    }

    /// Get a mutable reference to a loop by handle.
    #[inline]
    pub fn get_loop_mut(&mut self, id: LoopId) -> Result<&mut LoopData, KernelError> {
        let slot = self.loop_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Loop", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Loop", id.index())?;
        slot.version += 1;
        slot.data.as_mut()
            .ok_or_else(|| cold_err_deleted("Loop", id.index(), id.generation(), slot.generation))
    }

    /// Iterate over all active loops.
    pub fn iter_loops(&self) -> impl Iterator<Item = (LoopId, &LoopData)> {
        self.loop_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((LoopId::new(i as u32, slot.generation), data))
        })
    }

    /// Count of active loops.
    pub fn loop_count(&self) -> usize { self.active_loop_count }

    /// Indices of all active loop slots.
    pub fn active_loop_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.loop_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Generation of loop at slot index.
    pub fn loop_generation(&self, index: usize) -> Option<u32> {
        self.loop_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Version of loop at slot index.
    pub fn loop_version(&self, index: usize) -> Option<u32> {
        self.loop_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }
}

// ── Insert/Remove for plain entities (no index hooks) ──────────────

define_plain_crud!(@standard vertex, "Vertex", VertexId,   VertexData,   vertex_slots,    free_vertex_head,    active_vertex_count);
define_plain_crud!(@standard edge,   "Edge",   EdgeId,     EdgeData,     edge_slots,      free_edge_head,      active_edge_count);
define_plain_crud!(@standard shell,  "Shell",  ShellId,    ShellData,    shell_slots,     free_shell_head,     active_shell_count);
define_plain_crud!(@standard region, "Region", RegionId,   RegionData,   region_slots,    free_region_head,    active_region_count);
define_plain_crud!(@standard lump,   "Lump",   LumpId,     LumpData,     lump_slots,      free_lump_head,      active_lump_count);
define_plain_crud!(@standard body,   "Body",   BodyId,     BodyData,     body_slots,      free_body_head,      active_body_count);

// Loop — keyword-safe insert/remove
impl TopologyArena {
    /// Insert a new loop, returning its handle.
    pub(crate) fn insert_loop(&mut self, data: LoopData) -> LoopId {
        let (index, gen) = Self::insert_slot(&mut self.loop_slots, &mut self.free_loop_head, data);
        self.active_loop_count += 1;
        LoopId::new(index, gen)
    }

    /// Remove a loop, bumping the slot generation.
    pub(crate) fn remove_loop(&mut self, id: LoopId) -> Result<LoopData, KernelError> {
        let slot = self.loop_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Loop", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Loop", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Loop", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        slot.next_free = self.free_loop_head;
        self.free_loop_head = Some(id.index());
        self.active_loop_count -= 1;
        Ok(data)
    }
}

// ── Hooked insert/remove for Face and HalfEdge ─────────────────────

impl TopologyArena {
    /// Insert a new face, returning its handle. Updates shell→faces index.
    pub(crate) fn insert_face(&mut self, data: FaceData) -> FaceId {
        let shell = data.shell();
        let (index, gen) = Self::insert_slot(&mut self.face_slots, &mut self.free_face_head, data);
        self.active_face_count += 1;
        let id = FaceId::new(index, gen);
        self.index_add_face(id, shell);
        id
    }

    /// Remove a face, bumping the slot generation. Updates shell→faces index.
    pub(crate) fn remove_face(&mut self, id: FaceId) -> Result<FaceData, KernelError> {
        let slot = self.face_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Face", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Face", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Face", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        slot.next_free = self.free_face_head;
        self.free_face_head = Some(id.index());
        self.active_face_count -= 1;
        self.index_remove_face(id, data.shell());
        Ok(data)
    }

    /// Insert a new halfedge, returning its handle. Updates indexes.
    pub(crate) fn insert_half_edge(&mut self, data: HalfEdgeData) -> HalfEdgeId {
        let face = data.face();
        let origin = data.origin();
        let (index, gen) = Self::insert_slot(&mut self.half_edge_slots, &mut self.free_half_edge_head, data);
        self.active_half_edge_count += 1;
        let id = HalfEdgeId::new(index, gen);
        self.index_add_halfedge(id, face, origin);
        id
    }

    /// Remove a halfedge, bumping the slot generation. Updates indexes.
    pub(crate) fn remove_half_edge(&mut self, id: HalfEdgeId) -> Result<HalfEdgeData, KernelError> {
        let slot = self.half_edge_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("HalfEdge", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "HalfEdge", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("HalfEdge", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        slot.next_free = self.free_half_edge_head;
        self.free_half_edge_head = Some(id.index());
        self.active_half_edge_count -= 1;
        self.index_remove_halfedge(id, data.face(), data.origin());
        Ok(data)
    }
}

// ── MutableDraft Proxy Methods ─────────────────────────────────────

define_draft_proxies!(@standard face,      FaceId,     FaceData);
define_draft_proxies!(@standard half_edge, HalfEdgeId, HalfEdgeData);
define_draft_proxies!(@standard vertex,    VertexId,   VertexData);
define_draft_proxies!(@standard edge,      EdgeId,     EdgeData);
define_draft_proxies!(@standard shell,     ShellId,    ShellData);
define_draft_proxies!(@standard region,    RegionId,   RegionData);
define_draft_proxies!(@standard lump,      LumpId,     LumpData);
define_draft_proxies!(@standard body,      BodyId,     BodyData);

// Loop — keyword-safe draft proxies
impl crate::state::MutableDraft {
    /// Insert a new loop.
    pub fn insert_loop(&mut self, data: LoopData) -> LoopId {
        self.arena.insert_loop(data)
    }

    /// Remove a loop.
    pub fn remove_loop(&mut self, id: LoopId) -> Result<LoopData, KernelError> {
        self.arena.remove_loop(id)
    }
}

// Imports needed by the macro-generated code
use crate::handles::{
    FaceId, HalfEdgeId, VertexId, LoopId, EdgeId,
    ShellId, RegionId, LumpId, BodyId,
};
use crate::arena::mesh_schema::{FaceData, HalfEdgeData, VertexData, LoopData, EdgeData};
use crate::arena::containment_schema::{ShellData, RegionData, LumpData, BodyData};
