use serde::{Deserialize, Serialize};

use forge_core::{KernelError, TopologyError, ErrorContext, ErrorScope};
use crate::attributes::AttributeStore;
use crate::lineage_store::LineageStore;
use forge_core::{EntityRef, EntityKind};
use crate::handles::{FaceId, HalfEdgeId, VertexId, LoopId, ShellId, BodyId, LumpId, RegionId, EdgeId};

use super::schema::{Slot, FaceData, HalfEdgeData, VertexData, LoopData, ShellData, BodyData, LumpData, RegionData, EdgeData};

/// Entity storage for the halfedge mesh.
///
/// Holds faces, halfedges, vertices, and loops in arena-allocated vectors.
/// Each slot tracks its generation counter for stale-handle detection.
/// This struct is `Clone`-able and lives inside `Arc` for structural sharing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyArena {
    /// Arena-allocated face entities with generational handles.
    face_slots: Vec<Slot<FaceData>>,
    /// Arena-allocated halfedge entities with generational handles.
    half_edge_slots: Vec<Slot<HalfEdgeData>>,
    /// Arena-allocated vertex entities with generational handles.
    vertex_slots: Vec<Slot<VertexData>>,
    /// Arena-allocated loop entities with generational handles.
    loop_slots: Vec<Slot<LoopData>>,
    /// Arena-allocated shell entities with generational handles.
    shell_slots: Vec<Slot<ShellData>>,
    /// Arena-allocated solid entities with generational handles.
    body_slots: Vec<Slot<BodyData>>,
    /// Arena-allocated lump entities with generational handles.
    lump_slots: Vec<Slot<LumpData>>,
    /// Arena-allocated region entities with generational handles.
    region_slots: Vec<Slot<RegionData>>,
    /// Arena-allocated edge entities with generational handles.
    edge_slots: Vec<Slot<EdgeData>>,
    /// Side-car attribute storage for manufacturing metadata.
    attribute_store: AttributeStore,

    // -- O(1) Active Counts --
    active_face_count: usize,
    active_half_edge_count: usize,
    active_vertex_count: usize,
    active_loop_count: usize,
    active_shell_count: usize,
    active_body_count: usize,
    active_lump_count: usize,
    active_region_count: usize,
    active_edge_count: usize,
}

impl TopologyArena {
    /// Create an empty arena with no entities.
    pub fn new() -> Self {
        Self {
            face_slots: Vec::new(),
            half_edge_slots: Vec::new(),
            vertex_slots: Vec::new(),
            loop_slots: Vec::new(),
            shell_slots: Vec::new(),
            body_slots: Vec::new(),
            lump_slots: Vec::new(),
            region_slots: Vec::new(),
            edge_slots: Vec::new(),
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
        }
    }

    /// Insert a new face, returning its handle.
    pub fn insert_face(&mut self, data: FaceData, mut ls: Option<&mut LineageStore>) -> FaceId {
        let index = self.face_slots.len() as u32;
        let mut slot = Slot::empty();
        let gen = slot.occupy(data);
        self.face_slots.push(slot);
        self.active_face_count += 1;
        let id = FaceId::new(index, gen);
        if let Some(store) = ls.as_deref_mut() {
            if let Some(lin) = self.face_slots[index as usize].data.as_ref().unwrap().lineage().cloned() {
                store.record_creation(EntityRef::new(EntityKind::Face, id.index()), lin);
            }
        }
        id
    }

    /// Insert a single halfedge, returning its handle.
    ///
    /// The caller is responsible for setting the `radial_next` field correctly.
    pub fn insert_half_edge(&mut self, data: HalfEdgeData, mut ls: Option<&mut LineageStore>) -> HalfEdgeId {
        let index = self.half_edge_slots.len() as u32;
        let mut slot = Slot::empty();
        let gen = slot.occupy(data);
        self.half_edge_slots.push(slot);
        self.active_half_edge_count += 1;
        let id = HalfEdgeId::new(index, gen);
        if let Some(store) = ls.as_deref_mut() {
            if let Some(lin) = self.half_edge_slots[index as usize].data.as_ref().unwrap().lineage().cloned() {
                store.record_creation(EntityRef::new(EntityKind::HalfEdge, id.index()), lin);
            }
        }
        id
    }

    /// Insert a pair of radial halfedges and wire their `radial_next` fields reciprocally.
    ///
    /// Returns `(he_a, he_b)` where `he_a.radial_next == he_b` and `he_b.radial_next == he_a`.
    pub fn insert_radial_pair(
        &mut self,
        mut data_a: HalfEdgeData,
        mut data_b: HalfEdgeData,
        mut ls: Option<&mut LineageStore>,
    ) -> (HalfEdgeId, HalfEdgeId) {
        let base = self.half_edge_slots.len() as u32;

        let he_a_id = HalfEdgeId::new(base, 0);
        let he_b_id = HalfEdgeId::new(base + 1, 0);

        data_a.set_radial_next(he_b_id);
        data_b.set_radial_next(he_a_id);

        let mut slot_a = Slot::empty();
        let gen_a = slot_a.occupy(data_a);
        self.half_edge_slots.push(slot_a);

        let mut slot_b = Slot::empty();
        let gen_b = slot_b.occupy(data_b);
        self.half_edge_slots.push(slot_b);

        self.active_half_edge_count += 2;

        (HalfEdgeId::new(base, gen_a), HalfEdgeId::new(base + 1, gen_b))
    }

    /// Insert a new vertex, returning its handle.
    pub fn insert_vertex(&mut self, data: VertexData, mut ls: Option<&mut LineageStore>) -> VertexId {
        let index = self.vertex_slots.len() as u32;
        let mut slot = Slot::empty();
        let gen = slot.occupy(data);
        self.vertex_slots.push(slot);
        self.active_vertex_count += 1;
        let id = VertexId::new(index, gen);
        if let Some(store) = ls.as_deref_mut() {
            if let Some(lin) = self.vertex_slots[index as usize].data.as_ref().unwrap().lineage().cloned() {
                store.record_creation(EntityRef::new(EntityKind::Vertex, id.index()), lin);
            }
        }
        id
    }

    /// Insert a new loop, returning its handle.
    pub fn insert_loop(&mut self, data: LoopData, mut ls: Option<&mut LineageStore>) -> LoopId {
        let index = self.loop_slots.len() as u32;
        let mut slot = Slot::empty();
        let gen = slot.occupy(data);
        self.loop_slots.push(slot);
        self.active_loop_count += 1;
        LoopId::new(index, gen)
    }

    /// Get a face by handle, validating the generation.
    #[inline]
    pub fn get_face(&self, id: FaceId) -> Result<&FaceData, KernelError> {
        let slot = self.face_slots.get(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Face", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Face", id.index())?;
        slot.data.as_ref()
            .ok_or_else(|| cold_err_deleted("Face", id.index(), id.generation(), slot.generation))
    }

    /// Get a halfedge by handle, validating the generation.
    #[inline]
    pub fn get_half_edge(&self, id: HalfEdgeId) -> Result<&HalfEdgeData, KernelError> {
        let slot = self.half_edge_slots.get(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("HalfEdge", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "HalfEdge", id.index())?;
        slot.data.as_ref()
            .ok_or_else(|| cold_err_deleted("HalfEdge", id.index(), id.generation(), slot.generation))
    }

    /// Get a vertex by handle, validating the generation.
    #[inline]
    pub fn get_vertex(&self, id: VertexId) -> Result<&VertexData, KernelError> {
        let slot = self.vertex_slots.get(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Vertex", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Vertex", id.index())?;
        slot.data.as_ref()
            .ok_or_else(|| cold_err_deleted("Vertex", id.index(), id.generation(), slot.generation))
    }

    /// Get a loop by handle, validating the generation.
    #[inline]
    pub fn get_loop(&self, id: LoopId) -> Result<&LoopData, KernelError> {
        let slot = self.loop_slots.get(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Loop", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Loop", id.index())?;
        slot.data.as_ref()
            .ok_or_else(|| cold_err_deleted("Loop", id.index(), id.generation(), slot.generation))
    }

    /// Get a mutable reference to a face by handle.
    #[inline]
    pub fn get_face_mut(&mut self, id: FaceId) -> Result<&mut FaceData, KernelError> {
        let slot = self.face_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Face", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Face", id.index())?;
        
        slot.version += 1;

        slot.data.as_mut()
            .ok_or_else(|| cold_err_deleted("Face", id.index(), id.generation(), slot.generation))
    }

    /// Get a mutable reference to a halfedge by handle.
    #[inline]
    pub fn get_half_edge_mut(&mut self, id: HalfEdgeId) -> Result<&mut HalfEdgeData, KernelError> {
        let slot = self.half_edge_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("HalfEdge", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "HalfEdge", id.index())?;

        slot.version += 1;

        slot.data.as_mut()
            .ok_or_else(|| cold_err_deleted("HalfEdge", id.index(), id.generation(), slot.generation))
    }

    /// Get a mutable reference to a vertex by handle.
    #[inline]
    pub fn get_vertex_mut(&mut self, id: VertexId) -> Result<&mut VertexData, KernelError> {
        let slot = self.vertex_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Vertex", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Vertex", id.index())?;

        slot.version += 1;

        slot.data.as_mut()
            .ok_or_else(|| cold_err_deleted("Vertex", id.index(), id.generation(), slot.generation))
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

    /// Count of active (non-deleted) faces.
    pub fn face_count(&self) -> usize {
        self.active_face_count
    }

    /// Count of active halfedges.
    pub fn half_edge_count(&self) -> usize {
        self.active_half_edge_count
    }

    /// Count of active vertices.
    pub fn vertex_count(&self) -> usize {
        self.active_vertex_count
    }

    /// Count of active loops.
    pub fn loop_count(&self) -> usize {
        self.active_loop_count
    }

    /// Count of active shells.
    pub fn shell_count(&self) -> usize {
        self.active_shell_count
    }

    /// Count of active solids.
    pub fn body_count(&self) -> usize {
        self.active_body_count
    }

    /// Count of active lumps.
    pub fn lump_count(&self) -> usize {
        self.active_lump_count
    }

    /// Count of active regions.
    pub fn region_count(&self) -> usize {
        self.active_region_count
    }

    /// Count of active edges.
    pub fn edge_count(&self) -> usize {
        self.active_edge_count
    }

    /// Total face slot count (including vacant slots).
    pub fn face_slot_count(&self) -> usize {
        self.face_slots.len()
    }

    /// Total halfedge slot count (including vacant slots).
    pub fn half_edge_slot_count(&self) -> usize {
        self.half_edge_slots.len()
    }

    /// Total vertex slot count (including vacant slots).
    pub fn vertex_slot_count(&self) -> usize {
        self.vertex_slots.len()
    }

    /// Generation of face at slot index, or None if vacant/out-of-bounds.
    pub fn face_generation(&self, index: usize) -> Option<u32> {
        self.face_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Generation of halfedge at slot index, or None if vacant/out-of-bounds.
    pub fn half_edge_generation(&self, index: usize) -> Option<u32> {
        self.half_edge_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Generation of vertex at slot index, or None if vacant/out-of-bounds.
    pub fn vertex_generation(&self, index: usize) -> Option<u32> {
        self.vertex_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Version of face at slot index, or None if vacant/out-of-bounds.
    pub fn face_version(&self, index: usize) -> Option<u32> {
        self.face_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }

    /// Version of halfedge at slot index, or None if vacant/out-of-bounds.
    pub fn half_edge_version(&self, index: usize) -> Option<u32> {
        self.half_edge_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }

    /// Version of vertex at slot index, or None if vacant/out-of-bounds.
    pub fn vertex_version(&self, index: usize) -> Option<u32> {
        self.vertex_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }

    /// Bump the version of a face slot without requiring mutable data access.
    ///
    /// Used by operators to mark a face as "dirty" when its boundary
    /// half-edges are rewired. This enables the diff engine to detect
    /// transitive face modifications even when `FaceData` fields are unchanged.
    pub fn bump_face_version(&mut self, id: FaceId) -> Result<(), KernelError> {
        let slot = self.face_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Face", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Face", id.index())?;
        slot.version += 1;
        Ok(())
    }

    /// Read-only access to the attribute store.
    pub fn get_attribute_store(&self) -> &AttributeStore {
        &self.attribute_store
    }

    /// Mutable access to the attribute store.
    pub fn get_attribute_store_mut(&mut self) -> &mut AttributeStore {
        &mut self.attribute_store
    }

    /// Iterate over all active faces, yielding `(FaceId, &FaceData)` pairs.
    pub fn iter_faces(&self) -> impl Iterator<Item = (FaceId, &FaceData)> {
        self.face_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((FaceId::new(i as u32, slot.generation), data))
        })
    }

    /// Iterate over all active halfedges, yielding `(HalfEdgeId, &HalfEdgeData)` pairs.
    pub fn iter_half_edges(&self) -> impl Iterator<Item = (HalfEdgeId, &HalfEdgeData)> {
        self.half_edge_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((HalfEdgeId::new(i as u32, slot.generation), data))
        })
    }

    /// Iterate over all active vertices, yielding `(VertexId, &VertexData)` pairs.
    pub fn iter_vertices(&self) -> impl Iterator<Item = (VertexId, &VertexData)> {
        self.vertex_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((VertexId::new(i as u32, slot.generation), data))
        })
    }

    /// Iterate over all active loops, yielding `(LoopId, &LoopData)` pairs.
    pub fn iter_loops(&self) -> impl Iterator<Item = (LoopId, &LoopData)> {
        self.loop_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((LoopId::new(i as u32, slot.generation), data))
        })
    }

    /// Remove a face, bumping the generation of its slot.
    pub fn remove_face(&mut self, id: FaceId, mut ls: Option<&mut LineageStore>) -> Result<FaceData, KernelError> {
        let slot = self.face_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Face", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Face", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Face", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        self.active_face_count -= 1;
        if let Some(store) = ls.as_deref_mut() {
            let _ = store.record_deletion(EntityRef::new(EntityKind::Face, id.index()));
        }
        Ok(data)
    }

    /// Remove a vertex, bumping the generation of its slot.
    pub fn remove_vertex(&mut self, id: VertexId, mut ls: Option<&mut LineageStore>) -> Result<VertexData, KernelError> {
        let slot = self.vertex_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Vertex", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Vertex", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Vertex", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        self.active_vertex_count -= 1;
        if let Some(store) = ls.as_deref_mut() {
            let _ = store.record_deletion(EntityRef::new(EntityKind::Vertex, id.index()));
        }
        Ok(data)
    }

    /// Remove a halfedge, bumping the generation of its slot.
    pub fn remove_half_edge(&mut self, id: HalfEdgeId, mut ls: Option<&mut LineageStore>) -> Result<HalfEdgeData, KernelError> {
        let slot = self.half_edge_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("HalfEdge", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "HalfEdge", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("HalfEdge", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        self.active_half_edge_count -= 1;
        if let Some(store) = ls.as_deref_mut() {
            let _ = store.record_deletion(EntityRef::new(EntityKind::HalfEdge, id.index()));
        }
        Ok(data)
    }

    /// Remove a loop, bumping the generation of its slot.
    pub fn remove_loop(&mut self, id: LoopId, mut ls: Option<&mut LineageStore>) -> Result<LoopData, KernelError> {
        let slot = self.loop_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Loop", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Loop", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Loop", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        self.active_loop_count -= 1;
        Ok(data)
    }

    /// Indices of all active (occupied) face slots.
    ///
    /// Returns only slot indices where `data.is_some()`. Used by
    /// `compute_diff` to iterate O(active) instead of O(capacity).
    pub fn active_face_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.face_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Indices of all active (occupied) halfedge slots.
    pub fn active_half_edge_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.half_edge_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Indices of all active (occupied) vertex slots.
    pub fn active_vertex_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.vertex_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Indices of all active (occupied) loop slots.
    pub fn active_loop_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.loop_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Indices of all active (occupied) edge slots.
    pub fn active_edge_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.edge_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Indices of all active (occupied) shell slots.
    pub fn active_shell_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.shell_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Indices of all active (occupied) solid slots.
    pub fn active_body_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.body_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Generation of loop at slot index, or None if vacant/out-of-bounds.
    pub fn loop_generation(&self, index: usize) -> Option<u32> {
        self.loop_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Generation of edge at slot index, or None if vacant/out-of-bounds.
    pub fn edge_generation(&self, index: usize) -> Option<u32> {
        self.edge_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Generation of shell at slot index, or None if vacant/out-of-bounds.
    pub fn shell_generation(&self, index: usize) -> Option<u32> {
        self.shell_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Generation of solid at slot index, or None if vacant/out-of-bounds.
    pub fn body_generation(&self, index: usize) -> Option<u32> {
        self.body_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Version of loop at slot index, or None if vacant/out-of-bounds.
    pub fn loop_version(&self, index: usize) -> Option<u32> {
        self.loop_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }

    /// Version of edge at slot index, or None if vacant/out-of-bounds.
    pub fn edge_version(&self, index: usize) -> Option<u32> {
        self.edge_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }

    /// Version of shell at slot index, or None if vacant/out-of-bounds.
    pub fn shell_version(&self, index: usize) -> Option<u32> {
        self.shell_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }

    /// Version of solid at slot index, or None if vacant/out-of-bounds.
    pub fn body_version(&self, index: usize) -> Option<u32> {
        self.body_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }

    // ── Shell CRUD ──────────────────────────────────────────────────

    /// Insert a new shell, returning its handle.
    pub fn insert_shell(&mut self, data: ShellData, mut ls: Option<&mut LineageStore>) -> ShellId {
        let index = self.shell_slots.len() as u32;
        let mut slot = Slot::empty();
        let gen = slot.occupy(data);
        self.shell_slots.push(slot);
        self.active_shell_count += 1;
        let id = ShellId::new(index, gen);
        if let Some(store) = ls.as_deref_mut() {
            if let Some(lin) = self.shell_slots[index as usize].data.as_ref().unwrap().lineage().cloned() {
                store.record_creation(EntityRef::new(EntityKind::Shell, id.index()), lin);
            }
        }
        id
    }

    /// Get a shell by handle, validating the generation.
    #[inline]
    pub fn get_shell(&self, id: ShellId) -> Result<&ShellData, KernelError> {
        let slot = self.shell_slots.get(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Shell", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Shell", id.index())?;
        slot.data.as_ref()
            .ok_or_else(|| cold_err_deleted("Shell", id.index(), id.generation(), slot.generation))
    }

    /// Get a mutable reference to a shell by handle.
    #[inline]
    pub fn get_shell_mut(&mut self, id: ShellId) -> Result<&mut ShellData, KernelError> {
        let slot = self.shell_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Shell", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Shell", id.index())?;
        slot.version += 1;
        slot.data.as_mut()
            .ok_or_else(|| cold_err_deleted("Shell", id.index(), id.generation(), slot.generation))
    }

    /// Remove a shell, bumping the generation of its slot.
    pub fn remove_shell(&mut self, id: ShellId, mut ls: Option<&mut LineageStore>) -> Result<ShellData, KernelError> {
        let slot = self.shell_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Shell", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Shell", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Shell", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        self.active_shell_count -= 1;
        if let Some(store) = ls.as_deref_mut() {
            let _ = store.record_deletion(EntityRef::new(EntityKind::Shell, id.index()));
        }
        Ok(data)
    }

    /// Iterate over all active shells, yielding `(ShellId, &ShellData)` pairs.
    pub fn iter_shells(&self) -> impl Iterator<Item = (ShellId, &ShellData)> {
        self.shell_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((ShellId::new(i as u32, slot.generation), data))
        })
    }

    // ── Solid CRUD ──────────────────────────────────────────────────

    /// Insert a new solid, returning its handle.
    pub fn insert_body(&mut self, data: BodyData, mut ls: Option<&mut LineageStore>) -> BodyId {
        let index = self.body_slots.len() as u32;
        let mut slot = Slot::empty();
        let gen = slot.occupy(data);
        self.body_slots.push(slot);
        self.active_body_count += 1;
        let id = BodyId::new(index, gen);
        if let Some(store) = ls.as_deref_mut() {
            if let Some(lin) = self.body_slots[index as usize].data.as_ref().unwrap().lineage().cloned() {
                store.record_creation(EntityRef::new(EntityKind::Body, id.index()), lin);
            }
        }
        id
    }

    /// Get a solid by handle, validating the generation.
    #[inline]
    pub fn get_body(&self, id: BodyId) -> Result<&BodyData, KernelError> {
        let slot = self.body_slots.get(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Body", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Body", id.index())?;
        slot.data.as_ref()
            .ok_or_else(|| cold_err_deleted("Body", id.index(), id.generation(), slot.generation))
    }

    /// Get a mutable reference to a solid by handle.
    #[inline]
    pub fn get_body_mut(&mut self, id: BodyId) -> Result<&mut BodyData, KernelError> {
        let slot = self.body_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Body", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Body", id.index())?;
        slot.version += 1;
        slot.data.as_mut()
            .ok_or_else(|| cold_err_deleted("Body", id.index(), id.generation(), slot.generation))
    }

    /// Remove a solid, bumping the generation of its slot.
    pub fn remove_body(&mut self, id: BodyId, mut ls: Option<&mut LineageStore>) -> Result<BodyData, KernelError> {
        let slot = self.body_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Body", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Body", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Body", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        self.active_body_count -= 1;
        if let Some(store) = ls.as_deref_mut() {
            let _ = store.record_deletion(EntityRef::new(EntityKind::Body, id.index()));
        }
        Ok(data)
    }

    /// Iterate over all active solids, yielding `(BodyId, &BodyData)` pairs.
    pub fn iter_bodies(&self) -> impl Iterator<Item = (BodyId, &BodyData)> {
        self.body_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((BodyId::new(i as u32, slot.generation), data))
        })
    }

    // ── Lump CRUD ──────────────────────────────────────────────────────────

    /// Insert a new lump, returning its handle.
    pub fn insert_lump(&mut self, data: LumpData, mut ls: Option<&mut LineageStore>) -> LumpId {
        let index = self.lump_slots.len() as u32;
        let mut slot = Slot::empty();
        let gen = slot.occupy(data);
        self.lump_slots.push(slot);
        self.active_lump_count += 1;
        let id = LumpId::new(index, gen);
        if let Some(store) = ls.as_deref_mut() {
            if let Some(lin) = self.lump_slots[index as usize].data.as_ref().unwrap().lineage().cloned() {
                store.record_creation(EntityRef::new(EntityKind::Lump, id.index()), lin);
            }
        }
        id
    }

    /// Get a lump by handle, validating the generation.
    #[inline]
    pub fn get_lump(&self, id: LumpId) -> Result<&LumpData, KernelError> {
        let slot = self.lump_slots.get(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Lump", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Lump", id.index())?;
        slot.data.as_ref()
            .ok_or_else(|| cold_err_deleted("Lump", id.index(), id.generation(), slot.generation))
    }

    /// Get a mutable reference to a lump by handle.
    #[inline]
    pub fn get_lump_mut(&mut self, id: LumpId) -> Result<&mut LumpData, KernelError> {
        let slot = self.lump_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Lump", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Lump", id.index())?;
        slot.version += 1;
        slot.data.as_mut()
            .ok_or_else(|| cold_err_deleted("Lump", id.index(), id.generation(), slot.generation))
    }

    /// Remove a lump, bumping the generation of its slot.
    pub fn remove_lump(&mut self, id: LumpId, mut ls: Option<&mut LineageStore>) -> Result<LumpData, KernelError> {
        let slot = self.lump_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Lump", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Lump", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Lump", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        self.active_lump_count -= 1;
        if let Some(store) = ls.as_deref_mut() {
            let _ = store.record_deletion(EntityRef::new(EntityKind::Lump, id.index()));
        }
        Ok(data)
    }

    /// Iterate over all active lumps, yielding `(LumpId, &LumpData)` pairs.
    pub fn iter_lumps(&self) -> impl Iterator<Item = (LumpId, &LumpData)> {
        self.lump_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((LumpId::new(i as u32, slot.generation), data))
        })
    }

    /// Indices of all active (occupied) lump slots.
    pub fn active_lump_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.lump_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Generation of lump at slot index, or None if vacant/out-of-bounds.
    pub fn lump_generation(&self, index: usize) -> Option<u32> {
        self.lump_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Version of lump at slot index, or None if vacant/out-of-bounds.
    pub fn lump_version(&self, index: usize) -> Option<u32> {
        self.lump_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }

    // ── Region CRUD ────────────────────────────────────────────────────────

    /// Insert a new region, returning its handle.
    pub fn insert_region(&mut self, data: RegionData, mut ls: Option<&mut LineageStore>) -> RegionId {
        let index = self.region_slots.len() as u32;
        let mut slot = Slot::empty();
        let gen = slot.occupy(data);
        self.region_slots.push(slot);
        self.active_region_count += 1;
        let id = RegionId::new(index, gen);
        if let Some(store) = ls.as_deref_mut() {
            if let Some(lin) = self.region_slots[index as usize].data.as_ref().unwrap().lineage().cloned() {
                store.record_creation(EntityRef::new(EntityKind::Region, id.index()), lin);
            }
        }
        id
    }

    /// Get a region by handle, validating the generation.
    #[inline]
    pub fn get_region(&self, id: RegionId) -> Result<&RegionData, KernelError> {
        let slot = self.region_slots.get(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Region", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Region", id.index())?;
        slot.data.as_ref()
            .ok_or_else(|| cold_err_deleted("Region", id.index(), id.generation(), slot.generation))
    }

    /// Get a mutable reference to a region by handle.
    #[inline]
    pub fn get_region_mut(&mut self, id: RegionId) -> Result<&mut RegionData, KernelError> {
        let slot = self.region_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Region", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Region", id.index())?;
        slot.version += 1;
        slot.data.as_mut()
            .ok_or_else(|| cold_err_deleted("Region", id.index(), id.generation(), slot.generation))
    }

    /// Remove a region, bumping the generation of its slot.
    pub fn remove_region(&mut self, id: RegionId, mut ls: Option<&mut LineageStore>) -> Result<RegionData, KernelError> {
        let slot = self.region_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Region", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Region", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Region", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        self.active_region_count -= 1;
        if let Some(store) = ls.as_deref_mut() {
            let _ = store.record_deletion(EntityRef::new(EntityKind::Region, id.index()));
        }
        Ok(data)
    }

    /// Iterate over all active regions, yielding `(RegionId, &RegionData)` pairs.
    pub fn iter_regions(&self) -> impl Iterator<Item = (RegionId, &RegionData)> {
        self.region_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((RegionId::new(i as u32, slot.generation), data))
        })
    }

    /// Indices of all active (occupied) region slots.
    pub fn active_region_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.region_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Generation of region at slot index, or None if vacant/out-of-bounds.
    pub fn region_generation(&self, index: usize) -> Option<u32> {
        self.region_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Version of region at slot index, or None if vacant/out-of-bounds.
    pub fn region_version(&self, index: usize) -> Option<u32> {
        self.region_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }

    // ── Edge CRUD ───────────────────────────────────────────────────────────

    /// Insert a new edge, returning its handle.
    pub fn insert_edge(&mut self, data: EdgeData, mut ls: Option<&mut LineageStore>) -> EdgeId {
        let index = self.edge_slots.len() as u32;
        let mut slot = Slot::empty();
        let gen = slot.occupy(data);
        self.edge_slots.push(slot);
        self.active_edge_count += 1;
        let id = EdgeId::new(index, gen);
        if let Some(store) = ls.as_deref_mut() {
            if let Some(lin) = self.edge_slots[index as usize].data.as_ref().unwrap().lineage().cloned() {
                store.record_creation(EntityRef::new(EntityKind::Edge, id.index()), lin);
            }
        }
        id
    }

    /// Get an edge by handle, validating the generation.
    #[inline]
    pub fn get_edge(&self, id: EdgeId) -> Result<&EdgeData, KernelError> {
        let slot = self.edge_slots.get(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Edge", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Edge", id.index())?;
        slot.data.as_ref()
            .ok_or_else(|| cold_err_deleted("Edge", id.index(), id.generation(), slot.generation))
    }

    /// Get a mutable reference to an edge by handle.
    #[inline]
    pub fn get_edge_mut(&mut self, id: EdgeId) -> Result<&mut EdgeData, KernelError> {
        let slot = self.edge_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Edge", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Edge", id.index())?;
        slot.version += 1;
        slot.data.as_mut()
            .ok_or_else(|| cold_err_deleted("Edge", id.index(), id.generation(), slot.generation))
    }

    /// Remove an edge, bumping the generation of its slot.
    pub fn remove_edge(&mut self, id: EdgeId, mut ls: Option<&mut LineageStore>) -> Result<EdgeData, KernelError> {
        let slot = self.edge_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Edge", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Edge", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Edge", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        self.active_edge_count -= 1;
        if let Some(store) = ls.as_deref_mut() {
            let _ = store.record_deletion(EntityRef::new(EntityKind::Edge, id.index()));
        }
        Ok(data)
    }

    /// Iterate over all active edges, yielding `(EdgeId, &EdgeData)` pairs.
    pub fn iter_edges(&self) -> impl Iterator<Item = (EdgeId, &EdgeData)> {
        self.edge_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((EdgeId::new(i as u32, slot.generation), data))
        })
    }
}

impl Default for TopologyArena {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate that a handle's generation matches the slot's generation.
#[inline]
fn validate_generation(
    slot_gen: u32,
    handle_gen: u32,
    entity_type: &str,
    index: u32,
) -> Result<(), KernelError> {
    if slot_gen != handle_gen {
        return Err(cold_err_stale(entity_type, index, handle_gen, slot_gen));
    }
    Ok(())
}

#[inline(never)]
fn cold_err_bounds(kind: &str, idx: u32, gen: u32) -> KernelError {
    KernelError::TopologyViolation {
        err: TopologyError::StaleHandle {
            entity_kind: kind.to_string(),
            index: idx,
            expected_generation: gen,
            actual_generation: 0,
        },
        context: Some(ErrorContext {
            scope: ErrorScope::Entity { entity_kind: kind.to_string(), index: idx },
            suggested_fixes: Vec::new(),
            detail: format!("{} index {} out of bounds", kind, idx),
        }),
    }
}

#[cold]
#[inline(never)]
fn cold_err_stale(kind: &str, idx: u32, expected: u32, actual: u32) -> KernelError {
    KernelError::TopologyViolation {
        err: TopologyError::StaleHandle {
            entity_kind: kind.to_string(),
            index: idx,
            expected_generation: expected,
            actual_generation: actual,
        },
        context: Some(ErrorContext {
            scope: ErrorScope::Entity { entity_kind: kind.to_string(), index: idx },
            suggested_fixes: Vec::new(),
            detail: format!("Stale {} handle at index {} (expected gen {}, got gen {})", kind, idx, expected, actual),
        }),
    }
}

#[cold]
#[inline(never)]
fn cold_err_deleted(kind: &str, idx: u32, expected_gen: u32, actual_gen: u32) -> KernelError {
    KernelError::TopologyViolation {
        err: TopologyError::StaleHandle {
            entity_kind: kind.to_string(),
            index: idx,
            expected_generation: expected_gen,
            actual_generation: actual_gen,
        },
        context: Some(ErrorContext {
            scope: ErrorScope::Entity { entity_kind: kind.to_string(), index: idx },
            suggested_fixes: Vec::new(),
            detail: format!("{} {} has been deleted", kind, idx),
        }),
    }
}
