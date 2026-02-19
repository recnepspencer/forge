//! Face classification for Boolean operations.
//!
//! Classifies each face of a solid relative to the other solid.
//!
//! ALGORITHM: Flood-fill classification
//! 1. Build face adjacency graph via twin edges
//! 2. Decompose into connected patches (faces reachable via shared edges)
//! 3. Ray-cast classify ONE seed face per patch
//! 4. Propagate classification to all faces in the patch
//!
//! This is both a performance win (fewer ray casts) and a correctness win:
//! coplanar sub-faces that share edges get consistent classification instead
//! of each independently ray-casting (which can give inconsistent results
//! for points exactly on the boundary of the other solid).

use std::collections::{HashMap, HashSet, VecDeque};

use forge_core::KernelError;
use forge_core::result::{TracedDecision, DecisionId, DecisionKind, DecisionContext, DecisionTier, EntityRef};
use forge_topo::arena::TopologyArena;
use forge_topo::classify::classify_point_in_solid;
use forge_topo::handles::FaceId;

use forge_geom::{Aabb, BvhNode};

use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;
use super::eval::compute_face_centroid;
use super::schema::{FaceClassification, ClassifiedFace, FaceOrigin};

/// Classify all faces of one solid relative to the other solid.
///
/// Uses flood-fill: builds face adjacency, partitions into connected patches,
/// ray-casts one seed per patch, and propagates to the rest.
pub fn classify_faces(
    source_arena: &TopologyArena,
    source_geometry: &GeometryStore,
    other_arena: &TopologyArena,
    other_geometry: &GeometryStore,
    origin: FaceOrigin,
    ctx: &mut ModelingContext,
) -> Result<Vec<ClassifiedFace>, KernelError> {
    let config = ctx.get_tolerance_config().clone();
    let origin_label = match origin {
        FaceOrigin::Target => "Target",
        FaceOrigin::Tool => "Tool",
    };

    let accelerator_data = build_spatial_index(other_arena, other_geometry);
    let accelerator = accelerator_data.as_deref()
        .map(|bvh| bvh as &dyn forge_topo::classify::SpatialAccelerator);

    let adjacency = build_face_adjacency(source_arena);
    let patches = decompose_patches(source_arena, &adjacency);

    let mut classified = Vec::new();

    for patch in &patches {
        let seed_face = patch[0];
        let seed_class = classify_single_face(
            source_arena, source_geometry,
            other_arena, other_geometry,
            accelerator,
            seed_face, &config,
        )?;

        let class_label = classification_label(&seed_class);
        let mut seed_decision = TracedDecision::new(
            DecisionId(seed_face.index() as u64),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Classification {
                point: compute_face_centroid(source_arena, source_geometry, seed_face)
                    .unwrap_or([0.0; 3]),
                result: format!("{}:Face#{} → {} (seed)", origin_label, seed_face.index(), class_label),
            },
        );
        seed_decision.set_entity_scope(EntityRef::new("Face", seed_face.index()));
        ctx.get_decision_log_mut().record(seed_decision);
        classified.push(ClassifiedFace::new(seed_face, seed_class));

        for &face_id in &patch[1..] {
            let propagated_class = classify_single_face(
                source_arena, source_geometry,
                other_arena, other_geometry,
                accelerator,
                face_id, &config,
            )?;

            let prop_label = classification_label(&propagated_class);
            let mut decision = TracedDecision::new(
                DecisionId(face_id.index() as u64),
                DecisionKind::Exact,
                DecisionTier::Resolved,
                1.0,
                DecisionContext::Classification {
                    point: compute_face_centroid(source_arena, source_geometry, face_id)
                        .unwrap_or([0.0; 3]),
                    result: format!("{}:Face#{} → {} (patch of seed #{})",
                        origin_label, face_id.index(), prop_label, seed_face.index()),
                },
            );
            decision.set_entity_scope(EntityRef::new("Face", face_id.index()));
            ctx.get_decision_log_mut().record(decision);
            classified.push(ClassifiedFace::new(face_id, propagated_class));
        }
    }

    Ok(classified)
}

/// Build a face adjacency map via twin edges.
///
/// For each face, collects all neighboring faces reachable through
/// halfedge twins. Two faces are adjacent if they share an edge.
fn build_face_adjacency(arena: &TopologyArena) -> HashMap<u32, Vec<FaceId>> {
    let mut adjacency: HashMap<u32, Vec<FaceId>> = HashMap::new();

    for (he_id, he_data) in arena.iter_half_edges() {
        let face_a = he_data.face();
        let twin_id = he_data.twin();
        if he_id == twin_id {
            continue;
        }
        if let Ok(twin_data) = arena.get_half_edge(twin_id) {
            let face_b = twin_data.face();
            if face_a != face_b {
                let entry_a = adjacency.entry(face_a.index()).or_default();
                if !entry_a.iter().any(|f| *f == face_b) {
                    entry_a.push(face_b);
                }
                let entry_b = adjacency.entry(face_b.index()).or_default();
                if !entry_b.iter().any(|f| *f == face_a) {
                    entry_b.push(face_a);
                }
            }
        }
    }

    adjacency
}

/// Decompose faces into connected patches via BFS on adjacency.
///
/// Each patch is a group of faces all connected through shared edges.
fn decompose_patches(
    arena: &TopologyArena,
    adjacency: &HashMap<u32, Vec<FaceId>>,
) -> Vec<Vec<FaceId>> {
    let mut visited: HashSet<u32> = HashSet::new();
    let mut patches = Vec::new();

    for (face_id, _) in arena.iter_faces() {
        if visited.contains(&face_id.index()) {
            continue;
        }

        let mut patch = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(face_id);
        visited.insert(face_id.index());

        while let Some(fid) = queue.pop_front() {
            patch.push(fid);

            if let Some(neighbors) = adjacency.get(&fid.index()) {
                for neighbor in neighbors {
                    if !visited.contains(&neighbor.index()) {
                        visited.insert(neighbor.index());
                        queue.push_back(*neighbor);
                    }
                }
            }
        }

        patches.push(patch);
    }

    patches
}

/// Classify a single face by sampling its centroid against the other solid.
fn classify_single_face(
    source_arena: &TopologyArena,
    source_geometry: &GeometryStore,
    other_arena: &TopologyArena,
    other_geometry: &GeometryStore,
    accelerator: Option<&dyn forge_topo::classify::SpatialAccelerator>,
    face_id: FaceId,
    config: &crate::core::ToleranceConfig,
) -> Result<FaceClassification, KernelError> {
    let sample = compute_face_centroid(source_arena, source_geometry, face_id)?;

    let vertex_lookup = |index: u32| -> Result<[f64; 3], KernelError> {
        let gen = other_arena.vertex_generation(index as usize).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No active vertex at slot index {}", index),
                context: None,
            }
        })?;
        let vid = forge_topo::handles::VertexId::from_raw_parts(index, gen);
        other_geometry.get_vertex_position(vid).copied().ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No position for vertex {}", index),
                context: None,
            }
        })
    };

    let classification = classify_point_in_solid(
        other_arena,
        &vertex_lookup,
        accelerator,
        &sample,
        config.get_ray_extent(),
        config.get_edge_split_degeneracy(),
    )?;

    match classification {
        forge_topo::classify::PointClassification::Inside => Ok(FaceClassification::Inside),
        forge_topo::classify::PointClassification::Outside => Ok(FaceClassification::Outside),
        forge_topo::classify::PointClassification::OnBoundary(boundary_face_id) => {
            let normals_align = check_normal_alignment(
                source_geometry, face_id,
                other_geometry, boundary_face_id,
            );
            if normals_align {
                Ok(FaceClassification::OnBoundary)
            } else {
                Ok(FaceClassification::OppositeBoundary)
            }
        }
    }
}

/// Build BVH spatial index for the "other" solid (for accelerated point-in-solid).
fn build_spatial_index(
    arena: &TopologyArena,
    geometry: &GeometryStore,
) -> Option<Box<BvhNode<FaceId>>> {
    let mut face_aabbs = Vec::new();
    for (face_id, _) in arena.iter_faces() {
        let mut points = Vec::new();
        if let Ok(iter) = forge_topo::traverse::FaceEdgeIterator::new(arena, face_id) {
            for he_res in iter {
                if let Ok(he_id) = he_res {
                    if let Ok(he) = arena.get_half_edge(he_id) {
                        let vid = he.origin();
                        if let Some(pos) = geometry.get_vertex_position(vid) {
                            points.push(*pos);
                        }
                    }
                }
            }
        }
        if !points.is_empty() {
            if let Some(aabb) = Aabb::from_points(&points) {
                face_aabbs.push((face_id, aabb));
            }
        }
    }
    BvhNode::build(face_aabbs)
}

/// Check whether two faces have aligned normals (same direction).
fn check_normal_alignment(
    source_geom: &GeometryStore,
    source_face: FaceId,
    other_geom: &GeometryStore,
    other_face: FaceId,
) -> bool {
    let source_plane = source_geom.get_face_plane(source_face);
    let other_plane = other_geom.get_face_plane(other_face);

    match (source_plane, other_plane) {
        (Some(sp), Some(op)) => {
            let sn = sp.raw_normal();
            let on = op.raw_normal();
            let dot = sn[0] * on[0] + sn[1] * on[1] + sn[2] * on[2];
            dot > 0.0
        }
        _ => true,
    }
}

/// Human-readable label for a classification.
fn classification_label(class: &FaceClassification) -> &'static str {
    match class {
        FaceClassification::Inside => "Inside",
        FaceClassification::Outside => "Outside",
        FaceClassification::OnBoundary => "OnBoundary(aligned)",
        FaceClassification::OppositeBoundary => "OppositeBoundary(opposed)",
    }
}
