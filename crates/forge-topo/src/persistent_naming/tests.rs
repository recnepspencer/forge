//! Persistent naming unit tests.

use crate::b_rep::ShellKind;
use forge_core::EntityKind;

use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;
use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
use crate::entity_lifecycle::split_edge::SplitEdge;
use crate::transactions::TopologyState;

use crate::persistent_naming::{assign_name, resolve_name};

use crate::provenance::LineageStore;

#[test]
fn name_rebuild_resolve_finds_entity() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let original_face = mvf.face;

    // 1. Assign name to the created face
    let name = assign_name(draft.lineage_store(), original_face.into()).unwrap();

    assert_eq!(name.get_kind(), EntityKind::Face);

    let final_state = draft.commit().unwrap();

    // 2. Rebuild the exact same topology from scratch
    let state2 = TopologyState::empty();
    let mut draft2 = state2.into_mutation();
    let mvf2 = draft2
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let final_state2 = draft2.commit().unwrap();

    // 3. Resolve the name from state 1 in state 2
    let store2 = LineageStore::from_prior_events(final_state2.lineage_events());
    let matches = resolve_name(&store2, &name);

    assert_eq!(matches.len(), 1, "Must resolve to exactly one entity");

    let resolved_key = matches[0];
    assert_eq!(
        resolved_key,
        mvf2.face.into(),
        "Must resolve to the identically-built face"
    );
}

#[test]
fn name_split_produces_distinct_resolvable_hashes() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    // Build seed
    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();

    // Name the original face
    let name_a = assign_name(draft.lineage_store(), mvf.face.into()).unwrap();

    // Split it
    let se = draft
        .execute(SplitEdge {
            edge: mvf.half_edge,
        })
        .unwrap()
        .into_value();
    let mef = draft
        .execute(MakeEdgeFace {
            vertex_a: mvf.vertex,
            vertex_b: se.new_vertex,
            face: mvf.face,
        })
        .unwrap()
        .into_value();

    // Name the newly derived face
    let name_b = assign_name(draft.lineage_store(), mef.new_face.into()).unwrap();

    // Since Lineage::derive_from creates distinct hashes without needing ordinals,
    // the two faces must have different hashes despite sharing an ancestor.
    assert_ne!(name_a.get_ancestry_hash(), name_b.get_ancestry_hash());

    let final_state = draft.commit().unwrap();
    let final_store = LineageStore::from_prior_events(final_state.lineage_events());

    // Resolve both names
    let match_a = resolve_name(&final_store, &name_a);
    let match_b = resolve_name(&final_store, &name_b);

    assert_eq!(match_a.len(), 1, "Original face name must resolve uniquely");
    assert_eq!(
        match_a[0],
        mvf.face.into(),
        "Name A must map to the original face ID"
    );

    assert_eq!(match_b.len(), 1, "Derived face name must resolve uniquely");
    assert_eq!(
        match_b[0],
        mef.new_face.into(),
        "Name B must map to the new face ID"
    );
}
