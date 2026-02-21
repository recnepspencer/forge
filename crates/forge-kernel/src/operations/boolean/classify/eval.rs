//! Face classification evaluation.
//!
//! DOMAIN: Classify each face of one solid relative to another via ray-casting.
//! DEPENDENCIES: forge_topo::classify (point-in-solid), GeometryStore, BVH.
//!
//! ALGORITHM:
//! 1. Build BVH spatial index on the "other" solid.
//! 2. For each face of the "source" solid, compute its centroid.
//! 3. Ray-cast from centroid into "other" solid → Inside/Outside/OnBoundary.
//! 4. OnBoundary: check normal alignment → OnBoundary vs OppositeBoundary.
//! 5. Apply counterfactual overrides if present (P3.3).
//! 6. Record TracedDecision for every classification.

use forge_core::KernelError;
use forge_core::{TracedDecision, DecisionId, DecisionKind, DecisionContext, DecisionTier, EntityRef};
use forge_core::tracing::TopologyDelta;
use forge_topo::arena::TopologyArena;
use forge_topo::classify::classify_point_in_solid;
use forge_topo::handles::FaceId;
use forge_topo::traverse::FaceEdgeIterator;

use forge_geom::{Aabb, BvhNode};

use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;
use crate::operations::boolean::eval::compute_face_centroid;
use crate::operations::boolean::schema::{FaceClassification, ClassifiedFace, FaceOrigin};

/// Classify all faces of one solid relative to the other solid.
pub fn classify_faces(
    source_arena: &TopologyArena,
    source_geometry: &GeometryStore,
    other_arena: &TopologyArena,
    other_geometry: &GeometryStore,
    origin: FaceOrigin,
    ctx: &mut ModelingContext,
) -> Result<Vec<ClassifiedFace>, KernelError> {
    let mut config = ctx.get_tolerance_config().clone();
    let ray_extent = compute_ray_extent_from_bbox(other_arena, other_geometry, config.get_ray_extent());
    config.set_ray_extent(ray_extent);

    let accelerator_data = build_spatial_index(other_arena, other_geometry);
    let accelerator = accelerator_data.as_deref()
        .map(|bvh| bvh as &dyn forge_topo::classify::SpatialAccelerator);

    let origin_label = origin_to_label(origin);
    let mut classified = Vec::new();

    for (face_id, _) in source_arena.iter_faces() {
        let computed = classify_single_face(
            source_arena, source_geometry,
            other_arena, other_geometry,
            accelerator, face_id, &config, ctx,
        )?;

        let (final_class, overridden) = apply_override(ctx, face_id, computed);
        log_classification(ctx, face_id, &final_class, &computed, overridden, origin_label);
        classified.push(ClassifiedFace::new(face_id, final_class));
    }

    Ok(classified)
}

// ── Per-face classification ──────────────────────────────────────────────────

/// Classify a single face by ray-casting its centroid into the other solid.
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

    let classification = classify_point_in_solid(
        other_arena,
        &|index| lookup_vertex_position(other_arena, other_geometry, index),
        accelerator,
        &sample,
        config.get_ray_extent(),
        config.get_edge_split_degeneracy(),
    )?;

    let (class, escalation) = interpret_classification(
        classification, source_geometry, face_id, other_geometry,
    );

    if let Some(esc) = escalation {
        ctx.log_escalation(esc);
    }

    Ok(class)
}

/// Interpret a raw PointClassification into a FaceClassification.
///
/// For boundary hits, checks whether normals are aligned (same-facing)
/// or opposed, yielding OnBoundary vs OppositeBoundary.
fn interpret_classification(
    classification: forge_topo::classify::PointClassification,
    source_geom: &GeometryStore,
    source_face: FaceId,
    other_geom: &GeometryStore,
) -> (FaceClassification, Option<forge_math::arithmetic::filter::PrecisionEscalation>) {
    match classification {
        forge_topo::classify::PointClassification::Inside { escalation } =>
            (FaceClassification::Inside, escalation),
        forge_topo::classify::PointClassification::Outside { escalation } =>
            (FaceClassification::Outside, escalation),
        forge_topo::classify::PointClassification::OnBoundary(boundary_face) => {
            let aligned = check_normal_alignment(source_geom, source_face, other_geom, boundary_face);
            let class = if aligned {
                FaceClassification::OnBoundary
            } else {
                FaceClassification::OppositeBoundary
            };
            (class, None)
        }
    }
}

/// Look up a vertex position by raw slot index.
fn lookup_vertex_position(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    index: u32,
) -> Result<[f64; 3], KernelError> {
    let gen = arena.vertex_generation(index as usize).ok_or_else(|| {
        KernelError::InvalidInput {
            message: format!("No active vertex at slot index {}", index), context: None,
        }
    })?;
    let vid = forge_topo::handles::VertexId::from_raw_parts(index, gen);
    geometry.get_vertex_position(vid).copied().ok_or_else(|| {
        KernelError::InvalidInput {
            message: format!("No position for vertex {}", index), context: None,
        }
    })
}

// ── Override and logging ─────────────────────────────────────────────────────

/// Apply a counterfactual classification override if one exists.
fn apply_override(
    ctx: &ModelingContext,
    face_id: FaceId,
    computed: FaceClassification,
) -> (FaceClassification, bool) {
    let decision_id = DecisionId(face_id.index() as u64);
    match ctx.get_classification_override(decision_id) {
        Some(forced) => (forced, true),
        None => (computed, false),
    }
}

/// Record a TracedDecision for a face classification.
fn log_classification(
    ctx: &mut ModelingContext,
    face_id: FaceId,
    final_class: &FaceClassification,
    computed_class: &FaceClassification,
    overridden: bool,
    origin_label: &str,
) {
    let label = classification_label(final_class);
    let (kind, tier) = if overridden {
        (
            DecisionKind::Forced {
                reason: format!("{} → {}", classification_label(computed_class), label),
            },
            DecisionTier::Escalated,
        )
    } else {
        (DecisionKind::Exact, DecisionTier::Deterministic)
    };

    let mut decision = TracedDecision::new(
        DecisionId(face_id.index() as u64), kind, tier, 1.0,
        DecisionContext::Classification {
            point: [0.0; 3],
            result: format!("{}:Face#{} → {}", origin_label, face_id.index(), label),
        },
    );
    decision.set_entity_scope(EntityRef::new("Face", face_id.index()));
    decision.set_topology_delta(TopologyDelta::default());
    ctx.get_decision_log_mut().record(decision);
}

// ── Spatial indexing ─────────────────────────────────────────────────────────

/// Build BVH spatial index for the "other" solid.
fn build_spatial_index(
    arena: &TopologyArena,
    geometry: &GeometryStore,
) -> Option<Box<BvhNode<FaceId>>> {
    let face_aabbs: Vec<(FaceId, Aabb)> = arena.iter_faces()
        .filter_map(|(face_id, _)| {
            let aabb = compute_face_aabb(arena, geometry, face_id)?;
            Some((face_id, aabb))
        })
        .collect();
    BvhNode::build(face_aabbs)
}

/// Compute the AABB of a single face.
fn compute_face_aabb(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    face_id: FaceId,
) -> Option<Aabb> {
    let edges = FaceEdgeIterator::new(arena, face_id).ok()?;
    let mut points = Vec::new();
    for he_res in edges {
        let he_id = he_res.ok()?;
        let he = arena.get_half_edge(he_id).ok()?;
        if let Some(pos) = geometry.get_vertex_position(he.origin()) {
            points.push(*pos);
        }
    }
    Aabb::from_points(&points)
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Check whether two faces have aligned normals.
fn check_normal_alignment(
    source_geom: &GeometryStore,
    source_face: FaceId,
    other_geom: &GeometryStore,
    other_face: FaceId,
) -> bool {
    match (source_geom.get_face_plane(source_face), other_geom.get_face_plane(other_face)) {
        (Some(sp), Some(op)) => forge_geom::primitives::plane::normals_aligned_exact(sp, op),
        _ => true,
    }
}

/// Compute a scale-aware ray extent from the bounding box of a solid.
///
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
            min_pos = forge_math::linalg::component_min(min_pos, *pos);
            max_pos = forge_math::linalg::component_max(max_pos, *pos);
        }
    }

    let diagonal = forge_math::linalg::norm(forge_math::linalg::sub(max_pos, min_pos));
    (diagonal * 10.0).max(default_extent)
}

/// Human-readable label for a FaceClassification.
fn classification_label(class: &FaceClassification) -> &'static str {
    match class {
        FaceClassification::Inside => "Inside",
        FaceClassification::Outside => "Outside",
        FaceClassification::OnBoundary => "OnBoundary(aligned)",
        FaceClassification::OppositeBoundary => "OppositeBoundary(opposed)",
    }
}

/// Convert FaceOrigin to a label string.
fn origin_to_label(origin: FaceOrigin) -> &'static str {
    match origin {
        FaceOrigin::Target => "Target",
        FaceOrigin::Tool => "Tool",
    }
}
