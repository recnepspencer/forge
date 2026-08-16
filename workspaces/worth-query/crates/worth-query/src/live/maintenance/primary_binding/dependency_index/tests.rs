use std::sync::Arc;

use worth_foundational::facade::{
    AspectBinding, AspectContract, AspectContractRevision, AspectIdentity, AspectKey, AspectMask,
    AuthoritativeAspectChangeKind, FieldKey, ProjectionMask, ScalarAspectType,
};
use worth_runtime_bridge::facade::{
    BridgeSemanticDependencyCandidate, BridgeSemanticDependencyCandidateParts,
    BridgeSemanticLocality, RelationalBridgeRecordIdentityParts,
};

use super::{
    index_projection, select_projection_candidates, BoundPrimaryLookupProjection,
    BoundPrimaryProjectionBucket, WorthQueryBoundPrimaryDependencyIndex,
};

#[test]
fn exact_lookup_cost_does_not_grow_with_unrelated_installed_dependencies() {
    let record = RelationalBridgeRecordIdentityParts::entity(7, 11, 1);
    let relevant = candidate("relevant", 1, record);
    let mut dependencies = vec![relevant.clone()];
    dependencies.extend((0..64).map(|ordinal| {
        candidate(
            &format!("unrelated-{ordinal}"),
            ordinal + 2,
            RelationalBridgeRecordIdentityParts::entity(7, 100 + ordinal as u64, 1),
        )
    }));

    let index = WorthQueryBoundPrimaryDependencyIndex::build(&dependencies);
    let (selected, probes) = index.lookup(&relevant, &[]);

    assert_eq!(selected, vec![0]);
    assert_eq!(
        probes, 3,
        "exact-record, managed-record wildcard, and containing whole graph"
    );
}

#[test]
fn sibling_record_does_not_enter_the_exact_candidate_bucket() {
    let installed = candidate(
        "curve",
        1,
        RelationalBridgeRecordIdentityParts::entity(4, 5, 1),
    );
    let sibling = candidate(
        "curve",
        1,
        RelationalBridgeRecordIdentityParts::entity(4, 6, 1),
    );
    let index = WorthQueryBoundPrimaryDependencyIndex::build(&[installed]);

    let (selected, probes) = index.lookup(&sibling, &[]);

    assert!(selected.is_empty());
    assert_eq!(probes, 3);
}

#[test]
fn producer_change_superset_selects_consumer_change_subset() {
    let record = RelationalBridgeRecordIdentityParts::entity(7, 11, 1);
    let consumer = candidate("curve", 1, record);
    let delivered = candidate_with_changes(
        "curve",
        1,
        record,
        vec![
            AuthoritativeAspectChangeKind::FieldSet,
            AuthoritativeAspectChangeKind::FieldClear,
        ],
    );
    let index = WorthQueryBoundPrimaryDependencyIndex::build(&[consumer]);

    let (selected, probes) = index.lookup(&delivered, &[]);

    assert_eq!(selected, vec![0]);
    assert_eq!(probes, 6);
}

#[test]
fn projection_buckets_select_whole_parent_and_child_but_not_siblings() {
    let mut bucket = BoundPrimaryProjectionBucket::default();
    index_projection(&mut bucket, &AspectMask::whole_aspect(), 0);
    index_projection(&mut bucket, &AspectMask::new([path("profile")]), 1);
    index_projection(
        &mut bucket,
        &AspectMask::new([nested_path(&["profile", "name"])]),
        2,
    );
    index_projection(&mut bucket, &AspectMask::new([path("status")]), 3);

    let changed = nested_path(&["profile", "name"]);
    let mut selected = std::collections::BTreeSet::new();
    select_projection_candidates(
        &bucket,
        &BoundPrimaryLookupProjection::Paths(std::slice::from_ref(&changed)),
        &mut selected,
    );

    assert_eq!(selected.into_iter().collect::<Vec<_>>(), vec![0, 1, 2]);
}

fn candidate(
    aspect: &str,
    ordinal: usize,
    record: RelationalBridgeRecordIdentityParts,
) -> BridgeSemanticDependencyCandidate {
    candidate_with_changes(
        aspect,
        ordinal,
        record,
        vec![AuthoritativeAspectChangeKind::FieldSet],
    )
}

fn candidate_with_changes(
    aspect: &str,
    ordinal: usize,
    record: RelationalBridgeRecordIdentityParts,
    relevant_changes: Vec<AuthoritativeAspectChangeKind>,
) -> BridgeSemanticDependencyCandidate {
    BridgeSemanticDependencyCandidate::admit(BridgeSemanticDependencyCandidateParts {
        source_installation_identity: Arc::from("query-installation"),
        source_basis: Arc::from("query-basis"),
        source_runtime_authority: 1,
        source_installation_generation: 1,
        source_authority_binding_identity: Arc::from("query-binding"),
        source_stage_identity: None,
        source_node_identity: Arc::from(format!("node-{ordinal}")),
        dependency_ordinal: ordinal,
        declared_graph_role: Arc::from("primary"),
        graph_participation_identity: Arc::from("primary-graph"),
        graph_adapter_identity: Arc::from("primary-adapter"),
        source_record_identity: Some(record),
        observation_record_identity: Some(record),
        contract: AspectContract::scalar(
            AspectKey::new(aspect).unwrap(),
            AspectIdentity(ordinal as u64),
            AspectContractRevision(1),
            ScalarAspectType::UInt64,
        ),
        projection_mask: AspectMask::<ProjectionMask>::whole_aspect(),
        binding: AspectBinding::EntityField {
            field: FieldKey::new("value").unwrap(),
        },
        locality: BridgeSemanticLocality::SourceRecord,
        relevant_changes,
    })
    .unwrap()
}

fn nested_path(fields: &[&str]) -> worth_foundational::facade::CanonicalFieldPath {
    worth_foundational::facade::CanonicalFieldPath::new(
        fields
            .iter()
            .map(|field| FieldKey::new(*field).unwrap())
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn path(field: &str) -> worth_foundational::facade::CanonicalFieldPath {
    nested_path(&[field])
}
