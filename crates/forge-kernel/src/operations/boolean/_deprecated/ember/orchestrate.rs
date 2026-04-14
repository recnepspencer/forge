//! EMBER BSP Merge Pipeline — Entry Point.
//!
//! DOMAIN: The production boolean pipeline for planar solids. Converts
//! halfedge meshes to BSP trees, merges algebraically with exact arithmetic,
//! extracts boundary, and builds the result mesh. Never delegates to legacy.
//!
//! PIPELINE PHASES:
//!   1. Convert — halfedge → BspSolid (convex input assumption)
//!   2. Merge — BSP tree merge (exact Rational arithmetic)
//!   3. Extract — boundary ConvexCells from merged tree
//!   4. Mesh — combined halfedge mesh from cells (mesh.rs)
//!   5. Finalize — wrap in BooleanResult + OperationResult envelope
//!
//! DEPENDENCIES: worth-geom (BspSolid, merge_bsp, convex_to_bsp),
//!               mesh.rs (bsp_to_mesh), ModelingContext, OperationResult

use forge_core::tracing::{DecisionKind, DecisionTier, TraceAdjunctSet};
use forge_core::{KernelError, OperationMetrics, OperationResult};
use crate::geom_facade::{merge_bsp, BspNode, BspOp, BspSolid};
use crate::geom_facade::Plane;
use forge_topo::transactions::TopologyState;

use super::mesh::bsp_to_mesh;
use crate::core::{FinalizationError, ModelingContext, OperationFinalizer, TopologyHashBoundary};
use crate::geometry_state::GeometryState;
use crate::operations::boolean::result::BooleanIntrospection;
use crate::operations::boolean::{BooleanInput, BooleanOp, BooleanResult};

/// EMBER error types — only CurvedGeometry triggers fallback.
#[derive(Debug)]
pub enum EmberError {
    /// Input contains curved geometry that EMBER cannot handle.
    CurvedGeometry,
    /// BSP pipeline failed (convert, merge, extract, or mesh phase).
    PipelineError(KernelError),
}

impl From<KernelError> for EmberError {
    fn from(e: KernelError) -> Self {
        EmberError::PipelineError(e)
    }
}

impl From<worth_math::MathError> for EmberError {
    fn from(e: worth_math::MathError) -> Self {
        EmberError::PipelineError(KernelError::from(e))
    }
}

impl From<FinalizationError> for EmberError {
    fn from(e: FinalizationError) -> Self {
        EmberError::PipelineError(KernelError::InternalError {
            message: format!("ember finalization failed: {:?}", e),
            context: None,
        })
    }
}

/// Execute a Boolean using the EMBER BSP merge pipeline.
///
/// This is a fully self-contained pipeline: convert → merge → extract → mesh.
/// It never delegates to the legacy split-classify-stitch path.
///
/// Returns `Err(EmberError::CurvedGeometry)` only if inputs are non-planar.
pub fn execute_ember_boolean(
    input: BooleanInput,
) -> Result<OperationResult<Result<BooleanResult, KernelError>>, EmberError> {
    if input.has_curved_geometry() {
        return Err(EmberError::CurvedGeometry);
    }
    let topo_hash_before =
        input.target_topology().topology_hash() ^ input.tool_topology().topology_hash();

    let mut ctx = ModelingContext::default();
    let start_time = std::time::Instant::now();

    let inner_result = execute_pipeline(input, &mut ctx, start_time);

    let metrics = OperationMetrics {
        duration: start_time.elapsed(),
        ..OperationMetrics::default()
    };

    let mut envelope = OperationResult::new(inner_result);
    envelope.set_metrics(metrics);
    let topo_hash_after = match envelope.get_value() {
        Ok(result) => Some(result.topology().topology_hash()),
        Err(_) => None,
    };
    let mut finalizer = OperationFinalizer::new(&mut ctx);
    match envelope.get_value() {
        Ok(_) => {
            let _ = finalizer.collect_success(
                &mut envelope,
                TraceAdjunctSet::new(),
                TopologyHashBoundary {
                    before: Some(topo_hash_before),
                    after: topo_hash_after,
                },
                None,
            )?;
        }
        Err(_) => {
            let _ = finalizer.collect_error(
                &mut envelope,
                TraceAdjunctSet::new(),
                TopologyHashBoundary {
                    before: Some(topo_hash_before),
                    after: None,
                },
                None,
            )?;
        }
    }
    Ok(envelope)
}

/// The EMBER pipeline: convert → merge → extract → mesh → finalize.
fn execute_pipeline(
    input: BooleanInput,
    ctx: &mut ModelingContext,
    start_time: std::time::Instant,
) -> Result<BooleanResult, KernelError> {
    input
        .validate()
        .map_err(|e| e.with_phase("ember_validate"))?;
    let (target_topo, target_geom, tool_topo, tool_geom, operation) = input.into_parts();

    let bsp_op = to_bsp_op(operation);

    // ── Phase 1: Convert ────────────────────────────────────────────────────
    let target_bsp = ctx
        .scope("ember_convert_target", |ctx| {
            let bsp = halfedge_to_bsp(&target_topo, &target_geom)?;
            ctx.log_decision(
                DecisionKind::Exact,
                DecisionTier::Deterministic,
                [0.0, 0.0, 0.0],
                0.0,
                0.0,
            );
            Ok(bsp)
        })
        .map_err(|e: KernelError| e.with_phase("ember_convert"))?;

    let tool_bsp = ctx
        .scope("ember_convert_tool", |ctx| {
            let bsp = halfedge_to_bsp(&tool_topo, &tool_geom)?;
            ctx.log_decision(
                DecisionKind::Exact,
                DecisionTier::Deterministic,
                [0.0, 0.0, 0.0],
                0.0,
                0.0,
            );
            Ok(bsp)
        })
        .map_err(|e: KernelError| e.with_phase("ember_convert"))?;

    // ── Phase 2: Merge ──────────────────────────────────────────────────────
    let merged = ctx
        .scope("ember_merge", |ctx| {
            let result = merge_bsp(&target_bsp, &tool_bsp, bsp_op).map_err(KernelError::from)?;
            ctx.log_decision(
                DecisionKind::Exact,
                DecisionTier::Deterministic,
                [0.0, 0.0, 0.0],
                0.0,
                0.0,
            );
            Ok(result)
        })
        .map_err(|e: KernelError| e.with_phase("ember_merge"))?;

    // ── Phase 3+4: Extract + Mesh ───────────────────────────────────────────
    let (result_topo, result_geom, result_brep) = ctx
        .scope("ember_mesh", |ctx| bsp_to_mesh(&merged, ctx))
        .map_err(|e: KernelError| e.with_phase("ember_mesh"))?;

    // ── Phase 5: Finalize ───────────────────────────────────────────────────
    let target_face_count = target_topo.arena().face_count();
    let tool_face_count = tool_topo.arena().face_count();

    let introspection = BooleanIntrospection::new(0, &[], &[], start_time.elapsed());

    let result = BooleanResult::new(
        result_topo,
        result_geom,
        crate::brep::state::BrepState::new(),
        target_face_count,
        tool_face_count,
        introspection,
    );

    Ok(result)
}

/// Convert a halfedge mesh to a BspSolid using autopartition.
///
/// Extracts face polygons (vertex positions) from the mesh and builds
/// a BSP tree by recursively picking a face's plane as the splitter
/// and classifying other face polygons against it. This correctly
/// handles non-convex meshes (chained boolean results).
fn halfedge_to_bsp(topo: &TopologyState, geom: &GeometryState) -> Result<BspSolid, KernelError> {
    let mut planes: Vec<Plane> = Vec::new();
    let mut plane_map: Vec<usize> = Vec::new(); // face_idx → plane_idx
    let mut face_polygons: Vec<Vec<[f64; 3]>> = Vec::new();

    for (fid, _) in topo.arena().iter_faces() {
        let plane = geom
            .get_face_plane(fid)
            .ok_or_else(|| KernelError::InternalError {
                message: "EMBER: face missing plane in GeometryState".to_string(),
                context: None,
            })?;

        // Deduplicate planes by exact coefficients
        let pidx = find_or_insert_plane(&mut planes, plane);
        plane_map.push(pidx);

        // Extract face polygon vertices
        let verts = extract_face_polygon(topo, geom, fid)?;
        face_polygons.push(verts);
    }

    if planes.is_empty() {
        return Err(KernelError::InvalidInput {
            message: "EMBER: no face planes found in GeometryState".to_string(),
            context: None,
        });
    }

    // Build face descriptors for autopartition
    let face_descs: Vec<FaceDesc> = plane_map
        .iter()
        .zip(face_polygons.iter())
        .map(|(&pidx, verts)| FaceDesc {
            plane_idx: pidx,
            vertices: verts.clone(),
        })
        .collect();

    let root = build_autopartition(&planes, &face_descs);
    let root = root.simplify();

    Ok(BspSolid::new(planes, root))
}

/// A face polygon descriptor for autopartition BSP construction.
struct FaceDesc {
    /// Index into the shared plane set.
    plane_idx: usize,
    /// Vertex positions of the face polygon (ordered).
    vertices: Vec<[f64; 3]>,
}

/// Find an existing plane with matching exact coefficients, or insert a new one.
fn find_or_insert_plane(planes: &mut Vec<Plane>, plane: &Plane) -> usize {
    for (i, existing) in planes.iter().enumerate() {
        if crate::geom_facade::plane_exact_eq(existing, plane)
            || crate::geom_facade::coplanar_eq(existing, plane)
        {
            return i;
        }
    }
    planes.push(plane.clone());
    planes.len() - 1
}

/// Extract face polygon vertex positions from a halfedge mesh face.
fn extract_face_polygon(
    topo: &TopologyState,
    geom: &GeometryState,
    face_id: forge_topo::handles::FaceId,
) -> Result<Vec<[f64; 3]>, KernelError> {
    let face_data = topo.arena().get_face(face_id)?;
    let loop_id = face_data.loops.outer();
    let loop_data = topo.arena().get_loop(loop_id)?;
    let start_he = loop_data.half_edge();

    let mut vertices = Vec::new();
    let mut current = start_he;
    loop {
        let he = topo.arena().get_half_edge(current)?;
        let vid = he.origin();
        let pos = geom
            .get_vertex_position(vid)
            .ok_or_else(|| KernelError::InternalError {
                message: format!("EMBER: vertex {:?} missing position", vid),
                context: None,
            })?;
        vertices.push(*pos);
        current = he.next();
        if current == start_he {
            break;
        }
        if vertices.len() > 1000 {
            return Err(KernelError::InternalError {
                message: "EMBER: infinite loop in face polygon extraction".to_string(),
                context: None,
            });
        }
    }

    Ok(vertices)
}

/// Evaluate a point against a plane: returns n·p + d (positive = in front of plane).
fn evaluate_plane(plane: &Plane, point: &[f64; 3]) -> f64 {
    let n = plane.raw_normal();
    let d = plane.raw_offset();
    n[0] * point[0] + n[1] * point[1] + n[2] * point[2] + d
}

/// Build a BSP tree from face polygons using autopartition.
///
/// Picks the first face's plane as the splitting plane, classifies
/// other face polygons based on their vertex positions relative to
/// the splitter, and recurses. Faces with all vertices on the negative
/// side go to the neg child, positive to pos, spanning to both.
/// When no faces remain, returns a solid leaf (fully inside the solid).
fn build_autopartition(planes: &[Plane], faces: &[FaceDesc]) -> BspNode {
    if faces.is_empty() {
        return BspNode::solid();
    }

    let splitter = &faces[0];
    let splitter_plane = &planes[splitter.plane_idx];
    let remaining = &faces[1..];

    let mut neg_faces = Vec::new();
    let mut pos_faces = Vec::new();

    for face in remaining {
        if face.plane_idx == splitter.plane_idx {
            // Coplanar face — same plane, skip (it's part of the same boundary)
            // Put it on the neg side so the solid region is correctly bounded
            neg_faces.push(FaceDesc {
                plane_idx: face.plane_idx,
                vertices: face.vertices.clone(),
            });
            continue;
        }

        let mut has_neg = false;
        let mut has_pos = false;

        for vert in &face.vertices {
            let val = evaluate_plane(splitter_plane, vert);
            if val < -1e-10 {
                has_neg = true;
            } else if val > 1e-10 {
                has_pos = true;
            }
        }

        if has_neg && !has_pos {
            neg_faces.push(FaceDesc {
                plane_idx: face.plane_idx,
                vertices: face.vertices.clone(),
            });
        } else if has_pos && !has_neg {
            pos_faces.push(FaceDesc {
                plane_idx: face.plane_idx,
                vertices: face.vertices.clone(),
            });
        } else {
            // Spanning or all-on-plane → add to both sides
            neg_faces.push(FaceDesc {
                plane_idx: face.plane_idx,
                vertices: face.vertices.clone(),
            });
            pos_faces.push(FaceDesc {
                plane_idx: face.plane_idx,
                vertices: face.vertices.clone(),
            });
        }
    }

    let neg_child = build_autopartition(planes, &neg_faces);
    let pos_child = if pos_faces.is_empty() {
        BspNode::empty()
    } else {
        build_autopartition(planes, &pos_faces)
    };

    BspNode::split(splitter.plane_idx, neg_child, pos_child)
}

/// Map BooleanOp to BspOp.
fn to_bsp_op(op: BooleanOp) -> BspOp {
    match op {
        BooleanOp::Union => BspOp::Union,
        BooleanOp::Intersection => BspOp::Intersection,
        BooleanOp::Subtraction => BspOp::Subtraction,
    }
}

/// Dual-engine Boolean router — the recommended production entry point.
///
/// Routes operations to the appropriate pipeline:
/// - **Planar geometry** → EMBER BSP merge pipeline
/// - **Curved geometry** → parametric split-classify-stitch pipeline
/// - **EMBER failure** → automatic parametric fallback (temporary safety net)
pub fn execute_boolean_adaptive(
    input: BooleanInput,
) -> OperationResult<Result<BooleanResult, KernelError>> {
    use crate::operations::boolean::_deprecated::parametric::assemble::execute_boolean_direct;

    if input.has_curved_geometry() {
        return execute_boolean_direct(input);
    }

    let input_clone = input.clone();

    match execute_ember_boolean(input) {
        Ok(result) => result,
        Err(EmberError::CurvedGeometry) => execute_boolean_direct(input_clone),
        Err(EmberError::PipelineError(_)) => execute_boolean_direct(input_clone),
    }
}
