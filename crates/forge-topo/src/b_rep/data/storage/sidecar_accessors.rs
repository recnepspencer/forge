//! Typed accessors for side-car metadata on TopologyArena.
//!
//! DOMAIN: Read/write access to slot-parallel metadata vectors
//! that were stripped from entity structs (Milestone 1).

use crate::b_rep::data::storage::arena::TopologyArena;
use crate::b_rep::data::mesh::CoedgeInfo;
use crate::handles::{HalfEdgeId, EdgeId, VertexId, CurveRef, ShellId};
use forge_core::KernelError;
use smallvec::{SmallVec, smallvec};

impl TopologyArena {
    // ── Bridge flag (HalfEdge side-car) ─────────────────────────────

    /// Whether this halfedge is a synthetic bridge (inserted by `BridgeEdge`).
    pub fn is_bridge(&self, id: HalfEdgeId) -> bool {
        self.bridge_flags
            .get(id.index() as usize)
            .copied()
            .unwrap_or(false)
    }

    /// Mark or unmark this halfedge as a synthetic bridge.
    pub fn set_bridge(&mut self, id: HalfEdgeId, value: bool) {
        let idx = id.index() as usize;
        if idx >= self.bridge_flags.len() {
            self.bridge_flags.resize(idx + 1, false);
        }
        self.bridge_flags[idx] = value;
    }

    // ── Coedge metadata (HalfEdge side-car) ─────────────────────────

    /// Coedge metadata for this halfedge (UV trim curve + direction sense).
    pub fn coedge_info(&self, id: HalfEdgeId) -> Option<&CoedgeInfo> {
        self.coedge_data
            .get(id.index() as usize)
            .and_then(|opt| opt.as_ref())
    }

    /// Set or clear coedge metadata for this halfedge.
    pub fn set_coedge_info(&mut self, id: HalfEdgeId, info: Option<CoedgeInfo>) {
        let idx = id.index() as usize;
        if idx >= self.coedge_data.len() {
            self.coedge_data.resize(idx + 1, None);
        }
        self.coedge_data[idx] = info;
    }

    // ── 3D curve reference (Edge side-car) ──────────────────────────

    /// Opaque reference to this edge's 3D curve in the geometry store.
    pub fn edge_curve(&self, id: EdgeId) -> Option<CurveRef> {
        self.edge_curves
            .get(id.index() as usize)
            .and_then(|opt| *opt)
    }

    /// Set or clear the 3D curve reference for this edge.
    pub fn set_edge_curve(&mut self, id: EdgeId, curve: Option<CurveRef>) {
        let idx = id.index() as usize;
        if idx >= self.edge_curves.len() {
            self.edge_curves.resize(idx + 1, None);
        }
        self.edge_curves[idx] = curve;
    }

    // ── Wire Topology (Edge / Shell side-cars) ──────────────────────

    pub(crate) fn grow_shell_sidecars(&mut self, capacity: usize) {
        if self.shell_entry_edges.len() < capacity {
            self.shell_entry_edges.resize(capacity, None);
        }
    }

    pub(crate) fn clear_shell_sidecar(&mut self, index: usize) {
        if index < self.shell_entry_edges.len() {
            self.shell_entry_edges[index] = None;
        }
    }

    /// The parent shell for this wire-body edge, if any.
    pub fn edge_shell(&self, id: EdgeId) -> Option<ShellId> {
        self.edge_shells
            .get(id.index() as usize)
            .and_then(|opt| *opt)
    }

    /// Set or clear the parent shell for this wire-body edge.
    pub fn set_edge_shell(&mut self, id: EdgeId, shell: Option<ShellId>) {
        let idx = id.index() as usize;
        if idx >= self.edge_shells.len() {
            self.edge_shells.resize(idx + 1, None);
        }
        self.edge_shells[idx] = shell;
    }

    /// The representative entry edge for this wire shell, if any.
    pub fn shell_entry_edge(&self, id: ShellId) -> Option<EdgeId> {
        self.shell_entry_edges
            .get(id.index() as usize)
            .and_then(|opt| *opt)
    }

    /// Set or clear the representative entry edge for this wire shell.
    pub fn set_shell_entry_edge(&mut self, id: ShellId, edge: Option<EdgeId>) {
        let idx = id.index() as usize;
        if idx >= self.shell_entry_edges.len() {
            self.shell_entry_edges.resize(idx + 1, None);
        }
        self.shell_entry_edges[idx] = edge;
    }

    // ── 3-plane intersection provenance (Vertex side-car) ───────────

    /// The 3-plane intersection provenance for this vertex (sorted plane indices).
    pub fn vertex_provenance(&self, id: VertexId) -> Option<&[usize; 3]> {
        self.vertex_provenance
            .get(id.index() as usize)
            .and_then(|opt| opt.as_ref())
    }

    /// Set or clear the 3-plane intersection provenance for this vertex.
    pub fn set_vertex_provenance(&mut self, id: VertexId, provenance: Option<[usize; 3]>) {
        let idx = id.index() as usize;
        if idx >= self.vertex_provenance.len() {
            self.vertex_provenance.resize(idx + 1, None);
        }
        self.vertex_provenance[idx] = provenance;
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
        if let Some(extras) = self.nmt_extra_disks.get(&v) {
            entries.extend_from_slice(extras);
        }
        Ok(entries)
    }

    /// Number of disk entries at this vertex.
    pub fn disk_count(&self, v: VertexId) -> usize {
        1 + self.nmt_extra_disks.get(&v).map_or(0, |entries| entries.len())
    }

    /// Whether the vertex currently has extra NMT disk entries.
    pub fn is_vertex_nmt(&self, v: VertexId) -> bool {
        self.vertex_is_nmt
            .get(v.index() as usize)
            .copied()
            .unwrap_or(false)
    }

    /// Append an extra disk entry, marking this vertex as NMT.
    pub fn add_disk_entry(&mut self, v: VertexId, he: HalfEdgeId) {
        self.nmt_extra_disks.entry(v).or_default().push(he);
        let idx = v.index() as usize;
        if idx >= self.vertex_is_nmt.len() {
            self.vertex_is_nmt.resize(idx + 1, false);
        }
        self.vertex_is_nmt[idx] = true;
    }

    /// Remove an entry from the extra NMT disk list. Returns false if absent.
    pub fn remove_disk_entry(&mut self, v: VertexId, he: HalfEdgeId) -> bool {
        let Some(extras) = self.nmt_extra_disks.get_mut(&v) else {
            return false;
        };
        let Some(pos) = extras.iter().position(|&entry| entry == he) else {
            return false;
        };
        extras.swap_remove(pos);
        if extras.is_empty() {
            self.nmt_extra_disks.remove(&v);
            if let Some(flag) = self.vertex_is_nmt.get_mut(v.index() as usize) {
                *flag = false;
            }
        }
        true
    }

    /// Replace an extra NMT disk entry value. Returns false if old entry is absent.
    pub fn replace_disk_entry(&mut self, v: VertexId, old: HalfEdgeId, new: HalfEdgeId) -> bool {
        let Some(extras) = self.nmt_extra_disks.get_mut(&v) else {
            return false;
        };
        let Some(pos) = extras.iter().position(|&entry| entry == old) else {
            return false;
        };
        extras[pos] = new;
        true
    }

    /// Set the primary disk entry.
    pub fn set_primary_disk_entry(&mut self, v: VertexId, he: HalfEdgeId) -> Result<(), KernelError> {
        self.get_vertex_mut(v)?.set_primary_disk(he);
        Ok(())
    }

    // ── Lockstep growth helpers ─────────────────────────────────────

    /// Ensure the halfedge side-car vectors are at least `len` long.
    pub(crate) fn grow_halfedge_sidecars(&mut self, len: usize) {
        if self.bridge_flags.len() < len {
            self.bridge_flags.resize(len, false);
        }
        if self.coedge_data.len() < len {
            self.coedge_data.resize(len, None);
        }
    }

    /// Clear half-edge side-car data at the given slot index.
    pub(crate) fn clear_halfedge_sidecar(&mut self, index: usize) {
        if index < self.bridge_flags.len() {
            self.bridge_flags[index] = false;
        }
        if index < self.coedge_data.len() {
            self.coedge_data[index] = None;
        }
    }

    /// Ensure the edge side-car vectors are at least `len` long.
    pub(crate) fn grow_edge_sidecars(&mut self, len: usize) {
        if self.edge_curves.len() < len {
            self.edge_curves.resize(len, None);
        }
        if self.edge_shells.len() < len {
            self.edge_shells.resize(len, None);
        }
    }

    /// Clear edge side-car data at the given slot index.
    pub(crate) fn clear_edge_sidecar(&mut self, index: usize) {
        if index < self.edge_curves.len() {
            self.edge_curves[index] = None;
        }
        if index < self.edge_shells.len() {
            self.edge_shells[index] = None;
        }
    }

    /// Ensure the vertex side-car vectors are at least `len` long.
    pub(crate) fn grow_vertex_sidecars(&mut self, len: usize) {
        if self.vertex_provenance.len() < len {
            self.vertex_provenance.resize(len, None);
        }
        if self.vertex_is_nmt.len() < len {
            self.vertex_is_nmt.resize(len, false);
        }
    }

    /// Clear vertex side-car data at the given slot index.
    pub(crate) fn clear_vertex_sidecar(&mut self, index: usize) {
        if index < self.vertex_provenance.len() {
            self.vertex_provenance[index] = None;
        }
        if index < self.vertex_is_nmt.len() {
            self.vertex_is_nmt[index] = false;
        }
    }
}
