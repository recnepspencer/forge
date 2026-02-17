//! Handling for disjoint and contained solids (zero-split fast path).

use forge_core::KernelError;
use forge_topo::state::TopologyState;
use forge_topo::handles::{FaceId, HalfEdgeId};
use forge_topo::classify::classify_point_in_solid;

use crate::geometry_store::GeometryStore;
use crate::boolean::schema::{BooleanResult, BooleanOp};
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
) -> Result<Option<BooleanResult>, KernelError> {
    let config = crate::core::ToleranceConfig::default();

    let has_containment = check_containment(
        target_topo, target_geom,
        tool_topo, tool_geom,
        &config,
    )?;

    match has_containment {
        Containment::ToolInsideTarget => {
            execute_contained_boolean(
                target_topo, target_geom,
                tool_topo, tool_geom,
                operation, true,
            ).map(Some)
        }
        Containment::TargetInsideTool => {
            execute_contained_boolean(
                target_topo, target_geom,
                tool_topo, tool_geom,
                operation, false,
            ).map(Some)
        }
        Containment::Disjoint => {
            execute_disjoint_boolean(
                target_topo, target_geom,
                tool_topo, tool_geom,
                operation,
            ).map(Some)
        }
        Containment::Touching => {
            Ok(None)
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
    config: &crate::core::ToleranceConfig,
) -> Result<Containment, KernelError> {
    let tool_sample = sample_interior_point(tool_topo, tool_geom, config)?;

    let vertex_lookup_target = |index: u32| -> Result<[f64; 3], KernelError> {
        let gen = target_topo.arena().vertex_generation(index as usize).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No active vertex at slot index {}", index),
                context: None,
            }
        })?;
        let vid = forge_topo::handles::VertexId::new(index, gen);
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
        &tool_sample,
        config.get_ray_extent(),
    )?;

    if matches!(tool_in_target, forge_topo::classify::PointClassification::Inside) {
        return Ok(Containment::ToolInsideTarget);
    }

    let target_sample = sample_interior_point(target_topo, target_geom, config)?;

    let vertex_lookup_tool = |index: u32| -> Result<[f64; 3], KernelError> {
        let gen = tool_topo.arena().vertex_generation(index as usize).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No active vertex at slot index {}", index),
                context: None,
            }
        })?;
        let vid = forge_topo::handles::VertexId::new(index, gen);
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
        &target_sample,
        config.get_ray_extent(),
    )?;

    if matches!(target_in_tool, forge_topo::classify::PointClassification::Inside) {
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
        config,
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
    
    forge_geom::polygon::compute_polygon_centroid(&vertices).ok_or_else(|| {
        KernelError::InvalidInput {
            message: "No vertices with positions".to_string(),
            context: None,
        }
    })
}

/// Handle Booleans where tool is contained inside target (or vice versa).
fn execute_contained_boolean(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    operation: BooleanOp,
    tool_inside_target: bool,
) -> Result<BooleanResult, KernelError> {
    match (operation, tool_inside_target) {
        (BooleanOp::Union, true) => {
            assemble_complete_shells(
                target_topo, target_geom,
                None, false,
            )
        }
        (BooleanOp::Union, false) => {
            assemble_complete_shells(
                tool_topo, tool_geom,
                None, false,
            )
        }
        (BooleanOp::Intersection, true) => {
            assemble_complete_shells(
                tool_topo, tool_geom,
                None, false,
            )
        }
        (BooleanOp::Intersection, false) => {
            assemble_complete_shells(
                target_topo, target_geom,
                None, false,
            )
        }
        (BooleanOp::Subtraction, true) => {
            if are_solids_coincident(target_topo, target_geom, tool_topo, tool_geom)? {
                let empty_topo = TopologyState::empty();
                let empty_geom = GeometryStore::new();
                return Ok(BooleanResult::new(empty_topo, empty_geom, 0, 0, crate::boolean::schema::BooleanIntrospection::default()));
            }
            assemble_complete_shells(
                target_topo, target_geom,
                Some((tool_topo, tool_geom)),
                true,
            )
        }
        (BooleanOp::Subtraction, false) => {
            let empty_topo = TopologyState::empty();
            let empty_geom = GeometryStore::new();
            Ok(BooleanResult::new(empty_topo, empty_geom, 0, 0, crate::boolean::schema::BooleanIntrospection::default()))
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
) -> Result<BooleanResult, KernelError> {
    match operation {
        BooleanOp::Union => {
            assemble_complete_shells(
                target_topo, target_geom,
                Some((tool_topo, tool_geom)),
                false,
            )
        }
        BooleanOp::Intersection => {
            let empty_topo = TopologyState::empty();
            let empty_geom = GeometryStore::new();
            Ok(BooleanResult::new(empty_topo, empty_geom, 0, 0, crate::boolean::schema::BooleanIntrospection::default()))
        }
        BooleanOp::Subtraction => {
            assemble_complete_shells(
                target_topo, target_geom,
                None, false,
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
) -> Result<BooleanResult, KernelError> {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();
    let mut result_geom = GeometryStore::new();

    let primary_faces: Vec<FaceId> = primary_topo.arena().iter_faces()
        .map(|(fid, _)| fid)
        .collect();
    let primary_count = primary_faces.len();

    let mut primary_he: Vec<HalfEdgeId> = Vec::new();
    let mut primary_dedup = VertexDedup::new();

    copy_faces(
        &mut draft, &mut result_geom, &mut primary_dedup,
        &mut primary_he,
        primary_topo.arena(), primary_geom, &primary_faces,
        false,
    )?;

    stitch_twins(&mut draft, &primary_he)?;

    let mut secondary_count = 0usize;
    if let Some((sec_topo, sec_geom)) = secondary {
        let sec_faces: Vec<FaceId> = sec_topo.arena().iter_faces()
            .map(|(fid, _)| fid)
            .collect();
        secondary_count = sec_faces.len();

        let mut sec_he: Vec<HalfEdgeId> = Vec::new();
        let mut sec_dedup = VertexDedup::new();

        copy_faces(
            &mut draft, &mut result_geom, &mut sec_dedup,
            &mut sec_he,
            sec_topo.arena(), sec_geom, &sec_faces,
            reverse_secondary,
        )?;

        stitch_twins(&mut draft, &sec_he)?;
    }

    let topo = draft.commit()?;
    Ok(BooleanResult::new(topo, result_geom, primary_count, secondary_count, crate::boolean::schema::BooleanIntrospection::default()))
}

/// Check whether any face centroid from one solid lies on the boundary
/// of the other solid. This detects flush face contact (e.g. two cubes
/// sharing a face) that isn't caught by centroid-based containment checks.
fn has_overlapping_coplanar_faces(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    config: &crate::core::ToleranceConfig,
) -> Result<bool, KernelError> {
    let vertex_lookup_tool = |index: u32| -> Result<[f64; 3], KernelError> {
        let gen = tool_topo.arena().vertex_generation(index as usize).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No active vertex at slot index {}", index),
                context: None,
            }
        })?;
        let vid = forge_topo::handles::VertexId::new(index, gen);
        tool_geom.get_vertex_position(vid).copied().ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No position for vertex {}", index),
                context: None,
            }
        })
    };

    for (face_id, _) in target_topo.arena().iter_faces() {
        let centroid = crate::boolean::eval::compute_face_centroid(
            target_topo.arena(), target_geom, face_id,
        )?;

        let class = classify_point_in_solid(
            tool_topo.arena(),
            &vertex_lookup_tool,
            &centroid,
            config.get_ray_extent(),
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
        let vid = forge_topo::handles::VertexId::new(index, gen);
        target_geom.get_vertex_position(vid).copied().ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No position for vertex {}", index),
                context: None,
            }
        })
    };

    for (face_id, _) in tool_topo.arena().iter_faces() {
        let centroid = crate::boolean::eval::compute_face_centroid(
            tool_topo.arena(), tool_geom, face_id,
        )?;

        let class = classify_point_in_solid(
            target_topo.arena(),
            &vertex_lookup_target,
            &centroid,
            config.get_ray_extent(),
        )?;

        if !matches!(class, forge_topo::classify::PointClassification::OnBoundary(_)) {
            return Ok(false);
        }
    }

    Ok(true)
}
