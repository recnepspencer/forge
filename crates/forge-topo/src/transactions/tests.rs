//! Tests for TopologyState and MutableDraft.

use super::*;
use crate::provenance::{
    Lineage, LineageEntityRef, LineageEvent, OpSignature, RollbackContract,
    RollbackContractVersion, RollbackLineageMode, RollbackStrategy,
};
use forge_core::{EntityKind, EntityRef};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

#[test]
fn empty_state_has_epoch_zero() {
    let state = TopologyState::empty();
    assert_eq!(state.epoch(), 0);
    assert_eq!(state.topology_version(), 0);
    assert_eq!(state.geometry_version(), 0);
}

#[test]
fn rollback_contract_is_locked_to_snapshot_restore_v1() {
    let contract = RollbackContract::CURRENT;
    assert_eq!(contract.version, RollbackContractVersion::V1);
    assert_eq!(contract.strategy, RollbackStrategy::SnapshotRestore);
    assert_eq!(contract.lineage_mode, RollbackLineageMode::Reverted);
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
    draft
        .lineage_store
        .record_creation(EntityRef::new(EntityKind::Face, 42, 0), child.clone());

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
    assert_eq!(
        events.len(),
        2,
        "single-channel lineage must persist all events"
    );

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

use crate::b_rep::{
    BodyData, EdgeData, FaceData, HalfEdgeData, LoopData, LumpData, RegionData, ShellData,
    ShellKind, ShellOrientation, VertexData,
};
use crate::handles::{
    BodyId, EdgeId, FaceId, HalfEdgeId, LoopId, LumpId, RegionId, ShellId, VertexId,
};

/// Helper: insert one of each entity type into the draft via proxy hooks.
/// Returns all 9 handles in a tuple for removal tests.
fn insert_all_entity_kinds(
    draft: &mut MutableDraft,
) -> (
    FaceId,
    HalfEdgeId,
    VertexId,
    LoopId,
    EdgeId,
    ShellId,
    RegionId,
    LumpId,
    BodyId,
) {
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
    const INVARIANT_CONTRACT: crate::validators::invariant_id::InvariantContract =
        crate::validators::contract_registry::FULL_TOPO_WIRING;

    fn semantic_summary(&self) -> String {
        "Create one vertex".into()
    }

    fn execute(
        &self,
        draft: &mut MutableDraft,
        _recorder: &mut crate::provenance::LineageRecorder,
    ) -> Result<crate::operations::operator::ExecutionResult<Self::Output>, forge_core::KernelError>
    {
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
            HalfEdgeId::DANGLING,
            HalfEdgeId::DANGLING,
            HalfEdgeId::DANGLING,
            f,
            v,
            e,
        ),
        HalfEdgeData::new(
            HalfEdgeId::DANGLING,
            HalfEdgeId::DANGLING,
            HalfEdgeId::DANGLING,
            f,
            v,
            e,
        ),
    );

    let counts = draft.mutation_journal().count_created();

    assert_eq!(
        counts.half_edges, 2,
        "insert_radial_pair must record BOTH halfedges in the mutation journal"
    );
    assert_eq!(counts.total(), 2);
}

fn run_replay_determinism_pipeline_from_state(
    state: TopologyState,
) -> (
    TopologyState,
    crate::provenance::ReplayLog,
    Vec<u8>,
    u128,
    Vec<String>,
    crate::b_rep::data::storage::cache_runtime::TopoCacheTelemetry,
) {
    use crate::b_rep::ShellKind;
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::entity_lifecycle::split_edge::SplitEdge;

    let mut draft = state.into_mutation();

    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let se1 = draft
        .execute(SplitEdge {
            edge: mvf.half_edge,
        })
        .unwrap()
        .into_value();
    let _se2 = draft
        .execute(SplitEdge { edge: se1.he_mb })
        .unwrap()
        .into_value();

    let replay_log = draft.replay_log().clone();
    let replay_bytes = serde_json::to_vec(&replay_log).unwrap();
    let cache_telemetry = draft.arena().cache_telemetry().clone();
    let committed = draft.commit().unwrap();

    let lineage_sequence = committed
        .lineage_events()
        .iter()
        .map(|ev| match ev {
            LineageEvent::EntityCreated {
                entity,
                entity_snapshot,
                lineage,
            } => format!(
                "C:{:?}:{}:{}:{}:{}",
                entity.kind(),
                entity.index(),
                entity.generation(),
                entity_snapshot.map(|s| s.generation()).unwrap_or(0),
                lineage.get_ancestry_hash()
            ),
            LineageEvent::EntityDeleted {
                entity,
                entity_snapshot,
                lineage,
            } => format!(
                "D:{:?}:{}:{}:{}:{}",
                entity.kind(),
                entity.index(),
                entity.generation(),
                entity_snapshot.map(|s| s.generation()).unwrap_or(0),
                lineage.get_ancestry_hash()
            ),
            LineageEvent::EntityModified {
                entity,
                entity_snapshot,
                old_lineage,
                new_lineage,
            } => format!(
                "M:{:?}:{}:{}:{}:{}:{}",
                entity.kind(),
                entity.index(),
                entity.generation(),
                entity_snapshot.map(|s| s.generation()).unwrap_or(0),
                old_lineage.get_ancestry_hash(),
                new_lineage.get_ancestry_hash()
            ),
            LineageEvent::EntityReverted {
                entity,
                entity_snapshot,
                from_lineage,
                to_lineage,
            } => format!(
                "R:{:?}:{}:{}:{}:{}:{}",
                entity.kind(),
                entity.index(),
                entity.generation(),
                entity_snapshot.map(|s| s.generation()).unwrap_or(0),
                from_lineage.get_ancestry_hash(),
                to_lineage.get_ancestry_hash()
            ),
        })
        .collect();

    (
        committed.clone(),
        replay_log,
        replay_bytes,
        committed.topology_hash(),
        lineage_sequence,
        cache_telemetry,
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReplayDeterminismArtifact {
    pre_state: TopologyState,
    post_state: TopologyState,
    replay_log: crate::provenance::ReplayLog,
}

#[test]
fn determinism_golden_pipeline_roundtrip_preserves_replay_hash_and_lineage_ordering() {
    let pre = TopologyState::empty();
    let (post_a, replay_log_a, replay_a, hash_a, lineage_a, cache_a) =
        run_replay_determinism_pipeline_from_state(pre.clone());

    let artifact = ReplayDeterminismArtifact {
        pre_state: pre,
        post_state: post_a.clone(),
        replay_log: replay_log_a.clone(),
    };
    let blob = serde_json::to_vec(&artifact).expect("serialize determinism artifact");
    let decoded: ReplayDeterminismArtifact =
        serde_json::from_slice(&blob).expect("deserialize determinism artifact");

    let (post_b, replay_log_b, replay_b, hash_b, lineage_b, cache_b) =
        run_replay_determinism_pipeline_from_state(decoded.pre_state.clone());
    let (_post_c, _replay_log_c, replay_c, hash_c, lineage_c, cache_c) =
        run_replay_determinism_pipeline_from_state(decoded.pre_state.clone());

    let replay_decoded = serde_json::to_vec(&decoded.replay_log).expect("encode decoded replay");
    let lineage_decoded: Vec<String> = decoded
        .post_state
        .lineage_events()
        .iter()
        .map(|ev| match ev {
            LineageEvent::EntityCreated {
                entity,
                entity_snapshot,
                lineage,
            } => format!(
                "C:{:?}:{}:{}:{}:{}",
                entity.kind(),
                entity.index(),
                entity.generation(),
                entity_snapshot.map(|s| s.generation()).unwrap_or(0),
                lineage.get_ancestry_hash()
            ),
            LineageEvent::EntityDeleted {
                entity,
                entity_snapshot,
                lineage,
            } => format!(
                "D:{:?}:{}:{}:{}:{}",
                entity.kind(),
                entity.index(),
                entity.generation(),
                entity_snapshot.map(|s| s.generation()).unwrap_or(0),
                lineage.get_ancestry_hash()
            ),
            LineageEvent::EntityModified {
                entity,
                entity_snapshot,
                old_lineage,
                new_lineage,
            } => format!(
                "M:{:?}:{}:{}:{}:{}:{}",
                entity.kind(),
                entity.index(),
                entity.generation(),
                entity_snapshot.map(|s| s.generation()).unwrap_or(0),
                old_lineage.get_ancestry_hash(),
                new_lineage.get_ancestry_hash()
            ),
            LineageEvent::EntityReverted {
                entity,
                entity_snapshot,
                from_lineage,
                to_lineage,
            } => format!(
                "R:{:?}:{}:{}:{}:{}:{}",
                entity.kind(),
                entity.index(),
                entity.generation(),
                entity_snapshot.map(|s| s.generation()).unwrap_or(0),
                from_lineage.get_ancestry_hash(),
                to_lineage.get_ancestry_hash()
            ),
        })
        .collect();

    assert_eq!(replay_a, replay_b, "Replay log bytes must be identical");
    assert_eq!(
        replay_a, replay_c,
        "Replay log bytes must remain stable across repeated runs"
    );
    assert_eq!(
        replay_a, replay_decoded,
        "Serialized replay log in artifact must match rerun bytes"
    );
    assert_eq!(hash_a, hash_b, "Topology hash must be identical");
    assert_eq!(
        hash_a, hash_c,
        "Topology hash must remain stable across repeated runs"
    );
    assert_eq!(
        hash_a,
        decoded.post_state.topology_hash(),
        "Serialized post-state hash must match rerun hash"
    );
    assert!(
        replay_log_a.verify_determinism(&replay_log_b),
        "Replay determinism verifier must pass for reruns"
    );
    let cache_trace_a: Vec<Vec<String>> = replay_log_a
        .entries()
        .iter()
        .map(|e| e.cache_refresh_trace().to_vec())
        .collect();
    let cache_trace_b: Vec<Vec<String>> = replay_log_b
        .entries()
        .iter()
        .map(|e| e.cache_refresh_trace().to_vec())
        .collect();
    assert_eq!(
        cache_trace_a, cache_trace_b,
        "Cache refresh trace ordering/content must be deterministic"
    );
    assert!(
        cache_trace_a.iter().all(|entry| !entry.is_empty()),
        "Each operation should carry a non-empty cache refresh trace"
    );
    assert_eq!(
        post_a.topology_hash(),
        post_b.topology_hash(),
        "Committed post-state hashes must match"
    );
    assert_eq!(
        lineage_a, lineage_b,
        "Lineage event ordering must be identical"
    );
    assert_eq!(
        lineage_a, lineage_c,
        "Lineage event ordering must remain stable across repeated runs"
    );
    assert_eq!(
        lineage_a, lineage_decoded,
        "Serialized post-state lineage ordering must match rerun ordering"
    );
    assert_eq!(cache_a.flushes_by_domain, cache_b.flushes_by_domain);
    assert_eq!(cache_a.flushes_by_domain, cache_c.flushes_by_domain);
    assert!(
        cache_a
            .global_invalidations_by_domain
            .values()
            .all(|count| *count == 0),
        "determinism pipeline must not rely on global cache invalidation"
    );
}

#[test]
fn event_bus_wiring_with_real_operator_emits_lifecycle_events() {
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::transactions::{TopoOperationEvent, TopoSubscriberDataId};
    use forge_signal::facade::{EventSubscriber, SubscriberContext, SubscriberId};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct CountingSubscriber {
        started: Arc<AtomicUsize>,
        completed: Arc<AtomicUsize>,
        checkpoints: Arc<AtomicUsize>,
    }

    impl EventSubscriber for CountingSubscriber {
        type Event = TopoOperationEvent;
        type DataId = TopoSubscriberDataId;
        type RuntimeContext = crate::transactions::logic::mutable_draft::MutableDraft;

        fn id(&self) -> SubscriberId {
            SubscriberId::new(10)
        }

        fn name(&self) -> &'static str {
            "counting_subscriber"
        }

        fn requires(&self) -> &'static [TopoSubscriberDataId] {
            &[]
        }

        fn provides(&self) -> &'static [TopoSubscriberDataId] {
            &[]
        }

        fn on_event(&mut self, event: &Self::Event) {
            match event {
                TopoOperationEvent::OperationStarted { .. } => {
                    self.started.fetch_add(1, Ordering::SeqCst);
                }
                TopoOperationEvent::OperationCompleted { .. } => {
                    self.completed.fetch_add(1, Ordering::SeqCst);
                }
                _ => {}
            }
        }

        fn on_checkpoint(
            &mut self,
            _barrier: forge_signal::facade::CheckpointBarrier,
            _ctx: &mut SubscriberContext<TopoSubscriberDataId>,
            _runtime: &mut Self::RuntimeContext,
        ) -> Result<(), forge_signal::facade::SignalError> {
            self.checkpoints.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    let started = Arc::new(AtomicUsize::new(0));
    let completed = Arc::new(AtomicUsize::new(0));
    let checkpoints = Arc::new(AtomicUsize::new(0));

    let mut draft = TopologyState::empty().into_mutation();
    draft
        .event_bus_mut()
        .subscribe(Box::new(CountingSubscriber {
            started: started.clone(),
            completed: completed.clone(),
            checkpoints: checkpoints.clone(),
        }))
        .unwrap();

    draft
        .execute(MakeVertexFace {
            shell_kind: crate::b_rep::ShellKind::Sheet,
        })
        .expect("operator should execute under event bus wiring");
    let _committed = draft.commit().expect("commit should succeed");

    assert_eq!(started.load(Ordering::SeqCst), 1);
    assert_eq!(completed.load(Ordering::SeqCst), 1);
    assert!(
        checkpoints.load(Ordering::SeqCst) >= 1,
        "at least one checkpoint flush should have occurred"
    );
}

#[test]
fn operation_subscribers_stage_expected_outputs_after_execute() {
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::transactions::{
        EulerDeltaCheck, LineageSummary, MutationCounts, OperationArtifacts, ReplayStats,
        ValidationSummary, VersionCounters,
    };

    let mut draft = TopologyState::empty().into_mutation();
    draft
        .execute(MakeVertexFace {
            shell_kind: crate::b_rep::ShellKind::Sheet,
        })
        .expect("operator should execute");

    let journal_created = draft.mutation_journal().count_created();
    let journal_deleted = draft.mutation_journal().count_destroyed();
    let context = draft.event_bus_mut().context();
    let mutation = context
        .committed::<MutationCounts>(crate::transactions::TopoSubscriberDataId::MutationCounts)
        .expect("missing mutation operation output");
    assert_eq!(mutation.created, journal_created);
    assert_eq!(mutation.destroyed, journal_deleted);

    let versions = context
        .committed::<VersionCounters>(crate::transactions::TopoSubscriberDataId::VersionCounters)
        .expect("missing version operation output");
    assert_eq!(versions.topology_bumps, 1);
    assert_eq!(versions.geometry_bumps, 0);

    let replay = context
        .committed::<ReplayStats>(
            crate::transactions::TopoSubscriberDataId::ReplayEntryFinalization,
        )
        .expect("missing replay operation output");
    assert_eq!(replay.op_starts, 1);
    assert_eq!(replay.entry_records, 1);
    assert_eq!(replay.cache_trace_updates, 1);

    let euler = context
        .committed::<EulerDeltaCheck>(crate::transactions::TopoSubscriberDataId::EulerDeltaResult)
        .expect("missing euler operation output");
    assert!(euler.matched);

    let validation = context
        .committed::<ValidationSummary>(crate::transactions::TopoSubscriberDataId::ValidationResult)
        .expect("missing validation operation output");
    assert_eq!(validation.checks_failed, 0);

    let lineage = context
        .committed::<LineageSummary>(crate::transactions::TopoSubscriberDataId::LineageEvents)
        .expect("missing lineage operation output");
    assert_eq!(lineage.deletions_seen, 0);
    assert_eq!(lineage.deletions_stamped, 0);

    let artifacts = context
        .committed::<OperationArtifacts>(
            crate::transactions::TopoSubscriberDataId::OperationMetrics,
        )
        .expect("missing operation-artifacts operation output");
    assert_eq!(artifacts.entities_created, journal_created.total());
    assert_eq!(artifacts.entities_deleted, journal_deleted.total());
}

#[test]
fn operation_result_artifacts_are_sourced_from_subscriber_outputs() {
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;

    let mut draft = TopologyState::empty().into_mutation();
    let op_result = draft
        .execute(MakeVertexFace {
            shell_kind: crate::b_rep::ShellKind::Sheet,
        })
        .expect("operator should execute");

    let created = draft.mutation_journal().count_created();
    let deleted = draft.mutation_journal().count_destroyed();
    let metrics = op_result.get_metrics();
    let lineage_delta = op_result.get_lineage_delta();

    assert_eq!(metrics.entities_created, created.total());
    assert_eq!(metrics.entities_deleted, deleted.total());
    assert_eq!(lineage_delta.faces_created, created.faces);
    assert_eq!(lineage_delta.faces_deleted, deleted.faces);
}

#[test]
fn subscriber_checkpoint_failure_poisons_draft_and_drop_is_safe() {
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::transactions::{TopoOperationEvent, TopoSubscriberDataId};
    use forge_signal::facade::{EventSubscriber, SubscriberContext, SubscriberId};

    struct FailingSubscriber;

    impl EventSubscriber for FailingSubscriber {
        type Event = TopoOperationEvent;
        type DataId = TopoSubscriberDataId;
        type RuntimeContext = crate::transactions::logic::mutable_draft::MutableDraft;

        fn id(&self) -> SubscriberId {
            SubscriberId::new(11)
        }

        fn name(&self) -> &'static str {
            "failing_subscriber"
        }

        fn requires(&self) -> &'static [TopoSubscriberDataId] {
            &[]
        }

        fn provides(&self) -> &'static [TopoSubscriberDataId] {
            &[]
        }

        fn on_event(&mut self, _event: &Self::Event) {}

        fn on_checkpoint(
            &mut self,
            _barrier: forge_signal::facade::CheckpointBarrier,
            _ctx: &mut SubscriberContext<TopoSubscriberDataId>,
            _runtime: &mut Self::RuntimeContext,
        ) -> Result<(), forge_signal::facade::SignalError> {
            Err(forge_signal::facade::SignalError::internal(
                "intentional subscriber failure",
            ))
        }
    }

    let mut draft = TopologyState::empty().into_mutation();
    draft
        .event_bus_mut()
        .subscribe(Box::new(FailingSubscriber))
        .unwrap();

    let err = match draft.execute(MakeVertexFace {
        shell_kind: crate::b_rep::ShellKind::Sheet,
    }) {
        Ok(_) => panic!("expected subscriber checkpoint failure"),
        Err(err) => err,
    };
    let err_msg = err.to_string();
    assert!(
        err_msg.contains("Event bus checkpoint failed"),
        "unexpected error message: {err_msg}"
    );

    // Subsequent execution is blocked by poison flag.
    let second = draft.execute(MakeVertexFace {
        shell_kind: crate::b_rep::ShellKind::Sheet,
    });
    assert!(second.is_err());

    // Drop occurs at end of scope; test passes if no panic.
}

#[test]
fn rollback_callbacks_fire_once_when_execute_fails_then_draft_drops() {
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::transactions::{TopoOperationEvent, TopoSubscriberDataId};
    use forge_signal::facade::{EventSubscriber, SubscriberContext, SubscriberId};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct RollbackCounter {
        rollbacks: Arc<AtomicUsize>,
    }

    impl EventSubscriber for RollbackCounter {
        type Event = TopoOperationEvent;
        type DataId = TopoSubscriberDataId;
        type RuntimeContext = crate::transactions::logic::mutable_draft::MutableDraft;

        fn id(&self) -> SubscriberId {
            SubscriberId::new(12)
        }

        fn name(&self) -> &'static str {
            "rollback_counter"
        }

        fn requires(&self) -> &'static [TopoSubscriberDataId] {
            &[]
        }

        fn provides(&self) -> &'static [TopoSubscriberDataId] {
            &[]
        }

        fn on_event(&mut self, _event: &Self::Event) {}

        fn on_checkpoint(
            &mut self,
            _barrier: forge_signal::facade::CheckpointBarrier,
            _ctx: &mut SubscriberContext<TopoSubscriberDataId>,
            _runtime: &mut Self::RuntimeContext,
        ) -> Result<(), forge_signal::facade::SignalError> {
            Err(forge_signal::facade::SignalError::internal(
                "force rollback",
            ))
        }

        fn on_rollback(&mut self, _runtime: &mut Self::RuntimeContext) {
            self.rollbacks.fetch_add(1, Ordering::SeqCst);
        }
    }

    let rollback_count = Arc::new(AtomicUsize::new(0));
    {
        let mut draft = TopologyState::empty().into_mutation();
        draft
            .event_bus_mut()
            .subscribe(Box::new(RollbackCounter {
                rollbacks: rollback_count.clone(),
            }))
            .unwrap();

        let result = draft.execute(MakeVertexFace {
            shell_kind: crate::b_rep::ShellKind::Sheet,
        });
        assert!(result.is_err(), "subscriber failure should fail execute");
        // Draft drops here without commit.
    }

    assert_eq!(
        rollback_count.load(Ordering::SeqCst),
        1,
        "rollback callbacks must run exactly once per failed draft lifecycle"
    );
}

#[test]
fn topo_event_bus_rollback_honors_reverse_dependency_order() {
    use crate::transactions::{TopoOperationEvent, TopoSubscriberDataId};
    use forge_signal::facade::{
        CheckpointBarrier, EventBus, EventSubscriber, SubscriberContext, SubscriberId,
    };
    use std::sync::{Arc, Mutex};

    struct OrderedRollback {
        id: SubscriberId,
        name: &'static str,
        requires: &'static [TopoSubscriberDataId],
        provides: &'static [TopoSubscriberDataId],
        out: Arc<Mutex<Vec<&'static str>>>,
    }

    impl EventSubscriber for OrderedRollback {
        type Event = TopoOperationEvent;
        type DataId = TopoSubscriberDataId;
        type RuntimeContext = ();

        fn id(&self) -> SubscriberId {
            self.id
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn requires(&self) -> &'static [TopoSubscriberDataId] {
            self.requires
        }

        fn provides(&self) -> &'static [TopoSubscriberDataId] {
            self.provides
        }

        fn on_event(&mut self, _event: &Self::Event) {}

        fn on_checkpoint(
            &mut self,
            _barrier: CheckpointBarrier,
            _ctx: &mut SubscriberContext<TopoSubscriberDataId>,
            _runtime: &mut Self::RuntimeContext,
        ) -> Result<(), forge_signal::facade::SignalError> {
            Ok(())
        }

        fn on_rollback(&mut self, _runtime: &mut Self::RuntimeContext) {
            self.out
                .lock()
                .expect("rollback log poisoned")
                .push(self.name);
        }
    }

    let log: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
    let mut bus: EventBus<TopoOperationEvent, TopoSubscriberDataId> = EventBus::new();
    bus.subscribe(Box::new(OrderedRollback {
        id: SubscriberId::new(1001),
        name: "a",
        requires: &[],
        provides: &[TopoSubscriberDataId::MutationCounts],
        out: log.clone(),
    }))
    .expect("register a");
    bus.subscribe(Box::new(OrderedRollback {
        id: SubscriberId::new(1002),
        name: "b",
        requires: &[TopoSubscriberDataId::MutationCounts],
        provides: &[TopoSubscriberDataId::VersionCounters],
        out: log.clone(),
    }))
    .expect("register b");
    bus.subscribe(Box::new(OrderedRollback {
        id: SubscriberId::new(1003),
        name: "c",
        requires: &[TopoSubscriberDataId::VersionCounters],
        provides: &[TopoSubscriberDataId::TopologyHash],
        out: log.clone(),
    }))
    .expect("register c");

    bus.finalize_registration()
        .expect("dependency DAG should be valid");
    let mut runtime = ();
    bus.rollback(&mut runtime);

    assert_eq!(
        &*log.lock().expect("rollback log poisoned"),
        &["c", "b", "a"]
    );
}

#[test]
fn rollback_failure_keeps_previous_committed_operation_outputs_intact() {
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::transactions::{MutationCounts, TopoOperationEvent, TopoSubscriberDataId};
    use forge_signal::facade::{EventSubscriber, SubscriberContext, SubscriberId};

    struct FailingSubscriber;

    impl EventSubscriber for FailingSubscriber {
        type Event = TopoOperationEvent;
        type DataId = TopoSubscriberDataId;
        type RuntimeContext = crate::transactions::logic::mutable_draft::MutableDraft;

        fn id(&self) -> SubscriberId {
            SubscriberId::new(2000)
        }

        fn name(&self) -> &'static str {
            "failing_after_first_success"
        }

        fn requires(&self) -> &'static [TopoSubscriberDataId] {
            &[]
        }

        fn provides(&self) -> &'static [TopoSubscriberDataId] {
            &[]
        }

        fn on_event(&mut self, _event: &Self::Event) {}

        fn on_checkpoint(
            &mut self,
            _barrier: forge_signal::facade::CheckpointBarrier,
            _ctx: &mut SubscriberContext<TopoSubscriberDataId>,
            _runtime: &mut Self::RuntimeContext,
        ) -> Result<(), forge_signal::facade::SignalError> {
            Err(forge_signal::facade::SignalError::internal(
                "force rollback on checkpoint",
            ))
        }
    }

    let mut draft = TopologyState::empty().into_mutation();
    draft
        .execute(MakeVertexFace {
            shell_kind: crate::b_rep::ShellKind::Sheet,
        })
        .expect("first operation must succeed");
    let baseline_output = draft
        .event_bus_mut()
        .context()
        .committed::<MutationCounts>(TopoSubscriberDataId::MutationCounts)
        .expect("baseline mutation output missing")
        .clone();

    draft
        .event_bus_mut()
        .subscribe(Box::new(FailingSubscriber))
        .expect("register failing subscriber");

    let result = draft.execute(MakeVertexFace {
        shell_kind: crate::b_rep::ShellKind::Sheet,
    });
    assert!(
        result.is_err(),
        "second operation should fail at checkpoint"
    );

    let post_failure_output = draft
        .event_bus_mut()
        .context()
        .committed::<MutationCounts>(TopoSubscriberDataId::MutationCounts)
        .expect("post-failure mutation output missing")
        .clone();

    assert_eq!(
        post_failure_output, baseline_output,
        "rollback path must not mutate last committed operation outputs"
    );
}
