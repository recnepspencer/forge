//! Test support for spatial validator tests.
//!
//! Provides helpers for building synthetic B-Rep topology and mock tolerance providers.

use forge_topo::b_rep::{BodyData, FaceData, HalfEdgeData, LoopData, LumpData, RegionData, ShellData, ShellOrientation, VertexData};
use forge_topo::handles::{EdgeId, FaceId, HalfEdgeId, ShellId, VertexId};
use forge_topo::transactions::{DraftConfig, MutableDraft, TopologyState};
use forge_topo::validate::ValidationLevel;

/// Create an empty mutable draft with validation disabled.
pub fn empty_test_draft() -> MutableDraft {
    let mut config = DraftConfig::default();
    config.validation_level = ValidationLevel::None;
    TopologyState::empty().into_mutation_with(config)
}

/// Insert a Body → Lump → Region → Shell(Solid) hierarchy into the draft.
pub fn insert_test_solid_shell(draft: &mut MutableDraft) -> ShellId {
    let body = draft.insert_body(BodyData::new());
    let lump = draft.insert_lump(LumpData::new(body));
    let region = draft.insert_region(RegionData::new(lump));
    let shell = draft.insert_shell(ShellData::new(
        FaceId::new(u32::MAX, 0),
        forge_topo::b_rep::ShellKind::Solid(ShellOrientation::Outer),
        region,
    ));
    draft.arena_mut().get_body_mut(body).unwrap().add_lump(lump);
    draft.arena_mut().get_lump_mut(lump).unwrap().add_region(region);
    draft.arena_mut().get_region_mut(region).unwrap().add_shell(shell);
    shell
}

/// Build a triangle face with 3 half-edges in a loop.
/// Returns (face_id, v0, v1, v2).
pub fn build_triangle_face(draft: &mut MutableDraft) -> (FaceId, VertexId, VertexId, VertexId) {
    let placeholder_he = HalfEdgeId::new(0, 0);
    let placeholder_face = FaceId::new(0, 0);
    let placeholder_edge = EdgeId::new(0, 0);

    let v0 = draft.insert_vertex(VertexData::new(placeholder_he));
    let v1 = draft.insert_vertex(VertexData::new(placeholder_he));
    let v2 = draft.insert_vertex(VertexData::new(placeholder_he));

    let shell = insert_test_solid_shell(draft);
    let loop_id = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let face = draft.insert_face(FaceData::new(loop_id, shell));

    let h0 = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v0, placeholder_edge));
    let h1 = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v1, placeholder_edge));
    let h2 = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v2, placeholder_edge));

    let arena = draft.arena_mut();
    arena.get_half_edge_mut(h0).unwrap().set_next(h1);
    arena.get_half_edge_mut(h1).unwrap().set_next(h2);
    arena.get_half_edge_mut(h2).unwrap().set_next(h0);
    arena.get_half_edge_mut(h0).unwrap().set_prev(h2);
    arena.get_half_edge_mut(h1).unwrap().set_prev(h0);
    arena.get_half_edge_mut(h2).unwrap().set_prev(h1);
    arena.get_loop_mut(loop_id).unwrap().set_half_edge(h0);
    arena.get_loop_mut(loop_id).unwrap().set_face(face);
    arena.get_shell_mut(shell).unwrap().set_representative_face(face);

    (face, v0, v1, v2)
}

/// Build a single edge with 2 distinct vertices. Returns (he_id, v0, v1).
pub fn build_edge(draft: &mut MutableDraft) -> (HalfEdgeId, VertexId, VertexId) {
    use forge_topo::b_rep::EdgeData;
    let placeholder_he = HalfEdgeId::new(0, 0);
    let placeholder_face = FaceId::new(0, 0);

    let v0 = draft.insert_vertex(VertexData::new(placeholder_he));
    let v1 = draft.insert_vertex(VertexData::new(placeholder_he));
    let edge = draft.insert_edge(EdgeData::new(placeholder_he));

    let h0 = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v0, edge));
    let h1 = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v1, edge));

    let arena = draft.arena_mut();
    arena.get_half_edge_mut(h0).unwrap().set_next(h1);
    arena.get_half_edge_mut(h0).unwrap().set_radial_next(h1);
    arena.get_half_edge_mut(h1).unwrap().set_next(h0);
    arena.get_half_edge_mut(h1).unwrap().set_radial_next(h0);
    arena.get_edge_mut(edge).unwrap().set_half_edge(h0);

    (h0, v0, v1)
}

/// Build a self-loop edge (1 edge, 1 vertex). Returns (he_id, v0).
pub fn build_self_loop(draft: &mut MutableDraft) -> (HalfEdgeId, VertexId) {
    use forge_topo::b_rep::EdgeData;
    let placeholder_he = HalfEdgeId::new(0, 0);
    let placeholder_face = FaceId::new(0, 0);

    let v0 = draft.insert_vertex(VertexData::new(placeholder_he));
    let edge = draft.insert_edge(EdgeData::new(placeholder_he));

    let h0 = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v0, edge));
    let h1 = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v0, edge));

    let arena = draft.arena_mut();
    arena.get_half_edge_mut(h0).unwrap().set_next(h1);
    arena.get_half_edge_mut(h0).unwrap().set_radial_next(h1);
    arena.get_half_edge_mut(h1).unwrap().set_next(h0);
    arena.get_half_edge_mut(h1).unwrap().set_radial_next(h0);
    arena.get_edge_mut(edge).unwrap().set_half_edge(h0);

    (h0, v0)
}

/// A simple mock tolerance provider for testing.
#[derive(Debug)]
pub struct MockToleranceProvider {
    pub default_tolerance: f64,
}

impl forge_core::ToleranceProvider for MockToleranceProvider {
    fn global_default(&self) -> f64 { self.default_tolerance }
    fn vertex_tolerance(&self, _index: u32, _generation: u32) -> f64 { self.default_tolerance }
    fn edge_tolerance(&self, _index: u32, _generation: u32) -> f64 { self.default_tolerance }
}

impl Default for MockToleranceProvider {
    fn default() -> Self {
        Self { default_tolerance: 1e-6 }
    }
}
