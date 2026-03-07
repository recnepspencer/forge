use forge_spec::facade::{
    MakeVertexFaceMutation, SewEdgeMutation, SpecState, SplitEdgeMutation, UnsewEdgeMutation,
};

use crate::b_rep::ShellKind;
use crate::operations::entity_lifecycle::make_vertex_face::MakeVertexFace;
use crate::operations::entity_lifecycle::split_edge::SplitEdge;
use crate::operations::non_manifold::sew_edge::SewEdge;
use crate::operations::non_manifold::unsew_edge::UnsewEdge;
use crate::projection::facade::{ProjectionBuilder, compute_projected_topology_hash};
use crate::transactions::facade::{TopologyState, compute_arena_topology_hash};

#[test]
fn projected_sew_edge_matches_legacy_structural_signature() {
    let legacy_hash = build_legacy_sew_hash();
    let projected =
        ProjectionBuilder::build(&build_spec_sew_state()).expect("spec-state SewEdge projection should succeed");

    assert_eq!(
        legacy_hash,
        compute_projected_topology_hash(&projected)
    );
}

#[test]
fn projected_unsew_edge_matches_legacy_structural_signature() {
    let legacy = build_legacy_unsew_state();
    let projected =
        ProjectionBuilder::build(&build_spec_unsew_state()).expect("spec-state UnsewEdge projection should succeed");

    assert_eq!(
        compute_arena_topology_hash(legacy.arena()),
        compute_projected_topology_hash(&projected)
    );
}

fn build_spec_sew_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    let split = draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap()
        .value;
    draft
        .execute(SewEdgeMutation {
            half_edge_a: seed.half_edge,
            half_edge_b: split.he_mb,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_spec_unsew_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    let split = draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap()
        .value;
    draft
        .execute(SewEdgeMutation {
            half_edge_a: seed.half_edge,
            half_edge_b: split.he_mb,
        })
        .unwrap();
    draft
        .execute(UnsewEdgeMutation {
            half_edge_a: seed.half_edge,
            half_edge_b: split.he_mb,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_sew_hash() -> u128 {
    let mut draft = TopologyState::empty().into_mutation();
    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let split = draft
        .execute(SplitEdge { edge: mvf.half_edge })
        .unwrap()
        .into_value();
    draft
        .execute(SewEdge {
            he_a: mvf.half_edge,
            he_b: split.he_mb,
        })
        .unwrap();
    compute_arena_topology_hash(draft.arena())
}

fn build_legacy_unsew_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let split = draft
        .execute(SplitEdge { edge: mvf.half_edge })
        .unwrap()
        .into_value();
    draft
        .execute(SewEdge {
            he_a: mvf.half_edge,
            he_b: split.he_mb,
        })
        .unwrap();
    draft
        .execute(UnsewEdge {
            he_a: mvf.half_edge,
            he_b: split.he_mb,
        })
        .unwrap();
    draft.commit().unwrap()
}
