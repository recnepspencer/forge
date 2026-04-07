//! ConvexCell → halfedge mesh conversion — pure orchestration.
//!
//! Builds a complete halfedge mesh from a BSP ConvexCell by orchestrating:
//! 1. Input validation (via `shared_validators::facade`)
//! 2. Vertex insertion (via `shared_operations::facade`)
//! 3. Face/loop/halfedge construction (via `shared_operations::facade`)
//! 4. Twin stitching (via `shared_operations::facade`)
//! 5. Geometry binding validation (via `forge_spatial`)
//!
//! This file contains NO inline math, data structures, or validation logic.

use forge_core::KernelError;
use forge_geom::{build_convex_polyhedron, BspConfig, ConvexCell, Plane};
use forge_topo::handles::VertexId;
use forge_topo::provenance::{
    LineageMode, LineageRecorder, OperationLineageContext, FEATURE_ID_SYSTEM,
};
use forge_topo::transactions::{MutableDraft, TopologyState};

use crate::engine::facade::SolidEnvelope;

use crate::configuration::facade::ResolvedConfig;
use crate::context::scope::OperationScope;
use crate::engine::facade::AuditLevel;
use crate::engine::transaction::data::feature_event::{FeatureInvocationId, KernelFeatureEvent};
use crate::engine::transaction::data::operation_outputs::OperationEnvelopeOutput;
use crate::engine::transaction::data::subscriber_data_id::KernelSubscriberDataId;
use crate::engine::transaction::logic::feature_event_runtime::{
    FeatureEventRuntime, FeatureEventRuntimeContext,
};
use crate::geometry::facade::GeometryStore;
use crate::operations::shared_operations::facade::{
    emit_edge_curves, insert_faces_and_loops, make_solid_hierarchy, place_vertex_exact,
    stitch_twins, PlacementRegistry,
};
use forge_core::envelope::OperationResult;
use forge_signal::facade::runtime::CheckpointBarrier;

use crate::operations::shared_validators::facade::{
    validate_cell, validate_center_and_size, validate_coordinate, validate_dimension,
};

// ── Core orchestrator ────────────────────────────────────────────────────

/// Build a halfedge mesh from a BSP ConvexCell.
///
/// Pure orchestration: validates → inserts vertices → builds faces →
/// stitches twins → validates geometry bindings.
pub fn build_halfedge_mesh(
    cell: &ConvexCell,
    config: &ResolvedConfig,
) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    let mut runtime_ctx = FeatureEventRuntimeContext::from_config(config.config().clone());
    let mut event_runtime = FeatureEventRuntime::new()?;
    let invocation_id = FeatureInvocationId::new(1);
    let state_hash_before =
        forge_topo::transactions::compute_arena_topology_hash(TopologyState::empty().arena());

    event_runtime.begin(&mut runtime_ctx)?;
    event_runtime.emit(KernelFeatureEvent::OperationStarted {
        feature_kind: "build_halfedge_mesh",
        invocation_id,
        audit_level: AuditLevel::Summary,
        state_hash_before,
    });
    let operation_started = std::time::Instant::now();

    let build_result: Result<SolidEnvelope, KernelError> = (|| {
        let mut scope = OperationScope::new(config, &mut runtime_ctx.modeling_context);

        validate_cell(cell)?;

        let span = scope.sink.start_span("build_halfedge_mesh");
        let start = std::time::Instant::now();
        let tolerance = scope.config.scaled_vertex_tolerance();

        // Create a LineageRecorder for this primitive construction.
        // Uses FEATURE_ID_SYSTEM because build_halfedge_mesh is infrastructure,
        // not a user-facing feature. Pipeline-level features override this.
        let mut recorder = LineageRecorder::new(
            OperationLineageContext {
                feature_id: forge_topo::provenance::FEATURE_ID_SYSTEM,
                op_name: "build_halfedge_mesh",
                mode: LineageMode::Root,
            },
            1,
        );

        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mut geometry = GeometryStore::default();

        // 1. Insert vertices (dedup + exact positions)
        let vertex_ids = insert_vertices(
            &mut draft,
            &mut geometry,
            cell,
            tolerance,
            scope.sink,
            &mut recorder,
        )?;

        // 2. Create containment hierarchy
        let hierarchy = make_solid_hierarchy(&mut draft, &mut recorder)?;

        // 3. Build faces, loops, halfedges
        let edge_map = insert_faces_and_loops(
            &mut draft,
            &mut geometry,
            cell,
            &vertex_ids,
            hierarchy.shell,
            &mut recorder,
        )?;

        // 4. Stitch twin pointers
        let edges = stitch_twins(&mut draft, &edge_map, &mut recorder)?;

        // 4b. Emit edge curves (decoupled geometry post-pass)
        let tol_provider = forge_core::tolerance::FlatToleranceProvider::new(tolerance);
        emit_edge_curves(draft.arena(), &mut geometry, &edges, &tol_provider)?;

        // 5. Set representative face on shell
        let first_face = draft.arena().iter_faces().next().map(|(fid, _)| fid);
        if let Some(fid) = first_face {
            draft
                .arena_mut()
                .get_shell_mut(hierarchy.shell)?
                .set_representative_face(fid);
        }

        let topology = draft.commit()?;

        // 6. Validate geometry bindings
        crate::geometry::facade::validate_bindings(&geometry, topology.arena())?;
        forge_spatial::validate_geometry_completeness(
            topology.arena(),
            &|f| geometry.planes.contains(f),
            &|v| geometry.positions.contains(v),
            Some(&|f| geometry.surfaces.contains(f)),
            Some(&|e| geometry.curves.contains(e)),
        )?;

        scope
            .sink
            .end_span(span, start.elapsed().as_micros() as u64);

        Ok(SolidEnvelope::new(topology, geometry))
    })();

    let solid = match build_result {
        Ok(solid) => solid,
        Err(err) => {
            event_runtime.emit(KernelFeatureEvent::OperationFailed {
                invocation_id,
                error_summary: err.to_string(),
            });
            event_runtime.rollback(&mut runtime_ctx);
            return Err(err);
        }
    };

    let state_hash_after =
        forge_topo::transactions::compute_arena_topology_hash(solid.topology().arena());
    event_runtime.emit(KernelFeatureEvent::OperationCompleted {
        invocation_id,
        duration_micros: operation_started.elapsed().as_micros() as u64,
        state_hash_after,
    });
    event_runtime.flush(CheckpointBarrier::PerOperation, &mut runtime_ctx)?;

    let operation_output = event_runtime
        .event_bus()
        .context()
        .committed::<OperationEnvelopeOutput>(KernelSubscriberDataId::OperationEnvelope)
        .ok_or_else(|| KernelError::InternalError {
            message: "OperationEnvelope output missing after primitive event flush".to_string(),
            context: None,
        })?;

    let mut envelope = OperationResult::new(solid);
    apply_operation_output(&mut envelope, operation_output);
    Ok(envelope)
}

fn apply_operation_output(
    envelope: &mut OperationResult<SolidEnvelope>,
    output: &OperationEnvelopeOutput,
) {
    let mut merged_log = envelope.get_decision_log().clone();
    merged_log.merge(output.decision_log.clone());
    envelope.set_decision_log(merged_log);
    for warning in output.warnings.iter().cloned() {
        envelope.add_warning(warning);
    }
    let mut merged_metrics = envelope.get_metrics().clone();
    merged_metrics.accumulate(&output.metrics);
    envelope.set_metrics(merged_metrics);

    let mut merged_lineage = envelope.get_lineage_delta().clone();
    merged_lineage.accumulate(&output.lineage_delta);
    envelope.set_lineage_delta(merged_lineage);

    envelope.consume_budget(output.accumulated_error_budget);
    envelope.set_state_hash_before(output.state_hash_before);
    envelope.set_state_hash_after(output.state_hash_after);
    for summary in output.extra_summaries.iter().cloned() {
        envelope.add_extra_summary(summary);
    }
}

// ── Vertex insertion adapter ─────────────────────────────────────────────

/// Thin adapter: iterates ConvexCell vertices and delegates each placement
/// to `shared_operations::facade::place_vertex_exact`.
fn insert_vertices(
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    cell: &ConvexCell,
    tolerance: f64,
    sink: &mut dyn forge_core::tracing::DecisionSink,
    recorder: &mut LineageRecorder,
) -> Result<Vec<VertexId>, KernelError> {
    let mut vertex_ids = Vec::with_capacity(cell.vertex_count());
    let mut registry = PlacementRegistry::with_capacity(cell.vertex_count());
    let planes = cell.planes();

    for vert in cell.vertices() {
        let pos = *vert.position();
        let plane_indices = vert.plane_indices();
        let vid = place_vertex_exact(
            draft,
            geometry,
            &mut registry,
            pos,
            plane_indices,
            planes,
            tolerance,
            sink,
            recorder,
        )?;
        vertex_ids.push(vid);
    }

    Ok(vertex_ids)
}

// ── Primitive constructors (pure orchestration) ──────────────────────────

/// Build a convex solid from arbitrary planes.
pub fn make_convex_solid(
    planes: Vec<Plane>,
    config: &ResolvedConfig,
) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    let cell = build_convex_polyhedron(&planes, &BspConfig::default())?;
    build_halfedge_mesh(&cell, config)
}

/// Create a cube centered at `center` with side length `size`.
pub fn make_cube(
    center: [f64; 3],
    size: f64,
    config: &ResolvedConfig,
) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    validate_center_and_size(center, size, config.config())?;
    make_convex_solid(forge_geom::cube(center, size / 2.0)?, config)
}

/// Create a regular tetrahedron centered at `center` with the given `scale`.
pub fn make_tetrahedron(
    center: [f64; 3],
    scale: f64,
    config: &ResolvedConfig,
) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    validate_center_and_size(center, scale, config.config())?;
    make_convex_solid(forge_geom::tetrahedron(center, scale)?, config)
}

/// Create a regular dodecahedron centered at `center` with the given `scale`.
pub fn make_dodecahedron(
    center: [f64; 3],
    scale: f64,
    config: &ResolvedConfig,
) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    validate_center_and_size(center, scale, config.config())?;
    make_convex_solid(forge_geom::dodecahedron(center, scale)?, config)
}

/// Create an axis-aligned block with independent half-extents.
pub fn make_block(
    center: [f64; 3],
    half_extents: [f64; 3],
    config: &ResolvedConfig,
) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    for (i, &v) in center.iter().enumerate() {
        validate_coordinate(v, &format!("center[{i}]"))?;
    }
    for (i, &v) in half_extents.iter().enumerate() {
        validate_dimension(v, &format!("half_extents[{i}]"), config.config())?;
    }
    make_convex_solid(forge_geom::block(center, half_extents)?, config)
}

/// Create a regular prism (n-gon extrusion) centered at `center`.
pub fn make_prism(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
    config: &ResolvedConfig,
) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    for (i, &v) in center.iter().enumerate() {
        validate_coordinate(v, &format!("center[{i}]"))?;
    }
    validate_dimension(radius, "radius", config.config())?;
    validate_dimension(height, "height", config.config())?;
    validate_minimum_sides(sides, 3, "prism")?;
    make_convex_solid(forge_geom::prism(center, sides, radius, height)?, config)
}

/// Create a regular pyramid (n-gon base with apex) centered at `center`.
pub fn make_pyramid(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
    config: &ResolvedConfig,
) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    for (i, &v) in center.iter().enumerate() {
        validate_coordinate(v, &format!("center[{i}]"))?;
    }
    validate_dimension(radius, "radius", config.config())?;
    validate_dimension(height, "height", config.config())?;
    validate_minimum_sides(sides, 3, "pyramid")?;
    make_convex_solid(forge_geom::pyramid(center, sides, radius, height)?, config)
}

/// Create a wedge (triangular cross-section extrusion) centered at `center`.
pub fn make_wedge(
    center: [f64; 3],
    dimensions: [f64; 3],
    config: &ResolvedConfig,
) -> Result<OperationResult<SolidEnvelope>, KernelError> {
    for (i, &v) in center.iter().enumerate() {
        validate_coordinate(v, &format!("center[{i}]"))?;
    }
    let names = ["width", "depth", "height"];
    for (i, &v) in dimensions.iter().enumerate() {
        validate_dimension(v, names[i], config.config())?;
    }
    make_convex_solid(forge_geom::wedge(center, dimensions)?, config)
}

/// Validate that a polygon primitive has enough sides.
fn validate_minimum_sides(sides: u32, min: u32, name: &str) -> Result<(), KernelError> {
    if sides < min {
        return Err(KernelError::InvalidInput {
            message: format!("{name} needs at least {min} sides, got {sides}"),
            context: None,
        });
    }
    Ok(())
}
