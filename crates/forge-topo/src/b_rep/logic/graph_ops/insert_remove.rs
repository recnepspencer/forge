//! Insert and remove operations for all topology entity types.
//!
//! DOMAIN: Slot allocation/deallocation with side-car + index hook integration.
//!
//! Entity types fall into two categories:
//!
//! - **Mesh entities** (face, halfedge, vertex, edge, loop): explicit impls
//!   with side-car growth/clear and index hooks.
//! - **Containment entities** (shell, region, lump, body): macro-generated
//!   pure CRUD with no hooks.

use forge_core::KernelError;
use crate::b_rep::data::storage::slot::{validate_generation, cold_err_bounds, cold_err_deleted};
use crate::b_rep::data::storage::arena::TopologyArena;

// ════════════════════════════════════════════════════════════════════
// Containment entities — pure CRUD, no hooks
// ════════════════════════════════════════════════════════════════════

/// Generate insert/remove for entities WITHOUT side-car or index hooks.
macro_rules! define_plain_crud {
    (@standard $m:ident, $label:expr, $id:ty, $data:ty, $slots:ident, $free_head:ident, $count:ident) => {
        paste::paste! {
            impl TopologyArena {
                #[doc = concat!("Insert a new ", $label, ", returning its handle.")]
                pub fn [<insert_ $m>](&mut self, data: $data) -> $id {
                    let (index, gen) = Self::insert_slot(&mut self.$slots, &mut self.$free_head, data);
                    self.$count += 1;
                    <$id>::new(index, gen)
                }

                #[doc = concat!("Remove a ", $label, ", bumping the slot generation.")]
                pub fn [<remove_ $m>](&mut self, id: $id) -> Result<$data, KernelError> {
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

define_plain_crud!(@standard shell,  "Shell",  ShellId,    ShellData,    shell_slots,     free_shell_head,     active_shell_count);
define_plain_crud!(@standard region, "Region", RegionId,   RegionData,   region_slots,    free_region_head,    active_region_count);
define_plain_crud!(@standard lump,   "Lump",   LumpId,     LumpData,     lump_slots,      free_lump_head,      active_lump_count);
define_plain_crud!(@standard body,   "Body",   BodyId,     BodyData,     body_slots,      free_body_head,      active_body_count);

// ════════════════════════════════════════════════════════════════════
// Mesh entities — explicit impls with side-car + index hooks
// ════════════════════════════════════════════════════════════════════

// ── Loop (keyword-safe, no side-car) ────────────────────────────────

impl TopologyArena {
    /// Insert a new loop, returning its handle.
    pub fn insert_loop(&mut self, data: LoopData) -> LoopId {
        let (index, gen) = Self::insert_slot(&mut self.loop_slots, &mut self.free_loop_head, data);
        self.active_loop_count += 1;
        LoopId::new(index, gen)
    }

    /// Remove a loop, bumping the slot generation.
    pub fn remove_loop(&mut self, id: LoopId) -> Result<LoopData, KernelError> {
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

// ── Vertex (side-car: vertex_provenance) ────────────────────────────

impl TopologyArena {
    /// Insert a new vertex, returning its handle. Grows vertex side-cars.
    pub fn insert_vertex(&mut self, data: VertexData) -> VertexId {
        let (index, gen) = Self::insert_slot(&mut self.vertex_slots, &mut self.free_vertex_head, data);
        self.active_vertex_count += 1;
        self.grow_vertex_sidecars(self.vertex_slots.len());
        self.clear_vertex_sidecar(index as usize);
        VertexId::new(index, gen)
    }

    /// Remove a vertex, bumping the slot generation. Clears vertex side-cars.
    pub fn remove_vertex(&mut self, id: VertexId) -> Result<VertexData, KernelError> {
        let slot = self.vertex_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Vertex", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Vertex", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Vertex", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        slot.next_free = self.free_vertex_head;
        self.free_vertex_head = Some(id.index());
        self.active_vertex_count -= 1;
        self.clear_vertex_sidecar(id.index() as usize);
        self.nmt_extra_disks.remove(&id);
        Ok(data)
    }
}

// ── Edge (side-car: edge_curves) ────────────────────────────────────

impl TopologyArena {
    /// Insert a new edge, returning its handle. Grows edge side-cars.
    pub fn insert_edge(&mut self, data: EdgeData) -> EdgeId {
        let (index, gen) = Self::insert_slot(&mut self.edge_slots, &mut self.free_edge_head, data);
        self.active_edge_count += 1;
        self.grow_edge_sidecars(self.edge_slots.len());
        self.clear_edge_sidecar(index as usize);
        EdgeId::new(index, gen)
    }

    /// Remove an edge, bumping the slot generation. Clears edge side-cars.
    pub fn remove_edge(&mut self, id: EdgeId) -> Result<EdgeData, KernelError> {
        let slot = self.edge_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Edge", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Edge", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Edge", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        slot.next_free = self.free_edge_head;
        self.free_edge_head = Some(id.index());
        self.active_edge_count -= 1;
        self.clear_edge_sidecar(id.index() as usize);
        Ok(data)
    }
}

// ── Face (index hooks: shell→faces) ─────────────────────────────────

impl TopologyArena {
    /// Insert a new face, returning its handle. Updates shell→faces index.
    pub fn insert_face(&mut self, data: FaceData) -> FaceId {
        let shell = data.shell();
        let (index, gen) = Self::insert_slot(&mut self.face_slots, &mut self.free_face_head, data);
        self.active_face_count += 1;
        let id = FaceId::new(index, gen);
        self.index_add_face(id, shell);
        id
    }

    /// Remove a face, bumping the slot generation. Updates shell→faces index.
    pub fn remove_face(&mut self, id: FaceId) -> Result<FaceData, KernelError> {
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
}

// ── HalfEdge (side-car: bridge_flags + coedge_data, index hooks) ────

impl TopologyArena {
    /// Insert a new halfedge, returning its handle. Updates indexes + side-cars.
    pub fn insert_half_edge(&mut self, data: HalfEdgeData) -> HalfEdgeId {
        let face = data.face();
        let origin = data.origin();
        let (index, gen) = Self::insert_slot(&mut self.half_edge_slots, &mut self.free_half_edge_head, data);
        self.active_half_edge_count += 1;
        let id = HalfEdgeId::new(index, gen);
        self.index_add_halfedge(id, face, origin);
        self.grow_halfedge_sidecars(self.half_edge_slots.len());
        self.clear_halfedge_sidecar(index as usize);
        id
    }

    /// Remove a halfedge, bumping the slot generation. Updates indexes + side-cars.
    pub fn remove_half_edge(&mut self, id: HalfEdgeId) -> Result<HalfEdgeData, KernelError> {
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
        self.clear_halfedge_sidecar(id.index() as usize);
        Ok(data)
    }

    // ── Two-phase reserve/populate for circular wiring ──────────────

    /// Reserve a halfedge slot, returning its ID. The slot has no data yet.
    ///
    /// Use `populate_half_edge` to fill it after all circular IDs are known.
    pub fn reserve_half_edge(&mut self) -> HalfEdgeId {
        let (index, gen) = Self::reserve_slot(
            &mut self.half_edge_slots,
            &mut self.free_half_edge_head,
        );
        self.active_half_edge_count += 1;
        self.grow_halfedge_sidecars(self.half_edge_slots.len());
        HalfEdgeId::new(index, gen)
    }

    /// Fill a previously reserved halfedge slot with data. Updates indexes.
    ///
    /// # Panics
    /// Debug-panics if the slot is already occupied.
    pub fn populate_half_edge(&mut self, id: HalfEdgeId, data: HalfEdgeData) {
        let face = data.face();
        let origin = data.origin();
        Self::populate_slot(&mut self.half_edge_slots, id.index(), data);
        self.index_add_halfedge(id, face, origin);
        self.clear_halfedge_sidecar(id.index() as usize);
    }

    /// Reserve a face slot, returning its ID.
    pub fn reserve_face(&mut self) -> FaceId {
        let (index, gen) = Self::reserve_slot(
            &mut self.face_slots,
            &mut self.free_face_head,
        );
        self.active_face_count += 1;
        FaceId::new(index, gen)
    }

    /// Fill a previously reserved face slot with data. Updates indexes.
    pub fn populate_face(&mut self, id: FaceId, data: FaceData) {
        let shell = data.shell();
        Self::populate_slot(&mut self.face_slots, id.index(), data);
        self.index_add_face(id, shell);
    }

    /// Reserve a loop slot, returning its ID.
    pub fn reserve_loop(&mut self) -> LoopId {
        let (index, gen) = Self::reserve_slot(
            &mut self.loop_slots,
            &mut self.free_loop_head,
        );
        self.active_loop_count += 1;
        LoopId::new(index, gen)
    }

    /// Fill a previously reserved loop slot with data.
    pub fn populate_loop(&mut self, id: LoopId, data: LoopData) {
        Self::populate_slot(&mut self.loop_slots, id.index(), data);
    }

    /// Reserve a vertex slot, returning its ID. Grows vertex side-cars.
    pub fn reserve_vertex(&mut self) -> VertexId {
        let (index, gen) = Self::reserve_slot(
            &mut self.vertex_slots,
            &mut self.free_vertex_head,
        );
        self.active_vertex_count += 1;
        self.grow_vertex_sidecars(self.vertex_slots.len());
        VertexId::new(index, gen)
    }

    /// Fill a previously reserved vertex slot with data.
    pub fn populate_vertex(&mut self, id: VertexId, data: VertexData) {
        Self::populate_slot(&mut self.vertex_slots, id.index(), data);
        self.clear_vertex_sidecar(id.index() as usize);
    }

    /// Reserve an edge slot, returning its ID. Grows edge side-cars.
    pub fn reserve_edge(&mut self) -> EdgeId {
        let (index, gen) = Self::reserve_slot(
            &mut self.edge_slots,
            &mut self.free_edge_head,
        );
        self.active_edge_count += 1;
        self.grow_edge_sidecars(self.edge_slots.len());
        EdgeId::new(index, gen)
    }

    /// Fill a previously reserved edge slot with data.
    pub fn populate_edge(&mut self, id: EdgeId, data: EdgeData) {
        Self::populate_slot(&mut self.edge_slots, id.index(), data);
        self.clear_edge_sidecar(id.index() as usize);
    }
}

// Imports needed by the macro-generated code
use crate::handles::{
    FaceId, HalfEdgeId, VertexId, LoopId, EdgeId,
    ShellId, RegionId, LumpId, BodyId,
};
use crate::b_rep::data::mesh::{FaceData, HalfEdgeData, VertexData, LoopData, EdgeData};
use crate::b_rep::data::containment::{ShellData, RegionData, LumpData, BodyData};

