//! Tests for TopologyState and MutableDraft.

use std::sync::Arc;
use super::*;
use crate::lineage::{Lineage, LineageEntityRef, LineageEvent, OpSignature};
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
        EntityRef::new(EntityKind::Face, 42),
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
        arena: Arc::new(crate::arena::TopologyArena::new()),
        lineage_events: Arc::new(Vec::new()),
        reidentification_link_index: Arc::new(crate::topology::history::lineage_link::ReidentificationLinkIndex {
            schema_version: crate::topology::history::lineage_link::LinkSchemaVersion::V1,
            epoch: 1,
            records: vec![crate::topology::history::lineage_link::ReidentificationLinkRecord {
                schema_version: crate::topology::history::lineage_link::LinkSchemaVersion::V1,
                child_snapshot: crate::topology::history::lineage_link::TopoSnapshotHandleRef {
                    kind: EntityKind::Face,
                    index: 5,
                    generation: 2,
                },
                child_ancestry_hash: 42,
                parent_ancestry_hashes: vec![7],
                parent_linkage_mode: crate::lineage::ParentLinkageMode::Single,
                parent_snapshot: None,
                origin_kind: crate::topology::history::lineage_link::EntityOriginKind::EulerOperator,
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
            EntityRef::new(EntityKind::Face, first.0),
            LineageEntityRef::new(EntityKind::Face, first.0, first.1),
            child.clone(),
        );
        draft.lineage_store.record_creation_with_snapshot(
            EntityRef::new(EntityKind::Face, second.0),
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
fn commit_persists_manual_lineage_log_events() {
    let mut draft = TopologyState::empty().into_mutation();
    let root = Lineage::root(1, OpSignature::with_id("root", 1));
    let child = Lineage::derive(&root, OpSignature::with_id("child", 2));
    draft.log_lineage_event(LineageEvent::EntityCreated {
        entity: EntityRef::new(EntityKind::Face, 42),
        entity_snapshot: None,
        lineage: child.clone(),
    });

    let committed = draft.commit().expect("manual lineage log event commit");
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
fn commit_persists_both_lineage_channels_deterministically() {
    let mut draft = TopologyState::empty().into_mutation();

    let manual_root = Lineage::root(10, OpSignature::with_id("manual_root", 1));
    let manual_child = Lineage::derive(&manual_root, OpSignature::with_id("manual_child", 2));
    draft.log_lineage_event(LineageEvent::EntityCreated {
        entity: EntityRef::new(EntityKind::Face, 100),
        entity_snapshot: None,
        lineage: manual_child.clone(),
    });

    let store_root = Lineage::root(20, OpSignature::with_id("store_root", 1));
    let store_child = Lineage::derive(&store_root, OpSignature::with_id("store_child", 2));
    draft.lineage_store.record_creation_with_snapshot(
        EntityRef::new(EntityKind::Face, 200),
        LineageEntityRef::new(EntityKind::Face, 200, 7),
        store_child.clone(),
    );

    let committed = draft.commit().expect("mixed lineage channels commit");
    let events = committed.lineage_events();
    assert_eq!(events.len(), 2, "both lineage channels must be persisted");

    let mut manual_seen = false;
    let mut store_seen = false;
    for ev in events {
        match ev {
            LineageEvent::EntityCreated {
                entity,
                entity_snapshot,
                lineage,
            } if lineage.get_ancestry_hash() == manual_child.get_ancestry_hash() => {
                assert_eq!(entity.kind(), EntityKind::Face);
                assert_eq!(entity.index(), 100);
                assert!(
                    entity_snapshot.is_none(),
                    "manual channel event should remain explicit legacy/none"
                );
                manual_seen = true;
            }
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
                store_seen = true;
            }
            _ => {}
        }
    }
    assert!(
        manual_seen && store_seen,
        "must persist one event from each lineage channel"
    );
}
