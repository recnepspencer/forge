use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use schema::facade::platform::aspects::{
    DiagnosticsAspect, GeometryAspect, LineageAspect, NamingAspect, TopologyAspect,
};
use schema::facade::platform::relations::TopologyRelationKind;
use std::collections::BTreeSet;
use std::path::PathBuf;

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
    TopologySpliceRadialAdjacencyDeclaration, TOPOLOGY_OPERATOR_RELATION_COLLECTION,
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

    assert!(descriptor.touches_collection(TOPOLOGY_OPERATOR_RELATION_COLLECTION));
    assert!(descriptor.touches_aspect_path(TopologyTouchedAspect::TopologyStructure.as_str()));
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
    assert!(rewire_descriptor.touches_aspect_path(TopologyTouchedAspect::TopologyBoundary.as_str()));
    assert!(splice_descriptor.touches_aspect_path(TopologyTouchedAspect::TopologyRadial.as_str()));
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

#[test]
fn touched_graph_basis_files_satisfy_workspace_line_cap() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let basis_dir = manifest_dir.join("src/topology_operators/touched_graph_basis");
    let mut rust_files = std::fs::read_dir(&basis_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
        .collect::<Vec<_>>();
    rust_files.sort();

    for path in rust_files {
        let relative = path
            .strip_prefix(&manifest_dir)
            .unwrap()
            .display()
            .to_string();
        let contents = std::fs::read_to_string(&path).unwrap();
        let line_count = contents.lines().count();
        assert!(
            line_count <= 400,
            "{} has {} lines, above the workspace cap",
            relative,
            line_count
        );
    }
}

#[test]
fn production_touched_graph_basis_has_no_spatial_geometry_admission_bridge() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let basis_dir = manifest_dir.join("src/topology_operators/touched_graph_basis");
    let forbidden_patterns = [
        "WorthGeometryOnlyEvidence",
        "GeometryOnlyEvidence",
        "geometry_only_evidence",
        "spatial_sealed_receipt_admission",
        "from_spatial_boolean_receipt",
        "type_name::<",
    ];

    for entry in std::fs::read_dir(&basis_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }
        if path
            .file_name()
            .is_some_and(|file_name| file_name == "tests.rs")
        {
            continue;
        }
        let relative = path
            .strip_prefix(&manifest_dir)
            .unwrap()
            .display()
            .to_string();
        let contents = std::fs::read_to_string(&path).unwrap();
        for forbidden_pattern in forbidden_patterns {
            assert!(
                !contents.contains(forbidden_pattern),
                "{relative} still contains forbidden topology geometry admission pattern {forbidden_pattern}"
            );
        }
    }
}

fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

fn relation_id(slot: u64) -> RelationId {
    RelationId::new(PartitionId::main(), slot, 1)
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
