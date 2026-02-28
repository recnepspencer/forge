//! Insert and remove operations for all topology entity types.
//!
//! DOMAIN: Slot allocation and deallocation with generation bumping,
//! including index-hooked variants for Face and HalfEdge.

use forge_core::KernelError;
use crate::b_rep::storage::slot::{validate_generation, cold_err_bounds, cold_err_deleted};
use crate::b_rep::storage::TopologyArena;

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

use crate::handles::{
    FaceId, HalfEdgeId, VertexId, LoopId, EdgeId,
    ShellId, RegionId, LumpId, BodyId,
};
use crate::b_rep::entities::{
    FaceData, HalfEdgeData, VertexData, LoopData, EdgeData,
    ShellData, RegionData, LumpData, BodyData,
};

// ── Plain insert/remove (no index hooks) ────────────────────────────

define_plain_crud!(@standard vertex, "Vertex", VertexId,   VertexData,   vertex_slots,    free_vertex_head,    active_vertex_count);
define_plain_crud!(@standard edge,   "Edge",   EdgeId,     EdgeData,     edge_slots,      free_edge_head,      active_edge_count);
define_plain_crud!(@standard shell,  "Shell",  ShellId,    ShellData,    shell_slots,     free_shell_head,     active_shell_count);
define_plain_crud!(@standard region, "Region", RegionId,   RegionData,   region_slots,    free_region_head,    active_region_count);
define_plain_crud!(@standard lump,   "Lump",   LumpId,     LumpData,     lump_slots,      free_lump_head,      active_lump_count);
define_plain_crud!(@standard body,   "Body",   BodyId,     BodyData,     body_slots,      free_body_head,      active_body_count);

// ── Loop — keyword-safe insert/remove ───────────────────────────────

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
