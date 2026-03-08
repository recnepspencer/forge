use std::ptr;

use forge_spec::facade::{
    MakeFaceFromVerticesMutation, MakeIsolatedVertexMutation, MakeLoopInFaceFromVerticesMutation,
    MakeVertexFaceMutation, SpecShellKind, SpecState, SplitEdgeMutation,
};
use forge_topo::projection::ProjectedEntityRef;

use crate::engine::{
    contract::InvariantKind,
    facade::{SpecEnvelope, validate_spec_envelope_invariant},
};
use crate::proof::checkpoint::{ValidationCheckpoint, ValidationConfig};
use forge_signal::facade::NodeState;

#[test]
fn lazy_projection_materializes_from_spec_state() {
    let mut draft = SpecState::empty().into_draft();
    draft.execute(MakeVertexFaceMutation).unwrap();
    let spec = draft.commit().unwrap();

    let envelope = SpecEnvelope::from_spec(spec);
    let projection = envelope.projection().unwrap();

    assert_eq!(projection.body_count(), 1);
    assert_eq!(projection.face_count(), 1);
    assert_eq!(projection.half_edge_count(), 1);
    assert_eq!(projection.vertex_count(), 1);
    assert_eq!(envelope.body_count().unwrap(), 1);
    assert_eq!(envelope.face_count().unwrap(), 1);
    assert_eq!(envelope.vertex_count().unwrap(), 1);
    assert_eq!(envelope.edge_count().unwrap(), 1);
    assert_eq!(envelope.entity_count().unwrap(), 4);
    assert_eq!(envelope.body().unwrap().raw(), 0);
    assert_eq!(envelope.shell().unwrap().raw(), 0);
}

#[test]
fn lazy_projection_is_cached() {
    let mut draft = SpecState::empty().into_draft();
    draft.execute(MakeVertexFaceMutation).unwrap();
    let spec = draft.commit().unwrap();

    let envelope = SpecEnvelope::from_spec(spec);
    let first = envelope.projection().unwrap();
    let second = envelope.projection().unwrap();

    assert!(ptr::eq(first, second));
}

#[test]
fn projected_handle_lists_are_cached() {
    let mut draft = SpecState::empty().into_draft();
    draft.execute(MakeVertexFaceMutation).unwrap();
    let spec = draft.commit().unwrap();

    let envelope = SpecEnvelope::from_spec(spec);
    let first = envelope.faces().unwrap();
    let second = envelope.faces().unwrap();

    assert!(ptr::eq(first, second));
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].raw(), 0);
}

#[test]
fn projection_query_helpers_surface_face_loop_relationships() {
    let mut draft = SpecState::empty().into_draft();
    let v0 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let v1 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let v2 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let face = draft
        .execute(MakeFaceFromVerticesMutation {
            vertices: vec![v0, v1, v2],
        })
        .unwrap()
        .value
        .face;
    let h0 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let h1 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let h2 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    draft
        .execute(MakeLoopInFaceFromVerticesMutation {
            face,
            vertices: vec![h0, h1, h2],
        })
        .unwrap();
    let spec = draft.commit().unwrap();

    let envelope = SpecEnvelope::from_spec(spec);
    let face_id = envelope.faces().unwrap()[0];
    let edge_id = envelope.edges().unwrap()[0];
    let vertex_id = envelope.vertices().unwrap()[0];

    assert_eq!(envelope.face_loops(face_id).unwrap().len(), 2);
    assert_eq!(envelope.shell_faces(envelope.shell().unwrap()).unwrap().len(), 1);
    assert_eq!(
        envelope
            .loop_half_edges(envelope.face_loops(face_id).unwrap()[0])
            .unwrap()
            .len(),
        3
    );
    assert_eq!(envelope.face_half_edges(face_id).unwrap().len(), 6);
    assert_eq!(envelope.face_edges(face_id).unwrap().len(), 6);
    assert_eq!(envelope.edge_half_edges(edge_id).unwrap().len(), 1);
    assert_eq!(envelope.edge_faces(edge_id).unwrap().len(), 1);
    assert_eq!(envelope.radial_valence(edge_id).unwrap(), 1);
    assert!(envelope.is_boundary_edge(edge_id).unwrap());
    assert_eq!(envelope.vertex_outgoing_half_edges(vertex_id).unwrap().len(), 1);
    assert_eq!(envelope.vertex_faces(vertex_id).unwrap().len(), 1);
    assert_eq!(
        envelope
            .radial_half_edges(envelope.edge_half_edges(edge_id).unwrap()[0])
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn direct_projection_accessors_surface_shell_and_halfedge_metadata() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap();
    let spec = draft.commit().unwrap();

    let envelope = SpecEnvelope::from_spec(spec);
    let shell = envelope.shell().unwrap();
    let face = envelope.faces().unwrap()[0];
    let loop_id = envelope.face_loops(face).unwrap()[0];
    let loop_half_edges = envelope.loop_half_edges(loop_id).unwrap();
    let half_edge = loop_half_edges[0];
    let split_half_edge = loop_half_edges[1];
    let origin = envelope.half_edge_origin(half_edge).unwrap();
    let edge = envelope.half_edge_edge(half_edge).unwrap();

    assert_eq!(envelope.shell_kind(shell).unwrap(), SpecShellKind::Sheet);
    assert_eq!(envelope.face_shell(face).unwrap(), shell);
    assert_eq!(envelope.loop_face(loop_id).unwrap(), face);
    assert_eq!(envelope.half_edge_face(half_edge).unwrap(), face);
    assert!(envelope.vertices().unwrap().contains(&origin));
    assert_eq!(envelope.edge_representative_half_edge(edge).unwrap(), half_edge);
    assert_eq!(envelope.half_edge_next(half_edge).unwrap(), split_half_edge);
    assert_eq!(envelope.half_edge_prev(half_edge).unwrap(), split_half_edge);
    assert_eq!(envelope.half_edge_radial_next(half_edge).unwrap(), half_edge);
    assert_eq!(envelope.vertex_primary_half_edge(origin).unwrap(), Some(half_edge));
}

#[test]
fn envelope_validation_helper_matches_pipeline_spec_validation() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap();
    let spec = draft.commit().unwrap();

    let envelope = SpecEnvelope::from_spec(spec);
    let config = ValidationConfig {
        checkpoints: vec![ValidationCheckpoint::PostFeature],
        include_geometric: false,
        entity_limit: 0,
    };

    envelope
        .validate_invariant(&InvariantKind::ManifoldEdges, &config)
        .unwrap();
    validate_spec_envelope_invariant(&envelope, &InvariantKind::ManifoldEdges, &config).unwrap();
    let checkpoint_result = envelope
        .run_checkpoint(&config, ValidationCheckpoint::PostFeature)
        .unwrap();
    assert!(checkpoint_result.is_passed());
    assert!(!checkpoint_result.included_geometric());
}

#[test]
fn hierarchy_and_resolution_accessors_surface_projected_structure() {
    let mut draft = SpecState::empty().into_draft();
    draft.execute(MakeVertexFaceMutation).unwrap();
    let spec = draft.commit().unwrap();

    let envelope = SpecEnvelope::from_spec(spec);
    let body = envelope.body().unwrap();
    let lump = envelope.lumps().unwrap()[0];
    let region = envelope.regions().unwrap()[0];
    let shell = envelope.shell().unwrap();
    let face = envelope.faces().unwrap()[0];
    let loop_id = envelope.face_loops(face).unwrap()[0];
    let half_edge = envelope.loop_half_edges(loop_id).unwrap()[0];
    let edge = envelope.half_edge_edge(half_edge).unwrap();
    let vertex = envelope.half_edge_origin(half_edge).unwrap();

    assert_eq!(envelope.body_lumps(body).unwrap(), vec![lump]);
    assert_eq!(envelope.lump_body(lump).unwrap(), body);
    assert_eq!(envelope.lump_regions(lump).unwrap(), vec![region]);
    assert_eq!(envelope.region_lump(region).unwrap(), lump);
    assert_eq!(envelope.region_shells(region).unwrap(), vec![shell]);
    assert_eq!(envelope.shell_region(shell).unwrap(), region);
    assert_eq!(envelope.face_shell(face).unwrap(), shell);
    assert_eq!(envelope.face_outer_loop(face).unwrap(), loop_id);
    assert!(envelope.face_inner_loops(face).unwrap().is_empty());
    assert_eq!(envelope.face_surface_binding(face).unwrap(), None);
    assert_eq!(envelope.vertex_disk_components(vertex).unwrap(), vec![vec![half_edge]]);
    assert_eq!(envelope.half_edge_coedge_binding(half_edge).unwrap(), None);
    assert_eq!(envelope.edge_curve_binding(edge).unwrap(), None);
    assert_eq!(envelope.vertex_geometry_binding(vertex).unwrap(), None);

    assert_eq!(
        envelope.resolve(envelope.body_spec_id(body).unwrap()).unwrap(),
        Some(ProjectedEntityRef::Body(body))
    );
    assert_eq!(
        envelope.resolve(envelope.lump_spec_id(lump).unwrap()).unwrap(),
        Some(ProjectedEntityRef::Lump(lump))
    );
    assert_eq!(
        envelope.resolve(envelope.region_spec_id(region).unwrap()).unwrap(),
        Some(ProjectedEntityRef::Region(region))
    );
    assert_eq!(
        envelope.resolve(envelope.shell_spec_id(shell).unwrap()).unwrap(),
        Some(ProjectedEntityRef::Shell(shell))
    );
    assert_eq!(
        envelope.resolve(envelope.face_spec_id(face).unwrap()).unwrap(),
        Some(ProjectedEntityRef::Face(face))
    );
    assert_eq!(
        envelope.resolve(envelope.loop_spec_id(loop_id).unwrap()).unwrap(),
        Some(ProjectedEntityRef::Loop(loop_id))
    );
    assert_eq!(
        envelope.resolve(envelope.half_edge_spec_id(half_edge).unwrap()).unwrap(),
        Some(ProjectedEntityRef::HalfEdge(half_edge))
    );
    assert_eq!(
        envelope.resolve(envelope.edge_spec_id(edge).unwrap()).unwrap(),
        Some(ProjectedEntityRef::Edge(edge))
    );
    assert_eq!(
        envelope.resolve(envelope.vertex_spec_id(vertex).unwrap()).unwrap(),
        Some(ProjectedEntityRef::Vertex(vertex))
    );
}

#[test]
fn envelope_fingerprint_helper_matches_detail_level_contract() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap();
    let spec = draft.commit().unwrap();

    let envelope = SpecEnvelope::from_spec(spec);

    assert_eq!(
        envelope.fingerprint(crate::configuration::facade::FingerprintDetail::Standard).unwrap(),
        envelope.spec_fingerprint()
    );
    assert_eq!(
        envelope.fingerprint(crate::configuration::facade::FingerprintDetail::Full).unwrap(),
        envelope.projection_fingerprint().unwrap()
    );
}

#[test]
fn signal_backed_projection_starts_on_demand_and_only_recomputes_when_read() {
    let mut draft = SpecState::empty().into_draft();
    draft.execute(MakeVertexFaceMutation).unwrap();
    let spec = draft.commit().unwrap();

    let envelope = SpecEnvelope::from_spec(spec);

    assert_eq!(envelope.debug_signal_node_state("projection"), Some(NodeState::Dirty));
    let before = envelope.debug_signal_telemetry();

    let first = envelope.projection().unwrap();
    let second = envelope.projection().unwrap();

    assert_eq!(envelope.debug_signal_node_state("projection"), Some(NodeState::Clean));
    assert!(ptr::eq(first, second));

    let after = envelope.debug_signal_telemetry();
    assert_eq!(after.transaction_rollback_count, before.transaction_rollback_count);
    assert_eq!(after.ondemand_deferred_count, before.ondemand_deferred_count);
}

#[test]
fn signal_backed_validation_checkpoint_and_fingerprint_share_projection_substrate() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap();
    let spec = draft.commit().unwrap();

    let envelope = SpecEnvelope::from_spec(spec);

    assert_eq!(envelope.debug_signal_node_state("structure"), Some(NodeState::Dirty));
    assert_eq!(envelope.debug_signal_node_state("checkpoint"), Some(NodeState::Dirty));
    assert_eq!(
        envelope.debug_signal_node_state("full_fingerprint"),
        Some(NodeState::Dirty)
    );

    validate_spec_envelope_invariant(
        &envelope,
        &InvariantKind::ManifoldEdges,
        &ValidationConfig {
            checkpoints: vec![ValidationCheckpoint::PostFeature],
            include_geometric: false,
            entity_limit: 0,
        },
    )
    .unwrap();

    assert_eq!(envelope.debug_signal_node_state("projection"), Some(NodeState::Clean));
    assert_eq!(envelope.debug_signal_node_state("invariant"), Some(NodeState::Clean));

    let after_invariant = envelope.debug_signal_telemetry();

    envelope
        .run_checkpoint(
            &ValidationConfig {
                checkpoints: vec![ValidationCheckpoint::PostFeature],
                include_geometric: false,
                entity_limit: 0,
            },
            ValidationCheckpoint::PostFeature,
        )
        .unwrap();
    envelope
        .fingerprint(crate::configuration::facade::FingerprintDetail::Full)
        .unwrap();

    let after_all = envelope.debug_signal_telemetry();
    assert_eq!(envelope.debug_signal_node_state("checkpoint"), Some(NodeState::Clean));
    assert_eq!(
        envelope.debug_signal_node_state("full_fingerprint"),
        Some(NodeState::Clean)
    );
    assert!(after_all.nodes_recomputed >= after_invariant.nodes_recomputed);
    assert_eq!(after_all.transaction_rollback_count, 0);
}
