use forge_query::facade::ForgeQueryAspectTouch;
use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use schema::facade::platform::aspects::{
    DiagnosticsAspect, GeometryAspect, LineageAspect, NamingAspect, TopologyAspect,
};
use schema::facade::platform::relations::TopologyRelationKind;
use std::collections::BTreeSet;

mod line_cap;

use super::lowering::test_basis_from_parts;
use super::{
    topology_operator_touch_descriptor_from_touched_graph_basis,
    topology_rewire_loop_endpoint_touched_graph_basis,
    topology_splice_radial_adjacency_touched_graph_basis,
    topology_touched_aspect_from_schema_aspect,
    topology_touched_graph_basis_from_mutation_sequence, TopologyTouchedAspect,
    TopologyTouchedEntity, TopologyTouchedOperatingWorld,
    TopologyTouchedOperatingWorldIdentityDigest, TopologyTouchedOperatingWorldPosture,
    TopologyTouchedRelation, TopologyTouchedScope,
};
use crate::topology_operators::application::TopologyDeclarationMutationPayload;
use crate::topology_operators::{
    LoopEndpointKind, TopologyRewireLoopEndpointDeclaration,
    TopologySpliceRadialAdjacencyDeclaration,
};

#[test]
fn same_operator_intent_has_stable_touched_graph_digest_across_replay() {
    let declaration = TopologyRewireLoopEndpointDeclaration::new(
        relation_id(7),
        LoopEndpointKind::End,
        entity_id(8),
        entity_id(9),
    );

    let first = topology_rewire_loop_endpoint_touched_graph_basis(
        &declaration,
        TopologyTouchedOperatingWorld::mainline(),
    );
    let replay = topology_rewire_loop_endpoint_touched_graph_basis(
        &declaration,
        TopologyTouchedOperatingWorld::mainline(),
    );

    assert_eq!(first.digest(), replay.digest());
    assert_eq!(first.counters().entity_count(), 2);
    assert_eq!(first.counters().relation_count(), 1);
    assert_eq!(first.counters().relation_kind_count(), 1);
}

#[test]
fn benign_ordering_does_not_change_touched_graph_digest() {
    let canonical = test_basis_from_parts(
        vec![
            TopologyTouchedEntity::new(entity_id(1)),
            TopologyTouchedEntity::new(entity_id(2)),
        ],
        vec![TopologyTouchedRelation::new(relation_id(3))],
        vec![TopologyRelationKind::HalfEdgeNext],
        vec![
            TopologyTouchedAspect::TopologyBoundary,
            TopologyTouchedAspect::TopologyStructure,
        ],
        vec![TopologyTouchedScope::Loop, TopologyTouchedScope::Relation],
    );
    let reordered = test_basis_from_parts(
        vec![
            TopologyTouchedEntity::new(entity_id(2)),
            TopologyTouchedEntity::new(entity_id(1)),
        ],
        vec![TopologyTouchedRelation::new(relation_id(3))],
        vec![TopologyRelationKind::HalfEdgeNext],
        vec![
            TopologyTouchedAspect::TopologyStructure,
            TopologyTouchedAspect::TopologyBoundary,
        ],
        vec![TopologyTouchedScope::Relation, TopologyTouchedScope::Loop],
    );

    assert_eq!(canonical.digest(), reordered.digest());
}

#[test]
fn basis_lowers_to_query_descriptor_without_becoming_query_authority() {
    let basis = topology_rewire_loop_endpoint_touched_graph_basis(
        &TopologyRewireLoopEndpointDeclaration::new(
            relation_id(7),
            LoopEndpointKind::Start,
            entity_id(8),
            entity_id(9),
        ),
        TopologyTouchedOperatingWorld::mainline(),
    );

    let descriptor = topology_operator_touch_descriptor_from_touched_graph_basis(&basis).unwrap();

    assert_eq!(descriptor.declared_collection_count(), 1);
    assert!(descriptor.touches_aspect(&query_aspect_touch(
        TopologyTouchedAspect::TopologyStructure
    )));
    assert_eq!(
        descriptor.touched_aspect_count(),
        basis.counters().touched_aspect_count()
    );
}

#[test]
fn different_operator_intent_selects_different_touched_basis_and_descriptor_sets() {
    let rewire = TopologyRewireLoopEndpointDeclaration::new(
        relation_id(21),
        LoopEndpointKind::End,
        entity_id(22),
        entity_id(23),
    );
    let splice = TopologySpliceRadialAdjacencyDeclaration::new(
        relation_id(21),
        entity_id(22),
        entity_id(23),
    );

    let rewire_basis = topology_touched_graph_basis_from_mutation_sequence(
        &rewire.into_mutation_sequence(),
        TopologyTouchedOperatingWorld::mainline(),
    );
    let splice_basis = topology_touched_graph_basis_from_mutation_sequence(
        &splice.into_mutation_sequence(),
        TopologyTouchedOperatingWorld::mainline(),
    );
    let rewire_descriptor =
        topology_operator_touch_descriptor_from_touched_graph_basis(&rewire_basis).unwrap();
    let splice_descriptor =
        topology_operator_touch_descriptor_from_touched_graph_basis(&splice_basis).unwrap();

    assert_ne!(rewire_basis.digest(), splice_basis.digest());
    assert!(rewire_basis
        .aspects()
        .contains(&TopologyTouchedAspect::TopologyBoundary));
    assert!(splice_basis
        .aspects()
        .contains(&TopologyTouchedAspect::TopologyRadial));
    assert_ne!(
        rewire_descriptor.touched_aspect_count(),
        0,
        "rewire basis must select validator aspects"
    );
    assert!(rewire_descriptor
        .touches_aspect(&query_aspect_touch(TopologyTouchedAspect::TopologyBoundary)));
    assert!(splice_descriptor
        .touches_aspect(&query_aspect_touch(TopologyTouchedAspect::TopologyRadial)));
}

#[test]
fn touched_aspect_digest_and_counter_coverage_rejects_schema_drift() {
    let covered = concrete_schema_aspects()
        .into_iter()
        .map(topology_touched_aspect_from_schema_aspect)
        .collect::<BTreeSet<_>>();
    let declared = TopologyTouchedAspect::ALL
        .into_iter()
        .collect::<BTreeSet<_>>();
    let named = TopologyTouchedAspect::ALL
        .into_iter()
        .map(TopologyTouchedAspect::as_str)
        .collect::<BTreeSet<_>>();

    assert_eq!(covered, declared);
    assert_eq!(named.len(), TopologyTouchedAspect::ALL.len());

    let full = test_basis_from_parts(
        vec![TopologyTouchedEntity::new(entity_id(1))],
        vec![TopologyTouchedRelation::new(relation_id(1))],
        vec![TopologyRelationKind::HalfEdgeNext],
        TopologyTouchedAspect::ALL.to_vec(),
        vec![TopologyTouchedScope::Loop],
    );
    let mut missing = TopologyTouchedAspect::ALL.to_vec();
    missing.pop();
    let partial = test_basis_from_parts(
        vec![TopologyTouchedEntity::new(entity_id(1))],
        vec![TopologyTouchedRelation::new(relation_id(1))],
        vec![TopologyRelationKind::HalfEdgeNext],
        missing,
        vec![TopologyTouchedScope::Loop],
    );

    assert_eq!(
        full.counters().touched_aspect_count(),
        TopologyTouchedAspect::ALL.len()
    );
    assert_ne!(full.digest(), partial.digest());
}

#[test]
fn operating_world_identity_participates_in_touched_graph_digest() {
    let declaration = TopologySpliceRadialAdjacencyDeclaration::new(
        relation_id(11),
        entity_id(12),
        entity_id(13),
    );

    let branch_a = topology_splice_radial_adjacency_touched_graph_basis(
        &declaration,
        TopologyTouchedOperatingWorld::branch(test_world_identity("query-branch-digest-a")),
    );
    let branch_a_replay = topology_splice_radial_adjacency_touched_graph_basis(
        &declaration,
        TopologyTouchedOperatingWorld::branch(test_world_identity("query-branch-digest-a")),
    );
    let branch_b = topology_splice_radial_adjacency_touched_graph_basis(
        &declaration,
        TopologyTouchedOperatingWorld::branch(test_world_identity("query-branch-digest-b")),
    );
    let preview_a = topology_splice_radial_adjacency_touched_graph_basis(
        &declaration,
        TopologyTouchedOperatingWorld::preview(test_world_identity("query-preview-digest-a")),
    );
    let configured_domain = topology_splice_radial_adjacency_touched_graph_basis(
        &declaration,
        TopologyTouchedOperatingWorld::configured_domain_handle(test_world_identity(
            "configured-domain-digest-a",
        )),
    );

    assert_eq!(branch_a.digest(), branch_a_replay.digest());
    assert_ne!(branch_a.digest(), branch_b.digest());
    assert_ne!(branch_a.digest(), preview_a.digest());
    assert_ne!(branch_a.digest(), configured_domain.digest());
    assert_eq!(
        branch_a.operating_world().posture(),
        TopologyTouchedOperatingWorldPosture::Branch
    );
    assert_eq!(
        branch_a.operating_world().identity_digest(),
        Some("query-branch-digest-a")
    );
}

fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

fn relation_id(slot: u64) -> RelationId {
    RelationId::new(PartitionId::main(), slot, 1)
}

fn query_aspect_touch(aspect: TopologyTouchedAspect) -> ForgeQueryAspectTouch {
    ForgeQueryAspectTouch::whole_aspect(schema_aspect_for_touched_aspect(aspect).aspect_key())
}

fn schema_aspect_for_touched_aspect(
    aspect: TopologyTouchedAspect,
) -> schema::facade::platform::aspects::Aspect {
    match aspect {
        TopologyTouchedAspect::TopologyStructure => {
            schema::facade::platform::aspects::Aspect::Topology(TopologyAspect::Structure)
        }
        TopologyTouchedAspect::TopologyOwnership => {
            schema::facade::platform::aspects::Aspect::Topology(TopologyAspect::Ownership)
        }
        TopologyTouchedAspect::TopologyBoundary => {
            schema::facade::platform::aspects::Aspect::Topology(TopologyAspect::Boundary)
        }
        TopologyTouchedAspect::TopologyRadial => {
            schema::facade::platform::aspects::Aspect::Topology(TopologyAspect::Radial)
        }
        TopologyTouchedAspect::GeometryBinding => {
            schema::facade::platform::aspects::Aspect::Geometry(GeometryAspect::Binding)
        }
        TopologyTouchedAspect::GeometryEmbedding => {
            schema::facade::platform::aspects::Aspect::Geometry(GeometryAspect::Embedding)
        }
        TopologyTouchedAspect::GeometryProvenance => {
            schema::facade::platform::aspects::Aspect::Geometry(GeometryAspect::Provenance)
        }
        TopologyTouchedAspect::GeometryApproximation => {
            schema::facade::platform::aspects::Aspect::Geometry(GeometryAspect::Approximation)
        }
        TopologyTouchedAspect::GeometryUvAnchoring => {
            schema::facade::platform::aspects::Aspect::Geometry(GeometryAspect::UvAnchoring)
        }
        TopologyTouchedAspect::GeometryCarrier => {
            schema::facade::platform::aspects::Aspect::Geometry(GeometryAspect::Carrier)
        }
        TopologyTouchedAspect::GeometryPrecision => {
            schema::facade::platform::aspects::Aspect::Geometry(GeometryAspect::Precision)
        }
        TopologyTouchedAspect::GeometryFallback => {
            schema::facade::platform::aspects::Aspect::Geometry(GeometryAspect::Fallback)
        }
        TopologyTouchedAspect::LineageProvenance => {
            schema::facade::platform::aspects::Aspect::Lineage(LineageAspect::Provenance)
        }
        TopologyTouchedAspect::NamingPersistentName => {
            schema::facade::platform::aspects::Aspect::Naming(NamingAspect::PersistentName)
        }
        TopologyTouchedAspect::DiagnosticsDecisions => {
            schema::facade::platform::aspects::Aspect::Diagnostics(DiagnosticsAspect::Decisions)
        }
        TopologyTouchedAspect::DiagnosticsInterpretations => {
            schema::facade::platform::aspects::Aspect::Diagnostics(
                DiagnosticsAspect::Interpretations,
            )
        }
    }
}

fn test_world_identity(value: &'static str) -> TopologyTouchedOperatingWorldIdentityDigest {
    TopologyTouchedOperatingWorldIdentityDigest::for_test(value)
}

fn concrete_schema_aspects() -> Vec<schema::facade::platform::aspects::Aspect> {
    TopologyAspect::ALL
        .into_iter()
        .map(schema::facade::platform::aspects::Aspect::Topology)
        .chain(
            GeometryAspect::ALL
                .into_iter()
                .map(schema::facade::platform::aspects::Aspect::Geometry),
        )
        .chain(
            LineageAspect::ALL
                .into_iter()
                .map(schema::facade::platform::aspects::Aspect::Lineage),
        )
        .chain(
            NamingAspect::ALL
                .into_iter()
                .map(schema::facade::platform::aspects::Aspect::Naming),
        )
        .chain(
            DiagnosticsAspect::ALL
                .into_iter()
                .map(schema::facade::platform::aspects::Aspect::Diagnostics),
        )
        .collect()
}
