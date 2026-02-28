//! CRUD operations for mesh entities: Face, HalfEdge, Vertex, Loop, Edge.
//!
//! DOMAIN: Insert, get, remove, iterate, count, and version/generation
//! queries for the core halfedge mesh entities.

use forge_core::KernelError;

use crate::arena::core::TopologyArena;
use crate::arena::slot::{validate_generation, cold_err_bounds, cold_err_deleted};
use crate::arena::mesh_schema::*;
use crate::handles::{FaceId, HalfEdgeId, VertexId, LoopId, EdgeId};

impl TopologyArena {
    // ── Face ────────────────────────────────────────────────────

    /// Insert a new face, returning its handle.
    pub(crate) fn insert_face(&mut self, data: FaceData) -> FaceId {
        let shell = data.shell();
        let (index, gen) = Self::insert_slot(&mut self.face_slots, &mut self.free_face_head, data);
        self.active_face_count += 1;
        let face_id = FaceId::new(index, gen);
        self.index_add_face(face_id, shell);
        face_id
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

    /// Remove a face, bumping the generation of its slot.
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

    /// Iterate over all active faces, yielding `(FaceId, &FaceData)` pairs.
    pub fn iter_faces(&self) -> impl Iterator<Item = (FaceId, &FaceData)> {
        self.face_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((FaceId::new(i as u32, slot.generation), data))
        })
    }

    /// Count of active (non-deleted) faces.
    pub fn face_count(&self) -> usize { self.active_face_count }

    /// Total face slot count (including vacant slots).
    pub fn face_slot_count(&self) -> usize { self.face_slots.len() }

    /// Indices of all active (occupied) face slots.
    pub fn active_face_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.face_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Generation of face at slot index, or None if vacant/out-of-bounds.
    pub fn face_generation(&self, index: usize) -> Option<u32> {
        self.face_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Version of face at slot index, or None if vacant/out-of-bounds.
    pub fn face_version(&self, index: usize) -> Option<u32> {
        self.face_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
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

    // ── HalfEdge ───────────────────────────────────────────────

    /// Insert a single halfedge, returning its handle.
    ///
    /// The caller is responsible for setting the `radial_next` field correctly.
    pub(crate) fn insert_half_edge(&mut self, data: HalfEdgeData) -> HalfEdgeId {
        let face = data.face();
        let origin = data.origin();
        let (index, gen) = Self::insert_slot(&mut self.half_edge_slots, &mut self.free_half_edge_head, data);
        self.active_half_edge_count += 1;
        let he_id = HalfEdgeId::new(index, gen);
        self.index_add_halfedge(he_id, face, origin);
        he_id
    }

    /// Insert a pair of radial halfedges and wire their `radial_next` fields reciprocally.
    ///
    /// Returns `(he_a, he_b)` where `he_a.radial_next == he_b` and `he_b.radial_next == he_a`.
    pub(crate) fn insert_radial_pair(
        &mut self,
        mut data_a: HalfEdgeData,
        mut data_b: HalfEdgeData,
    ) -> (HalfEdgeId, HalfEdgeId) {
        data_a.set_radial_next(HalfEdgeId::new(u32::MAX, 0));
        data_b.set_radial_next(HalfEdgeId::new(u32::MAX, 0));

        let he_a_id = self.insert_half_edge(data_a);
        let he_b_id = self.insert_half_edge(data_b);

        if let Some(he_a) = self.half_edge_slots[he_a_id.index() as usize].data.as_mut() {
            he_a.set_radial_next(he_b_id);
        }
        if let Some(he_b) = self.half_edge_slots[he_b_id.index() as usize].data.as_mut() {
            he_b.set_radial_next(he_a_id);
        }

        (he_a_id, he_b_id)
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

    /// Remove a halfedge, bumping the generation of its slot.
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

    /// Iterate over all active halfedges, yielding `(HalfEdgeId, &HalfEdgeData)` pairs.
    pub fn iter_half_edges(&self) -> impl Iterator<Item = (HalfEdgeId, &HalfEdgeData)> {
        self.half_edge_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((HalfEdgeId::new(i as u32, slot.generation), data))
        })
    }

    /// Count of active halfedges.
    pub fn half_edge_count(&self) -> usize { self.active_half_edge_count }

    /// Total halfedge slot count (including vacant slots).
    pub fn half_edge_slot_count(&self) -> usize { self.half_edge_slots.len() }

    /// Indices of all active (occupied) halfedge slots.
    pub fn active_half_edge_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.half_edge_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Generation of halfedge at slot index, or None if vacant/out-of-bounds.
    pub fn half_edge_generation(&self, index: usize) -> Option<u32> {
        self.half_edge_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Version of halfedge at slot index, or None if vacant/out-of-bounds.
    pub fn half_edge_version(&self, index: usize) -> Option<u32> {
        self.half_edge_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }

    // ── Vertex ─────────────────────────────────────────────────

    /// Insert a new vertex, returning its handle.
    pub(crate) fn insert_vertex(&mut self, data: VertexData) -> VertexId {
        let (index, gen) = Self::insert_slot(&mut self.vertex_slots, &mut self.free_vertex_head, data);
        self.active_vertex_count += 1;
        VertexId::new(index, gen)
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

    /// Remove a vertex, bumping the generation of its slot.
    pub(crate) fn remove_vertex(&mut self, id: VertexId) -> Result<VertexData, KernelError> {
        let slot = self.vertex_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Vertex", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Vertex", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Vertex", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        slot.next_free = self.free_vertex_head;
        self.free_vertex_head = Some(id.index());
        self.active_vertex_count -= 1;
        Ok(data)
    }

    /// Iterate over all active vertices, yielding `(VertexId, &VertexData)` pairs.
    pub fn iter_vertices(&self) -> impl Iterator<Item = (VertexId, &VertexData)> {
        self.vertex_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((VertexId::new(i as u32, slot.generation), data))
        })
    }

    /// Count of active vertices.
    pub fn vertex_count(&self) -> usize { self.active_vertex_count }

    /// Total vertex slot count (including vacant slots).
    pub fn vertex_slot_count(&self) -> usize { self.vertex_slots.len() }

    /// Indices of all active (occupied) vertex slots.
    pub fn active_vertex_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.vertex_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Generation of vertex at slot index, or None if vacant/out-of-bounds.
    pub fn vertex_generation(&self, index: usize) -> Option<u32> {
        self.vertex_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Version of vertex at slot index, or None if vacant/out-of-bounds.
    pub fn vertex_version(&self, index: usize) -> Option<u32> {
        self.vertex_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }

    // ── Loop ───────────────────────────────────────────────────

    /// Insert a new loop, returning its handle.
    pub(crate) fn insert_loop(&mut self, data: LoopData) -> LoopId {
        let (index, gen) = Self::insert_slot(&mut self.loop_slots, &mut self.free_loop_head, data);
        self.active_loop_count += 1;
        LoopId::new(index, gen)
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

    /// Remove a loop, bumping the generation of its slot.
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

    /// Iterate over all active loops, yielding `(LoopId, &LoopData)` pairs.
    pub fn iter_loops(&self) -> impl Iterator<Item = (LoopId, &LoopData)> {
        self.loop_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((LoopId::new(i as u32, slot.generation), data))
        })
    }

    /// Count of active loops.
    pub fn loop_count(&self) -> usize { self.active_loop_count }

    /// Indices of all active (occupied) loop slots.
    pub fn active_loop_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.loop_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Generation of loop at slot index, or None if vacant/out-of-bounds.
    pub fn loop_generation(&self, index: usize) -> Option<u32> {
        self.loop_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Version of loop at slot index, or None if vacant/out-of-bounds.
    pub fn loop_version(&self, index: usize) -> Option<u32> {
        self.loop_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }

    // ── Edge ───────────────────────────────────────────────────

    /// Insert a new edge, returning its handle.
    pub(crate) fn insert_edge(&mut self, data: EdgeData) -> EdgeId {
        let (index, gen) = Self::insert_slot(&mut self.edge_slots, &mut self.free_edge_head, data);
        self.active_edge_count += 1;
        EdgeId::new(index, gen)
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
    pub(crate) fn remove_edge(&mut self, id: EdgeId) -> Result<EdgeData, KernelError> {
        let slot = self.edge_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Edge", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Edge", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Edge", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        slot.next_free = self.free_edge_head;
        self.free_edge_head = Some(id.index());
        self.active_edge_count -= 1;
        Ok(data)
    }

    /// Iterate over all active edges, yielding `(EdgeId, &EdgeData)` pairs.
    pub fn iter_edges(&self) -> impl Iterator<Item = (EdgeId, &EdgeData)> {
        self.edge_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((EdgeId::new(i as u32, slot.generation), data))
        })
    }

    /// Count of active edges.
    pub fn edge_count(&self) -> usize { self.active_edge_count }

    /// Indices of all active (occupied) edge slots.
    pub fn active_edge_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.edge_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Generation of edge at slot index, or None if vacant/out-of-bounds.
    pub fn edge_generation(&self, index: usize) -> Option<u32> {
        self.edge_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Version of edge at slot index, or None if vacant/out-of-bounds.
    pub fn edge_version(&self, index: usize) -> Option<u32> {
        self.edge_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }

    /// Helper to fetch topological endpoints of an undirected edge.
    pub fn get_edge_endpoints(
        &self,
        edge_id: crate::handles::EdgeId,
    ) -> Result<(VertexId, VertexId), KernelError> {
        let he_id = self.get_edge(edge_id)?.half_edge();
        let he = self.get_half_edge(he_id)?;
        let origin = he.origin();
        let dest = self.get_half_edge(he.next())?.origin();
        Ok((origin, dest))
    }
}
