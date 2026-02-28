//! Macro-generated accessor methods for all topology entity types.
//!
//! DOMAIN: get, get_mut, iter, count, active_indices, generation,
//! and version methods on TopologyArena.

use forge_core::KernelError;
use crate::b_rep::data::storage::slot::{validate_generation, cold_err_bounds, cold_err_deleted};
use crate::b_rep::data::storage::arena::TopologyArena;

/// Generate accessor methods (get, get_mut, iter, count, indices, generation, version).
///
/// The `loop` entry uses explicit method names to avoid keyword issues.
macro_rules! define_entity_accessors {
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

// Imports needed by the macro-generated code
use crate::handles::{
    FaceId, HalfEdgeId, VertexId, LoopId, EdgeId,
    ShellId, RegionId, LumpId, BodyId,
};
use crate::b_rep::data::mesh::{FaceData, HalfEdgeData, VertexData, LoopData, EdgeData};
use crate::b_rep::data::containment::{ShellData, RegionData, LumpData, BodyData};
