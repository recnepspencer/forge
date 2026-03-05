//! Tests for TopologyState and MutableDraft.

use std::sync::Arc;
use super::*;
use crate::provenance::{Lineage, LineageEntityRef, LineageEvent, OpSignature};
use forge_core::{EntityKind, EntityRef};
use serde_json::Value;

#[test]
fn empty_state_has_epoch_zero() {
    let state = TopologyState::empty();
    assert_eq!(state.epoch(), 0);
    assert_eq!(state.topology_version(), 0);
    assert_eq!(state.geometry_version(), 0);
}

#[test]
fn commit_increments_epoch() {
    let state = TopologyState::empty();
    let draft = state.into_mutation();
    let new_state = draft.commit().unwrap();
    assert_eq!(new_state.epoch(), 1);
}

#[test]
fn drop_without_commit_is_safe() {
    let state = TopologyState::empty();
    {
        let _draft_dropped_without_commit = state.clone().into_mutation();
    }
    assert_eq!(state.epoch(), 0);
}

#[test]
fn original_state_unchanged_after_mutation() {
    let original = TopologyState::empty();
    let draft = original.clone().into_mutation();
    let mutated = draft.commit().unwrap();

    assert_eq!(original.epoch(), 0);
    assert_eq!(mutated.epoch(), 1);
}

#[test]
fn sequential_mutations_produce_increasing_epochs() {
    let s0 = TopologyState::empty();
    let s1 = s0.clone().into_mutation().commit().unwrap();
    let s2 = s1.clone().into_mutation().commit().unwrap();
    let s3 = s2.clone().into_mutation().commit().unwrap();

    assert_eq!(s0.epoch(), 0);
    assert_eq!(s1.epoch(), 1);
    assert_eq!(s2.epoch(), 2);
    assert_eq!(s3.epoch(), 3);
}

#[test]
fn topology_hash_is_deterministic() {
    let state = TopologyState::empty();

    let first_mutation = state.clone().into_mutation().commit().unwrap();
    let second_mutation = state.into_mutation().commit().unwrap();

    assert_eq!(
        first_mutation.topology_hash(),
        second_mutation.topology_hash()
    );
}

#[test]
fn geometry_only_commit_preserves_topology_hash() {
    let state = TopologyState::empty();

    let mut draft_topo = state.into_mutation();
    draft_topo.bump_topology_version();
    let after_topo = draft_topo.commit().unwrap();

    let mut draft_geom = after_topo.clone().into_mutation();
    draft_geom.bump_geometry_version();
    let after_geom = draft_geom.commit().unwrap();

    assert_eq!(after_topo.topology_hash(), after_geom.topology_hash());
}

#[test]
fn commit_builds_reidentification_index_from_generational_lineage_events() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let lineage = Lineage::root(7, OpSignature::with_id("make_face", 1));
    draft.lineage_store.record_creation_with_snapshot(
        EntityRef::new(EntityKind::Face, 42, 3),
        LineageEntityRef::new(EntityKind::Face, 42, 3),
        lineage.clone(),
    );

    let committed = draft.commit().unwrap();
    let hits = committed
        .reidentification_link_index()
        .find_by_child_hash(lineage.get_ancestry_hash(), Some(EntityKind::Face));
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].child_snapshot.index, 42);
    assert_eq!(hits[0].child_snapshot.generation, 3);
}

#[test]
fn topology_state_serde_round_trip_preserves_reidentification_index() {
    let committed = TopologyState {
        epoch: 1,
        topology_version: 1,
        geometry_version: 0,
        topology_hash: 0,
        arena: Arc::new(crate::b_rep::TopologyArena::new()),
        lineage_events: Arc::new(Vec::new()),
        reidentification_link_index: Arc::new(crate::provenance::ReidentificationLinkIndex {
            schema_version: crate::provenance::LinkSchemaVersion::V1,
            epoch: 1,
            records: vec![crate::provenance::ReidentificationLinkRecord {
                schema_version: crate::provenance::LinkSchemaVersion::V1,
                child_snapshot: crate::provenance::TopoSnapshotHandleRef {
                    kind: EntityKind::Face,
                    index: 5,
                    generation: 2,
                },
                child_ancestry_hash: 42,
                parent_ancestry_hashes: vec![7],
                parent_linkage_mode: crate::provenance::ParentLinkageMode::Single,
                parent_snapshot: None,
                origin_kind: crate::provenance::EntityOriginKind::TopoOperator,
                creation_op_name: "make_face".to_string(),
                creation_op_invocation: 1,
                epoch: 1,
                origin_features: vec![1],
            }],
        }),
    };

    let json = serde_json::to_value(&committed).unwrap();
    let decoded: TopologyState = serde_json::from_value(json).unwrap();
    let hits = decoded
        .reidentification_link_index()
        .find_by_child_hash(42, Some(EntityKind::Face));
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].child_snapshot.generation, 2);
}

#[test]
fn topology_state_legacy_deserialize_missing_reidentification_index_defaults_empty() {
    let state = TopologyState::empty();
    let mut json = serde_json::to_value(&state).unwrap();
    if let Value::Object(map) = &mut json {
        map.remove("reidentification_link_index");
    }
    let decoded: TopologyState = serde_json::from_value(json).unwrap();
    assert!(decoded.reidentification_link_index().records.is_empty());
}

#[test]
fn commit_level_reidentification_index_order_is_deterministic_despite_event_insertion_order() {
    let make_state = |first: (u32, u32), second: (u32, u32)| {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let root = Lineage::root(1, OpSignature::with_id("root", 1));
        let child = Lineage::derive(&root, OpSignature::with_id("derive", 2));
        draft.lineage_store.record_creation_with_snapshot(
            EntityRef::new(EntityKind::Face, first.0, first.1),
            LineageEntityRef::new(EntityKind::Face, first.0, first.1),
            child.clone(),
        );
        draft.lineage_store.record_creation_with_snapshot(
            EntityRef::new(EntityKind::Face, second.0, second.1),
            LineageEntityRef::new(EntityKind::Face, second.0, second.1),
            child.clone(),
        );
        (draft.commit().unwrap(), child.get_ancestry_hash())
    };

    let (a, hash_a) = make_state((20, 2), (10, 1));
    let (b, hash_b) = make_state((10, 1), (20, 2));
    assert_eq!(hash_a, hash_b);

    let seq_a: Vec<(u32, u32)> = a
        .reidentification_link_index()
        .find_by_child_hash(hash_a, Some(EntityKind::Face))
        .into_iter()
        .map(|r| (r.child_snapshot.index, r.child_snapshot.generation))
        .collect();
    let seq_b: Vec<(u32, u32)> = b
        .reidentification_link_index()
        .find_by_child_hash(hash_b, Some(EntityKind::Face))
        .into_iter()
        .map(|r| (r.child_snapshot.index, r.child_snapshot.generation))
        .collect();
    assert_eq!(seq_a, seq_b);
}

#[test]
fn commit_persists_lineage_store_events() {
    let mut draft = TopologyState::empty().into_mutation();
    let root = Lineage::root(1, OpSignature::with_id("root", 1));
    let child = Lineage::derive(&root, OpSignature::with_id("child", 2));
    draft.lineage_store.record_creation(
        EntityRef::new(EntityKind::Face, 42, 0),
        child.clone(),
    );

    let committed = draft.commit().expect("lineage store event commit");
    assert_eq!(committed.lineage_events().len(), 1);
    match &committed.lineage_events()[0] {
        LineageEvent::EntityCreated {
            entity,
            entity_snapshot,
            lineage,
        } => {
            assert_eq!(entity.kind(), EntityKind::Face);
            assert_eq!(entity.index(), 42);
            assert!(entity_snapshot.is_none());
            assert_eq!(lineage.get_ancestry_hash(), child.get_ancestry_hash());
        }
        other => panic!("expected EntityCreated event, got {:?}", other),
    }
}

#[test]
fn commit_persists_lineage_store_events_with_snapshots() {
    let mut draft = TopologyState::empty().into_mutation();

    let store_root = Lineage::root(20, OpSignature::with_id("store_root", 1));
    let store_child = Lineage::derive(&store_root, OpSignature::with_id("store_child", 2));
    draft.lineage_store.record_creation_with_snapshot(
        EntityRef::new(EntityKind::Face, 200, 7),
        LineageEntityRef::new(EntityKind::Face, 200, 7),
        store_child.clone(),
    );

    let plain_root = Lineage::root(10, OpSignature::with_id("plain_root", 1));
    let plain_child = Lineage::derive(&plain_root, OpSignature::with_id("plain_child", 2));
    draft.lineage_store.record_creation(
        EntityRef::new(EntityKind::Face, 100, 0),
        plain_child.clone(),
    );

    let committed = draft.commit().expect("mixed lineage events commit");
    let events = committed.lineage_events();
    assert_eq!(events.len(), 2, "single-channel lineage must persist all events");

    let mut snapshot_seen = false;
    let mut plain_seen = false;
    for ev in events {
        match ev {
            LineageEvent::EntityCreated {
                entity,
                entity_snapshot,
                lineage,
            } if lineage.get_ancestry_hash() == store_child.get_ancestry_hash() => {
                assert_eq!(entity.kind(), EntityKind::Face);
                assert_eq!(entity.index(), 200);
                assert_eq!(
                    *entity_snapshot,
                    Some(LineageEntityRef::new(EntityKind::Face, 200, 7)),
                    "lineage_store event must preserve generational snapshot"
                );
                snapshot_seen = true;
            }
            LineageEvent::EntityCreated {
                entity,
                entity_snapshot,
                lineage,
            } if lineage.get_ancestry_hash() == plain_child.get_ancestry_hash() => {
                assert_eq!(entity.kind(), EntityKind::Face);
                assert_eq!(entity.index(), 100);
                assert!(
                    entity_snapshot.is_none(),
                    "plain record_creation should have no snapshot"
                );
                plain_seen = true;
            }
            _ => {}
        }
    }
    assert!(
        snapshot_seen && plain_seen,
        "must persist events from both record_creation and record_creation_with_snapshot"
    );
}

// =====================================================================
// MutationJournal Adversarial Tests
// =====================================================================

use crate::handles::{
    HalfEdgeId, LoopId, ShellId, FaceId, VertexId, EdgeId, RegionId, LumpId, BodyId,
};
use crate::b_rep::{
    FaceData, HalfEdgeData, VertexData, LoopData, EdgeData,
    ShellData, ShellKind, ShellOrientation, RegionData, LumpData, BodyData,
};

/// Helper: insert one of each entity type into the draft via proxy hooks.
/// Returns all 9 handles in a tuple for removal tests.
fn insert_all_entity_kinds(
    draft: &mut MutableDraft,
) -> (FaceId, HalfEdgeId, VertexId, LoopId, EdgeId, ShellId, RegionId, LumpId, BodyId) {
    // Use DANGLING sentinels for cross-references — we only care that
    // the proxy hooks fire, not that the topology is valid.
    let b1 = draft.insert_body(BodyData::new());
    let lu1 = draft.insert_lump(LumpData::new(b1));
    let f1 = draft.insert_face(FaceData::new(LoopId::DANGLING, ShellId::DANGLING));
    let r1 = draft.insert_region(RegionData::new(lu1));
    let s1 = draft.insert_shell(ShellData::new(
        f1,
        ShellKind::Solid(ShellOrientation::Outer),
        r1,
    ));
    let v1 = draft.insert_vertex(VertexData::new(HalfEdgeId::DANGLING));
    let e1 = draft.insert_edge(EdgeData::new(HalfEdgeId::DANGLING));
    let l1 = draft.insert_loop(LoopData::new(HalfEdgeId::DANGLING, f1));
    let he1 = draft.insert_half_edge(HalfEdgeData::new(
        HalfEdgeId::DANGLING,
        HalfEdgeId::DANGLING,
        HalfEdgeId::DANGLING,
        f1,
        v1,
        e1,
    ));
    (f1, he1, v1, l1, e1, s1, r1, lu1, b1)
}

#[test]
fn journal_records_all_creations() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    // Create one of every entity kind through the draft proxy.
    let _ = insert_all_entity_kinds(&mut draft);

    let created = draft.mutation_journal().count_created();

    assert_eq!(created.faces, 1);
    assert_eq!(created.half_edges, 1);
    assert_eq!(created.vertices, 1);
    assert_eq!(created.loops, 1);
    assert_eq!(created.edges, 1);
    assert_eq!(created.shells, 1);
    assert_eq!(created.regions, 1);
    assert_eq!(created.lumps, 1);
    assert_eq!(created.bodies, 1);
    assert_eq!(created.total(), 9);

    // Ensure no destructions were inadvertently recorded.
    assert_eq!(draft.mutation_journal().count_destroyed().total(), 0);
}

#[test]
fn journal_records_all_destructions() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let (f1, he1, v1, l1, e1, s1, r1, lu1, b1) = insert_all_entity_kinds(&mut draft);

    // Reset so we only observe destructions from this point forward.
    draft.mutation_journal_mut().reset();

    // Destroy everything.
    let _ = draft.remove_half_edge(he1);
    let _ = draft.remove_loop(l1);
    let _ = draft.remove_face(f1);
    let _ = draft.remove_vertex(v1);
    let _ = draft.remove_edge(e1);
    let _ = draft.remove_shell(s1);
    let _ = draft.remove_region(r1);
    let _ = draft.remove_lump(lu1);
    let _ = draft.remove_body(b1);

    let destroyed = draft.mutation_journal().count_destroyed();

    assert_eq!(destroyed.faces, 1);
    assert_eq!(destroyed.half_edges, 1);
    assert_eq!(destroyed.vertices, 1);
    assert_eq!(destroyed.loops, 1);
    assert_eq!(destroyed.edges, 1);
    assert_eq!(destroyed.shells, 1);
    assert_eq!(destroyed.regions, 1);
    assert_eq!(destroyed.lumps, 1);
    assert_eq!(destroyed.bodies, 1);
    assert_eq!(destroyed.total(), 9);

    assert_eq!(draft.mutation_journal().count_created().total(), 0);
}

#[derive(Debug, Clone)]
struct MockVertexCreator;
impl crate::operations::operator::TopoOperator for MockVertexCreator {
    type Output = ();
    const NAME: &'static str = "mock_vertex_creator";
    const INVARIANT_CONTRACT: crate::validators::invariant_id::InvariantContract = crate::validators::contract_registry::FULL_TOPO_WIRING;

    fn semantic_summary(&self) -> String {
        "Create one vertex".into()
    }

    fn execute(
        &self,
        draft: &mut MutableDraft,
        _recorder: &mut crate::provenance::LineageRecorder,
    ) -> Result<crate::operations::operator::ExecutionResult<Self::Output>, forge_core::KernelError> {
        let _v = draft.insert_vertex(VertexData::new(HalfEdgeId::DANGLING));
        Ok(crate::operations::operator::ExecutionResult {
            value: (),
            declared_delta: crate::operations::operator::EulerDelta {
                vertices: 1,
                ..Default::default()
            },
        })
    }
}

#[test]
fn journal_resets_between_operations() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    assert_eq!(draft.mutation_journal().creation_count(), 0);

    // First operation: creates 1 vertex.
    let _ = draft.execute(MockVertexCreator).unwrap();

    // After execute, `created` is still populated (only `destroyed` was drained
    // for auto-stamping). We can read the journal to confirm:
    assert_eq!(draft.mutation_journal().creation_count(), 1);

    // The start of the NEXT execute calls `reset()`.
    // Verify reset zeroes both vectors.
    draft.mutation_journal_mut().reset();
    assert_eq!(draft.mutation_journal().creation_count(), 0);
    assert_eq!(draft.mutation_journal().destruction_count(), 0);
}

#[test]
fn journal_counts_match_arena_delta() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let vertices_before = draft.arena().vertex_count();

    let v1 = draft.insert_vertex(VertexData::new(HalfEdgeId::DANGLING));
    let _v2 = draft.insert_vertex(VertexData::new(HalfEdgeId::DANGLING));
    let _ = draft.remove_vertex(v1);

    let vertices_after = draft.arena().vertex_count();
    let arena_net = vertices_after as i32 - vertices_before as i32;

    let j_created = draft.mutation_journal().count_created().vertices as i32;
    let j_destroyed = draft.mutation_journal().count_destroyed().vertices as i32;
    let journal_net = j_created - j_destroyed;

    // 2 created, 1 deleted → net +1.
    assert_eq!(j_created, 2);
    assert_eq!(j_destroyed, 1);
    assert_eq!(arena_net, 1);
    assert_eq!(
        journal_net, arena_net,
        "Journal net delta must precisely track the arena's objective delta"
    );
}

#[test]
fn radial_pair_records_two_creations() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let f = draft.insert_face(FaceData::new(LoopId::DANGLING, ShellId::DANGLING));
    let v = draft.insert_vertex(VertexData::new(HalfEdgeId::DANGLING));
    let e = draft.insert_edge(EdgeData::new(HalfEdgeId::DANGLING));

    // Reset: we only want to count the radial pair insertion.
    draft.mutation_journal_mut().reset();

    let _ = draft.insert_radial_pair(
        HalfEdgeData::new(
            HalfEdgeId::DANGLING, HalfEdgeId::DANGLING, HalfEdgeId::DANGLING, f, v, e,
        ),
        HalfEdgeData::new(
            HalfEdgeId::DANGLING, HalfEdgeId::DANGLING, HalfEdgeId::DANGLING, f, v, e,
        ),
    );

    let counts = draft.mutation_journal().count_created();

    assert_eq!(
        counts.half_edges, 2,
        "insert_radial_pair must record BOTH halfedges in the mutation journal"
    );
    assert_eq!(counts.total(), 2);
}
