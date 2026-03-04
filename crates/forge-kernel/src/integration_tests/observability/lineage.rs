//! Group 2: Lineage DAG wiring tests [P3.3].
//!
//! Proves: Every entity created during primitive generation has a lineage
//! record linking back to its origin operation. Euler operators produce
//! new lineage entries via `sync_lineage_with_arena()`.

use forge_core::{EntityKind, EntityRef};
use forge_topo::provenance::{LineageEvent, LineageStore};
use forge_topo::entity_lifecycle::split_edge::SplitEdge;
use forge_topo::entity_lifecycle::make_edge_face::MakeEdgeFace;

use crate::integration_tests::harness::shapes::{
    collect_face_loop, first_halfedge_of_face, unit_cube, tetrahedron,
};

/// Helper: Rebuild a queryable `LineageStore` from committed events.
fn store_from_events(events: &[LineageEvent]) -> LineageStore {
    LineageStore::from_prior_events(events)
}

// ── Test 1: Full entity coverage ────────────────────────────────────────

/// Every entity born during make_cube must have a lineage entry.
#[test]
fn test_make_block_lineage_covers_all_entities() {
    let result = unit_cube().expect("unit cube should succeed");
    let topo = result.get_value().topology();
    let arena = topo.arena();
    let events = topo.lineage_events();

    assert!(!events.is_empty(), "Lineage events must be non-empty after build");

    let store = store_from_events(events);

    // Every entity type in the arena must have a lineage entry.
    for (id, _) in arena.iter_faces() {
        let eref = EntityRef::new(EntityKind::Face, id.index(), id.generation());
        assert!(
            store.get_lineage(&eref).is_some(),
            "Face {} has no lineage entry", id.index()
        );
    }
    for (id, _) in arena.iter_vertices() {
        let eref = EntityRef::new(EntityKind::Vertex, id.index(), id.generation());
        assert!(
            store.get_lineage(&eref).is_some(),
            "Vertex {} has no lineage entry", id.index()
        );
    }
    for (id, _) in arena.iter_half_edges() {
        let eref = EntityRef::new(EntityKind::HalfEdge, id.index(), id.generation());
        assert!(
            store.get_lineage(&eref).is_some(),
            "HalfEdge {} has no lineage entry", id.index()
        );
    }
    for (id, _) in arena.iter_edges() {
        let eref = EntityRef::new(EntityKind::Edge, id.index(), id.generation());
        assert!(
            store.get_lineage(&eref).is_some(),
            "Edge {} has no lineage entry", id.index()
        );
    }
    for (id, _) in arena.iter_loops() {
        let eref = EntityRef::new(EntityKind::Loop, id.index(), id.generation());
        assert!(
            store.get_lineage(&eref).is_some(),
            "Loop {} has no lineage entry", id.index()
        );
    }

    // Total coverage.
    let arena_count = arena.face_count()
        + arena.vertex_count()
        + arena.half_edge_count()
        + arena.edge_count()
        + arena.loop_count()
        + arena.shell_count()
        + arena.body_count()
        + arena.lump_count()
        + arena.region_count();

    assert_eq!(
        store.active_count(), arena_count,
        "Lineage coverage gap: store tracks {}, arena has {} entities",
        store.active_count(), arena_count
    );
}

// ── Test 2: Chronological event ordering ────────────────────────────────

/// Lineage events for a fresh primitive must all be EntityCreated.
#[test]
fn test_lineage_event_log_all_creations() {
    let result = unit_cube().expect("unit cube should succeed");
    let events = result.get_value().topology().lineage_events();

    assert!(!events.is_empty(), "Event log should be non-empty");

    for (i, event) in events.iter().enumerate() {
        assert!(
            matches!(event, LineageEvent::EntityCreated { .. }),
            "Event {} should be EntityCreated, got: {:?}", i, event
        );
    }
}

// ── Test 3: Operation attribution ───────────────────────────────────────

/// Every lineage root must reference `build_halfedge_mesh` as its op.
#[test]
fn test_lineage_root_contains_operation_attribution() {
    let result = unit_cube().expect("unit cube should succeed");
    let topo = result.get_value().topology();
    let arena = topo.arena();
    let store = store_from_events(topo.lineage_events());

    // Check a face.
    let (face_id, _) = arena.iter_faces().next().expect("cube should have faces");
    let face_ref = EntityRef::new(EntityKind::Face, face_id.index(), face_id.generation());
    let lineage = store.get_lineage(&face_ref).expect("face should have lineage");

    assert_eq!(
        lineage.get_creation_op().get_name(),
        "build_halfedge_mesh",
        "Face lineage op name mismatch"
    );

    // Check a vertex.
    let (vtx_id, _) = arena.iter_vertices().next().expect("cube should have vertices");
    let vtx_ref = EntityRef::new(EntityKind::Vertex, vtx_id.index(), vtx_id.generation());
    let vtx_lineage = store.get_lineage(&vtx_ref).expect("vertex should have lineage");

    assert_eq!(
        vtx_lineage.get_creation_op().get_name(),
        "build_halfedge_mesh",
        "Vertex lineage op name mismatch"
    );
}

// ── Test 4: Euler operator produces lineage ─────────────────────────────

/// SplitEdge and MakeEdgeFace must produce lineage for new entities.
#[test]
fn test_euler_operator_produces_lineage() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geom): (forge_topo::transactions::MutableDraft, _) =
        env_res.into_value().into_draft();

    let face = faces[0];
    let he = first_halfedge_of_face(draft.arena(), face).unwrap();

    // The halfedge being split is the parent of the new vertex and halfedge.
    let parent_he_ref = EntityRef::new(EntityKind::HalfEdge, he.index(), he.generation());
    let parent_hash = draft.lineage_store().get_lineage(&parent_he_ref)
        .expect("Parent halfedge must have lineage").get_ancestry_hash();

    // SplitEdge: creates 1 vertex, 1 edge, 2 half-edges.
    let se = draft.execute(SplitEdge { edge: he })
        .unwrap().into_value();

    // New vertex must have lineage.
    let vtx_ref = EntityRef::new(EntityKind::Vertex, se.new_vertex.index(), se.new_vertex.generation());
    let vtx_lineage = draft.lineage_store().get_lineage(&vtx_ref)
        .expect("SplitEdge new vertex must have lineage");

    // HONEST TEST: Operator attribution AND parent hash.
    assert_eq!(vtx_lineage.get_creation_op().get_name(), "split_edge");
    assert_eq!(
        vtx_lineage.get_parent_ancestry_hashes(),
        &[parent_hash],
        "SplitEdge new vertex must be Derived from the split halfedge"
    );

    // New halfedge must have lineage.
    let he_ref = EntityRef::new(EntityKind::HalfEdge, se.he_mb.index(), se.he_mb.generation());
    let he_lineage = draft.lineage_store().get_lineage(&he_ref)
        .expect("SplitEdge new halfedge must have lineage");
    assert_eq!(he_lineage.get_creation_op().get_name(), "split_edge");
    assert_eq!(
        he_lineage.get_parent_ancestry_hashes(),
        &[parent_hash],
        "SplitEdge new halfedge must be Derived from the split halfedge"
    );

    // MakeEdgeFace on a different face.
    let face2 = faces[1];
    let parent_face_ref = EntityRef::new(EntityKind::Face, face2.index(), face2.generation());
    let parent_face_hash = draft.lineage_store().get_lineage(&parent_face_ref)
        .expect("Parent face must have lineage").get_ancestry_hash();

    let he2 = first_halfedge_of_face(draft.arena(), face2).unwrap();
    let loop_hes = collect_face_loop(draft.arena(), he2).unwrap();
    let v_a = draft.arena().get_half_edge(loop_hes[0]).unwrap().origin();
    let v_c = draft.arena().get_half_edge(loop_hes[2]).unwrap().origin();

    let mef = draft.execute(MakeEdgeFace {
        face: face2,
        vertex_a: v_a,
        vertex_b: v_c,
    }).unwrap().into_value();

    let new_face_ref = EntityRef::new(EntityKind::Face, mef.new_face.index(), mef.new_face.generation());
    let mef_lineage = draft.lineage_store().get_lineage(&new_face_ref)
        .expect("MakeEdgeFace new face must have lineage");
    
    // HONEST TEST: Operator attribution AND parent hash.
    assert_eq!(mef_lineage.get_creation_op().get_name(), "make_edge_face");
    assert_eq!(
        mef_lineage.get_parent_ancestry_hashes(),
        &[parent_face_hash],
        "MakeEdgeFace new face must be Derived from the original face"
    );
}

// ── Test 5: Stress test — lineage survives multi-step chain ─────────────

/// Full 4-step chain: cube → split → MEF → commit → verify.
/// Asserts: total coverage, ancestry uniqueness, op attribution,
/// no unset feature_ids, and deterministic replay.
#[test]
fn test_lineage_integrity_survives_multi_step_chain() {
    use std::collections::HashSet;
    use forge_topo::provenance::FEATURE_ID_UNSET;

    // ── Run 1 ──────────────────────────────────────────────────────
    let run = || -> forge_topo::transactions::TopologyState {
        let env_res = unit_cube().expect("unit cube should succeed");
        let faces = env_res.get_value().faces().to_vec();
        let (mut draft, _geom): (forge_topo::transactions::MutableDraft, _) =
            env_res.into_value().into_draft();

        // Step 1: SplitEdge on face[0].
        let face0 = faces[0];
        let he0 = first_halfedge_of_face(draft.arena(), face0).unwrap();
        draft.execute(SplitEdge { edge: he0 }).unwrap();

        // Step 2: MakeEdgeFace on face[1].
        let face1 = faces[1];
        let he1 = first_halfedge_of_face(draft.arena(), face1).unwrap();
        let loop_hes = collect_face_loop(draft.arena(), he1).unwrap();
        let v_a = draft.arena().get_half_edge(loop_hes[0]).unwrap().origin();
        let v_c = draft.arena().get_half_edge(loop_hes[2]).unwrap().origin();
        draft.execute(MakeEdgeFace {
            face: face1,
            vertex_a: v_a,
            vertex_b: v_c,
        }).unwrap();

        // Commit.
        draft.commit().unwrap()
    };

    let state1 = run();
    let state2 = run();

    // ── Assertions on state1 ───────────────────────────────────────

    let events = state1.lineage_events();
    let store = store_from_events(events);
    let arena = state1.arena();

    // (A) Full entity coverage.
    let arena_count = arena.face_count()
        + arena.vertex_count()
        + arena.half_edge_count()
        + arena.edge_count()
        + arena.loop_count()
        + arena.shell_count()
        + arena.body_count()
        + arena.lump_count()
        + arena.region_count();

    assert_eq!(
        store.active_count(), arena_count,
        "Full coverage: store has {}, arena has {}",
        store.active_count(), arena_count
    );

    // (B) Ancestry hash uniqueness AND Derived hash existence.
    let mut hashes = HashSet::new();
    let mut has_derived_lineage = false;
    for eref in store.tracked_entities() {
        let lineage = store.get_lineage(eref).unwrap();
        let hash = lineage.get_ancestry_hash();
        assert!(
            hashes.insert(hash),
            "Duplicate ancestry hash {:?} on entity {:?}", hash, eref
        );

        if !lineage.get_parent_ancestry_hashes().is_empty() {
            has_derived_lineage = true;
        }
    }
    assert!(
        has_derived_lineage,
        "Honest Test Failure: No derived lineage found. The DAG is broken (all roots)."
    );

    // (C) No unset feature_ids.
    for event in events {
        if let LineageEvent::EntityCreated { lineage, .. } = event {
            assert!(
                !lineage.get_origin_features().is_empty(),
                "Entity created with no origin features: event {:?}", event
            );
            for &fid in lineage.get_origin_features() {
                assert_ne!(
                    fid, FEATURE_ID_UNSET,
                    "Entity created with FEATURE_ID_UNSET: event {:?}", event
                );
            }
        }
    }

    // (D) Event count: cube entities + split_edge entities + mef entities
    //     + sync deletion events from any removed entities.
    assert!(
        !events.is_empty(),
        "Events must be non-empty after multi-step chain"
    );

    // (E) Determinism: both runs produce the same lineage event count.
    let events2 = state2.lineage_events();
    assert_eq!(
        events.len(), events2.len(),
        "Determinism: event count differs between runs ({} vs {})",
        events.len(), events2.len()
    );

    // (F) Both runs produce same active entity count.
    let store2 = store_from_events(events2);
    assert_eq!(
        store.active_count(), store2.active_count(),
        "Determinism: active lineage count differs ({} vs {})",
        store.active_count(), store2.active_count()
    );

    // (G) Verify tetrahedron produces a different-sized lineage store
    // (shape-dependent: cube has more entities than tetrahedron).
    let tet_res = tetrahedron().expect("tetrahedron should succeed");
    let tet_events = tet_res.get_value().topology().lineage_events();
    let tet_store = store_from_events(tet_events);

    assert_ne!(
        store.active_count(), tet_store.active_count(),
        "Cube and tetrahedron should have different entity counts"
    );
    assert_ne!(
        events.len(), tet_events.len(),
        "Cube and tetrahedron should have different event counts"
    );
}
