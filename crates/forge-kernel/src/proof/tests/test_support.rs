//! Test support adapters for proof-validation suites.
//!
//! Shields proof-validation tests from `forge-topo` API churn by providing
//! stable wrappers around arena mutation methods and geometric validation calls.

use forge_core::{FlatToleranceProvider, KernelError};
use crate::spatial::validate_geometric_invariants;
use forge_topo::arena::{
    BodyData, EdgeData, FaceData, HalfEdgeData, LoopData, LumpData, RegionData, ShellData,
    ShellKind, ShellOrientation, TopologyArena, VertexData,
};
use forge_topo::handles::{EdgeId, FaceId, HalfEdgeId, LoopId, ShellId, VertexId};
use forge_topo::lineage_store::LineageStore;

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

/// Create a valid test shell hierarchy in a raw `TopologyArena`.
///
/// Returns the created `ShellId` and wires:
/// `Body -> Lump -> Region -> Shell`.
pub fn insert_test_solid_shell(arena: &mut TopologyArena) -> ShellId {
    let body = TopologyArena::insert_body(arena, BodyData::new());
    let lump = TopologyArena::insert_lump(arena, LumpData::new(body));
    let region = TopologyArena::insert_region(arena, RegionData::new(lump));
    let shell = TopologyArena::insert_shell(
        arena,
        ShellData::new(
            FaceId::new(u32::MAX, 0),
            ShellKind::Solid(ShellOrientation::Outer),
            region,
        ),
        None,
    );
    arena.get_body_mut(body).unwrap().add_lump(lump);
    arena.get_lump_mut(lump).unwrap().add_region(region);
    arena.get_region_mut(region).unwrap().add_shell(shell);
    shell
}

/// Materialize `EdgeData` entities from existing halfedge radial cycles.
///
/// Many older tests build synthetic halfedge loops first and wire `radial_next`
/// manually. Newer validators count edges through `halfedge.edge()`, so these
/// synthetic arenas must also create first-class `EdgeData` and assign each
/// halfedge to the edge for its radial cycle.
pub fn materialize_edge_entities_from_radials(
    arena: &mut TopologyArena,
) -> Result<(), KernelError> {
    let halfedge_ids: Vec<HalfEdgeId> = arena.iter_half_edges().map(|(id, _)| id).collect();
    let mut visited: std::collections::BTreeSet<u32> = std::collections::BTreeSet::new();

    for seed in halfedge_ids {
        if !visited.insert(seed.index()) {
            continue;
        }

        let mut cycle: Vec<HalfEdgeId> = vec![seed];
        let mut current = arena.get_half_edge(seed)?.radial_next();
        let bound = arena.half_edge_count().max(1);
        let mut steps = 0usize;

        while current != seed {
            if !visited.insert(current.index()) {
                break;
            }
            cycle.push(current);
            current = arena.get_half_edge(current)?.radial_next();
            steps += 1;
            if steps > bound {
                return Err(KernelError::InternalError {
                    message: "Radial cycle exceeded halfedge count while materializing test edges"
                        .to_string(),
                    context: None,
                });
            }
        }

        let edge_id = TopologyArena::insert_edge(arena, EdgeData::new(seed));
        assign_edge_to_cycle(arena, &cycle, edge_id)?;
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

/// Back-compat test-only extension methods for `TopologyArena`.
///
/// These helpers preserve the older no-lineage call style in tests while the
/// arena now requires an optional `LineageStore`.
pub trait ArenaTestExt {
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

impl ArenaTestExt for TopologyArena {
    fn insert_face(&mut self, data: FaceData) -> FaceId {
        TopologyArena::insert_face(self, data, no_lineage())
    }

    fn insert_half_edge(&mut self, data: HalfEdgeData) -> HalfEdgeId {
        TopologyArena::insert_half_edge(self, data, no_lineage())
    }

    fn insert_radial_pair(
        &mut self,
        data_a: HalfEdgeData,
        data_b: HalfEdgeData,
    ) -> (HalfEdgeId, HalfEdgeId) {
        TopologyArena::insert_radial_pair(self, data_a, data_b, no_lineage())
    }

    fn insert_vertex(&mut self, data: VertexData) -> VertexId {
        TopologyArena::insert_vertex(self, data, no_lineage())
    }

    fn insert_loop(&mut self, data: LoopData) -> LoopId {
        TopologyArena::insert_loop(self, data, no_lineage())
    }

    fn remove_half_edge(&mut self, id: HalfEdgeId) -> Result<HalfEdgeData, KernelError> {
        TopologyArena::remove_half_edge(self, id, no_lineage())
    }
}

fn no_lineage() -> Option<&'static mut LineageStore> {
    None
}
