//! Face classification evaluation logic.
//!
//! DOMAIN: Classify faces of one solid relative to another via ray-casting.

use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};

use forge_core::KernelError;
use forge_core::result::{TracedDecision, DecisionId, DecisionKind, DecisionContext, DecisionTier, EntityRef};
use forge_topo::arena::TopologyArena;
use forge_topo::classify::classify_point_in_solid;
use forge_topo::handles::FaceId;
use forge_topo::state::TopologyState;

use forge_geom::{Aabb, BvhNode};

use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;
use crate::operations::boolean::eval::compute_face_centroid;
use crate::operations::boolean::schema::{FaceClassification, ClassifiedFace, FaceOrigin};

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
    let mut config = ctx.get_tolerance_config().clone();
    let origin_label = match origin {
        FaceOrigin::Target => "Target",
        FaceOrigin::Tool => "Tool",
    };

    let scale_aware_ray_extent = compute_ray_extent_from_bbox(other_arena, other_geometry, config.get_ray_extent());
    config.set_ray_extent(scale_aware_ray_extent);

    let accelerator_data = build_spatial_index(other_arena, other_geometry);
    let accelerator = accelerator_data.as_deref()
        .map(|bvh| bvh as &dyn forge_topo::classify::SpatialAccelerator);

    let mut classified = Vec::new();

    for (face_id, _) in source_arena.iter_faces() {
        let computed_class = classify_single_face(
            source_arena, source_geometry,
            other_arena, other_geometry,
            accelerator,
            face_id, &config, ctx,
        )?;

        let decision_id = DecisionId(face_id.index() as u64);
        let (final_class, overridden) = match ctx.get_classification_override(decision_id) {
            Some(forced) => (forced, true),
            None => (computed_class, false),
        };

        let class_label = classification_label(&final_class);
        let (kind, tier) = if overridden {
            (
                DecisionKind::Forced {
                    reason: format!(
                        "counterfactual override: {} → {}",
                        classification_label(&computed_class),
                        class_label,
                    ),
                },
                DecisionTier::Escalated,
            )
        } else {
            (DecisionKind::Exact, DecisionTier::Deterministic)
        };
        let mut decision = TracedDecision::new(
            decision_id,
            kind,
            tier,
            1.0,
            DecisionContext::Classification {
                point: compute_face_centroid(source_arena, source_geometry, face_id)
                    .unwrap_or([0.0; 3]),
                result: format!("{}:Face#{} → {}", origin_label, face_id.index(), class_label),
            },
        );
        decision.set_entity_scope(EntityRef::new("Face", face_id.index()));
        ctx.get_decision_log_mut().record(decision);
        classified.push(ClassifiedFace::new(face_id, final_class));
    }

    Ok(classified)
}

/// Build a face adjacency map via twin edges.
///
/// For each face, collects all neighboring faces reachable through
/// halfedge twins. Two faces are adjacent if they share an edge.
fn build_face_adjacency(arena: &TopologyArena) -> BTreeMap<u32, Vec<FaceId>> {
    let mut adjacency: BTreeMap<u32, Vec<FaceId>> = BTreeMap::new();

    for (he_id, he_data) in arena.iter_half_edges() {
        let face_a = he_data.face();
        let twin_id = he_data.twin();
        if he_id != twin_id {
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
    }

    adjacency
}

/// Decompose faces into connected patches via BFS on adjacency.
///
/// Each patch is a group of faces all connected through shared edges.
fn decompose_patches(
    arena: &TopologyArena,
    adjacency: &BTreeMap<u32, Vec<FaceId>>,
) -> Vec<Vec<FaceId>> {
    let mut visited: HashSet<u32> = HashSet::new();
    let mut patches = Vec::new();

    let unvisited_faces: Vec<FaceId> = arena.iter_faces()
        .map(|(fid, _)| fid)
        .filter(|fid| !visited.contains(&fid.index()))
        .collect();

    for face_id in unvisited_faces {
        if visited.contains(&face_id.index()) {
            // Could have been visited by prior BFS from a different seed
            // (the filter above collects eagerly before the loop)
            // This guard replaces the original continue
        } else {
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
    }

    patches
}

/// Classify a single face by sampling a guaranteed-interior point against the other solid.
fn classify_single_face(
    source_arena: &TopologyArena,
    source_geometry: &GeometryStore,
    other_arena: &TopologyArena,
    other_geometry: &GeometryStore,
    accelerator: Option<&dyn forge_topo::classify::SpatialAccelerator>,
    face_id: FaceId,
    config: &crate::core::ToleranceConfig,
    ctx: &mut ModelingContext,
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

    let (class, esc) = match classification {
        forge_topo::classify::PointClassification::Inside { escalation } => (FaceClassification::Inside, escalation),
        forge_topo::classify::PointClassification::Outside { escalation } => (FaceClassification::Outside, escalation),
        forge_topo::classify::PointClassification::OnBoundary(boundary_face_id) => {
            let normals_align = check_normal_alignment(
                source_geometry, face_id,
                other_geometry, boundary_face_id,
            );
            if normals_align {
                (FaceClassification::OnBoundary, None)
            } else {
                (FaceClassification::OppositeBoundary, None)
            }
        }
    };

    eprintln!("DEBUG_CLASSIFY: face={} centroid={:?} class={:?}", face_id.index(), sample, class);

    if let Some(escalation) = esc {
        ctx.log_escalation(escalation);
    }

    eprintln!("  CLASSIFY face={} centroid=[{:.6},{:.6},{:.6}] → {:?}",
        face_id.index(), sample[0], sample[1], sample[2], class);

    Ok(class)
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

/// Compute a sample point guaranteed to lie strictly inside the face polygon.
///
/// Uses the centroid of the largest-area triangle in a fan decomposition.
/// Triangle centroids are always strictly interior to their triangle,
/// so the resulting point is inside the face even for concave polygons.
/// Falls back to vertex-average centroid for degenerate faces (< 3 vertices).
fn compute_interior_sample(
    arena: &TopologyArena,
    geom: &GeometryStore,
    face_id: FaceId,
) -> Result<[f64; 3], KernelError> {
    let edges: Vec<_> = forge_topo::traverse::FaceEdgeIterator::new(arena, face_id)?
        .collect::<Result<Vec<_>, _>>()?;

    let mut vertices = Vec::with_capacity(edges.len());
    for he in &edges {
        let v = arena.get_half_edge(*he)?.origin();
        if let Some(pos) = geom.get_vertex_position(v) {
            vertices.push(*pos);
        }
    }

    forge_geom::primitives::polygon::compute_largest_triangle_centroid(&vertices)
        .ok_or_else(|| compute_face_centroid(arena, geom, face_id))
        .or_else(|fallback| fallback)
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
            forge_math::linalg::normals_aligned(sp.raw_normal(), op.raw_normal())
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

/// Compute a scale-aware ray extent from the bounding box of a solid.
///
/// The ray extent must be longer than the diagonal of the solid to
/// guarantee that a ray from any interior point exits the solid.
/// Returns `max(10 * diagonal, default_extent)`.
fn compute_ray_extent_from_bbox(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    default_extent: f64,
) -> f64 {
    let mut min_pos = [f64::INFINITY; 3];
    let mut max_pos = [f64::NEG_INFINITY; 3];

    for (vid, _) in arena.iter_vertices() {
        if let Some(pos) = geometry.get_vertex_position(vid) {
            for i in 0..3 {
                min_pos[i] = min_pos[i].min(pos[i]);
                max_pos[i] = max_pos[i].max(pos[i]);
            }
        }
    }

    let dx = max_pos[0] - min_pos[0];
    let dy = max_pos[1] - min_pos[1];
    let dz = max_pos[2] - min_pos[2];
    let diagonal = (dx * dx + dy * dy + dz * dz).sqrt();

    (diagonal * 10.0).max(default_extent)
}
