//! Containment classification and zero-split dispatch.
//!
//! DOMAIN: When the split phase produces zero face splits, classify the
//! spatial relationship between two non-intersecting solids and dispatch
//! to the appropriate assembly function.
//!
//! DEPENDENCIES: classify (point-in-solid), assemble (shell assembly).
//! INVARIANTS: Always records a TracedDecision for the containment result.

use forge_core::KernelError;
use forge_core::{
    TracedDecision, DecisionId, DecisionKind, DecisionContext, DecisionTier,
};
use forge_topo::state::TopologyState;
use forge_topo::classify::classify_point_in_solid;

use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;
use crate::operations::boolean::schema::{BooleanOp, BooleanResult};

use super::assemble::{
    execute_contained_boolean, execute_disjoint_boolean,
    execute_touching_boolean,
};

/// Classification of how two non-intersecting solids relate.
pub(super) enum Containment {
    /// Tool is fully inside target (or coincident).
    ToolInsideTarget,
    /// Target is fully inside tool.
    TargetInsideTool,
    /// Solids are disjoint with no boundary contact.
    Disjoint,
    /// Solids share coplanar overlapping faces (flush contact).
    Touching,
}

/// Fast path for non-intersecting solids (zero face splits).
pub fn execute_zero_split(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    operation: BooleanOp,
    ctx: &mut ModelingContext,
) -> Result<Option<BooleanResult>, KernelError> {
    let containment = check_containment(target_topo, target_geom, tool_topo, tool_geom, ctx)?;
    log_containment(&containment, operation, ctx);
    dispatch_containment(containment, target_topo, target_geom, tool_topo, tool_geom, operation, ctx)
}

// ── Containment classification ───────────────────────────────────────────────

/// Determine the spatial relationship between two solids by sampling.
pub(super) fn check_containment(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    ctx: &mut ModelingContext,
) -> Result<Containment, KernelError> {
    let config = ctx.get_tolerance_config().clone();

    let tool_sample = sample_interior_point(tool_topo, tool_geom)?;
    let tool_in_target = classify_sample(
        &tool_sample, target_topo, target_geom, &config, ctx,
    )?;

    if is_inside(&tool_in_target) {
        return Ok(Containment::ToolInsideTarget);
    }

    let target_sample = sample_interior_point(target_topo, target_geom)?;
    let target_in_tool = classify_sample(
        &target_sample, tool_topo, tool_geom, &config, ctx,
    )?;

    if is_inside(&target_in_tool) {
        return Ok(Containment::TargetInsideTool);
    }

    if is_on_boundary(&tool_in_target) || is_on_boundary(&target_in_tool) {
        return Ok(Containment::Touching);
    }

    if has_overlapping_coplanar_faces(target_topo, target_geom, tool_topo, tool_geom, &config)? {
        return Ok(Containment::Touching);
    }

    Ok(Containment::Disjoint)
}

/// Classify a sample point against a solid, logging any precision escalation.
fn classify_sample(
    point: &[f64; 3],
    topo: &TopologyState,
    geom: &GeometryStore,
    config: &crate::core::ToleranceConfig,
    ctx: &mut ModelingContext,
) -> Result<forge_topo::classify::PointClassification, KernelError> {
    let result = classify_point_in_solid(
        topo.arena(),
        &|index| lookup_vertex(topo, geom, index),
        None,
        point,
        config.get_ray_extent(),
        config.get_edge_split_degeneracy(),
    )?;

    if let Some(esc) = extract_escalation(&result) {
        ctx.log_escalation(esc);
    }

    Ok(result)
}

/// Look up a vertex position by raw slot index.
fn lookup_vertex(
    topo: &TopologyState,
    geom: &GeometryStore,
    index: u32,
) -> Result<[f64; 3], KernelError> {
    let gen = topo.arena().vertex_generation(index as usize).ok_or_else(|| {
        KernelError::InvalidInput {
            message: format!("No active vertex at slot index {}", index), context: None,
        }
    })?;
    let vid = forge_topo::handles::VertexId::from_raw_parts(index, gen);
    geom.get_vertex_position(vid).copied().ok_or_else(|| {
        KernelError::InvalidInput {
            message: format!("No position for vertex {}", index), context: None,
        }
    })
}

/// Extract a precision escalation from a point classification result.
fn extract_escalation(
    cls: &forge_topo::classify::PointClassification,
) -> Option<forge_math::arithmetic::precision::PrecisionEscalation> {
    match cls {
        forge_topo::classify::PointClassification::Inside { escalation } => escalation.clone(),
        forge_topo::classify::PointClassification::Outside { escalation } => escalation.clone(),
        _ => None,
    }
}

/// Check if a classification result is Inside.
fn is_inside(cls: &forge_topo::classify::PointClassification) -> bool {
    matches!(cls, forge_topo::classify::PointClassification::Inside { .. })
}

/// Check if a classification result is OnBoundary.
fn is_on_boundary(cls: &forge_topo::classify::PointClassification) -> bool {
    matches!(cls, forge_topo::classify::PointClassification::OnBoundary(_))
}

/// Sample a point inside a solid by averaging all vertex positions.
fn sample_interior_point(
    topo: &TopologyState,
    geom: &GeometryStore,
) -> Result<[f64; 3], KernelError> {
    let vertices: Vec<[f64; 3]> = topo.arena().iter_vertices()
        .filter_map(|(vid, _)| geom.get_vertex_position(vid).copied())
        .collect();

    forge_geom::primitives::polygon::compute_polygon_centroid(&vertices).ok_or_else(|| {
        KernelError::InvalidInput {
            message: "No vertices with positions".to_string(), context: None,
        }
    })
}

// ── Coplanar overlap detection ───────────────────────────────────────────────

/// Check whether any faces between the two solids are coplanar or
/// any face centroid lies on the other solid's boundary.
fn has_overlapping_coplanar_faces(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    config: &crate::core::ToleranceConfig,
) -> Result<bool, KernelError> {
    if has_coplanar_plane_pair(target_topo, target_geom, tool_topo, tool_geom) {
        return Ok(true);
    }

    has_boundary_centroid(target_topo, target_geom, tool_topo, tool_geom, config)
}

/// Check if any face plane from target exactly matches any from tool.
fn has_coplanar_plane_pair(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
) -> bool {
    for (face_a, _) in target_topo.arena().iter_faces() {
        let Some(plane_a) = target_geom.get_face_plane(face_a) else { continue };
        for (face_b, _) in tool_topo.arena().iter_faces() {
            let Some(plane_b) = tool_geom.get_face_plane(face_b) else { continue };
            if forge_geom::primitives::plane::exact_eq(plane_a, plane_b) {
                return true;
            }
        }
    }
    false
}

/// Check if any target face centroid lies on the tool's boundary.
fn has_boundary_centroid(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    config: &crate::core::ToleranceConfig,
) -> Result<bool, KernelError> {
    for (face_id, _) in target_topo.arena().iter_faces() {
        let centroid = crate::operations::boolean::eval::compute_face_centroid(
            target_topo.arena(), target_geom, face_id,
        )?;

        let class = classify_point_in_solid(
            tool_topo.arena(),
            &|index| lookup_vertex(tool_topo, tool_geom, index),
            None,
            &centroid,
            config.get_ray_extent(),
            config.get_edge_split_degeneracy(),
        )?;

        if is_on_boundary(&class) {
            return Ok(true);
        }
    }
    Ok(false)
}

// ── Dispatch and logging ─────────────────────────────────────────────────────

/// Dispatch to the appropriate assembly function based on containment.
///
/// Delegates to specialised assembly paths per containment class.
/// Contained subtraction uses in-place splice to create an inner cavity.
fn dispatch_containment(
    containment: Containment,
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    operation: BooleanOp,
    ctx: &mut ModelingContext,
) -> Result<Option<BooleanResult>, KernelError> {
    match containment {
        Containment::ToolInsideTarget => execute_contained_boolean(
            target_topo, target_geom, tool_topo, tool_geom, operation, true, ctx,
        ).map(Some),
        Containment::TargetInsideTool => execute_contained_boolean(
            target_topo, target_geom, tool_topo, tool_geom, operation, false, ctx,
        ).map(Some),
        Containment::Disjoint => execute_disjoint_boolean(
            target_topo, target_geom, tool_topo, tool_geom, operation, ctx,
        ).map(Some),
        Containment::Touching => execute_touching_boolean(
            target_topo, target_geom, tool_topo, tool_geom, operation, ctx,
        ).map(Some),
    }
}

/// Record a TracedDecision for the containment classification.
fn log_containment(containment: &Containment, operation: BooleanOp, ctx: &mut ModelingContext) {
    let label = match containment {
        Containment::ToolInsideTarget => "ToolInsideTarget",
        Containment::TargetInsideTool => "TargetInsideTool",
        Containment::Disjoint => "Disjoint",
        Containment::Touching => "Touching",
    };
    ctx.get_decision_log_mut().record(TracedDecision::new(
        DecisionId(1),
        DecisionKind::Exact,
        DecisionTier::Deterministic, 1.0,
        DecisionContext::Degeneracy {
            description: format!("Zero-split containment: {:?} (op={:?})", label, operation),
        },
    ));
}

/// Check whether two solids are coincident (all tool faces on
/// the target boundary). Used to detect A−A=∅ in subtraction.
pub(super) fn are_solids_coincident(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
) -> Result<bool, KernelError> {
    if target_topo.arena().face_count() != tool_topo.arena().face_count() {
        return Ok(false);
    }

    let config = crate::core::ToleranceConfig::default();

    for (face_id, _) in tool_topo.arena().iter_faces() {
        let centroid = crate::operations::boolean::eval::compute_face_centroid(
            tool_topo.arena(), tool_geom, face_id,
        )?;

        let class = classify_point_in_solid(
            target_topo.arena(),
            &|index| lookup_vertex(target_topo, target_geom, index),
            None,
            &centroid,
            config.get_ray_extent(),
            config.get_edge_split_degeneracy(),
        )?;

        if !is_on_boundary(&class) {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Compute the characteristic scale for the disjoint assembly path.
pub fn compute_disjoint_scale(
    primary_arena: &forge_topo::arena::TopologyArena,
    primary_geom: &GeometryStore,
    secondary: Option<(&forge_topo::arena::TopologyArena, &GeometryStore)>,
) -> f64 {
    let mut min_pos = [f64::INFINITY; 3];
    let mut max_pos = [f64::NEG_INFINITY; 3];

    for (vid, _) in primary_arena.iter_vertices() {
        if let Some(pos) = primary_geom.get_vertex_position(vid) {
            min_pos = forge_math::linalg::component_min(min_pos, *pos);
            max_pos = forge_math::linalg::component_max(max_pos, *pos);
        }
    }

    if let Some((sec_arena, sec_geom)) = secondary {
        for (vid, _) in sec_arena.iter_vertices() {
            if let Some(pos) = sec_geom.get_vertex_position(vid) {
                min_pos = forge_math::linalg::component_min(min_pos, *pos);
                max_pos = forge_math::linalg::component_max(max_pos, *pos);
            }
        }
    }

    forge_math::linalg::norm(forge_math::linalg::sub(max_pos, min_pos)).max(1e-15)
}
