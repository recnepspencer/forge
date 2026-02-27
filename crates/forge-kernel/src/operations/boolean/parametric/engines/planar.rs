//! Planar Boolean engine — wraps existing pipeline functions as trait impls.
//!
//! DOMAIN: All faces are planar. Uses exact predicates for split,
//! ray-cast for classify, EMBER exact coplanar resolution, halfedge
//! stitch for assemble, and coplanar merge for postprocess.
//!
//! DEPENDENCIES: split::split_all_faces, classify::classify_faces,
//!   boolean::ember::classify::apply_ember_coplanar_overrides,
//!   assemble_result, postprocess::merge_coplanar_faces

use std::collections::BTreeMap;

use forge_core::KernelError;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::state::TopologyState;
use forge_topo::validate::{validate_topology, ValidationLevel};

use crate::core::{KernelState, ModelingContext};
use crate::geometry_state::GeometryState;
use crate::operations::boolean::classify_schema::{ClassifiedFace, FaceOrigin};
use crate::operations::boolean::parametric::split::SplitPhaseResult;
use crate::operations::boolean::parametric::traits::{
    BooleanAssembler, BooleanClassifier, BooleanEngine, BooleanPostprocessor, BooleanSplitter,
    CoplanarResolver,
};
use crate::shared_ops::vertex_identity::VertexMatchKey;

/// Planar splitter — delegates to `split_all_faces`.
pub struct PlanarSplitter;

impl BooleanSplitter for PlanarSplitter {
    fn split(
        &self,
        target_topo: TopologyState,
        target_geom: GeometryState,
        tool_topo: TopologyState,
        tool_geom: GeometryState,
        ctx: &mut ModelingContext,
    ) -> Result<SplitPhaseResult, KernelError> {
        crate::operations::boolean::parametric::split::split_all_faces(
            target_topo,
            target_geom,
            tool_topo,
            tool_geom,
            ctx,
        )
    }
}

/// Ray-cast classifier — delegates to `classify_faces`.
pub struct RayCastClassifier;

impl BooleanClassifier for RayCastClassifier {
    fn classify(
        &self,
        source_arena: &TopologyArena,
        source_geom: &GeometryState,
        other_arena: &TopologyArena,
        other_geom: &GeometryState,
        origin: FaceOrigin,
        ctx: &mut ModelingContext,
    ) -> Result<Vec<ClassifiedFace>, KernelError> {
        crate::operations::boolean::parametric::classify::classify_faces(
            source_arena,
            source_geom,
            other_arena,
            other_geom,
            origin,
            ctx,
        )
    }
}

/// EMBER coplanar resolver — exact rational plane comparison + AABB overlap.
pub struct EmberCoplanarResolver;

impl CoplanarResolver for EmberCoplanarResolver {
    fn resolve_coplanars(
        &self,
        target_classified: &mut Vec<ClassifiedFace>,
        tool_classified: &mut Vec<ClassifiedFace>,
        target_topo: &TopologyState,
        target_geom: &GeometryState,
        tool_topo: &TopologyState,
        tool_geom: &GeometryState,
    ) {
        crate::operations::boolean::ember::classify::apply_ember_coplanar_overrides(
            target_classified,
            tool_classified,
            target_topo,
            target_geom,
            tool_topo,
            tool_geom,
        );
    }
}

/// No-op coplanar resolver — legacy behavior (no coplanar override).
pub struct NoopCoplanarResolver;

impl CoplanarResolver for NoopCoplanarResolver {
    fn resolve_coplanars(
        &self,
        _target_classified: &mut Vec<ClassifiedFace>,
        _tool_classified: &mut Vec<ClassifiedFace>,
        _target_topo: &TopologyState,
        _target_geom: &GeometryState,
        _tool_topo: &TopologyState,
        _tool_geom: &GeometryState,
    ) {
        // Legacy behavior: no coplanar override injection
    }
}

/// Halfedge assembler — delegates to `assemble_result`.
pub struct HalfEdgeAssembler;

impl BooleanAssembler for HalfEdgeAssembler {
    fn assemble(
        &self,
        target_arena: &TopologyArena,
        target_geom: &GeometryState,
        target_faces: &[FaceId],
        target_prov: &BTreeMap<VertexId, VertexMatchKey>,
        tool_arena: &TopologyArena,
        tool_geom: &GeometryState,
        tool_faces: &[FaceId],
        tool_prov: &BTreeMap<VertexId, VertexMatchKey>,
        reverse_tool: bool,
        ctx: &mut ModelingContext,
    ) -> Result<KernelState, KernelError> {
        crate::operations::boolean::parametric::assemble::merge::assemble_result(
            target_arena,
            target_geom,
            target_faces,
            target_prov,
            tool_arena,
            tool_geom,
            tool_faces,
            tool_prov,
            reverse_tool,
            ctx,
        )
    }
}

/// Standard postprocessor — merge coplanar + remove redundant vertices.
pub struct StandardPostprocessor;

impl BooleanPostprocessor for StandardPostprocessor {
    fn postprocess(
        &self,
        state: KernelState,
        ctx: &mut ModelingContext,
    ) -> Result<KernelState, KernelError> {
        let state = if std::env::var("FORGE_SKIP_COPLANAR_POSTPROCESS")
            .ok()
            .as_deref()
            == Some("1")
        {
            state
        } else {
            let (new_state, _) =
                crate::operations::boolean::parametric::postprocess::merge_coplanar_faces(
                    state, ctx,
                )?;
            new_state
        };
        if std::env::var("FORGE_DEBUG_VALIDATE_PHASES").ok().as_deref() == Some("1") {
            let (topo, _, _) = state.as_parts();
            match validate_topology(topo.arena(), ValidationLevel::Full) {
                Ok(()) => eprintln!("[phase-check] postprocess merge_coplanar valid"),
                Err(e) => eprintln!("[phase-check] postprocess merge_coplanar invalid: {}", e),
            }
        }
        let (new_state, _) =
            crate::operations::boolean::parametric::postprocess::remove_redundant_vertices(
                state, ctx,
            )?;
        let state = new_state;
        if std::env::var("FORGE_DEBUG_VALIDATE_PHASES").ok().as_deref() == Some("1") {
            let (topo, _, _) = state.as_parts();
            match validate_topology(topo.arena(), ValidationLevel::Full) {
                Ok(()) => eprintln!("[phase-check] postprocess remove_redundant valid"),
                Err(e) => eprintln!("[phase-check] postprocess remove_redundant invalid: {}", e),
            }
        }
        Ok(state)
    }
}

/// Convex-only postprocessor — skips coplanar merge to keep faces convex.
///
/// When boolean results feed into subsequent operations (chained booleans),
/// merging coplanar faces creates concave polygons that the splitter
/// cannot handle. This postprocessor only removes redundant collinear
/// vertices, preserving the convex face invariant.
pub struct ConvexOnlyPostprocessor;

impl BooleanPostprocessor for ConvexOnlyPostprocessor {
    fn postprocess(
        &self,
        state: KernelState,
        ctx: &mut ModelingContext,
    ) -> Result<KernelState, KernelError> {
        let (new_state, _) =
            crate::operations::boolean::parametric::postprocess::remove_redundant_vertices(
                state, ctx,
            )?;
        Ok(new_state)
    }
}

/// Build the planar Boolean engine with EMBER coplanar resolution.
pub fn planar_engine() -> BooleanEngine {
    BooleanEngine::new(
        Box::new(PlanarSplitter),
        Box::new(RayCastClassifier),
        Box::new(EmberCoplanarResolver),
        Box::new(HalfEdgeAssembler),
        Box::new(StandardPostprocessor),
    )
}

/// Build the planar Boolean engine without coplanar resolution (legacy).
pub fn planar_engine_parametric() -> BooleanEngine {
    BooleanEngine::new(
        Box::new(PlanarSplitter),
        Box::new(RayCastClassifier),
        Box::new(NoopCoplanarResolver),
        Box::new(HalfEdgeAssembler),
        Box::new(StandardPostprocessor),
    )
}

/// Build a chain-safe engine that keeps faces convex (no coplanar merge).
///
/// Use this when the result will be fed into another boolean operation.
/// Faces remain as convex fragments, avoiding the concave polygon
/// shattering bug in the splitter.
pub fn planar_engine_convex() -> BooleanEngine {
    BooleanEngine::new(
        Box::new(PlanarSplitter),
        Box::new(RayCastClassifier),
        Box::new(NoopCoplanarResolver),
        Box::new(HalfEdgeAssembler),
        Box::new(ConvexOnlyPostprocessor),
    )
}
