//! Containment classification and detection logic for zero-split path.

use forge_core::KernelError;
use forge_core::result::{
    TracedDecision, DecisionId, DecisionKind, DecisionContext, DecisionTier,
};
use forge_topo::state::TopologyState;
use forge_topo::handles::FaceId;
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
///
/// When the split phase produces zero splits, the two solids have no
/// volumetric intersection. They may be disjoint, touching at a point/edge,
/// or fully coplanar/identical. Handle each operation type directly:
///
/// - **Union**: copy both complete shells into a new arena
/// - **Intersection**: return empty (no overlap)
/// - **Subtraction**: copy target shell only (tool doesn't cut into it)
pub fn execute_zero_split(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    operation: BooleanOp,
    ctx: &mut ModelingContext,
) -> Result<Option<BooleanResult>, KernelError> {
    let has_containment = check_containment(
        target_topo, target_geom,
        tool_topo, tool_geom,
        ctx,
    )?;

    ctx.get_decision_log_mut().record(TracedDecision::new(
        DecisionId(1),
        DecisionKind::Exact,
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Degeneracy {
            description: format!(
                "Zero-split containment: {:?} (op={:?})",
                match &has_containment {
                    Containment::ToolInsideTarget => "ToolInsideTarget",
                    Containment::TargetInsideTool => "TargetInsideTool",
                    Containment::Disjoint => "Disjoint",
                    Containment::Touching => "Touching",
                },
                operation,
            ),
        },
    ));

    match has_containment {
        Containment::ToolInsideTarget => {
            execute_contained_boolean(
                target_topo, target_geom,
                tool_topo, tool_geom,
                operation, true, ctx
            ).map(Some)
        }
        Containment::TargetInsideTool => {
            execute_contained_boolean(
                target_topo, target_geom,
                tool_topo, tool_geom,
                operation, false, ctx
            ).map(Some)
        }
        Containment::Disjoint => {
            execute_disjoint_boolean(
                target_topo, target_geom,
                tool_topo, tool_geom,
                operation,
                ctx,
            ).map(Some)
        }
        Containment::Touching => {
            execute_touching_boolean(
                target_topo, target_geom,
                tool_topo, tool_geom,
                operation,
                ctx,
            ).map(Some)
        }
    }
}

/// Sample one vertex from each solid and classify against the other
/// to determine containment.
pub(super) fn check_containment(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    ctx: &mut ModelingContext,
) -> Result<Containment, KernelError> {
    let config = ctx.get_tolerance_config().clone();
    let tool_sample = sample_interior_point(tool_topo, tool_geom, &config)?;

    let vertex_lookup_target = |index: u32| -> Result<[f64; 3], KernelError> {
        let gen = target_topo.arena().vertex_generation(index as usize).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No active vertex at slot index {}", index),
                context: None,
            }
        })?;
        let vid = forge_topo::handles::VertexId::from_raw_parts(index, gen);
        target_geom.get_vertex_position(vid).copied().ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No position for vertex {}", index),
                context: None,
            }
        })
    };

    let tool_in_target = classify_point_in_solid(
        target_topo.arena(),
        &vertex_lookup_target,
        None,
        &tool_sample,
        config.get_ray_extent(),
        config.get_edge_split_degeneracy(),
    )?;

    let extract_esc = |cls: &forge_topo::classify::PointClassification| -> Option<forge_math::arithmetic::filter::PrecisionEscalation> {
        match cls {
            forge_topo::classify::PointClassification::Inside { escalation } => escalation.clone(),
            forge_topo::classify::PointClassification::Outside { escalation } => escalation.clone(),
            _ => None,
        }
    };

    eprintln!("[CONTAINMENT] tool_sample={:?} tool_in_target={:?}", tool_sample, tool_in_target);

    if let Some(escalation) = extract_esc(&tool_in_target) {
        ctx.log_escalation(escalation);
    }

    if matches!(tool_in_target, forge_topo::classify::PointClassification::Inside { .. }) {
        return Ok(Containment::ToolInsideTarget);
    }

    let target_sample = sample_interior_point(target_topo, target_geom, &config)?;

    let vertex_lookup_tool = |index: u32| -> Result<[f64; 3], KernelError> {
        let gen = tool_topo.arena().vertex_generation(index as usize).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No active vertex at slot index {}", index),
                context: None,
            }
        })?;
        let vid = forge_topo::handles::VertexId::from_raw_parts(index, gen);
        tool_geom.get_vertex_position(vid).copied().ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No position for vertex {}", index),
                context: None,
            }
        })
    };

    let target_in_tool = classify_point_in_solid(
        tool_topo.arena(),
        &vertex_lookup_tool,
        None,
        &target_sample,
        config.get_ray_extent(),
        config.get_edge_split_degeneracy(),
    )?;

    if let Some(escalation) = extract_esc(&target_in_tool) {
        ctx.log_escalation(escalation);
    }

    if matches!(target_in_tool, forge_topo::classify::PointClassification::Inside { .. }) {
        return Ok(Containment::TargetInsideTool);
    }

    if matches!(tool_in_target, forge_topo::classify::PointClassification::OnBoundary(_))
        || matches!(target_in_tool, forge_topo::classify::PointClassification::OnBoundary(_))
    {
        return Ok(Containment::Touching);
    }

    if has_overlapping_coplanar_faces(
        target_topo, target_geom,
        tool_topo, tool_geom,
        &config,
    )? {
        return Ok(Containment::Touching);
    }

    Ok(Containment::Disjoint)
}

/// Sample a point strictly inside a solid by averaging all vertex positions
/// (the centroid of a convex solid is always interior).
fn sample_interior_point(
    topo: &TopologyState,
    geom: &GeometryStore,
    _config: &crate::core::ToleranceConfig,
) -> Result<[f64; 3], KernelError> {
    let mut vertices = Vec::new();
    for (vid, _) in topo.arena().iter_vertices() {
        if let Some(pos) = geom.get_vertex_position(vid) {
            vertices.push(*pos);
        }
    }

    forge_geom::primitives::polygon::compute_polygon_centroid(&vertices).ok_or_else(|| {
        KernelError::InvalidInput {
            message: "No vertices with positions".to_string(),
            context: None,
        }
    })
}

/// Check whether any face centroid from one solid lies on the boundary
/// of the other solid, or if any face planes are coplanar between the two solids.
fn has_overlapping_coplanar_faces(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    config: &crate::core::ToleranceConfig,
) -> Result<bool, KernelError> {
    for (face_a, _) in target_topo.arena().iter_faces() {
        let Some(plane_a) = target_geom.get_face_plane(face_a) else { continue };

        for (face_b, _) in tool_topo.arena().iter_faces() {
            let Some(plane_b) = tool_geom.get_face_plane(face_b) else { continue };

            if forge_geom::primitives::plane::exact_eq(
                plane_a, plane_b,
            ) {
                return Ok(true);
            }
        }
    }

    let vertex_lookup_tool = |index: u32| -> Result<[f64; 3], KernelError> {
        let gen = tool_topo.arena().vertex_generation(index as usize).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No active vertex at slot index {}", index),
                context: None,
            }
        })?;
        let vid = forge_topo::handles::VertexId::from_raw_parts(index, gen);
        tool_geom.get_vertex_position(vid).copied().ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No position for vertex {}", index),
                context: None,
            }
        })
    };

    for (face_id, _) in target_topo.arena().iter_faces() {
        let centroid = crate::operations::boolean::eval::compute_face_centroid(
            target_topo.arena(), target_geom, face_id,
        )?;

        let class = classify_point_in_solid(
            tool_topo.arena(),
            &vertex_lookup_tool,
            None,
            &centroid,
            config.get_ray_extent(),
                config.get_edge_split_degeneracy(),
        )?;

        if matches!(class, forge_topo::classify::PointClassification::OnBoundary(_)) {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Check whether two solids are coincident (all faces of each are on the
/// boundary of the other). Used to detect A-A=∅ in subtraction.
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

    let vertex_lookup_target = |index: u32| -> Result<[f64; 3], KernelError> {
        let gen = target_topo.arena().vertex_generation(index as usize).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No active vertex at slot index {}", index),
                context: None,
            }
        })?;
        let vid = forge_topo::handles::VertexId::from_raw_parts(index, gen);
        target_geom.get_vertex_position(vid).copied().ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No position for vertex {}", index),
                context: None,
            }
        })
    };

    for (face_id, _) in tool_topo.arena().iter_faces() {
        let centroid = crate::operations::boolean::eval::compute_face_centroid(
            tool_topo.arena(), tool_geom, face_id,
        )?;

        let class = classify_point_in_solid(
            target_topo.arena(),
            &vertex_lookup_target,
            None,
            &centroid,
            config.get_ray_extent(),
                config.get_edge_split_degeneracy(),
        )?;

        if !matches!(class, forge_topo::classify::PointClassification::OnBoundary(_)) {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Compute the characteristic scale for the disjoint assembly path.
///
/// Takes the primary arena and optionally a secondary arena, returns the
/// bounding box diagonal (floored at 1e-15).
pub(super) fn compute_disjoint_scale(
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

    let diagonal = forge_math::linalg::norm(forge_math::linalg::sub(max_pos, min_pos));

    diagonal.max(1e-15)
}
