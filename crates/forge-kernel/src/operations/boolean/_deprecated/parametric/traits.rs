//! Boolean pipeline phase traits.
//!
//! DOMAIN: Define the contract for each phase of the Boolean pipeline.
//! Each geometry engine (planar, NURBS) provides its own implementations.
//! The orchestrator calls these traits — it never calls concrete functions directly.
//!
//! PHASES:
//!   1. Split — cut both solids along mutual intersections
//!   2. Classify — label each face as inside/outside/boundary
//!   3. Coplanar — detect and resolve coplanar face pairs
//!   4. Assemble — stitch selected faces into the result solid
//!   5. Postprocess — merge coplanar faces, remove redundant vertices
//!
//! INVARIANT: Select is NOT a trait — it's pure logic (keep/drop based on
//! operation + classification), identical for all geometry types.

use std::collections::BTreeMap;

use forge_core::KernelError;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::state::TopologyState;

use crate::core::{KernelState, ModelingContext};
use crate::geometry_state::GeometryState;
use crate::operations::boolean::classify_schema::{ClassifiedFace, FaceOrigin};
use crate::operations::boolean::_deprecated::parametric::split::SplitPhaseResult;
use crate::shared_ops::vertex::identity::VertexMatchKey;

/// Split both solids along their mutual intersections.
///
/// Takes ownership of both topologies and geometries, returns the
/// split result containing both modified solids, split counts, and
/// vertex provenance maps for cross-solid deduplication.
pub trait BooleanSplitter {
    fn split(
        &self,
        target_topo: TopologyState,
        target_geom: GeometryState,
        tool_topo: TopologyState,
        tool_geom: GeometryState,
        ctx: &mut ModelingContext,
    ) -> Result<SplitPhaseResult, KernelError>;
}

/// Classify each face of one solid relative to the other solid.
///
/// Returns a classification label (Inside, Outside, OnBoundary,
/// OppositeBoundary) for each face in the source solid.
pub trait BooleanClassifier {
    fn classify(
        &self,
        source_arena: &TopologyArena,
        source_geom: &GeometryState,
        other_arena: &TopologyArena,
        other_geom: &GeometryState,
        origin: FaceOrigin,
        ctx: &mut ModelingContext,
    ) -> Result<Vec<ClassifiedFace>, KernelError>;
}

/// Detect and resolve coplanar face pairs between two post-split solids.
///
/// May reclassify faces to prevent asymmetric treatment of shared
/// planar boundaries. Called after classify, before select.
pub trait CoplanarResolver {
    fn resolve_coplanars(
        &self,
        target_classified: &mut Vec<ClassifiedFace>,
        tool_classified: &mut Vec<ClassifiedFace>,
        target_topo: &TopologyState,
        target_geom: &GeometryState,
        tool_topo: &TopologyState,
        tool_geom: &GeometryState,
    );
}

/// Assemble the final solid from selected faces of both solids.
///
/// Copies selected faces into a new topology, stitches shared edges,
/// and resolves cross-solid vertex merging using provenance keys.
pub trait BooleanAssembler {
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
    ) -> Result<KernelState, KernelError>;
}

/// Post-process the assembled result.
///
/// Merge coplanar adjacent faces, remove redundant vertices,
/// and perform any engine-specific cleanup.
pub trait BooleanPostprocessor {
    fn postprocess(
        &self,
        state: KernelState,
        ctx: &mut ModelingContext,
    ) -> Result<KernelState, KernelError>;
}

/// A complete Boolean engine — one implementation per geometry class.
///
/// The orchestrator calls these traits in order. Different geometry
/// types provide different implementations while sharing the same
/// pipeline structure.
pub struct BooleanEngine {
    splitter: Box<dyn BooleanSplitter>,
    classifier: Box<dyn BooleanClassifier>,
    coplanar_resolver: Box<dyn CoplanarResolver>,
    assembler: Box<dyn BooleanAssembler>,
    postprocessor: Box<dyn BooleanPostprocessor>,
}

impl BooleanEngine {
    /// Create a new engine from individual phase implementations.
    pub fn new(
        splitter: Box<dyn BooleanSplitter>,
        classifier: Box<dyn BooleanClassifier>,
        coplanar_resolver: Box<dyn CoplanarResolver>,
        assembler: Box<dyn BooleanAssembler>,
        postprocessor: Box<dyn BooleanPostprocessor>,
    ) -> Self {
        Self {
            splitter,
            classifier,
            coplanar_resolver,
            assembler,
            postprocessor,
        }
    }

    /// The split phase implementation.
    pub fn splitter(&self) -> &dyn BooleanSplitter {
        &*self.splitter
    }

    /// The classify phase implementation.
    pub fn classifier(&self) -> &dyn BooleanClassifier {
        &*self.classifier
    }

    /// The coplanar resolution implementation.
    pub fn coplanar_resolver(&self) -> &dyn CoplanarResolver {
        &*self.coplanar_resolver
    }

    /// The assemble phase implementation.
    pub fn assembler(&self) -> &dyn BooleanAssembler {
        &*self.assembler
    }

    /// The postprocess phase implementation.
    pub fn postprocessor(&self) -> &dyn BooleanPostprocessor {
        &*self.postprocessor
    }
}
