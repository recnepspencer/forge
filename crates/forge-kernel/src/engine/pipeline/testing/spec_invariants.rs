use crate::engine::contracts::contract::InvariantKind;
use crate::engine::facade::{SpecEnvelope, validate_spec_envelope_invariant};
use crate::proof::checkpoint::{ValidationCheckpoint, ValidationConfig};
use forge_spec::facade::{
    MakeShellFaceMutation, MakeSolidMutation, MakeVertexFaceMutation, RelationKind, SpecNodeKind,
    SpecShellKind, SpecShellOrientation, SpecState, SplitEdgeMutation,
};

#[test]
fn validate_spec_envelope_invariant_accepts_valid_projection() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    draft
        .execute(SplitEdgeMutation {
            half_edge: seed.value.half_edge,
        })
        .unwrap();
    let spec = draft.commit().unwrap();
    let envelope = SpecEnvelope::from_spec(spec);

    validate_spec_envelope_invariant(
        &envelope,
        &InvariantKind::ManifoldEdges,
        &post_feature_config(),
    )
    .unwrap();
}

#[test]
fn validate_spec_envelope_invariant_rejects_invalid_solid_shell_projection() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let shell_face = draft
        .execute(MakeShellFaceMutation {
            region: solid.value.region,
            kind: SpecShellKind::Solid(SpecShellOrientation::Outer),
        })
        .unwrap();
    draft
        .execute(SplitEdgeMutation {
            half_edge: shell_face.value.half_edge,
        })
        .unwrap();
    let spec = draft.commit().unwrap();
    let envelope = SpecEnvelope::from_spec(spec);

    let error = validate_spec_envelope_invariant(
        &envelope,
        &InvariantKind::ManifoldEdges,
        &post_feature_config(),
    )
    .unwrap_err();

    assert!(format!("{error}").contains("projected_shell_consistency"));
}

#[test]
fn validate_spec_envelope_invariant_rejects_spec_with_unreachable_halfedge() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap();

    let orphan_edge = draft.create_node(SpecNodeKind::Edge, None, "orphan-edge").unwrap();
    let orphan_half_edge = draft
        .create_node(SpecNodeKind::HalfEdge, None, "orphan-half-edge")
        .unwrap();

    for (kind, source, target, role) in [
        (RelationKind::HalfEdgeNext, orphan_half_edge, orphan_half_edge, "orphan-next"),
        (
            RelationKind::HalfEdgeRadialNext,
            orphan_half_edge,
            orphan_half_edge,
            "orphan-radial",
        ),
        (
            RelationKind::HalfEdgeUsesEdge,
            orphan_half_edge,
            orphan_edge,
            "orphan-edge-link",
        ),
        (
            RelationKind::HalfEdgeOriginVertex,
            orphan_half_edge,
            seed.vertex,
            "orphan-origin",
        ),
        (
            RelationKind::HalfEdgeBoundsFace,
            orphan_half_edge,
            seed.face,
            "orphan-face",
        ),
    ] {
        draft.add_relation(kind, source, target, 0, role).unwrap();
    }

    let spec = draft.commit().unwrap();
    let envelope = SpecEnvelope::from_spec(spec);

    let error = validate_spec_envelope_invariant(
        &envelope,
        &InvariantKind::ManifoldEdges,
        &post_feature_config(),
    )
    .unwrap_err();

    assert!(
        format!("{error}").contains("projected_face_loop_membership_complete"),
        "unexpected error: {error}"
    );
}

#[test]
fn validate_spec_envelope_invariant_skips_when_post_feature_is_disabled() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    draft
        .execute(MakeShellFaceMutation {
            region: solid.value.region,
            kind: SpecShellKind::Solid(SpecShellOrientation::Outer),
        })
        .unwrap();
    let spec = draft.commit().unwrap();
    let envelope = SpecEnvelope::from_spec(spec);

    let config = ValidationConfig {
        checkpoints: vec![],
        include_geometric: false,
        entity_limit: 0,
    };

    validate_spec_envelope_invariant(&envelope, &InvariantKind::ManifoldEdges, &config).unwrap();
}

fn post_feature_config() -> ValidationConfig {
    ValidationConfig {
        checkpoints: vec![ValidationCheckpoint::PostFeature],
        include_geometric: false,
        entity_limit: 0,
    }
}
