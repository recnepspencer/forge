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
    TracedDecision, DecisionId, DecisionKind, DecisionContext, DecisionTier, ToleranceProvider,
};
use forge_topo::state::TopologyState;

use crate::spatial::{classify_point_in_solid, PointClassification};

use crate::core::ModelingContext;
use crate::geometry_state::GeometryState;
use crate::operations::boolean::schema::BooleanOp;
use crate::operations::boolean::result::BooleanResult;

use super::assemble::{
    execute_contained_boolean, execute_disjoint_boolean, execute_touching_boolean,
};
use crate::shared_ops::equivalence::{are_solids_coincident, has_boundary_centroid};
use crate::spatial::{combined_solid_scale, compute_solid_centroid};

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
    target_geom: &GeometryState,
    tool_topo: &TopologyState,
    tool_geom: &GeometryState,
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
    target_geom: &GeometryState,
    tool_topo: &TopologyState,
    tool_geom: &GeometryState,
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
    geom: &GeometryState,
    _config: &crate::core::ToleranceConfig,
    ctx: &mut ModelingContext,
) -> Result<PointClassification, KernelError> {
    let result = classify_point_in_solid(
        topo.arena(),
        &|index| lookup_vertex(topo, geom, index),
        None,
        point,
        geom as &dyn ToleranceProvider,
    )?;

    if let Some(esc) = extract_escalation(&result) {
        ctx.log_escalation(esc);
    }

    Ok(result)
}

/// Look up a vertex position by raw slot index.
fn lookup_vertex(
    topo: &TopologyState,
    geom: &GeometryState,
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
    cls: &PointClassification,
) -> Option<forge_math::arithmetic::precision::PrecisionEscalation> {
    match cls {
        PointClassification::Inside { escalation } => escalation.clone(),
        PointClassification::Outside { escalation } => escalation.clone(),
        _ => None,
    }
}

/// Check if a classification result is Inside.
fn is_inside(cls: &PointClassification) -> bool {
    matches!(cls, PointClassification::Inside { .. })
}

/// Check if a classification result is OnBoundary.
fn is_on_boundary(cls: &PointClassification) -> bool {
    matches!(cls, PointClassification::OnBoundary(_))
}

/// Sample a point inside a solid by averaging all vertex positions.
fn sample_interior_point(
    topo: &TopologyState,
    geom: &GeometryState,
) -> Result<[f64; 3], KernelError> {
    compute_solid_centroid(topo.arena(), &|vid| geom.get_vertex_position(vid).copied())
}

// ── Coplanar overlap detection ───────────────────────────────────────────────

/// Check whether any faces between the two solids are coplanar or
/// any face centroid lies on the other solid's boundary.
fn has_overlapping_coplanar_faces(
    target_topo: &TopologyState,
    target_geom: &GeometryState,
    tool_topo: &TopologyState,
    tool_geom: &GeometryState,
    _config: &crate::core::ToleranceConfig,
) -> Result<bool, KernelError> {
    // 1. Fast BVH check for exact rational coplanarity
    let coincidence_graph = crate::shared_ops::coincidence::build_face_coincidence_prepass(
        target_topo.arena(), target_geom,
        tool_topo.arena(), tool_geom
    );
    if !coincidence_graph.is_empty() {
        return Ok(true);
    }

    // 2. Slower sample check for intersecting boundaries
    has_boundary_centroid(target_topo.arena(), target_geom, tool_topo.arena(), tool_geom)
}

// Old implementations of has_coplanar_plane_pair, are_solids_coincident,
// and has_boundary_centroid were deleted here.

// ── Dispatch and logging ─────────────────────────────────────────────────────

/// Dispatch to the appropriate assembly function based on containment.
///
/// Delegates to specialised assembly paths per containment class.
/// Contained subtraction uses in-place splice to create an inner cavity.
fn dispatch_containment(
    containment: Containment,
    target_topo: &TopologyState,
    target_geom: &GeometryState,
    tool_topo: &TopologyState,
    tool_geom: &GeometryState,
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

/// Compute the characteristic scale for the disjoint assembly path.
pub fn compute_disjoint_scale(
    primary_arena: &forge_topo::arena::TopologyArena,
    primary_geom: &GeometryState,
    secondary: Option<(&forge_topo::arena::TopologyArena, &GeometryState)>,
) -> f64 {
    let sec = secondary.map(|(a, g)| {
        let cb: &dyn Fn(forge_topo::handles::VertexId) -> Option<[f64; 3]> = &|vid| g.get_vertex_position(vid).copied();
        (a, cb)
    });
    
    match sec {
        Some((a, cb)) => combined_solid_scale(
            primary_arena, &|vid| primary_geom.get_vertex_position(vid).copied(), Some((a, cb))
        ),
        None => combined_solid_scale(
            primary_arena, &|vid| primary_geom.get_vertex_position(vid).copied(), None
        )
    }
}
