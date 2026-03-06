//! O(1) reverse indexes for parent→child entity lookups.
//!
//! DOMAIN: Maintains IndexMap indexes that map container entities
//! to their contained entities. These are derived data, rebuilt
//! from entity fields on deserialization.
//!
//! INVARIANTS:
//! - Indexes are always in sync with entity data fields
//! - Indexes are `#[serde(skip)]` and rebuilt via `rebuild_indexes()`

use crate::b_rep::data::storage::arena::TopologyArena;
use crate::b_rep::data::storage::cache_runtime::{TopoCacheDomain, TopoCacheEffect};
use crate::handles::{FaceId, HalfEdgeId, ShellId, VertexId};
use forge_core::KernelError;
use smallvec::smallvec;

impl TopologyArena {
    /// Faces belonging to a given shell. Returns empty slice if shell unknown.
    pub fn faces_of_shell(&self, shell: ShellId) -> &[FaceId] {
        self.indexes
            .shell_faces
            .get(&shell)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Halfedges belonging to a given face. Returns empty slice if face unknown.
    pub fn halfedges_of_face(&self, face: FaceId) -> &[HalfEdgeId] {
        self.indexes
            .face_halfedges
            .get(&face)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Halfedges originating from a given vertex. Returns empty slice if vertex unknown.
    pub fn halfedges_from_vertex(&self, vertex: VertexId) -> &[HalfEdgeId] {
        self.indexes
            .vertex_halfedges
            .get(&vertex)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Faces of one shell after forcing Tier-0 shell-face cache freshness.
    pub fn faces_of_shell_fresh(&mut self, shell: ShellId) -> Result<&[FaceId], KernelError> {
        self.ensure_cache_domain_fresh(TopoCacheDomain::ShellFaces)?;
        Ok(self.faces_of_shell(shell))
    }

    /// Halfedges of one face after forcing Tier-0 face-halfedge cache freshness.
    pub fn halfedges_of_face_fresh(&mut self, face: FaceId) -> Result<&[HalfEdgeId], KernelError> {
        self.ensure_cache_domain_fresh(TopoCacheDomain::FaceHalfedges)?;
        Ok(self.halfedges_of_face(face))
    }

    /// Halfedges of one vertex after forcing Tier-0 vertex-halfedge cache freshness.
    pub fn halfedges_from_vertex_fresh(
        &mut self,
        vertex: VertexId,
    ) -> Result<&[HalfEdgeId], KernelError> {
        self.ensure_cache_domain_fresh(TopoCacheDomain::VertexHalfedges)?;
        Ok(self.halfedges_from_vertex(vertex))
    }

    /// Register a face in the shell→faces index (called from insert_face).
    pub(crate) fn index_add_face(&mut self, face: FaceId, shell: ShellId) {
        self.indexes
            .shell_faces
            .entry(shell)
            .or_default()
            .push(face);
        self.mark_cache_effect(TopoCacheEffect::ShellFacesChanged {
            shells: smallvec![shell],
        });
    }

    /// Remove a face from the shell→faces index (called from remove_face).
    pub(crate) fn index_remove_face(&mut self, face: FaceId, shell: ShellId) {
        if let Some(faces) = self.indexes.shell_faces.get_mut(&shell) {
            if let Some(pos) = faces.iter().position(|&f| f == face) {
                faces.swap_remove(pos);
            }
        }
        self.mark_cache_effect(TopoCacheEffect::ShellFacesChanged {
            shells: smallvec![shell],
        });
    }

    /// Register a halfedge in both face→halfedges and vertex→halfedges indexes.
    pub(crate) fn index_add_halfedge(&mut self, he: HalfEdgeId, face: FaceId, origin: VertexId) {
        self.indexes
            .face_halfedges
            .entry(face)
            .or_default()
            .push(he);
        self.indexes
            .vertex_halfedges
            .entry(origin)
            .or_default()
            .push(he);
        self.mark_cache_effect(TopoCacheEffect::FaceHalfedgesChanged {
            faces: smallvec![face],
        });
        self.mark_cache_effect(TopoCacheEffect::VertexHalfedgesChanged {
            vertices: smallvec![origin],
        });
    }

    /// Remove a halfedge from both face→halfedges and vertex→halfedges indexes.
    pub(crate) fn index_remove_halfedge(&mut self, he: HalfEdgeId, face: FaceId, origin: VertexId) {
        if let Some(hes) = self.indexes.face_halfedges.get_mut(&face) {
            if let Some(pos) = hes.iter().position(|&h| h == he) {
                hes.swap_remove(pos);
            }
        }
        if let Some(hes) = self.indexes.vertex_halfedges.get_mut(&origin) {
            if let Some(pos) = hes.iter().position(|&h| h == he) {
                hes.swap_remove(pos);
            }
        }
        self.mark_cache_effect(TopoCacheEffect::FaceHalfedgesChanged {
            faces: smallvec![face],
        });
        self.mark_cache_effect(TopoCacheEffect::VertexHalfedgesChanged {
            vertices: smallvec![origin],
        });
    }

    /// Rebuild all indexes from entity data. Called after deserialization.
    ///
    /// Collects only the needed fields (not full entity clones) to avoid
    /// unnecessary allocation at scale.
    pub fn rebuild_indexes(&mut self) {
        self.indexes.shell_faces.clear();
        self.indexes.face_halfedges.clear();
        self.indexes.vertex_halfedges.clear();

        // Collect (face_id, shell) — no FaceData clone
        let face_shells: Vec<_> = self
            .connectivity
            .face_slots
            .iter()
            .enumerate()
            .filter_map(|(i, slot)| {
                let data = slot.data.as_ref()?;
                Some((FaceId::new(i as u32, slot.generation), data.shell()))
            })
            .collect();

        for (face_id, shell) in face_shells {
            self.indexes
                .shell_faces
                .entry(shell)
                .or_default()
                .push(face_id);
        }

        // Collect (he_id, face, origin) — no HalfEdgeData clone
        let he_refs: Vec<_> = self
            .connectivity
            .half_edge_slots
            .iter()
            .enumerate()
            .filter_map(|(i, slot)| {
                let data = slot.data.as_ref()?;
                Some((
                    HalfEdgeId::new(i as u32, slot.generation),
                    data.face(),
                    data.origin(),
                ))
            })
            .collect();

        for (he_id, face, origin) in he_refs {
            self.indexes
                .face_halfedges
                .entry(face)
                .or_default()
                .push(he_id);
            self.indexes
                .vertex_halfedges
                .entry(origin)
                .or_default()
                .push(he_id);
        }
    }

    /// Rebuild the shell->faces index for one shell from ground-truth face slots.
    pub(crate) fn rebuild_shell_faces_for_shell(
        &mut self,
        shell: ShellId,
    ) -> Result<(), KernelError> {
        let mut faces = self
            .iter_faces()
            .filter_map(|(face_id, face)| (face.shell() == shell).then_some(face_id))
            .collect::<Vec<_>>();
        faces.sort_unstable();

        if faces.is_empty() {
            self.indexes.shell_faces.swap_remove(&shell);
        } else {
            self.indexes.shell_faces.insert(shell, faces.into());
        }
        Ok(())
    }

    /// Rebuild the face->halfedges index for one face from ground-truth halfedge slots.
    pub(crate) fn rebuild_face_halfedges_for_face(
        &mut self,
        face: FaceId,
    ) -> Result<(), KernelError> {
        let mut halfedges = self
            .iter_half_edges()
            .filter_map(|(he_id, he)| (he.face() == face).then_some(he_id))
            .collect::<Vec<_>>();
        halfedges.sort_unstable();

        if halfedges.is_empty() {
            self.indexes.face_halfedges.swap_remove(&face);
        } else {
            self.indexes.face_halfedges.insert(face, halfedges.into());
        }
        Ok(())
    }

    /// Rebuild the vertex->halfedges index for one vertex from ground-truth halfedge slots.
    pub(crate) fn rebuild_vertex_halfedges_for_vertex(
        &mut self,
        vertex: VertexId,
    ) -> Result<(), KernelError> {
        let mut halfedges = self
            .iter_half_edges()
            .filter_map(|(he_id, he)| (he.origin() == vertex).then_some(he_id))
            .collect::<Vec<_>>();
        halfedges.sort_unstable();

        if halfedges.is_empty() {
            self.indexes.vertex_halfedges.swap_remove(&vertex);
        } else {
            self.indexes
                .vertex_halfedges
                .insert(vertex, halfedges.into());
        }
        Ok(())
    }

    /// Rebuild all shell->faces entries from ground-truth face slots.
    pub(crate) fn rebuild_shell_face_index(&mut self) -> Result<(), KernelError> {
        self.indexes.shell_faces.clear();
        let mut by_shell = std::collections::BTreeMap::<ShellId, Vec<FaceId>>::new();
        for (face_id, face) in self.iter_faces() {
            by_shell.entry(face.shell()).or_default().push(face_id);
        }
        for (shell, mut faces) in by_shell {
            faces.sort_unstable();
            self.indexes.shell_faces.insert(shell, faces.into());
        }
        Ok(())
    }

    /// Rebuild all face->halfedges entries from ground-truth halfedge slots.
    pub(crate) fn rebuild_face_halfedge_index(&mut self) -> Result<(), KernelError> {
        self.indexes.face_halfedges.clear();
        let mut by_face = std::collections::BTreeMap::<FaceId, Vec<HalfEdgeId>>::new();
        for (he_id, he) in self.iter_half_edges() {
            by_face.entry(he.face()).or_default().push(he_id);
        }
        for (face, mut halfedges) in by_face {
            halfedges.sort_unstable();
            self.indexes.face_halfedges.insert(face, halfedges.into());
        }
        Ok(())
    }

    /// Rebuild all vertex->halfedges entries from ground-truth halfedge slots.
    pub(crate) fn rebuild_vertex_halfedge_index(&mut self) -> Result<(), KernelError> {
        self.indexes.vertex_halfedges.clear();
        let mut by_vertex = std::collections::BTreeMap::<VertexId, Vec<HalfEdgeId>>::new();
        for (he_id, he) in self.iter_half_edges() {
            by_vertex.entry(he.origin()).or_default().push(he_id);
        }
        for (vertex, mut halfedges) in by_vertex {
            halfedges.sort_unstable();
            self.indexes
                .vertex_halfedges
                .insert(vertex, halfedges.into());
        }
        Ok(())
    }

    pub(crate) fn remove_shell_face_index_entry(&mut self, shell: ShellId) {
        self.indexes.shell_faces.swap_remove(&shell);
    }

    pub(crate) fn remove_face_halfedge_index_entry(&mut self, face: FaceId) {
        self.indexes.face_halfedges.swap_remove(&face);
    }

    pub(crate) fn remove_vertex_halfedge_index_entry(&mut self, vertex: VertexId) {
        self.indexes.vertex_halfedges.swap_remove(&vertex);
    }
}
