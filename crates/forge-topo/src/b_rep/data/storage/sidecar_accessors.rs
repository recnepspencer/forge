//! Typed accessors for side-car metadata on TopologyArena.
//!
//! DOMAIN: Read/write access to slot-parallel metadata vectors
//! that were stripped from entity structs (Milestone 1).

use crate::b_rep::data::mesh::{EdgeRadialClass, VertexDiskClass};
use crate::b_rep::data::storage::arena::TopologyArena;
use crate::handles::{EdgeId, HalfEdgeId, VertexId};
use forge_core::{KernelError, TopologyError};
use smallvec::{smallvec, SmallVec};

impl TopologyArena {
    // ── Radial classification (HalfEdge side-car + fallback recompute) ──────

    /// Classify topology of an edge use from a halfedge.
    ///
    /// Uses cached radial valence when available; otherwise recomputes from
    /// the radial ring and stores the result for all members of that ring.
    pub fn classify_half_edge(&self, he: HalfEdgeId) -> Result<EdgeRadialClass, KernelError> {
        self.get_half_edge(he)?;
        let idx = he.index() as usize;

        if let Some(&cached) = self.metadata.radial_valence.get(idx) {
            if cached > 0 {
                #[cfg(debug_assertions)]
                {
                    let recomputed = self.compute_radial_valence(he)?;
                    debug_assert_eq!(
                        cached as usize,
                        recomputed,
                        "cached radial valence drift for halfedge {}: cached={}, recomputed={}",
                        he.index(),
                        cached,
                        recomputed,
                    );
                }
                return Ok(classify_radial_count(cached as usize));
            }
        }

        let recomputed = self.compute_radial_valence(he)?;
        Ok(classify_radial_count(recomputed))
    }

    /// Classify topology of an edge from its representative halfedge.
    pub fn classify_edge(&self, edge: EdgeId) -> Result<EdgeRadialClass, KernelError> {
        let rep = self.get_edge(edge)?.half_edge();
        self.classify_half_edge(rep)
    }

    /// Classify vertex disk state from primary + extra disk entries.
    pub fn classify_vertex(&self, vertex: VertexId) -> Result<VertexDiskClass, KernelError> {
        let _ = self.get_vertex(vertex)?;
        let count = self.disk_count(vertex);
        if count <= 1 {
            Ok(VertexDiskClass::Single)
        } else {
            Ok(VertexDiskClass::Multi { count })
        }
    }

    // ── Wire Topology (Edge / Shell side-cars) ──────────────────────

    pub(crate) fn grow_shell_sidecars(&mut self, capacity: usize) {
        if self.metadata.shell_entry_edges.len() < capacity {
            self.metadata.shell_entry_edges.resize(capacity, None);
        }
    }

    pub(crate) fn clear_shell_sidecar(&mut self, index: usize) {
        if index < self.metadata.shell_entry_edges.len() {
            self.metadata.shell_entry_edges[index] = None;
        }
    }

    // ── Vertex disk entries (NMT side-car) ──────────────────────────

    /// Primary disk entry (always present).
    pub fn primary_disk_entry(&self, v: VertexId) -> Result<HalfEdgeId, KernelError> {
        Ok(self.get_vertex(v)?.primary_disk())
    }

    /// All disk entries: primary plus any NMT extras.
    pub fn disk_entries(&self, v: VertexId) -> Result<SmallVec<[HalfEdgeId; 4]>, KernelError> {
        let primary = self.get_vertex(v)?.primary_disk();
        let mut entries = smallvec![primary];
        if let Some(extras) = self.metadata.nmt_extra_disks.get(&v) {
            entries.extend_from_slice(extras);
        }
        Ok(entries)
    }

    /// Number of disk entries at this vertex.
    pub fn disk_count(&self, v: VertexId) -> usize {
        1 + self
            .metadata
            .nmt_extra_disks
            .get(&v)
            .map_or(0, |entries| entries.len())
    }

    /// Whether the vertex currently has extra NMT disk entries.
    pub fn is_vertex_nmt(&self, v: VertexId) -> bool {
        self.metadata
            .vertex_is_nmt
            .get(v.index() as usize)
            .copied()
            .unwrap_or(false)
    }

    /// Append an extra disk entry, marking this vertex as NMT.
    pub fn add_disk_entry(&mut self, v: VertexId, he: HalfEdgeId) {
        self.metadata.nmt_extra_disks.entry(v).or_default().push(he);
        let idx = v.index() as usize;
        if idx >= self.metadata.vertex_is_nmt.len() {
            self.metadata.vertex_is_nmt.resize(idx + 1, false);
        }
        self.metadata.vertex_is_nmt[idx] = true;
    }

    /// Remove an entry from the extra NMT disk list. Returns false if absent.
    pub fn remove_disk_entry(&mut self, v: VertexId, he: HalfEdgeId) -> bool {
        let Some(extras) = self.metadata.nmt_extra_disks.get_mut(&v) else {
            return false;
        };
        let Some(pos) = extras.iter().position(|&entry| entry == he) else {
            return false;
        };
        extras.swap_remove(pos);
        if extras.is_empty() {
            self.metadata.nmt_extra_disks.remove(&v);
            if let Some(flag) = self.metadata.vertex_is_nmt.get_mut(v.index() as usize) {
                *flag = false;
            }
        }
        true
    }

    /// Replace an extra NMT disk entry value. Returns false if old entry is absent.
    pub fn replace_disk_entry(&mut self, v: VertexId, old: HalfEdgeId, new: HalfEdgeId) -> bool {
        let Some(extras) = self.metadata.nmt_extra_disks.get_mut(&v) else {
            return false;
        };
        let Some(pos) = extras.iter().position(|&entry| entry == old) else {
            return false;
        };
        extras[pos] = new;
        true
    }

    /// Set the primary disk entry.
    pub fn set_primary_disk_entry(
        &mut self,
        v: VertexId,
        he: HalfEdgeId,
    ) -> Result<(), KernelError> {
        self.get_vertex_mut(v)?.set_primary_disk(he);
        Ok(())
    }

    /// Reset disk entries to an explicit primary + extras list.
    ///
    /// This is the canonical write path after rebuilding disk entries:
    /// it clears stale extras/flags and reapplies extras deterministically.
    pub fn reset_disk_entries(
        &mut self,
        v: VertexId,
        primary: HalfEdgeId,
        extras: &[HalfEdgeId],
    ) -> Result<(), KernelError> {
        self.get_vertex_mut(v)?.set_primary_disk(primary);
        self.metadata.nmt_extra_disks.remove(&v);
        if let Some(flag) = self.metadata.vertex_is_nmt.get_mut(v.index() as usize) {
            *flag = false;
        }
        for &he in extras {
            self.add_disk_entry(v, he);
        }
        Ok(())
    }

    /// Refresh cached radial valence for every halfedge in a radial ring.
    pub(crate) fn refresh_cached_radial_valence_for_ring(
        &mut self,
        start: HalfEdgeId,
    ) -> Result<(), KernelError> {
        let valence = self.compute_radial_valence(start)?;
        let mut current = start;
        let bound = self.half_edge_count().max(1);
        let encoded = u8::try_from(valence).unwrap_or(u8::MAX);

        for step in 0..=bound {
            let idx = current.index() as usize;
            if idx >= self.metadata.radial_valence.len() {
                self.metadata.radial_valence.resize(idx + 1, 0);
            }
            self.metadata.radial_valence[idx] = encoded;

            let next = self.get_half_edge(current)?.radial_next();
            if next == start {
                break;
            }
            current = next;

            if step == bound {
                return Err(KernelError::TopologyViolation {
                    err: TopologyError::LoopCorruption {
                        walk_kind: "refresh_cached_radial_valence_for_ring".to_string(),
                        seed_index: start.index(),
                        last_visited_index: current.index(),
                        steps_taken: step + 1,
                        entity_bound: bound,
                    },
                    context: None,
                });
            }
        }

        Ok(())
    }

    /// Rebuild cached radial valence for every active halfedge ring.
    ///
    /// This is a correctness backstop for operators that mutate radial links
    /// without calling targeted cache refresh hooks.
    pub(crate) fn rebuild_cached_radial_valence(&mut self) -> Result<(), KernelError> {
        let slot_len = self.connectivity.half_edge_slots.len();
        if self.metadata.radial_valence.len() < slot_len {
            self.metadata.radial_valence.resize(slot_len, 0);
        }
        for value in self.metadata.radial_valence.iter_mut().take(slot_len) {
            *value = 0;
        }

        let halfedges: Vec<_> = self.iter_half_edges().map(|(id, _)| id).collect();
        for he in halfedges {
            let idx = he.index() as usize;
            if self.metadata.radial_valence.get(idx).copied().unwrap_or(0) == 0 {
                self.refresh_cached_radial_valence_for_ring(he)?;
            }
        }
        Ok(())
    }

    // ── Lockstep growth helpers ─────────────────────────────────────

    /// Ensure the halfedge side-car vectors are at least `len` long.
    pub(crate) fn grow_halfedge_sidecars(&mut self, len: usize) {
        if self.metadata.bridge_flags.len() < len {
            self.metadata.bridge_flags.resize(len, false);
        }
        if self.metadata.coedge_data.len() < len {
            self.metadata.coedge_data.resize(len, None);
        }
        if self.metadata.radial_valence.len() < len {
            self.metadata.radial_valence.resize(len, 0);
        }
    }

    /// Clear half-edge side-car data at the given slot index.
    pub(crate) fn clear_halfedge_sidecar(&mut self, index: usize) {
        if index < self.metadata.bridge_flags.len() {
            self.metadata.bridge_flags[index] = false;
        }
        if index < self.metadata.coedge_data.len() {
            self.metadata.coedge_data[index] = None;
        }
        if index < self.metadata.radial_valence.len() {
            self.metadata.radial_valence[index] = 0;
        }
    }

    /// Ensure the edge side-car vectors are at least `len` long.
    pub(crate) fn grow_edge_sidecars(&mut self, len: usize) {
        if self.metadata.edge_curves.len() < len {
            self.metadata.edge_curves.resize(len, None);
        }
        if self.metadata.edge_shells.len() < len {
            self.metadata.edge_shells.resize(len, None);
        }
    }

    /// Clear edge side-car data at the given slot index.
    pub(crate) fn clear_edge_sidecar(&mut self, index: usize) {
        if index < self.metadata.edge_curves.len() {
            self.metadata.edge_curves[index] = None;
        }
        if index < self.metadata.edge_shells.len() {
            self.metadata.edge_shells[index] = None;
        }
    }

    /// Ensure the vertex side-car vectors are at least `len` long.
    pub(crate) fn grow_vertex_sidecars(&mut self, len: usize) {
        if self.metadata.vertex_provenance.len() < len {
            self.metadata.vertex_provenance.resize(len, None);
        }
        if self.metadata.vertex_is_nmt.len() < len {
            self.metadata.vertex_is_nmt.resize(len, false);
        }
    }

    /// Clear vertex side-car data at the given slot index.
    pub(crate) fn clear_vertex_sidecar(&mut self, index: usize) {
        if index < self.metadata.vertex_provenance.len() {
            self.metadata.vertex_provenance[index] = None;
        }
        if index < self.metadata.vertex_is_nmt.len() {
            self.metadata.vertex_is_nmt[index] = false;
        }
    }

    fn compute_radial_valence(&self, start: HalfEdgeId) -> Result<usize, KernelError> {
        let mut count = 1usize;
        let mut current = self.get_half_edge(start)?.radial_next();
        let bound = self.half_edge_count().max(1);

        for step in 0..=bound {
            if current == start {
                return Ok(count);
            }
            count += 1;
            current = self.get_half_edge(current)?.radial_next();

            if step == bound {
                return Err(KernelError::TopologyViolation {
                    err: TopologyError::LoopCorruption {
                        walk_kind: "compute_radial_valence".to_string(),
                        seed_index: start.index(),
                        last_visited_index: current.index(),
                        steps_taken: step + 1,
                        entity_bound: bound,
                    },
                    context: None,
                });
            }
        }

        unreachable!("radial valence loop exited unexpectedly");
    }
}

fn classify_radial_count(count: usize) -> EdgeRadialClass {
    match count {
        0 | 1 => EdgeRadialClass::Boundary,
        2 => EdgeRadialClass::Manifold,
        _ => EdgeRadialClass::NonManifold,
    }
}
