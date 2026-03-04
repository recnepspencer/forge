//! Body lifecycle operations — Clone, Split, Merge, Detach, Attach.
//!
//! DOMAIN: Higher-level operations on Body (solid) entities that
//! compose lower-level Lump and Shell operations.
//!
//! DEPENDENCIES: `arena` (entity storage)

use forge_core::KernelError;

use crate::b_rep::{BodyData, LumpData, RegionData};
use crate::handles::{BodyId, LumpId, RegionId, ShellId};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::operator::TopoOperator;
use crate::validators::invariant_id::InvariantContract;

// ── SplitBody ───────────────────────────────────────────────────────

/// Split a body by moving a subset of its lumps into a new body.
#[derive(Debug)]
pub struct SplitBody {
    /// The body to split.
    pub body: BodyId,
    /// The lumps to move to the new body.
    pub lumps_to_move: Vec<LumpId>,
}

/// Output of the SplitBody operator.
pub struct SplitBodyOutput {
    /// The new body containing the moved lumps.
    pub new_body: BodyId,
}

impl TopoOperator for SplitBody {
    type Output = SplitBodyOutput;

    const NAME: &'static str = "split_body";

    const INVARIANT_CONTRACT: InvariantContract = crate::conservative_contract!();

    fn semantic_summary(&self) -> String {
        format!(
            "Split body {} by moving {} lumps to new body",
            self.body.index(), self.lumps_to_move.len()
        )
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        if self.lumps_to_move.is_empty() {
            return Err(KernelError::InvalidInput {
                message: "SplitBody: must move at least one lump".to_string(),
                context: None,
            });
        }

        let existing_lumps: Vec<LumpId> = draft.arena().get_body(self.body)?.lumps().to_vec();
        if self.lumps_to_move.len() >= existing_lumps.len() {
            return Err(KernelError::InvalidInput {
                message: "SplitBody: cannot move all lumps — original body would be empty".to_string(),
                context: None,
            });
        }

        for &lump in &self.lumps_to_move {
            if !existing_lumps.contains(&lump) {
                return Err(KernelError::InvalidInput {
                    message: format!(
                        "SplitBody: lump {} does not belong to body {}",
                        lump.index(), self.body.index()
                    ),
                    context: None,
                });
            }
        }

        let new_body = draft.insert_body(BodyData::new());

        for &lump in &self.lumps_to_move {
            draft.arena_mut().get_body_mut(self.body)?.remove_lump(lump);
            draft.arena_mut().get_lump_mut(lump)?.set_body(new_body);
            draft.arena_mut().get_body_mut(new_body)?.add_lump(lump);
        }

        Ok(ExecutionResult {
            value: SplitBodyOutput { new_body },
            declared_delta: EulerDelta {
                solids: 1,
                ..EulerDelta::default()
            },
        })
    }
}

// ── MergeBodies ─────────────────────────────────────────────────────

/// Merge all lumps from source body into target body, then destroy source.
#[derive(Debug)]
pub struct MergeBodies {
    /// The target body (survives).
    pub target: BodyId,
    /// The source body (destroyed after merge).
    pub source: BodyId,
}

impl TopoOperator for MergeBodies {
    type Output = ();

    const NAME: &'static str = "merge_bodies";

    const INVARIANT_CONTRACT: InvariantContract = crate::conservative_contract!();

    fn semantic_summary(&self) -> String {
        format!("Merge body {} into body {}", self.source.index(), self.target.index())
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        if self.target == self.source {
            return Err(KernelError::InvalidInput {
                message: "MergeBodies: cannot merge a body with itself".to_string(),
                context: None,
            });
        }

        let source_lumps: Vec<LumpId> = draft.arena().get_body(self.source)?.lumps().to_vec();

        for &lump in &source_lumps {
            draft.arena_mut().get_body_mut(self.source)?.remove_lump(lump);
            draft.arena_mut().get_lump_mut(lump)?.set_body(self.target);
            draft.arena_mut().get_body_mut(self.target)?.add_lump(lump);
        }

        draft.remove_body(self.source)?;

        Ok(ExecutionResult {
            value: (),
            declared_delta: EulerDelta {
                solids: -1,
                ..EulerDelta::default()
            },
        })
    }
}

// ── CloneBody ───────────────────────────────────────────────────────

/// Deep-clone a body and all its child entities.
#[derive(Debug)]
pub struct CloneBody {
    /// The body to clone.
    pub body: BodyId,
}

/// Output of the CloneBody operator.
pub struct CloneBodyOutput {
    /// The cloned body.
    pub cloned_body: BodyId,
}

impl TopoOperator for CloneBody {
    type Output = CloneBodyOutput;

    const NAME: &'static str = "clone_body";

    const INVARIANT_CONTRACT: InvariantContract = crate::conservative_contract!();

    fn semantic_summary(&self) -> String {
        format!("Deep clone body {}", self.body.index())
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        use std::collections::BTreeMap;

        // ── Phase 1: Collect ALL source data (immutable borrows) ────
        let lumps: Vec<LumpId> = draft.arena().get_body(self.body)?.lumps().to_vec();

        struct FaceInfo {
            old_face: crate::handles::FaceId,
            old_outer_loop: crate::handles::LoopId,
            old_inner_loops: Vec<crate::handles::LoopId>,
            old_shell: ShellId,
        }

        struct HeInfo {
            old_he: crate::handles::HalfEdgeId,
            old_next: crate::handles::HalfEdgeId,
            old_prev: crate::handles::HalfEdgeId,
            old_radial: crate::handles::HalfEdgeId,
            old_face: crate::handles::FaceId,
            old_origin: crate::handles::VertexId,
            old_edge: crate::handles::EdgeId,
        }

        struct VertexInfo {
            old_vertex: crate::handles::VertexId,
            old_outgoing: crate::handles::HalfEdgeId,
        }

        struct EdgeInfo {
            old_edge: crate::handles::EdgeId,
            old_he: crate::handles::HalfEdgeId,
        }

        struct ShellInfo {
            old_shell: ShellId,
            kind: crate::b_rep::ShellKind,
            faces: Vec<FaceInfo>,
            halfedges: Vec<HeInfo>,
            vertices: Vec<VertexInfo>,
            edges: Vec<EdgeInfo>,
        }

        struct LoopInfo {
            old_loop: crate::handles::LoopId,
            old_he: crate::handles::HalfEdgeId,
        }

        struct RegionInfo {
            old_region: RegionId,
            outer_shell: Option<ShellId>,
            shells: Vec<ShellInfo>,
        }

        struct LumpInfo {
            regions: Vec<RegionInfo>,
        }

        let mut lump_infos: Vec<LumpInfo> = Vec::new();
        let mut all_loops: Vec<LoopInfo> = Vec::new();

        for &old_lump in &lumps {
            let regions: Vec<RegionId> = draft.arena().get_lump(old_lump)?.regions().to_vec();
            let mut region_infos = Vec::new();

            for &old_region in &regions {
                let region_data = draft.arena().get_region(old_region)?;
                let outer_shell = region_data.outer_shell();
                let mut all_shells_list = Vec::new();
                if let Some(os) = outer_shell {
                    all_shells_list.push(os);
                }
                all_shells_list.extend_from_slice(region_data.inner_shells());

                let mut shell_infos = Vec::new();
                let mut seen_vertices = std::collections::BTreeSet::new();
                let mut seen_edges = std::collections::BTreeSet::new();

                for &old_shell in &all_shells_list {
                    let kind = draft.arena().get_shell(old_shell)?.kind();

                    let shell_faces: Vec<crate::handles::FaceId> =
                        draft.arena().faces_of_shell(old_shell).to_vec();

                    let mut face_infos = Vec::new();
                    for &old_face in &shell_faces {
                        let fd = draft.arena().get_face(old_face)?;
                        let old_outer_loop = fd.outer_loop();
                        let old_inner_loops = fd.inner_loops().to_vec();

                        // Collect loop info
                        let loop_he = draft.arena().get_loop(old_outer_loop)?.half_edge();
                        all_loops.push(LoopInfo { old_loop: old_outer_loop, old_he: loop_he });

                        for &il in &old_inner_loops {
                            let ilhe = draft.arena().get_loop(il)?.half_edge();
                            all_loops.push(LoopInfo { old_loop: il, old_he: ilhe });
                        }

                        face_infos.push(FaceInfo {
                            old_face,
                            old_outer_loop,
                            old_inner_loops,
                            old_shell,
                        });
                    }

                    // Collect halfedges for this shell via face→halfedges index
                    let mut shell_he_ids = Vec::new();
                    for &face in &shell_faces {
                        shell_he_ids.extend_from_slice(draft.arena().halfedges_of_face(face));
                    }

                    let mut he_infos = Vec::new();
                    let mut vertex_infos = Vec::new();
                    let mut edge_infos = Vec::new();

                    for &old_he in &shell_he_ids {
                        let hd = draft.arena().get_half_edge(old_he)?;
                        he_infos.push(HeInfo {
                            old_he,
                            old_next: hd.next(),
                            old_prev: hd.prev(),
                            old_radial: hd.radial_next(),
                            old_face: hd.face(),
                            old_origin: hd.origin(),
                            old_edge: hd.edge(),
                        });

                        if !seen_vertices.contains(&hd.origin()) {
                            let vd = draft.arena().get_vertex(hd.origin())?;
                            vertex_infos.push(VertexInfo {
                                old_vertex: hd.origin(),
                                old_outgoing: vd.outgoing(),
                            });
                            seen_vertices.insert(hd.origin());
                        }

                        if !seen_edges.contains(&hd.edge()) {
                            let ed = draft.arena().get_edge(hd.edge())?;
                            edge_infos.push(EdgeInfo {
                                old_edge: hd.edge(),
                                old_he: ed.half_edge(),
                            });
                            seen_edges.insert(hd.edge());
                        }
                    }

                    shell_infos.push(ShellInfo {
                        old_shell,
                        kind,
                        faces: face_infos,
                        halfedges: he_infos,
                        vertices: vertex_infos,
                        edges: edge_infos,
                    });
                }

                region_infos.push(RegionInfo {
                    old_region,
                    outer_shell,
                    shells: shell_infos,
                });
            }

            lump_infos.push(LumpInfo { regions: region_infos });
        }

        // ── Phase 2: Create all new entities (mutable borrows) ──────
        let new_body = draft.insert_body(BodyData::new());

        let mut vertex_map: BTreeMap<crate::handles::VertexId, crate::handles::VertexId> = BTreeMap::new();
        let mut edge_map: BTreeMap<crate::handles::EdgeId, crate::handles::EdgeId> = BTreeMap::new();
        let mut half_edge_map: BTreeMap<crate::handles::HalfEdgeId, crate::handles::HalfEdgeId> = BTreeMap::new();
        let mut face_map: BTreeMap<crate::handles::FaceId, crate::handles::FaceId> = BTreeMap::new();
        let mut loop_map: BTreeMap<crate::handles::LoopId, crate::handles::LoopId> = BTreeMap::new();
        let mut shell_map: BTreeMap<ShellId, ShellId> = BTreeMap::new();

        let mut total_shells = 0i32;
        let mut total_faces = 0i32;
        let mut total_loops = 0i32;
        let mut total_half_edges = 0i32;
        let mut total_edges = 0i32;
        let mut total_vertices = 0i32;

        for lump_info in &lump_infos {
            let new_lump = draft.insert_lump(LumpData::new(new_body));
            draft.arena_mut().get_body_mut(new_body)?.add_lump(new_lump);

            for region_info in &lump_info.regions {
                let new_region = draft.insert_region(RegionData::new(new_lump));
                draft.arena_mut().get_lump_mut(new_lump)?.add_region(new_region);

                for shell_info in &region_info.shells {
                    let new_shell = draft.insert_shell(crate::b_rep::ShellData::new(
                        crate::handles::FaceId::DANGLING,
                        shell_info.kind,
                        new_region,
                    ));
                    shell_map.insert(shell_info.old_shell, new_shell);
                    draft.arena_mut().get_region_mut(new_region)?.add_shell(new_shell);
                    total_shells += 1;

                    // Create faces and loops
                    for fi in &shell_info.faces {
                        let new_loop = draft.insert_loop(crate::b_rep::LoopData::new(
                            crate::handles::HalfEdgeId::DANGLING,
                            crate::handles::FaceId::DANGLING,
                        ));
                        loop_map.insert(fi.old_outer_loop, new_loop);
                        total_loops += 1;

                        let new_face = draft.insert_face(crate::b_rep::FaceData::new(
                            new_loop,
                            new_shell,
                        ));
                        face_map.insert(fi.old_face, new_face);
                        total_faces += 1;

                        for &old_inner in &fi.old_inner_loops {
                            let new_inner = draft.insert_loop(crate::b_rep::LoopData::new(
                                crate::handles::HalfEdgeId::DANGLING,
                                crate::handles::FaceId::DANGLING,
                            ));
                            loop_map.insert(old_inner, new_inner);
                            draft.arena_mut().get_face_mut(new_face)?.add_inner_loop(new_inner);
                            total_loops += 1;
                        }
                    }

                    // Create vertices
                    for vi in &shell_info.vertices {
                        if !vertex_map.contains_key(&vi.old_vertex) {
                            let new_v = draft.insert_vertex(crate::b_rep::VertexData::new(
                                crate::handles::HalfEdgeId::DANGLING,
                            ));
                            vertex_map.insert(vi.old_vertex, new_v);
                            total_vertices += 1;
                        }
                    }

                    // Create edges
                    for ei in &shell_info.edges {
                        if !edge_map.contains_key(&ei.old_edge) {
                            let new_e = draft.insert_edge(crate::b_rep::EdgeData::new(
                                crate::handles::HalfEdgeId::DANGLING,
                            ));
                            edge_map.insert(ei.old_edge, new_e);
                            total_edges += 1;
                        }
                    }

                    // Create halfedges
                    for hi in &shell_info.halfedges {
                        let new_face = face_map[&hi.old_face];
                        let new_origin = vertex_map[&hi.old_origin];
                        let new_edge = edge_map[&hi.old_edge];

                        let new_he = draft.insert_half_edge(crate::b_rep::HalfEdgeData::new(
                            crate::handles::HalfEdgeId::DANGLING,
                            crate::handles::HalfEdgeId::DANGLING,
                            crate::handles::HalfEdgeId::DANGLING,
                            new_face,
                            new_origin,
                            new_edge,
                        ));
                        half_edge_map.insert(hi.old_he, new_he);
                        total_half_edges += 1;
                    }

                    // Wire halfedge next/prev/radial_next
                    for hi in &shell_info.halfedges {
                        let new_he = half_edge_map[&hi.old_he];
                        if let Some(&new_next) = half_edge_map.get(&hi.old_next) {
                            draft.arena_mut().get_half_edge_mut(new_he)?.set_next(new_next);
                        }
                        if let Some(&new_prev) = half_edge_map.get(&hi.old_prev) {
                            draft.arena_mut().get_half_edge_mut(new_he)?.set_prev(new_prev);
                        }
                        if let Some(&new_radial) = half_edge_map.get(&hi.old_radial) {
                            draft.arena_mut().get_half_edge_mut(new_he)?.set_radial_next(new_radial);
                        } else {
                            draft.arena_mut().get_half_edge_mut(new_he)?.set_radial_next(new_he);
                        }
                    }

                    // Wire vertex → outgoing
                    for vi in &shell_info.vertices {
                        if let Some(&new_v) = vertex_map.get(&vi.old_vertex) {
                            if let Some(&new_out) = half_edge_map.get(&vi.old_outgoing) {
                                draft.arena_mut().get_vertex_mut(new_v)?.set_outgoing(new_out);
                            }
                        }
                    }

                    // Wire edge → halfedge
                    for ei in &shell_info.edges {
                        if let Some(&new_e) = edge_map.get(&ei.old_edge) {
                            if let Some(&new_he) = half_edge_map.get(&ei.old_he) {
                                draft.arena_mut().get_edge_mut(new_e)?.set_half_edge(new_he);
                            }
                        }
                    }

                    // Wire shell → representative face
                    if let Some(fi) = shell_info.faces.first() {
                        let new_face = face_map[&fi.old_face];
                        draft.arena_mut().get_shell_mut(new_shell)?.set_representative_face(new_face);
                    }
                }

                // Wire loops
                for li in &all_loops {
                    if let Some(&new_loop) = loop_map.get(&li.old_loop) {
                        if let Some(&new_he) = half_edge_map.get(&li.old_he) {
                            draft.arena_mut().get_loop_mut(new_loop)?.set_half_edge(new_he);
                        }
                    }
                }

                // Wire region outer shell
                if let Some(os) = region_info.outer_shell {
                    if let Some(&ns) = shell_map.get(&os) {
                        draft.arena_mut().get_region_mut(new_region)?.set_outer_shell(ns);
                    }
                }
            }
        }

        let total_regions = lump_infos.iter().map(|l| l.regions.len() as i32).sum::<i32>();

        Ok(ExecutionResult {
            value: CloneBodyOutput { cloned_body: new_body },
            declared_delta: EulerDelta {
                vertices: total_vertices,
                half_edges: total_half_edges,
                faces: total_faces,
                loops: total_loops,
                edges: total_edges,
                shells: total_shells,
                solids: 1,
                lumps: lumps.len() as i32,
                regions: total_regions,
            },
        })
    }
}
