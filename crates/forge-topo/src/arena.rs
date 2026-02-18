//! Arena-based entity storage for topology.
//!
//! DOMAIN: Entity allocation and retrieval with generational handles.
//!
//! INVARIANTS:
//! - Handles encode a generation counter to detect stale references
//! - Slots are reusable after deletion (generation is bumped)
//! - All accessors validate generation before returning data
//!
//! DEPENDENCIES: `handles` (typed IDs), `lineage` (inline provenance)

use serde::{Deserialize, Serialize};

use forge_core::{KernelError, TopologyError, ErrorContext, ErrorScope};
use crate::attributes::AttributeStore;
use crate::handles::{FaceId, HalfEdgeId, VertexId, LoopId};
use crate::lineage::Lineage;

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
    /// Side-car attribute storage for manufacturing metadata.
    attribute_store: AttributeStore,
}

/// A slot in the arena that may be occupied or vacant.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Slot<T: Clone> {
    /// The current generation of this slot.
    generation: u32,
    /// The current version of the data in this slot (increments on mutation).
    version: u32,
    /// The data, if the slot is occupied.
    data: Option<T>,
}

impl<T: Clone> Slot<T> {
    /// Create a new empty slot at generation 0.
    fn empty() -> Self {
        Self {
            generation: 0,
            version: 0,
            data: None,
        }
    }

    /// Occupy this slot with data, returning the current generation.
    /// Resets version to 0.
    fn occupy(&mut self, data: T) -> u32 {
        self.data = Some(data);
        self.version = 0;
        self.generation
    }
}

/// Data stored for each face.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceData {
    /// The outer boundary loop of this face.
    pub outer_loop: LoopId,
    /// Inline lineage for provenance tracking.
    pub lineage: Option<Lineage>,
}

/// Data stored for each halfedge.
///
/// Twin, next, and prev are all explicit pointers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HalfEdgeData {
    /// The twin halfedge (on the adjacent face).
    pub twin: HalfEdgeId,
    /// The next halfedge in the face loop.
    pub next: HalfEdgeId,
    /// The previous halfedge in the face loop.
    pub prev: HalfEdgeId,
    /// The face this halfedge borders.
    pub face: FaceId,
    /// The origin vertex.
    pub origin: VertexId,
    /// Inline lineage for provenance tracking.
    pub lineage: Option<Lineage>,
}

/// Data stored for each vertex.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexData {
    /// One outgoing halfedge (for traversal entry).
    pub outgoing: HalfEdgeId,
    /// Inline lineage for provenance tracking.
    pub lineage: Option<Lineage>,
}

/// Data stored for each loop (boundary of a face).
///
/// Each face has at least one loop (outer boundary).
/// Future: inner loops represent holes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopData {
    /// One halfedge on this loop (entry point for traversal).
    pub half_edge: HalfEdgeId,
    /// The face this loop belongs to.
    pub face: FaceId,
}

impl TopologyArena {
    /// Create an empty arena with no entities.
    pub fn new() -> Self {
        Self {
            face_slots: Vec::new(),
            half_edge_slots: Vec::new(),
            vertex_slots: Vec::new(),
            loop_slots: Vec::new(),
            attribute_store: AttributeStore::new(),
        }
    }

    /// Insert a new face, returning its handle.
    pub fn insert_face(&mut self, data: FaceData) -> FaceId {
        let index = self.face_slots.len() as u32;
        let mut slot = Slot::empty();
        let gen = slot.occupy(data);
        self.face_slots.push(slot);
        FaceId::new(index, gen)
    }

    /// Insert a single halfedge, returning its handle.
    ///
    /// The caller is responsible for setting the `twin` field correctly.
    pub fn insert_half_edge(&mut self, data: HalfEdgeData) -> HalfEdgeId {
        let index = self.half_edge_slots.len() as u32;
        let mut slot = Slot::empty();
        let gen = slot.occupy(data);
        self.half_edge_slots.push(slot);
        HalfEdgeId::new(index, gen)
    }

    /// Insert a pair of twin halfedges and wire their `twin` fields reciprocally.
    ///
    /// Returns `(he_a, he_b)` where `he_a.twin == he_b` and `he_b.twin == he_a`.
    pub fn insert_half_edge_pair(
        &mut self,
        mut data_a: HalfEdgeData,
        mut data_b: HalfEdgeData,
    ) -> (HalfEdgeId, HalfEdgeId) {
        let base = self.half_edge_slots.len() as u32;

        let he_a_id = HalfEdgeId::new(base, 0);
        let he_b_id = HalfEdgeId::new(base + 1, 0);

        data_a.twin = he_b_id;
        data_b.twin = he_a_id;

        let mut slot_a = Slot::empty();
        let gen_a = slot_a.occupy(data_a);
        self.half_edge_slots.push(slot_a);

        let mut slot_b = Slot::empty();
        let gen_b = slot_b.occupy(data_b);
        self.half_edge_slots.push(slot_b);

        (HalfEdgeId::new(base, gen_a), HalfEdgeId::new(base + 1, gen_b))
    }

    /// Insert a new vertex, returning its handle.
    pub fn insert_vertex(&mut self, data: VertexData) -> VertexId {
        let index = self.vertex_slots.len() as u32;
        let mut slot = Slot::empty();
        let gen = slot.occupy(data);
        self.vertex_slots.push(slot);
        VertexId::new(index, gen)
    }

    /// Insert a new loop, returning its handle.
    pub fn insert_loop(&mut self, data: LoopData) -> LoopId {
        let index = self.loop_slots.len() as u32;
        let mut slot = Slot::empty();
        let gen = slot.occupy(data);
        self.loop_slots.push(slot);
        LoopId::new(index, gen)
    }

    /// Get a face by handle, validating the generation.
    pub fn get_face(&self, id: FaceId) -> Result<&FaceData, KernelError> {
        let slot = self.face_slots.get(id.index() as usize).ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Face",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: 0,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Face", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Face index {} out of bounds", id.index()),
                }),
            }
        })?;
        validate_generation(slot.generation, id.generation(), "Face", id.index())?;
        slot.data.as_ref().ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Face",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: slot.generation,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Face", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Face {} has been deleted", id.index()),
                }),
            }
        })
    }

    /// Get a halfedge by handle, validating the generation.
    pub fn get_half_edge(&self, id: HalfEdgeId) -> Result<&HalfEdgeData, KernelError> {
        let slot = self.half_edge_slots.get(id.index() as usize).ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "HalfEdge",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: 0,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "HalfEdge", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("HalfEdge index {} out of bounds", id.index()),
                }),
            }
        })?;
        validate_generation(slot.generation, id.generation(), "HalfEdge", id.index())?;
        slot.data.as_ref().ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "HalfEdge",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: slot.generation,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "HalfEdge", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("HalfEdge {} has been deleted", id.index()),
                }),
            }
        })
    }

    /// Get a vertex by handle, validating the generation.
    pub fn get_vertex(&self, id: VertexId) -> Result<&VertexData, KernelError> {
        let slot = self.vertex_slots.get(id.index() as usize).ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Vertex",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: 0,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Vertex", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Vertex index {} out of bounds", id.index()),
                }),
            }
        })?;
        validate_generation(slot.generation, id.generation(), "Vertex", id.index())?;
        slot.data.as_ref().ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Vertex",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: slot.generation,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Vertex", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Vertex {} has been deleted", id.index()),
                }),
            }
        })
    }

    /// Get a loop by handle, validating the generation.
    pub fn get_loop(&self, id: LoopId) -> Result<&LoopData, KernelError> {
        let slot = self.loop_slots.get(id.index() as usize).ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Loop",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: 0,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Loop", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Loop index {} out of bounds", id.index()),
                }),
            }
        })?;
        validate_generation(slot.generation, id.generation(), "Loop", id.index())?;
        slot.data.as_ref().ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Loop",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: slot.generation,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Loop", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Loop {} has been deleted", id.index()),
                }),
            }
        })
    }

    /// Get a mutable reference to a face by handle.
    pub fn get_face_mut(&mut self, id: FaceId) -> Result<&mut FaceData, KernelError> {
        let slot = self.face_slots.get_mut(id.index() as usize).ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Face",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: 0,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Face", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Face index {} out of bounds", id.index()),
                }),
            }
        })?;
        validate_generation(slot.generation, id.generation(), "Face", id.index())?;
        
        // Increment version on mutable access
        slot.version += 1;

        slot.data.as_mut().ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Face",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: slot.generation,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Face", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Face {} has been deleted", id.index()),
                }),
            }
        })
    }

    /// Get a mutable reference to a halfedge by handle.
    pub fn get_half_edge_mut(&mut self, id: HalfEdgeId) -> Result<&mut HalfEdgeData, KernelError> {
        let slot = self.half_edge_slots.get_mut(id.index() as usize).ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "HalfEdge",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: 0,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "HalfEdge", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("HalfEdge index {} out of bounds", id.index()),
                }),
            }
        })?;
        validate_generation(slot.generation, id.generation(), "HalfEdge", id.index())?;

        // Increment version on mutable access
        slot.version += 1;

        slot.data.as_mut().ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "HalfEdge",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: slot.generation,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "HalfEdge", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("HalfEdge {} has been deleted", id.index()),
                }),
            }
        })
    }

    /// Get a mutable reference to a vertex by handle.
    pub fn get_vertex_mut(&mut self, id: VertexId) -> Result<&mut VertexData, KernelError> {
        let slot = self.vertex_slots.get_mut(id.index() as usize).ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Vertex",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: 0,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Vertex", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Vertex index {} out of bounds", id.index()),
                }),
            }
        })?;
        validate_generation(slot.generation, id.generation(), "Vertex", id.index())?;

        // Increment version on mutable access
        slot.version += 1;

        slot.data.as_mut().ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Vertex",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: slot.generation,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Vertex", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Vertex {} has been deleted", id.index()),
                }),
            }
        })
    }

    /// Get a mutable reference to a loop by handle.
    pub fn get_loop_mut(&mut self, id: LoopId) -> Result<&mut LoopData, KernelError> {
        let slot = self.loop_slots.get_mut(id.index() as usize).ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Loop",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: 0,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Loop", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Loop index {} out of bounds", id.index()),
                }),
            }
        })?;
        validate_generation(slot.generation, id.generation(), "Loop", id.index())?;

        // Increment version on mutable access
        slot.version += 1;

        slot.data.as_mut().ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Loop",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: slot.generation,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Loop", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Loop {} has been deleted", id.index()),
                }),
            }
        })
    }

    /// Count of active (non-deleted) faces.
    pub fn face_count(&self) -> usize {
        self.face_slots.iter().filter(|s| s.data.is_some()).count()
    }

    /// Count of active halfedges.
    pub fn half_edge_count(&self) -> usize {
        self.half_edge_slots.iter().filter(|s| s.data.is_some()).count()
    }

    /// Count of active vertices.
    pub fn vertex_count(&self) -> usize {
        self.vertex_slots.iter().filter(|s| s.data.is_some()).count()
    }

    /// Count of active loops.
    pub fn loop_count(&self) -> usize {
        self.loop_slots.iter().filter(|s| s.data.is_some()).count()
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
        let slot = self.face_slots.get_mut(id.index() as usize).ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Face",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: 0,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Face", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Face index {} out of bounds", id.index()),
                }),
            }
        })?;
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
    pub fn remove_face(&mut self, id: FaceId) -> Result<FaceData, KernelError> {
        let slot = self.face_slots.get_mut(id.index() as usize).ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Face",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: 0,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Face", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Face index {} out of bounds", id.index()),
                }),
            }
        })?;
        validate_generation(slot.generation, id.generation(), "Face", id.index())?;
        let data = slot.data.take().ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Face",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: slot.generation,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Face", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Face {} already deleted", id.index()),
                }),
            }
        })?;
        slot.generation += 1;
        Ok(data)
    }

    /// Remove a vertex, bumping the generation of its slot.
    pub fn remove_vertex(&mut self, id: VertexId) -> Result<VertexData, KernelError> {
        let slot = self.vertex_slots.get_mut(id.index() as usize).ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Vertex",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: 0,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Vertex", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Vertex index {} out of bounds", id.index()),
                }),
            }
        })?;
        validate_generation(slot.generation, id.generation(), "Vertex", id.index())?;
        let data = slot.data.take().ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Vertex",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: slot.generation,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Vertex", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Vertex {} already deleted", id.index()),
                }),
            }
        })?;
        slot.generation += 1;
        Ok(data)
    }

    /// Remove a halfedge, bumping the generation of its slot.
    pub fn remove_half_edge(&mut self, id: HalfEdgeId) -> Result<HalfEdgeData, KernelError> {
        let slot = self.half_edge_slots.get_mut(id.index() as usize).ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "HalfEdge",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: 0,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "HalfEdge", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("HalfEdge index {} out of bounds", id.index()),
                }),
            }
        })?;
        validate_generation(slot.generation, id.generation(), "HalfEdge", id.index())?;
        let data = slot.data.take().ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "HalfEdge",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: slot.generation,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "HalfEdge", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("HalfEdge {} already deleted", id.index()),
                }),
            }
        })?;
        slot.generation += 1;
        Ok(data)
    }

    /// Remove a loop, bumping the generation of its slot.
    pub fn remove_loop(&mut self, id: LoopId) -> Result<LoopData, KernelError> {
        let slot = self.loop_slots.get_mut(id.index() as usize).ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Loop",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: 0,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Loop", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Loop index {} out of bounds", id.index()),
                }),
            }
        })?;
        validate_generation(slot.generation, id.generation(), "Loop", id.index())?;
        let data = slot.data.take().ok_or_else(|| {
            KernelError::TopologyViolation {
                err: TopologyError::StaleHandle {
                    entity_kind: "Loop",
                    index: id.index(),
                    expected_generation: id.generation(),
                    actual_generation: slot.generation,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Entity { entity_kind: "Loop", index: id.index() },
                    suggested_fixes: Vec::new(),
                    detail: format!("Loop {} already deleted", id.index()),
                }),
            }
        })?;
        slot.generation += 1;
        Ok(data)
    }
}

impl Default for TopologyArena {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate that a handle's generation matches the slot's generation.
fn validate_generation(
    slot_gen: u32,
    handle_gen: u32,
    entity_type: &'static str,
    index: u32,
) -> Result<(), KernelError> {
    if slot_gen != handle_gen {
        return Err(KernelError::TopologyViolation {
            err: TopologyError::StaleHandle {
                entity_kind: entity_type,
                index,
                expected_generation: handle_gen,
                actual_generation: slot_gen,
            },
            context: Some(ErrorContext {
                scope: ErrorScope::Entity { entity_kind: entity_type, index },
                suggested_fixes: Vec::new(),
                detail: format!(
                    "{} handle generation {} does not match slot generation {}",
                    entity_type, handle_gen, slot_gen
                ),
            }),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_vertex_data() -> VertexData {
        VertexData {
            outgoing: HalfEdgeId::new(0, 0),
            lineage: None,
        }
    }

    fn dummy_face_data() -> FaceData {
        FaceData {
            outer_loop: LoopId::new(0, 0),
            lineage: None,
        }
    }

    #[test]
    fn insert_and_get_vertex() {
        let mut arena = TopologyArena::new();
        let id = arena.insert_vertex(dummy_vertex_data());
        let vertex = arena.get_vertex(id);
        assert!(vertex.is_ok());
    }

    #[test]
    fn insert_and_get_face() {
        let mut arena = TopologyArena::new();
        let id = arena.insert_face(dummy_face_data());
        let face = arena.get_face(id);
        assert!(face.is_ok());
    }

    #[test]
    fn stale_handle_returns_error() {
        let mut arena = TopologyArena::new();
        let id = arena.insert_vertex(dummy_vertex_data());
        arena.remove_vertex(id).unwrap();
        let result = arena.get_vertex(id);
        assert!(result.is_err());
    }

    #[test]
    fn entity_counts() {
        let mut arena = TopologyArena::new();
        assert_eq!(arena.vertex_count(), 0);
        assert_eq!(arena.face_count(), 0);

        arena.insert_vertex(dummy_vertex_data());
        arena.insert_vertex(dummy_vertex_data());
        arena.insert_face(dummy_face_data());

        assert_eq!(arena.vertex_count(), 2);
        assert_eq!(arena.face_count(), 1);
    }

    #[test]
    fn remove_decrements_count() {
        let mut arena = TopologyArena::new();
        let id = arena.insert_vertex(dummy_vertex_data());
        assert_eq!(arena.vertex_count(), 1);
        arena.remove_vertex(id).unwrap();
        assert_eq!(arena.vertex_count(), 0);
    }

    #[test]
    fn out_of_bounds_handle_returns_error() {
        let arena = TopologyArena::new();
        let fake_id = VertexId::new(999, 0);
        let result = arena.get_vertex(fake_id);
        assert!(result.is_err());
    }

    #[test]
    fn clone_is_independent() {
        let mut arena = TopologyArena::new();
        let id = arena.insert_vertex(dummy_vertex_data());

        let arena_clone = arena.clone();
        arena.remove_vertex(id).unwrap();

        assert_eq!(arena.vertex_count(), 0);
        assert_eq!(arena_clone.vertex_count(), 1);
    }

    #[test]
    fn singular_halfedge_insertion() {
        let mut arena = TopologyArena::new();
        let face = arena.insert_face(dummy_face_data());
        let vertex = arena.insert_vertex(dummy_vertex_data());

        let he_id = arena.insert_half_edge(HalfEdgeData {
            twin: HalfEdgeId::new(u32::MAX, 0),
            next: HalfEdgeId::new(0, 0),
            prev: HalfEdgeId::new(0, 0),
            face,
            origin: vertex,
            lineage: None,
        });
        assert_eq!(he_id.index(), 0);
        assert_eq!(arena.half_edge_count(), 1);
    }

    #[test]
    fn paired_halfedge_insertion_sets_twins() {
        let mut arena = TopologyArena::new();
        let face = arena.insert_face(dummy_face_data());
        let vertex = arena.insert_vertex(dummy_vertex_data());

        let (he0, he1) = arena.insert_half_edge_pair(
            HalfEdgeData {
                twin: HalfEdgeId::new(u32::MAX, 0),
                next: HalfEdgeId::new(0, 0),
                prev: HalfEdgeId::new(0, 0),
                face,
                origin: vertex,
                lineage: None,
            },
            HalfEdgeData {
                twin: HalfEdgeId::new(u32::MAX, 0),
                next: HalfEdgeId::new(0, 0),
                prev: HalfEdgeId::new(0, 0),
                face,
                origin: vertex,
                lineage: None,
            },
        );
        assert_eq!(arena.half_edge_count(), 2);
        assert_eq!(arena.get_half_edge(he0).unwrap().twin, he1);
        assert_eq!(arena.get_half_edge(he1).unwrap().twin, he0);
    }

    #[test]
    fn loop_insert_and_get() {
        let mut arena = TopologyArena::new();
        let face = arena.insert_face(dummy_face_data());
        let loop_id = arena.insert_loop(LoopData {
            half_edge: HalfEdgeId::new(0, 0),
            face,
        });
        assert_eq!(arena.loop_count(), 1);
        assert!(arena.get_loop(loop_id).is_ok());
    }
}
