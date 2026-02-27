//! Face classification evaluation.
//!
//! DOMAIN: Classify each face of one solid relative to another via ray-casting.
//! DEPENDENCIES: forge_topo::classify (point-in-solid), GeometryState, BVH.
//!
//! ALGORITHM:
//! 1. Build BVH spatial index on the "other" solid.
//! 2. For each face of the "source" solid, compute its centroid.
//! 3. Ray-cast from centroid into "other" solid → Inside/Outside/OnBoundary.
//! 4. OnBoundary → multi-sample fallback: perturb along face normal (±ε),
//!    re-classify both points. If they agree → use unambiguous result.
//!    If they disagree → normal alignment → OnBoundary vs OppositeBoundary.
//! 5. Apply counterfactual overrides if present (P3.3).
//! 6. Record TracedDecision for every classification.

use forge_core::tracing::TopologyDelta;
use forge_core::KernelError;
use forge_core::{
    DecisionContext, DecisionId, DecisionKind, DecisionTier, EntityRef, ToleranceProvider,
    TracedDecision,
};
use forge_geom::BvhNode;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::FaceId;

use crate::core::ModelingContext;
use crate::geometry_state::GeometryState;
use crate::operations::boolean::classify_schema::{ClassifiedFace, FaceClassification, FaceOrigin};
use crate::shared_ops::centroid::compute_face_centroid;
use crate::spatial::{
    all_face_bounds, classify_point_in_solid, classify_point_on_face,
    classify_point_with_perturbation, FacePointClassification, PointClassification,
    SpatialAccelerator,
};

/// Classify all faces of one solid relative to the other solid.
pub fn classify_faces(
    source_arena: &TopologyArena,
    source_geometry: &GeometryState,
    other_arena: &TopologyArena,
    other_geometry: &GeometryState,
    origin: FaceOrigin,
    ctx: &mut ModelingContext,
) -> Result<Vec<ClassifiedFace>, KernelError> {
    let config = ctx.get_tolerance_config().clone();

    let accelerator_data = build_spatial_index(other_arena, other_geometry);
    let accelerator = accelerator_data
        .as_deref()
        .map(|bvh| bvh as &dyn SpatialAccelerator);

    let origin_label = origin_to_label(origin);
    let mut classified = Vec::new();

    for (face_id, _) in source_arena.iter_faces() {
        let computed = classify_single_face(
            source_arena,
            source_geometry,
            other_arena,
            other_geometry,
            accelerator,
            face_id,
            &config,
            ctx,
        )?;

        if std::env::var("FORGE_DEBUG_CLASSIFY_PROVENANCE")
            .ok()
            .as_deref()
            == Some("1")
        {
            let sample = compute_face_centroid(source_arena, source_geometry, face_id)
                .unwrap_or([f64::NAN; 3]);
            let lineage_str = source_arena
                .get_face(face_id)
                .ok()
                .and_then(|f| f.lineage())
                .map(|lin| {
                    format!(
                        "{}#{}",
                        lin.get_creation_op().get_name(),
                        lin.get_creation_op().get_invocation_id()
                    )
                })
                .unwrap_or_else(|| "no-lineage".to_string());
            eprintln!(
                "[classify-prov] {} F#{} {:?} sample=[{:.6},{:.6},{:.6}] {}",
                origin_label,
                face_id.index(),
                computed,
                sample[0],
                sample[1],
                sample[2],
                lineage_str
            );
        }

        let (final_class, overridden) = apply_override(ctx, face_id, computed);
        log_classification(
            ctx,
            face_id,
            &final_class,
            &computed,
            overridden,
            origin_label,
        );
        classified.push(ClassifiedFace::new(face_id, final_class));
    }

    Ok(classified)
}

// ── Per-face classification ──────────────────────────────────────────────────

/// Classify a single face by ray-casting its centroid into the other solid.
///
/// When the centroid lands exactly on the other solid's boundary
/// ("kissing" contact), a multi-sample fallback perturbs the sample
/// along the face normal to resolve the ambiguity.
fn classify_single_face(
    source_arena: &TopologyArena,
    source_geometry: &GeometryState,
    other_arena: &TopologyArena,
    other_geometry: &GeometryState,
    accelerator: Option<&dyn SpatialAccelerator>,
    face_id: FaceId,
    config: &crate::core::ToleranceConfig,
    ctx: &mut ModelingContext,
) -> Result<FaceClassification, KernelError> {
    let sample =
        compute_face_centroid(source_arena, source_geometry, face_id).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!(
                    "Face {:?} has degenerate geometry (no vertices/area)",
                    face_id
                ),
                context: None,
            }
        })?;

    let primary = classify_point_in_solid(
        other_arena,
        &|index| lookup_vertex_position(other_arena, other_geometry, index),
        accelerator,
        &sample,
        other_geometry as &dyn ToleranceProvider,
    )?;

    let classification = maybe_multisample_refine(
        source_arena,
        source_geometry,
        other_arena,
        other_geometry,
        accelerator,
        face_id,
        sample,
        primary,
    )?;

    let (class, escalation) = match &classification {
        PointClassification::OnBoundary(_) => resolve_boundary_classification(
            source_arena,
            source_geometry,
            face_id,
            other_arena,
            other_geometry,
            accelerator,
            &classification,
            config,
        )?,
        _ => interpret_classification(classification, source_geometry, face_id, other_geometry),
    };

    if let Some(esc) = escalation {
        ctx.log_escalation(esc);
    }

    Ok(class)
}

fn maybe_multisample_refine(
    source_arena: &TopologyArena,
    source_geometry: &GeometryState,
    other_arena: &TopologyArena,
    other_geometry: &GeometryState,
    accelerator: Option<&dyn SpatialAccelerator>,
    face_id: FaceId,
    centroid: [f64; 3],
    primary: PointClassification,
) -> Result<PointClassification, KernelError> {
    if matches!(primary, PointClassification::OnBoundary(_)) {
        return Ok(primary);
    }
    if !face_needs_multisample(source_arena, face_id) {
        return Ok(primary);
    }

    let samples = collect_interior_face_samples(source_arena, source_geometry, face_id, centroid)?;
    if samples.len() <= 1 {
        return Ok(primary);
    }

    let mut inside = 0usize;
    let mut outside = 0usize;
    let mut first_boundary: Option<FaceId> = None;

    for p in samples {
        let cls = classify_point_in_solid(
            other_arena,
            &|index| lookup_vertex_position(other_arena, other_geometry, index),
            accelerator,
            &p,
            other_geometry as &dyn ToleranceProvider,
        )?;
        match cls {
            PointClassification::Inside { .. } => inside += 1,
            PointClassification::Outside { .. } => outside += 1,
            PointClassification::OnBoundary(fid) => {
                if first_boundary.is_none() {
                    first_boundary = Some(fid);
                }
            }
        }
    }

    if std::env::var("FORGE_DEBUG_MULTISAMPLE").ok().as_deref() == Some("1") {
        eprintln!(
            "[multisample] face#{} votes inside={} outside={} boundary={}",
            face_id.index(),
            inside,
            outside,
            usize::from(first_boundary.is_some())
        );
    }

    if inside > outside && first_boundary.is_none() {
        return Ok(PointClassification::Inside { escalation: None });
    }
    if outside > inside && first_boundary.is_none() {
        return Ok(PointClassification::Outside { escalation: None });
    }
    if let Some(fid) = first_boundary {
        return Ok(PointClassification::OnBoundary(fid));
    }

    Ok(primary)
}

fn face_needs_multisample(source_arena: &TopologyArena, face_id: FaceId) -> bool {
    let Some(face) = source_arena.get_face(face_id).ok() else {
        return false;
    };
    let Some(lineage) = face.lineage() else {
        return false;
    };
    lineage
        .get_creation_op()
        .get_name()
        .starts_with("make_edge_face")
}

fn collect_interior_face_samples(
    arena: &TopologyArena,
    geometry: &GeometryState,
    face_id: FaceId,
    centroid: [f64; 3],
) -> Result<Vec<[f64; 3]>, KernelError> {
    let mut verts: Vec<[f64; 3]> = Vec::new();
    let loops = forge_topo::polygon::face_loop_vertices(arena, face_id)?;
    if let Some(outer_loop) = loops.first() {
        for vertex in outer_loop {
            if let Some(p) = geometry.get_vertex_position(*vertex) {
                verts.push(*p);
            }
        }
    }

    if verts.len() < 3 {
        return Ok(vec![centroid]);
    }

    let pos_fn = |v: forge_topo::handles::VertexId| geometry.get_vertex_position(v).copied();
    let mut samples: Vec<[f64; 3]> = Vec::new();
    let mut push_if_on_face = |p: [f64; 3]| -> Result<(), KernelError> {
        match classify_point_on_face(
            arena,
            face_id,
            &p,
            &pos_fn,
            source_geometry_as_tol(geometry),
        )? {
            FacePointClassification::OnFace => {
                if !samples.iter().any(|q| same_point(q, &p)) {
                    samples.push(p);
                }
            }
            _ => {}
        }
        Ok(())
    };

    push_if_on_face(centroid)?;

    let n = verts.len();
    for i in 0..n.min(8) {
        let a = verts[i];
        let b = verts[(i + 1) % n];
        let edge_mid = [
            (a[0] + b[0]) * 0.5,
            (a[1] + b[1]) * 0.5,
            (a[2] + b[2]) * 0.5,
        ];
        let inset = [
            edge_mid[0] * 0.35 + centroid[0] * 0.65,
            edge_mid[1] * 0.35 + centroid[1] * 0.65,
            edge_mid[2] * 0.35 + centroid[2] * 0.65,
        ];
        push_if_on_face(inset)?;

        let fan = [
            (a[0] + b[0] + centroid[0]) / 3.0,
            (a[1] + b[1] + centroid[1]) / 3.0,
            (a[2] + b[2] + centroid[2]) / 3.0,
        ];
        push_if_on_face(fan)?;

        let toward_a = [
            centroid[0] * 0.6 + a[0] * 0.4,
            centroid[1] * 0.6 + a[1] * 0.4,
            centroid[2] * 0.6 + a[2] * 0.4,
        ];
        push_if_on_face(toward_a)?;

        let toward_b = [
            centroid[0] * 0.6 + b[0] * 0.4,
            centroid[1] * 0.6 + b[1] * 0.4,
            centroid[2] * 0.6 + b[2] * 0.4,
        ];
        push_if_on_face(toward_b)?;
    }

    if samples.is_empty() {
        samples.push(centroid);
    }

    Ok(samples)
}

fn source_geometry_as_tol(geometry: &GeometryState) -> &dyn ToleranceProvider {
    geometry as &dyn ToleranceProvider
}

fn same_point(a: &[f64; 3], b: &[f64; 3]) -> bool {
    forge_geom::primitives::point::is_same_point_within(a, b, 1e-12)
}

/// Interpret a raw PointClassification into a FaceClassification.
///
/// For boundary hits, checks whether normals are aligned (same-facing)
/// or opposed, yielding OnBoundary vs OppositeBoundary.
fn interpret_classification(
    classification: PointClassification,
    source_geom: &GeometryState,
    source_face: FaceId,
    other_geom: &GeometryState,
) -> (
    FaceClassification,
    Option<forge_math::arithmetic::precision::PrecisionEscalation>,
) {
    match classification {
        PointClassification::Inside { escalation } => (FaceClassification::Inside, escalation),
        PointClassification::Outside { escalation } => (FaceClassification::Outside, escalation),
        PointClassification::OnBoundary(boundary_face) => {
            let aligned =
                check_normal_alignment(source_geom, source_face, other_geom, boundary_face);
            let class = if aligned {
                FaceClassification::OnBoundary
            } else {
                FaceClassification::OppositeBoundary
            };
            (class, None)
        }
    }
}

/// Resolve an OnBoundary classification via multi-sample normal perturbation.
///
/// When the centroid lands exactly on the other solid's boundary,
/// we perturb the sample in both directions along the source face
/// normal and re-classify. If both agree, the unambiguous result
/// is returned. Otherwise, falls back to normal alignment.
fn resolve_boundary_classification(
    source_arena: &TopologyArena,
    source_geometry: &GeometryState,
    source_face: FaceId,
    other_arena: &TopologyArena,
    other_geometry: &GeometryState,
    accelerator: Option<&dyn SpatialAccelerator>,
    original: &PointClassification,
    config: &crate::core::ToleranceConfig,
) -> Result<
    (
        FaceClassification,
        Option<forge_math::arithmetic::precision::PrecisionEscalation>,
    ),
    KernelError,
> {
    let normal = match source_geometry.get_face_plane(source_face) {
        Some(plane) => plane.raw_normal(),
        None => {
            return Ok(interpret_classification(
                original.clone(),
                source_geometry,
                source_face,
                other_geometry,
            ));
        }
    };

    let centroid =
        compute_face_centroid(source_arena, source_geometry, source_face).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!(
                    "Face {:?} has degenerate geometry (no vertices/area)",
                    source_face
                ),
                context: None,
            }
        })?;
    let epsilon = config.get_edge_split_degeneracy() * 100.0;

    let perturbed = classify_point_with_perturbation(
        other_arena,
        &|index| lookup_vertex_position(other_arena, other_geometry, index),
        accelerator,
        &centroid,
        normal,
        epsilon,
        other_geometry as &dyn ToleranceProvider,
    )?;
    if let Some(classification) = perturbed {
        return Ok((
            to_face_classification(&classification),
            extract_escalation(&classification),
        ));
    }

    Ok(interpret_classification(
        original.clone(),
        source_geometry,
        source_face,
        other_geometry,
    ))
}

/// Convert a PointClassification to a simple FaceClassification.
fn to_face_classification(pc: &PointClassification) -> FaceClassification {
    match pc {
        PointClassification::Inside { .. } => FaceClassification::Inside,
        PointClassification::Outside { .. } => FaceClassification::Outside,
        PointClassification::OnBoundary(_) => FaceClassification::OnBoundary,
    }
}

/// Extract the escalation from a PointClassification, if any.
fn extract_escalation(
    pc: &PointClassification,
) -> Option<forge_math::arithmetic::precision::PrecisionEscalation> {
    match pc {
        PointClassification::Inside { escalation } => escalation.clone(),
        PointClassification::Outside { escalation } => escalation.clone(),
        PointClassification::OnBoundary(_) => None,
    }
}

/// Look up a vertex position by raw slot index.
fn lookup_vertex_position(
    arena: &TopologyArena,
    geometry: &GeometryState,
    index: u32,
) -> Result<[f64; 3], KernelError> {
    let gen = arena
        .vertex_generation(index as usize)
        .ok_or_else(|| KernelError::InvalidInput {
            message: format!("No active vertex at slot index {}", index),
            context: None,
        })?;
    let vid = forge_topo::handles::VertexId::from_raw_parts(index, gen);
    geometry
        .get_vertex_position(vid)
        .copied()
        .ok_or_else(|| KernelError::InvalidInput {
            message: format!("No position for vertex {}", index),
            context: None,
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
        DecisionId(face_id.index() as u64),
        kind,
        tier,
        1.0,
        DecisionContext::Classification {
            point: [0.0; 3],
            result: format!("{}:Face#{} → {}", origin_label, face_id.index(), label),
        },
    );
    decision.set_entity_scope(EntityRef::new(
        forge_core::EntityKind::Face,
        face_id.index(),
    ));
    decision.set_topology_delta(TopologyDelta::default());
    ctx.get_decision_log_mut().record(decision);
}

// ── Spatial indexing ─────────────────────────────────────────────────────────

/// Build BVH spatial index for the "other" solid.
fn build_spatial_index(
    arena: &TopologyArena,
    geometry: &GeometryState,
) -> Option<Box<BvhNode<FaceId>>> {
    let face_aabbs = all_face_bounds(arena, &|vid| geometry.get_vertex_position(vid).copied()).ok();
    face_aabbs.and_then(|aabbs| BvhNode::build(aabbs))
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Check whether two faces have aligned normals.
fn check_normal_alignment(
    source_geom: &GeometryState,
    source_face: FaceId,
    other_geom: &GeometryState,
    other_face: FaceId,
) -> bool {
    match (
        source_geom.get_face_plane(source_face),
        other_geom.get_face_plane(other_face),
    ) {
        (Some(sp), Some(op)) => forge_geom::primitives::plane::normals_aligned_exact(sp, op),
        _ => true,
    }
}

/// Compute a scale-aware ray extent from the bounding box of a solid.
///
/// Returns `max(10 * diagonal, default_extent)`.
fn compute_ray_extent_from_bbox(
    arena: &TopologyArena,
    geometry: &GeometryState,
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
        FaceClassification::Ambiguous => "Ambiguous",
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
