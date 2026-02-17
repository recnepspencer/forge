//! Boolean result assembly via topology copy.
//!
//! Selects faces from both solids based on their classification and
//! the operation type, then copies them into a new topology arena
//! with handle remapping.

use std::collections::HashMap;

use forge_core::KernelError;
use forge_topo::arena::{FaceData, HalfEdgeData, VertexData, LoopData};
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId, LoopId};
use forge_topo::state::{TopologyState, MutableDraft};

use crate::geometry_store::GeometryStore;
use super::schema::{
    BooleanInput, BooleanOp, BooleanResult,
    FaceClassification, FaceOrigin, ClassifiedFace,
};
use super::classify::classify_faces;
use super::split::split_all_faces;

/// Execute a Boolean operation on two solids.
///
/// Pipeline:
/// 1. Split faces of both solids along mutual intersections
/// 2. Classify faces of target relative to tool
/// 3. Classify faces of tool relative to target
/// 4. Select faces based on operation type
/// 5. Copy selected face topology into a new arena with remapped handles
pub fn execute_boolean(input: BooleanInput) -> Result<BooleanResult, KernelError> {
    let (target_topo, target_geom, tool_topo, tool_geom, operation) = input.into_parts();

    let split_result = split_all_faces(target_topo, target_geom, tool_topo, tool_geom)?;
    let (target_topo, target_geom, tool_topo, tool_geom) = split_result.into_parts();

    let target_classified = classify_faces(
        target_topo.arena(),
        &target_geom,
        tool_topo.arena(),
        &tool_geom,
        FaceOrigin::Target,
    )?;

    let tool_classified = classify_faces(
        tool_topo.arena(),
        &tool_geom,
        target_topo.arena(),
        &target_geom,
        FaceOrigin::Tool,
    )?;

    let selected_target = select_faces(&target_classified, FaceOrigin::Target, operation);
    let selected_tool = select_faces(&tool_classified, FaceOrigin::Tool, operation);

    let target_face_count = selected_target.len();
    let tool_face_count = selected_tool.len();

    let (result_topo, result_geom) = copy_selected_faces(
        target_topo.arena(),
        &target_geom,
        &selected_target,
        tool_topo.arena(),
        &tool_geom,
        &selected_tool,
    )?;

    Ok(BooleanResult::new(
        result_topo,
        result_geom,
        target_face_count,
        tool_face_count,
    ))
}

/// Select faces to keep based on the Boolean operation type.
///
/// | Operation     | Keep from Target         | Keep from Tool          |
/// |---------------|--------------------------|-------------------------|
/// | Union         | Outside + OnBoundary     | Outside                 |
/// | Intersection  | Inside + OnBoundary      | Inside                  |
/// | Subtraction   | Outside + OnBoundary     | Inside                  |
///
/// OnBoundary faces are coplanar shared faces. We keep them from the
/// target only to avoid duplicating shared boundary geometry.
fn select_faces(
    classified: &[ClassifiedFace],
    origin: FaceOrigin,
    operation: BooleanOp,
) -> Vec<FaceId> {
    let keep_primary = match (origin, operation) {
        (FaceOrigin::Target, BooleanOp::Union) => FaceClassification::Outside,
        (FaceOrigin::Target, BooleanOp::Intersection) => FaceClassification::Inside,
        (FaceOrigin::Target, BooleanOp::Subtraction) => FaceClassification::Outside,
        (FaceOrigin::Tool, BooleanOp::Union) => FaceClassification::Outside,
        (FaceOrigin::Tool, BooleanOp::Intersection) => FaceClassification::Inside,
        (FaceOrigin::Tool, BooleanOp::Subtraction) => FaceClassification::Inside,
    };

    let keep_boundary = matches!(origin, FaceOrigin::Target);

    classified
        .iter()
        .filter(|f| {
            f.classification() == keep_primary
                || (keep_boundary && f.classification() == FaceClassification::OnBoundary)
        })
        .map(|f| f.face())
        .collect()
}

/// Tracks the mapping from old handles (in a source arena) to new handles
/// (in the destination arena). Each source arena gets its own remapper
/// to avoid index collisions.
struct HandleRemapper {
    /// Old VertexId → New VertexId.
    vertices: HashMap<u64, VertexId>,
    /// Old HalfEdgeId → New HalfEdgeId.
    half_edges: HashMap<u64, HalfEdgeId>,
    /// Old FaceId → New FaceId.
    faces: HashMap<u64, FaceId>,
    /// Old LoopId → New LoopId.
    loops: HashMap<u64, LoopId>,
}

impl HandleRemapper {
    /// Create an empty remapper.
    fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            half_edges: HashMap::new(),
            faces: HashMap::new(),
            loops: HashMap::new(),
        }
    }
}

/// Pack a handle's (index, generation) into a u64 key for HashMap lookup.
fn pack(index: u32, generation: u32) -> u64 {
    (u64::from(generation) << 32) | u64::from(index)
}

/// Copy selected faces from both source arenas into a new topology.
///
/// Each source arena gets its own `HandleRemapper` to avoid index
/// collisions (both arenas start from index 0). The Euler formula
/// check expects χ = 2*S where S = number of shells.
fn copy_selected_faces(
    target_arena: &forge_topo::arena::TopologyArena,
    target_geom: &GeometryStore,
    target_faces: &[FaceId],
    tool_arena: &forge_topo::arena::TopologyArena,
    tool_geom: &GeometryStore,
    tool_faces: &[FaceId],
) -> Result<(TopologyState, GeometryStore), KernelError> {
    if target_faces.is_empty() && tool_faces.is_empty() {
        return Err(KernelError::InvalidInput {
            message: "Boolean operation produced no faces".to_string(),
            context: None,
        });
    }

    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();
    let mut result_geom = GeometryStore::new();

    let mut target_remapper = HandleRemapper::new();
    copy_faces_from_arena(
        &mut draft, &mut result_geom, &mut target_remapper,
        target_arena, target_geom, target_faces,
    )?;

    let mut tool_remapper = HandleRemapper::new();
    copy_faces_from_arena(
        &mut draft, &mut result_geom, &mut tool_remapper,
        tool_arena, tool_geom, tool_faces,
    )?;

    let topo = draft.commit()?;
    Ok((topo, result_geom))
}

/// Copy faces from a source arena into the draft.
///
/// For each selected face:
/// 1. Ensure all vertices in the face's loop are inserted (with dedup)
/// 2. Insert all halfedges with remapped references
/// 3. Insert the face and loop
/// 4. Wire up all prev/next/twin pointers using post-pass
fn copy_faces_from_arena(
    draft: &mut MutableDraft,
    result_geom: &mut GeometryStore,
    remapper: &mut HandleRemapper,
    source_arena: &forge_topo::arena::TopologyArena,
    source_geom: &GeometryStore,
    selected_faces: &[FaceId],
) -> Result<(), KernelError> {
    for &face_id in selected_faces {
        let face_data = source_arena.get_face(face_id)?;
        let loop_data = source_arena.get_loop(face_data.outer_loop)?;
        let old_loop_id = face_data.outer_loop;

        let loop_halfedges = collect_loop_halfedges(source_arena, loop_data.half_edge)?;

        for &old_he_id in &loop_halfedges {
            let he_data = source_arena.get_half_edge(old_he_id)?;
            let old_vertex = he_data.origin;
            let vkey = pack(old_vertex.index(), old_vertex.generation());

            if !remapper.vertices.contains_key(&vkey) {
                let new_vid = draft.arena_mut().insert_vertex(VertexData {
                    outgoing: HalfEdgeId::new(u32::MAX, 0),
                    lineage: he_data.lineage.clone(),
                });
                remapper.vertices.insert(vkey, new_vid);

                if let Some(pos) = source_geom.get_vertex_position(old_vertex) {
                    result_geom.set_vertex_position(new_vid, *pos);
                }
            }
        }

        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let placeholder_loop = LoopId::new(u32::MAX, 0);

        let new_face = draft.arena_mut().insert_face(FaceData {
            outer_loop: placeholder_loop,
            lineage: face_data.lineage.clone(),
        });
        remapper.faces.insert(
            pack(face_id.index(), face_id.generation()),
            new_face,
        );

        if let Some(plane) = source_geom.get_face_plane(face_id) {
            result_geom.set_face_plane(new_face, plane.clone());
        }

        let new_loop = draft.arena_mut().insert_loop(LoopData {
            half_edge: placeholder_he,
            face: new_face,
        });
        remapper.loops.insert(
            pack(old_loop_id.index(), old_loop_id.generation()),
            new_loop,
        );

        let mut new_he_ids = Vec::with_capacity(loop_halfedges.len());
        for &old_he_id in &loop_halfedges {
            let he_data = source_arena.get_half_edge(old_he_id)?;
            let vkey = pack(he_data.origin.index(), he_data.origin.generation());
            let new_origin = remapper.vertices[&vkey];

            let new_he = draft.arena_mut().insert_half_edge(HalfEdgeData {
                twin: placeholder_he,
                next: placeholder_he,
                prev: placeholder_he,
                face: new_face,
                origin: new_origin,
                lineage: he_data.lineage.clone(),
            });
            remapper.half_edges.insert(
                pack(old_he_id.index(), old_he_id.generation()),
                new_he,
            );
            new_he_ids.push(new_he);
        }

        let he_count = new_he_ids.len();
        for i in 0..he_count {
            let next_i = (i + 1) % he_count;
            let prev_i = if i == 0 { he_count - 1 } else { i - 1 };

            let arena = draft.arena_mut();
            arena.get_half_edge_mut(new_he_ids[i])?.next = new_he_ids[next_i];
            arena.get_half_edge_mut(new_he_ids[i])?.prev = new_he_ids[prev_i];
        }

        if !new_he_ids.is_empty() {
            draft.arena_mut().get_face_mut(new_face)?.outer_loop = new_loop;
            draft.arena_mut().get_loop_mut(new_loop)?.half_edge = new_he_ids[0];
        }

        for &new_he in &new_he_ids {
            let origin = draft.arena().get_half_edge(new_he)?.origin;
            draft.arena_mut().get_vertex_mut(origin)?.outgoing = new_he;
        }
    }

    stitch_twin_pointers(draft, remapper, source_arena, selected_faces)?;

    Ok(())
}

/// Collect all halfedge IDs in a face's outer loop.
fn collect_loop_halfedges(
    arena: &forge_topo::arena::TopologyArena,
    start_he: HalfEdgeId,
) -> Result<Vec<HalfEdgeId>, KernelError> {
    let mut result = Vec::new();
    let mut current = start_he;
    let max_iterations: usize = 10_000;

    for _ in 0..max_iterations {
        result.push(current);
        let next = arena.get_half_edge(current)?.next;
        current = next;
        if current == start_he {
            return Ok(result);
        }
    }

    Err(KernelError::InternalError {
        message: "Loop limit exceeded in collect_loop_halfedges".to_string(),
        context: None,
    })
}

/// Stitch twin pointers for copied halfedges from a single source arena.
///
/// For each copied halfedge, check if its old twin was also copied.
/// If not, we generate a proper boundary loop by linking the `next` and `prev`
/// pointers of the new boundary twins.
fn stitch_twin_pointers(
    draft: &mut MutableDraft,
    remapper: &HandleRemapper,
    source_arena: &forge_topo::arena::TopologyArena,
    selected_faces: &[FaceId],
) -> Result<(), KernelError> {
    let mut unpaired: Vec<(HalfEdgeId, VertexId, VertexId)> = Vec::new();

    for &face_id in selected_faces {
        let face_data = source_arena.get_face(face_id)?;
        let loop_data = source_arena.get_loop(face_data.outer_loop)?;
        let loop_hes = collect_loop_halfedges(source_arena, loop_data.half_edge)?;

        for &old_he_id in &loop_hes {
            let he_key = pack(old_he_id.index(), old_he_id.generation());
            let new_he = remapper.half_edges[&he_key];

            let old_twin_id = source_arena.get_half_edge(old_he_id)?.twin;
            let twin_key = pack(old_twin_id.index(), old_twin_id.generation());

            if let Some(&new_twin) = remapper.half_edges.get(&twin_key) {
                draft.arena_mut().get_half_edge_mut(new_he)?.twin = new_twin;
            } else {
                let he_data = draft.arena().get_half_edge(new_he)?.clone();
                let next_he = he_data.next;
                let dest = draft.arena().get_half_edge(next_he)?.origin;
                unpaired.push((new_he, he_data.origin, dest));
            }
        }
    }

    let placeholder = HalfEdgeId::new(u32::MAX, 0);
    let mut boundary_twins: HashMap<HalfEdgeId, HalfEdgeId> = HashMap::new();
    let mut twins_by_origin: HashMap<VertexId, HalfEdgeId> = HashMap::new();

    for &(he_in, _origin, dest) in &unpaired {
        let current_twin = draft.arena().get_half_edge(he_in)?.twin;

        if current_twin == placeholder {
            let boundary_face = draft.arena().get_half_edge(he_in)?.face;

            let he_out = draft.arena_mut().insert_half_edge(HalfEdgeData {
                twin: he_in,
                next: placeholder,
                prev: placeholder,
                face: boundary_face,
                origin: dest,
                lineage: None,
            });

            draft.arena_mut().get_half_edge_mut(he_in)?.twin = he_out;
            boundary_twins.insert(he_in, he_out);
            twins_by_origin.insert(dest, he_out);
        }
    }

    for &(he_in, origin, _dest) in &unpaired {
        if let Some(&he_out) = boundary_twins.get(&he_in) {
            if let Some(&he_out_next) = twins_by_origin.get(&origin) {
                draft.arena_mut().get_half_edge_mut(he_out)?.next = he_out_next;
                draft.arena_mut().get_half_edge_mut(he_out_next)?.prev = he_out;
            } else {
                return Err(KernelError::InternalError {
                    message: "Open boundary is non-manifold; cannot stitch boundary loop".to_string(),
                    context: None,
                });
            }
        }
    }

    Ok(())
}
