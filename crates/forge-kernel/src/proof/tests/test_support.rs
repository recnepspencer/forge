//! Test support adapters for proof-validation suites.
//!
//! Shields proof-validation tests from `forge-topo` API churn by providing
//! stable wrappers around arena mutation methods and geometric validation calls.

use forge_core::{FlatToleranceProvider, KernelError};
use forge_spatial::validate_geometric_invariants;
use forge_topo::b_rep::{
    EdgeData, FaceData, HalfEdgeData, LoopData, TopologyArena, VertexData,
};
use forge_topo::b_rep::{
    BodyData, LumpData, RegionData, ShellData, ShellKind, ShellOrientation,
};
use forge_topo::handles::{EdgeId, FaceId, HalfEdgeId, LoopId, ShellId, VertexId};
use forge_topo::transactions::{MutableDraft, TopologyState};

/// Validate geometric invariants assuming all faces are planar.
pub fn validate_geometric_invariants_all_faces(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    area_threshold: f64,
    edge_length_threshold: f64,
) -> Result<(), KernelError> {
    let tol = FlatToleranceProvider::new(area_threshold.sqrt().max(edge_length_threshold));
    validate_geometric_invariants(arena, position_fn, &all_faces_planar, &tol)
}

/// Validate geometric invariants with an explicit planarity predicate.
pub fn validate_geometric_invariants_with_planarity(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    is_planar: &dyn Fn(FaceId) -> bool,
    area_threshold: f64,
    edge_length_threshold: f64,
) -> Result<(), KernelError> {
    let tol = FlatToleranceProvider::new(area_threshold.sqrt().max(edge_length_threshold));
    validate_geometric_invariants(arena, position_fn, is_planar, &tol)
}

fn all_faces_planar(_face: FaceId) -> bool {
    true
}

/// Create a valid test shell hierarchy via MutableDraft.
///
/// Returns the MutableDraft with the created shell wired into:
/// `Body -> Lump -> Region -> Shell`.
pub fn insert_test_solid_shell(draft: &mut MutableDraft) -> ShellId {
    let body = draft.insert_body(BodyData::new());
    let lump = draft.insert_lump(LumpData::new(body));
    let region = draft.insert_region(RegionData::new(lump));
    let shell = draft.insert_shell(ShellData::new(
        FaceId::new(u32::MAX, 0),
        ShellKind::Solid(ShellOrientation::Outer),
        region,
    ));
    draft.arena_mut().get_body_mut(body).unwrap().add_lump(lump);
    draft.arena_mut().get_lump_mut(lump).unwrap().add_region(region);
    draft.arena_mut().get_region_mut(region).unwrap().add_shell(shell);
    shell
}

/// Materialize `EdgeData` entities from existing halfedge radial cycles.
///
/// Many older tests build synthetic halfedge loops first and wire `radial_next`
/// manually. Newer validators count edges through `halfedge.edge()`, so these
/// synthetic arenas must also create first-class `EdgeData` and assign each
/// halfedge to the edge for its radial cycle.
pub fn materialize_edge_entities_from_radials(
    draft: &mut MutableDraft,
) -> Result<(), KernelError> {
    let halfedge_ids: Vec<HalfEdgeId> = draft.arena().iter_half_edges().map(|(id, _)| id).collect();
    let mut visited: std::collections::BTreeSet<u32> = std::collections::BTreeSet::new();

    for seed in halfedge_ids {
        if !visited.insert(seed.index()) {
            continue;
        }

        let mut cycle: Vec<HalfEdgeId> = vec![seed];
        let mut current = draft.arena().get_half_edge(seed)?.radial_next();
        let bound = draft.arena().half_edge_count().max(1);
        let mut steps = 0usize;

        while current != seed {
            if !visited.insert(current.index()) {
                break;
            }
            cycle.push(current);
            current = draft.arena().get_half_edge(current)?.radial_next();
            steps += 1;
            if steps > bound {
                return Err(KernelError::InternalError {
                    message: "Radial cycle exceeded halfedge count while materializing test edges"
                        .to_string(),
                    context: None,
                });
            }
        }

        let edge_id = draft.insert_edge(EdgeData::new(seed));
        assign_edge_to_cycle(draft.arena_mut(), &cycle, edge_id)?;
    }

    Ok(())
}

fn assign_edge_to_cycle(
    arena: &mut TopologyArena,
    cycle: &[HalfEdgeId],
    edge_id: EdgeId,
) -> Result<(), KernelError> {
    for &he_id in cycle {
        arena.get_half_edge_mut(he_id)?.set_edge(edge_id);
    }
    arena.get_edge_mut(edge_id)?.set_half_edge(cycle[0]);
    Ok(())
}

/// Back-compat test-only extension methods for `MutableDraft`.
///
/// These helpers preserve the older call style in tests while the
/// insert methods are now `pub(crate)` on `TopologyArena`.
pub trait DraftTestExt {
    fn insert_face(&mut self, data: FaceData) -> FaceId;
    fn insert_half_edge(&mut self, data: HalfEdgeData) -> HalfEdgeId;
    fn insert_radial_pair(
        &mut self,
        data_a: HalfEdgeData,
        data_b: HalfEdgeData,
    ) -> (HalfEdgeId, HalfEdgeId);
    fn insert_vertex(&mut self, data: VertexData) -> VertexId;
    fn insert_loop(&mut self, data: LoopData) -> LoopId;
    fn remove_half_edge(&mut self, id: HalfEdgeId) -> Result<HalfEdgeData, KernelError>;
}

impl DraftTestExt for MutableDraft {
    fn insert_face(&mut self, data: FaceData) -> FaceId {
        MutableDraft::insert_face(self, data)
    }

    fn insert_half_edge(&mut self, data: HalfEdgeData) -> HalfEdgeId {
        MutableDraft::insert_half_edge(self, data)
    }

    fn insert_radial_pair(
        &mut self,
        data_a: HalfEdgeData,
        data_b: HalfEdgeData,
    ) -> (HalfEdgeId, HalfEdgeId) {
        MutableDraft::insert_radial_pair(self, data_a, data_b)
    }

    fn insert_vertex(&mut self, data: VertexData) -> VertexId {
        MutableDraft::insert_vertex(self, data)
    }

    fn insert_loop(&mut self, data: LoopData) -> LoopId {
        MutableDraft::insert_loop(self, data)
    }

    fn remove_half_edge(&mut self, id: HalfEdgeId) -> Result<HalfEdgeData, KernelError> {
        MutableDraft::remove_half_edge(self, id)
    }
}
