//! Face classification step — classify all faces of one solid relative to another.
//!
//! DOMAIN: For each face of the `source` solid, determines whether it lies
//! Inside, Outside, OnBoundary, or OppositeBoundary of the `other` solid.
//! Records a `TracedDecision` per face and applies counterfactual overrides (P3.3).
//!
//! CONSUMERS: boolean (RayCastClassifier), future: fillet (tool selection),
//!            shell (inner/outer face assignment)
//!
//! ALGORITHM:
//! 1. Build BVH spatial index on the `other` solid.
//! 2. For each face, compute centroid via `shared_ops::centroid`.
//! 3. Ray-cast from centroid → primary classification.
//! 4. Multi-sample fallback for intersection-derived faces (boundary landing).
//! 5. Perturbation fallback if primary is OnBoundary.
//! 6. Apply P3.3 counterfactual override if present.
//! 7. Log TracedDecision.

use forge_core::tracing::TopologyDelta;
use forge_core::KernelError;
use forge_core::{
    DecisionContext, DecisionId, DecisionKind, DecisionTier, EntityRef, ToleranceProvider,
    TracedDecision,
};
use forge_topo::arena::TopologyArena;
use forge_topo::handles::FaceId;

use crate::core::ModelingContext;
use crate::geom_facade::BvhNode;
use crate::geometry_state::GeometryState;
use crate::operations::boolean::classify_schema::{ClassifiedFace, FaceClassification, FaceOrigin};

/// True when this face was created by a `make_edge_face` Euler operator
/// during the Boolean split phase.
fn is_intersection_face(arena: &TopologyArena, face_id: FaceId) -> bool {
    let Some(face) = arena.get_face(face_id).ok() else { return false };
    let Some(lineage) = face.lineage() else { return false };
    lineage.get_creation_op().get_name().starts_with("make_edge_face")
}
use crate::shared_ops::vertex::centroid::compute_face_centroid;
use crate::shared_ops::spatial::normal_alignment::faces_have_aligned_normals;
use crate::shared_ops::vertex::lookup::lookup_vertex_position_by_slot;
use crate::spatial::{
    all_face_bounds, classify_point_in_solid, classify_point_with_perturbation,
    face_interior_samples, FacePointClassification, PointClassification, SpatialAccelerator,
    classify_point_on_face,
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
    let point_coincidence_tol = ctx.get_tolerance().get_spatial_tolerance();

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
            point_coincidence_tol,
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
fn classify_single_face(
    source_arena: &TopologyArena,
    source_geometry: &GeometryState,
    other_arena: &TopologyArena,
    other_geometry: &GeometryState,
    accelerator: Option<&dyn SpatialAccelerator>,
    face_id: FaceId,
    config: &crate::core::ToleranceConfig,
    point_coincidence_tol: f64,
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

    let pos_fn = |idx: u32| lookup_vertex_position_by_slot(other_arena, other_geometry, idx);

    let primary = classify_point_in_solid(
        other_arena,
        &pos_fn,
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
        config,
        point_coincidence_tol,
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
    config: &crate::core::ToleranceConfig,
    point_coincidence_tol: f64,
) -> Result<PointClassification, KernelError> {
    if matches!(primary, PointClassification::OnBoundary(_)) {
        return Ok(primary);
    }
    if !is_intersection_face(source_arena, face_id) {
        return Ok(primary);
    }

    let pos_fn_src = |v: forge_topo::handles::VertexId| source_geometry.get_vertex_position(v).copied();
    let samples = face_interior_samples(
        source_arena,
        &pos_fn_src,
        face_id,
        centroid,
        source_geometry as &dyn ToleranceProvider,
        point_coincidence_tol,
    )?;

    if samples.len() <= 1 {
        return Ok(primary);
    }

    let pos_fn_other = |idx: u32| lookup_vertex_position_by_slot(other_arena, other_geometry, idx);
    let mut inside = 0usize;
    let mut outside = 0usize;
    let mut first_boundary: Option<FaceId> = None;

    for p in samples {
        let cls = classify_point_in_solid(
            other_arena,
            &pos_fn_other,
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
            let aligned = faces_have_aligned_normals(source_geom, source_face, other_geom, boundary_face);
            let class = if aligned {
                FaceClassification::OnBoundary
            } else {
                FaceClassification::OppositeBoundary
            };
            (class, None)
        }
    }
}

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
    let pos_fn = |idx: u32| lookup_vertex_position_by_slot(other_arena, other_geometry, idx);

    let perturbed = classify_point_with_perturbation(
        other_arena,
        &pos_fn,
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

fn to_face_classification(pc: &PointClassification) -> FaceClassification {
    match pc {
        PointClassification::Inside { .. } => FaceClassification::Inside,
        PointClassification::Outside { .. } => FaceClassification::Outside,
        PointClassification::OnBoundary(_) => FaceClassification::OnBoundary,
    }
}

fn extract_escalation(
    pc: &PointClassification,
) -> Option<forge_math::arithmetic::precision::PrecisionEscalation> {
    match pc {
        PointClassification::Inside { escalation } => escalation.clone(),
        PointClassification::Outside { escalation } => escalation.clone(),
        PointClassification::OnBoundary(_) => None,
    }
}

// ── Override and logging ─────────────────────────────────────────────────────

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

fn build_spatial_index(
    arena: &TopologyArena,
    geometry: &GeometryState,
) -> Option<Box<BvhNode<FaceId>>> {
    let face_aabbs = all_face_bounds(arena, &|vid| geometry.get_vertex_position(vid).copied()).ok();
    face_aabbs.and_then(|aabbs| BvhNode::build(aabbs))
}

// ── Label helpers ────────────────────────────────────────────────────────────

fn classification_label(class: &FaceClassification) -> &'static str {
    match class {
        FaceClassification::Inside => "Inside",
        FaceClassification::Outside => "Outside",
        FaceClassification::Ambiguous => "Ambiguous",
        FaceClassification::OnBoundary => "OnBoundary(aligned)",
        FaceClassification::OppositeBoundary => "OppositeBoundary(opposed)",
    }
}

fn origin_to_label(origin: FaceOrigin) -> &'static str {
    match origin {
        FaceOrigin::Target => "Target",
        FaceOrigin::Tool => "Tool",
    }
}
