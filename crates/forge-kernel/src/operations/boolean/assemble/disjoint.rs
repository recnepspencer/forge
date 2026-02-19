//! Handling for disjoint and contained solids (zero-split fast path).

use forge_core::KernelError;
use forge_core::result::{
    TracedDecision, DecisionId, DecisionKind, DecisionContext, DecisionTier, EntityRef
};
use forge_topo::state::TopologyState;
use forge_topo::handles::{FaceId, HalfEdgeId};
use forge_topo::classify::classify_point_in_solid;
use forge_topo::replay::ReplayLog;
use forge_topo::lineage::{LineageEvent, EntityKind, Lineage, OpSignature};

use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;
use crate::operations::boolean::schema::{BooleanResult, BooleanOp};
use super::copy::{copy_faces, VertexDedup};
use super::stitch::stitch_twins;

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
    let config = ctx.get_tolerance_config().clone();

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
            execute_disjoint_boolean(
                target_topo, target_geom,
                tool_topo, tool_geom,
                operation,
                ctx,
            ).map(Some)
        }
    }
}

/// Classification of how two non-intersecting solids relate.
enum Containment {
    /// Tool is fully inside target (or coincident).
    ToolInsideTarget,
    /// Target is fully inside tool.
    TargetInsideTool,
    /// Solids are disjoint with no boundary contact.
    Disjoint,
    /// Solids share coplanar overlapping faces (flush contact).
    Touching,
}

/// Sample one vertex from each solid and classify against the other
/// to determine containment.
fn check_containment(
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
        None, // spatial_index
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
        None, // spatial_index
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

/// Handle Booleans where tool is contained inside target (or vice versa).
///
/// Face counts in the result must reflect which solid contributed the faces:
/// target faces vs tool faces, not just "primary" vs "secondary".
fn execute_contained_boolean(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    operation: BooleanOp,
    tool_inside_target: bool,
    ctx: &mut ModelingContext,
) -> Result<BooleanResult, KernelError> {
    let target_fc = target_topo.arena().face_count();
    let tool_fc = tool_topo.arena().face_count();

    match (operation, tool_inside_target) {
        // Union with tool inside target: keep outer shell only
        (BooleanOp::Union, true) => {
            let mut r = assemble_complete_shells(
                target_topo, target_geom, None, false, ctx
            )?;
            r.set_face_counts(target_fc, 0);
            Ok(r)
        }
        // Union with target inside tool: keep outer shell only
        (BooleanOp::Union, false) => {
            let mut r = assemble_complete_shells(
                tool_topo, tool_geom, None, false, ctx
            )?;
            r.set_face_counts(0, tool_fc);
            Ok(r)
        }
        // Intersection with tool inside target: keep inner (tool)
        (BooleanOp::Intersection, true) => {
            let mut r = assemble_complete_shells(
                tool_topo, tool_geom, None, false, ctx
            )?;
            r.set_face_counts(0, tool_fc);
            Ok(r)
        }
        // Intersection with target inside tool: keep inner (target)
        (BooleanOp::Intersection, false) => {
            let mut r = assemble_complete_shells(
                target_topo, target_geom, None, false, ctx
            )?;
            r.set_face_counts(target_fc, 0);
            Ok(r)
        }
        // Subtraction with tool inside target: keep both (tool reversed)
        (BooleanOp::Subtraction, true) => {
            if are_solids_coincident(target_topo, target_geom, tool_topo, tool_geom)? {
                let empty_topo = TopologyState::empty();
                let empty_geom = GeometryStore::new();
                return Ok(BooleanResult::new(empty_topo, empty_geom, 0, 0, crate::operations::boolean::schema::BooleanIntrospection::default(), ReplayLog::new(), Vec::new()));
            }
            let mut r = assemble_complete_shells(
                target_topo, target_geom,
                Some((tool_topo, tool_geom)),
                true,
                ctx,
            )?;
            r.set_face_counts(target_fc, tool_fc);
            Ok(r)
        }
        // Subtraction with target inside tool: empty result
        (BooleanOp::Subtraction, false) => {
            let empty_topo = TopologyState::empty();
            let empty_geom = GeometryStore::new();
            Ok(BooleanResult::new(empty_topo, empty_geom, 0, 0, crate::operations::boolean::schema::BooleanIntrospection::default(), ReplayLog::new(), Vec::new()))
        }
    }
}

/// Handle Booleans where the two solids are disjoint.
fn execute_disjoint_boolean(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    operation: BooleanOp,
    ctx: &mut ModelingContext,
) -> Result<BooleanResult, KernelError> {
    match operation {
        BooleanOp::Union => {
            assemble_complete_shells(
                target_topo, target_geom,
                Some((tool_topo, tool_geom)),
                false,
                ctx,
            )
        }
        BooleanOp::Intersection => {
            let empty_topo = TopologyState::empty();
            let empty_geom = GeometryStore::new();
            Ok(BooleanResult::new(empty_topo, empty_geom, 0, 0, crate::operations::boolean::schema::BooleanIntrospection::default(), ReplayLog::new(), Vec::new()))
        }
        BooleanOp::Subtraction => {
            assemble_complete_shells(
                target_topo, target_geom,
                None, false, ctx
            )
        }
    }
}

/// Copy one or two complete solids into a fresh arena.
///
/// Each shell gets its own vertex dedup and twin stitching pass,
/// so they remain independent manifolds. No vertex merging across
/// shells — touching/shared vertices stay separate.
fn assemble_complete_shells(
    primary_topo: &TopologyState,
    primary_geom: &GeometryStore,
    secondary: Option<(&TopologyState, &GeometryStore)>,
    reverse_secondary: bool,
    ctx: &mut ModelingContext,
) -> Result<BooleanResult, KernelError> {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut result_geom = GeometryStore::new();

    let primary_faces: Vec<FaceId> = primary_topo.arena().iter_faces()
        .map(|(fid, _)| fid)
        .collect();
    let primary_count = primary_faces.len();

    let mut primary_he: Vec<HalfEdgeId> = Vec::new();
    let mut primary_dedup = VertexDedup::new();
    let mut primary_vertex_map: std::collections::HashMap<crate::operations::boolean::eval::VertexMatchKey, forge_topo::handles::VertexId> = std::collections::HashMap::new();
    let mut primary_spatial: super::copy::SpatialVertexIndex = super::copy::SpatialVertexIndex::new();

    copy_faces(
        &mut draft, &mut result_geom, &mut primary_dedup,
        &mut primary_he,
        &mut primary_vertex_map,
        &mut primary_spatial,
        primary_topo.arena(), primary_geom, &primary_faces,
        false,
        None,
    )?;

    stitch_twins(&mut draft, &primary_he, &result_geom, ctx)?;

    let mut secondary_count = 0usize;
    if let Some((sec_topo, sec_geom)) = secondary {
        let sec_faces: Vec<FaceId> = sec_topo.arena().iter_faces()
            .map(|(fid, _)| fid)
            .collect();
        secondary_count = sec_faces.len();

        let mut sec_he: Vec<HalfEdgeId> = Vec::new();
        let mut sec_dedup = VertexDedup::new();
        let mut sec_vertex_map: std::collections::HashMap<crate::operations::boolean::eval::VertexMatchKey, forge_topo::handles::VertexId> = std::collections::HashMap::new();
        let mut sec_spatial: super::copy::SpatialVertexIndex = super::copy::SpatialVertexIndex::new();

        copy_faces(
            &mut draft, &mut result_geom, &mut sec_dedup,
            &mut sec_he,
            &mut sec_vertex_map,
            &mut sec_spatial,
            sec_topo.arena(), sec_geom, &sec_faces,
            reverse_secondary,
            None,
        )?;

        stitch_twins(&mut draft, &sec_he, &result_geom, ctx)?;
    }

    let topo = draft.commit()?;
    let lineage_events: Vec<LineageEvent> = topo.arena().iter_faces()
        .map(|(fid, _)| LineageEvent::EntityCreated {
            entity_kind: EntityKind::Face,
            lineage: Lineage::root(fid.index() as u64, OpSignature::new("assemble_complete_shells")),
        })
        .collect();
    Ok(BooleanResult::new(topo, result_geom, primary_count, secondary_count, crate::operations::boolean::schema::BooleanIntrospection::default(), ReplayLog::new(), lineage_events))
}

/// Check whether any face centroid from one solid lies on the boundary
/// of the other solid, or if any face planes are coplanar between the two solids.
/// This detects flush face contact (e.g. two cubes sharing a face or vertex-on-face
/// partial overlap) that isn't caught by centroid-based containment checks.
fn has_overlapping_coplanar_faces(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    config: &crate::core::ToleranceConfig,
) -> Result<bool, KernelError> {
    let coplanar_angle_eps = config.get_coplanar_angle_epsilon();
    let coplanar_offset_eps = config.get_coplanar_offset_epsilon();

    for (face_a, _) in target_topo.arena().iter_faces() {
        let Some(plane_a) = target_geom.get_face_plane(face_a) else { continue };

        for (face_b, _) in tool_topo.arena().iter_faces() {
            let Some(plane_b) = tool_geom.get_face_plane(face_b) else { continue };

            if forge_geom::primitives::plane::is_coplanar(
                plane_a, plane_b,
                coplanar_angle_eps,
                coplanar_offset_eps,
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
fn are_solids_coincident(
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
