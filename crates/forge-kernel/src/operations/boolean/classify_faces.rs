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
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::FaceId;

use crate::configuration::facade::{ResolvedConfig, ToleranceConfig};
use forge_geom::facade::{BvhNode, Plane};
use crate::geometry::facade::GeometryView;
use crate::observability::facade::KernelSpan;
use crate::operations::boolean::counterfactual::CounterfactualOverrides;
use crate::operations::boolean::classify_schema::{ClassifiedFace, FaceClassification, FaceOrigin};

/// True when this face was created by a `make_edge_face` Euler operator
/// during the Boolean split phase.
///
/// TODO(lineage-phase-3): Re-implement once MutableDraft lineage is wired.
fn is_intersection_face(_arena: &TopologyArena, _face_id: FaceId) -> bool {
    // Lineage was stripped from FaceData; this needs MutableDraft-based lookup.
    false
}

use forge_spatial::{
    all_face_bounds, classify_point_in_solid, classify_point_with_perturbation,
    face_interior_samples, PointClassification, SpatialAccelerator,
};

// ── Inlined helpers (formerly in crate::shared_ops, now deleted) ─────────────

/// Compute the centroid of a face by averaging its vertex positions.
fn compute_face_centroid(
    arena: &TopologyArena,
    geom: &impl GeometryView,
    face_id: FaceId,
) -> Option<[f64; 3]> {
    use forge_topo::queries::traverse::FaceAllEdgesIterator;
    let mut sum = [0.0_f64; 3];
    let mut count = 0usize;
    let iter = FaceAllEdgesIterator::new(arena, face_id).ok()?;
    for he_res in iter {
        let he_id = he_res.ok()?;
        let he = arena.get_half_edge(he_id).ok()?;
        if let Some(pos) = geom.get_vertex_position(he.origin()) {
            sum[0] += pos[0];
            sum[1] += pos[1];
            sum[2] += pos[2];
            count += 1;
        }
    }
    if count == 0 { return None; }
    Some([sum[0] / count as f64, sum[1] / count as f64, sum[2] / count as f64])
}

/// Check whether two faces have aligned normals (dot > 0).
fn faces_have_aligned_normals(
    geom_a: &impl GeometryView,
    face_a: FaceId,
    geom_b: &impl GeometryView,
    face_b: FaceId,
) -> bool {
    let (Some(plane_a), Some(plane_b)) = (geom_a.get_face_plane(face_a), geom_b.get_face_plane(face_b)) else {
        return false;
    };
    let na = plane_a.normal();
    let nb = plane_b.normal();
    (na[0] * nb[0] + na[1] * nb[1] + na[2] * nb[2]) > 0.0
}

/// Look up a vertex position by its raw slot index, returning Result for compatibility
/// with forge-spatial's `classify_point_in_solid` callback signature.
fn lookup_vertex_position_result(
    arena: &TopologyArena,
    geom: &impl GeometryView,
    slot: u32,
) -> Result<[f64; 3], forge_core::KernelError> {
    let vid = forge_topo::handles::VertexId::new(slot, 0);
    if arena.get_vertex(vid).is_err() {
        return Err(forge_core::KernelError::InvalidInput {
            message: format!("Vertex slot {} not alive", slot),
            context: None,
        });
    }
    geom.get_vertex_position(vid)
        .copied()
        .ok_or_else(|| forge_core::KernelError::InvalidInput {
            message: format!("Vertex {} has no position binding", slot),
            context: None,
        })
}

/// Classify all faces of one solid relative to the other solid.
pub fn classify_faces(
    source_arena: &TopologyArena,
    source_geometry: &impl GeometryView,
    other_arena: &TopologyArena,
    other_geometry: &impl GeometryView,
    config: &ResolvedConfig,
    origin: FaceOrigin,
    overrides: &CounterfactualOverrides,
) -> Result<Vec<ClassifiedFace>, KernelError> {
    let tolerance = config.tolerance_config();
    let point_coincidence_tol = config.spatial_tolerance();

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
            &tolerance,
            point_coincidence_tol,
        )?;

        if std::env::var("FORGE_DEBUG_CLASSIFY_PROVENANCE")
            .ok()
            .as_deref()
            == Some("1")
        {
            let sample = compute_face_centroid(source_arena, source_geometry, face_id)
                .unwrap_or([f64::NAN; 3]);
            let lineage_str = "no-lineage".to_string();
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

        let (final_class, overridden) = apply_override(overrides, face_id, computed);
        log_classification(face_id, &final_class, &computed, overridden, origin_label);
        classified.push(ClassifiedFace::new(face_id, final_class));
    }

    Ok(classified)
}

// ── Per-face classification ──────────────────────────────────────────────────

/// Classify a single face by ray-casting its centroid into the other solid.
fn classify_single_face(
    source_arena: &TopologyArena,
    source_geometry: &impl GeometryView,
    other_arena: &TopologyArena,
    other_geometry: &impl GeometryView,
    accelerator: Option<&dyn SpatialAccelerator>,
    face_id: FaceId,
    config: &ToleranceConfig,
    point_coincidence_tol: f64,
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

    let pos_fn = |idx: u32| lookup_vertex_position_result(other_arena, other_geometry, idx);

    let tol_provider = crate::geometry::facade::GeometryToleranceProvider::new(other_geometry);
    let primary = classify_point_in_solid(
        other_arena,
        &pos_fn,
        accelerator,
        &sample,
        &tol_provider,
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
        log_escalation(face_id, esc);
    }

    Ok(class)
}

fn maybe_multisample_refine(
    source_arena: &TopologyArena,
    source_geometry: &impl GeometryView,
    other_arena: &TopologyArena,
    other_geometry: &impl GeometryView,
    accelerator: Option<&dyn SpatialAccelerator>,
    face_id: FaceId,
    centroid: [f64; 3],
    primary: PointClassification,
    _config: &ToleranceConfig,
    point_coincidence_tol: f64,
) -> Result<PointClassification, KernelError> {
    if matches!(primary, PointClassification::OnBoundary(_)) {
        return Ok(primary);
    }
    if !is_intersection_face(source_arena, face_id) {
        return Ok(primary);
    }

    let tol_provider_src = crate::geometry::facade::GeometryToleranceProvider::new(source_geometry);
    let pos_fn_src = |v: forge_topo::handles::VertexId| source_geometry.get_vertex_position(v).copied();
    let samples = face_interior_samples(
        source_arena,
        &pos_fn_src,
        face_id,
        centroid,
        &tol_provider_src,
        point_coincidence_tol,
    )?;

    if samples.len() <= 1 {
        return Ok(primary);
    }

    let pos_fn_other = |idx: u32| lookup_vertex_position_result(other_arena, other_geometry, idx);
    let tol_provider_other = crate::geometry::facade::GeometryToleranceProvider::new(other_geometry);
    let mut inside = 0usize;
    let mut outside = 0usize;
    let mut first_boundary: Option<FaceId> = None;

    for p in samples {
        let cls = classify_point_in_solid(
            other_arena,
            &pos_fn_other,
            accelerator,
            &p,
            &tol_provider_other,
        )?;
        match cls {
            PointClassification::Inside { escalation: _ } => inside += 1,
            PointClassification::Outside { escalation: _ } => outside += 1,
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
    source_geom: &impl GeometryView,
    source_face: FaceId,
    other_geom: &impl GeometryView,
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
    source_geometry: &impl GeometryView,
    source_face: FaceId,
    other_arena: &TopologyArena,
    other_geometry: &impl GeometryView,
    accelerator: Option<&dyn SpatialAccelerator>,
    original: &PointClassification,
    config: &ToleranceConfig,
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
    let pos_fn = |idx: u32| lookup_vertex_position_result(other_arena, other_geometry, idx);

    let tol_provider = crate::geometry::facade::GeometryToleranceProvider::new(other_geometry);
    let perturbed = classify_point_with_perturbation(
        other_arena,
        &pos_fn,
        accelerator,
        &centroid,
        normal,
        epsilon,
        &tol_provider,
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
        PointClassification::Inside { escalation: _ } => FaceClassification::Inside,
        PointClassification::Outside { escalation: _ } => FaceClassification::Outside,
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
    overrides: &CounterfactualOverrides,
    face_id: FaceId,
    computed: FaceClassification,
) -> (FaceClassification, bool) {
    let decision_id = DecisionId(face_id.index() as u64);
    match overrides.get(decision_id) {
        Some(forced) => (forced, true),
        None => (computed, false),
    }
}

fn log_classification(
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
    KernelSpan::record_decision(decision);
}

fn log_escalation(
    face_id: FaceId,
    escalation: forge_math::arithmetic::precision::PrecisionEscalation,
) {
    if escalation.resolved_at <= forge_math::arithmetic::precision::PrecisionMode::Float64 {
        return;
    }

    let decision = TracedDecision::new(
        DecisionId((face_id.index() as u64) << 32 | 0xE5C4_1A7E),
        DecisionKind::Exact,
        DecisionTier::Escalated,
        escalation.disagreement_magnitude.unwrap_or(0.0),
        DecisionContext::PrecisionEscalation { escalation },
    );
    KernelSpan::record_decision(decision);
}

// ── Spatial indexing ─────────────────────────────────────────────────────────

fn build_spatial_index(
    arena: &TopologyArena,
    geometry: &impl GeometryView,
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
