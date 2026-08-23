use std::sync::Arc;

use worth_foundational::facade::{
    AbsenceLaw, AspectBinding, AspectContract, AspectContractRevision, AspectEvolutionPolicy,
    AspectIdentity, AspectKey, AspectMask, AuthoritativeAspectChangeKind, CanonicalFieldPath,
    FieldDeclaration, FieldKey, FieldRequirement, ProjectionMask, ScalarAspectType,
    StructAspectShape,
};
use worth_runtime_bridge::facade::{
    BridgeSemanticDependencyCandidate, BridgeSemanticDependencyCandidateParts,
    BridgeSemanticLocality, RelationalBridgeRecordIdentityParts,
};

use super::{
    bound_change_locality_matches, bound_delivery_matches, bound_semantic_change_matches,
    bound_structural_change_matches, bridge_candidate_accepts_change,
};

#[test]
fn unequal_but_overlapping_masks_and_change_sets_match() {
    let consumer = candidate(
        AspectMask::new([path("value")]),
        vec![AuthoritativeAspectChangeKind::FieldSet],
    );
    let delivered = candidate(
        AspectMask::whole_aspect(),
        vec![
            AuthoritativeAspectChangeKind::FieldSet,
            AuthoritativeAspectChangeKind::FieldClear,
        ],
    );

    assert!(bound_delivery_matches(&consumer, &delivered, &[]));
}

#[test]
fn disjoint_masks_or_change_sets_do_not_match() {
    let consumer = candidate(
        AspectMask::new([path("value")]),
        vec![AuthoritativeAspectChangeKind::FieldSet],
    );
    let disjoint_mask = candidate(
        AspectMask::new([path("status")]),
        vec![AuthoritativeAspectChangeKind::FieldSet],
    );
    let disjoint_change = candidate(
        AspectMask::whole_aspect(),
        vec![AuthoritativeAspectChangeKind::FieldClear],
    );

    assert!(!bound_delivery_matches(&consumer, &disjoint_mask, &[]));
    assert!(!bound_delivery_matches(&consumer, &disjoint_change, &[]));
}

#[test]
fn bound_candidate_uses_bridge_owned_whole_aspect_change_intersection() {
    let consumer = candidate(
        AspectMask::new([path("value")]),
        vec![AuthoritativeAspectChangeKind::FieldSet],
    );
    let whole =
        worth_runtime_bridge::facade::BridgeSemanticAspectChange::from_authoritative_publication(
            AspectKey::new("risk").unwrap(),
            AspectIdentity(7),
            AspectContractRevision(1),
            AspectBinding::EntityField {
                field: FieldKey::new("value").unwrap(),
            },
            AuthoritativeAspectChangeKind::WholeAspectSet,
            None,
        );
    assert!(bridge_candidate_accepts_change(&consumer, &whole));
}

#[test]
fn record_locality_is_matched_against_the_same_delivered_change() {
    let record_a = RelationalBridgeRecordIdentityParts::entity(7, 11, 1);
    let record_b = RelationalBridgeRecordIdentityParts::entity(7, 12, 1);
    let consumer = record_candidate(BridgeSemanticLocality::SourceRecord, Some(record_a));
    let delivered = record_candidate(BridgeSemanticLocality::ManagedSourceRecord, None);

    assert!(bound_change_locality_matches(
        &consumer,
        &delivered,
        Some(record_a)
    ));
    assert!(!bound_change_locality_matches(
        &consumer,
        &delivered,
        Some(record_b)
    ));
}

#[test]
fn semantic_path_and_record_locality_cannot_cross_product() {
    let record_a = RelationalBridgeRecordIdentityParts::entity(7, 11, 1);
    let record_b = RelationalBridgeRecordIdentityParts::entity(7, 12, 1);
    let consumer = record_candidate(BridgeSemanticLocality::SourceRecord, Some(record_a));
    let delivered = record_candidate(BridgeSemanticLocality::ManagedSourceRecord, None);
    let status_a = semantic_change("status");
    let value_b = semantic_change("value");

    assert!(!bound_semantic_change_matches(
        &consumer,
        &delivered,
        &status_a,
        Some(record_a)
    ));
    assert!(!bound_semantic_change_matches(
        &consumer,
        &delivered,
        &value_b,
        Some(record_b)
    ));
    assert!(bound_semantic_change_matches(
        &consumer,
        &delivered,
        &value_b,
        Some(record_a)
    ));
}

#[test]
fn structural_kind_and_record_locality_cannot_cross_product() {
    let record_a = RelationalBridgeRecordIdentityParts::entity(7, 11, 1);
    let record_b = RelationalBridgeRecordIdentityParts::entity(7, 12, 1);
    let consumer = structural_record_candidate(
        BridgeSemanticLocality::SourceRecord,
        Some(record_a),
        vec![AuthoritativeAspectChangeKind::StructuralDelete],
    );
    let delivered = structural_record_candidate(
        BridgeSemanticLocality::ManagedSourceRecord,
        None,
        vec![
            AuthoritativeAspectChangeKind::StructuralCreate,
            AuthoritativeAspectChangeKind::StructuralDelete,
        ],
    );

    assert!(!bound_structural_change_matches(
        &consumer,
        &delivered,
        AuthoritativeAspectChangeKind::StructuralCreate,
        Some(record_a)
    ));
    assert!(!bound_structural_change_matches(
        &consumer,
        &delivered,
        AuthoritativeAspectChangeKind::StructuralDelete,
        Some(record_b)
    ));
    assert!(bound_structural_change_matches(
        &consumer,
        &delivered,
        AuthoritativeAspectChangeKind::StructuralDelete,
        Some(record_a)
    ));
}

fn candidate(
    projection_mask: AspectMask<ProjectionMask>,
    relevant_changes: Vec<AuthoritativeAspectChangeKind>,
) -> BridgeSemanticDependencyCandidate {
    BridgeSemanticDependencyCandidate::admit(BridgeSemanticDependencyCandidateParts {
        source_installation_identity: Arc::from("query-installation"),
        source_basis: Arc::from("query-basis"),
        source_runtime_authority: 1,
        source_installation_generation: 1,
        source_authority_binding_identity: Arc::from("query-binding"),
        source_stage_identity: None,
        source_node_identity: Arc::from("query-node"),
        dependency_ordinal: 1,
        declared_graph_role: Arc::from("primary"),
        graph_participation_identity: Arc::from("primary-graph"),
        graph_adapter_identity: Arc::from("primary-adapter"),
        source_record_identity: None,
        observation_record_identity: None,
        contract: AspectContract::struct_aspect(
            AspectKey::new("risk").unwrap(),
            AspectIdentity(7),
            AspectContractRevision(1),
            StructAspectShape::new(["value", "status"].map(|field| {
                FieldDeclaration::new(
                    FieldKey::new(field).unwrap(),
                    ScalarAspectType::UInt64,
                    FieldRequirement::Required,
                    AbsenceLaw::Required,
                    AspectEvolutionPolicy::ExplicitBreakRequired,
                )
                .unwrap()
            }))
            .unwrap(),
        ),
        projection_mask,
        binding: AspectBinding::EntityField {
            field: FieldKey::new("value").unwrap(),
        },
        locality: BridgeSemanticLocality::WholeLogicalGraph,
        relevant_changes,
    })
    .unwrap()
}

fn record_candidate(
    locality: BridgeSemanticLocality,
    source_record_identity: Option<RelationalBridgeRecordIdentityParts>,
) -> BridgeSemanticDependencyCandidate {
    record_candidate_with(
        locality,
        source_record_identity,
        AspectBinding::EntityField {
            field: FieldKey::new("value").unwrap(),
        },
        vec![AuthoritativeAspectChangeKind::FieldSet],
    )
}

fn record_candidate_with(
    locality: BridgeSemanticLocality,
    source_record_identity: Option<RelationalBridgeRecordIdentityParts>,
    binding: AspectBinding,
    relevant_changes: Vec<AuthoritativeAspectChangeKind>,
) -> BridgeSemanticDependencyCandidate {
    BridgeSemanticDependencyCandidate::admit(BridgeSemanticDependencyCandidateParts {
        source_installation_identity: Arc::from("query-installation"),
        source_basis: Arc::from("query-basis"),
        source_runtime_authority: 1,
        source_installation_generation: 1,
        source_authority_binding_identity: Arc::from("query-binding"),
        source_stage_identity: None,
        source_node_identity: Arc::from("query-node"),
        dependency_ordinal: 1,
        declared_graph_role: Arc::from("primary"),
        graph_participation_identity: Arc::from("primary-graph"),
        graph_adapter_identity: Arc::from("primary-adapter"),
        source_record_identity,
        observation_record_identity: source_record_identity,
        contract: AspectContract::struct_aspect(
            AspectKey::new("risk").unwrap(),
            AspectIdentity(7),
            AspectContractRevision(1),
            StructAspectShape::new(["value", "status"].map(|field| {
                FieldDeclaration::new(
                    FieldKey::new(field).unwrap(),
                    ScalarAspectType::UInt64,
                    FieldRequirement::Required,
                    AbsenceLaw::Required,
                    AspectEvolutionPolicy::ExplicitBreakRequired,
                )
                .unwrap()
            }))
            .unwrap(),
        ),
        projection_mask: AspectMask::new([path("value")]),
        binding,
        locality,
        relevant_changes,
    })
    .unwrap()
}

fn structural_record_candidate(
    locality: BridgeSemanticLocality,
    source_record_identity: Option<RelationalBridgeRecordIdentityParts>,
    relevant_changes: Vec<AuthoritativeAspectChangeKind>,
) -> BridgeSemanticDependencyCandidate {
    record_candidate_with(
        locality,
        source_record_identity,
        AspectBinding::StructuralRegion,
        relevant_changes,
    )
}

fn semantic_change(field: &str) -> worth_runtime_bridge::facade::BridgeSemanticAspectChange {
    worth_runtime_bridge::facade::BridgeSemanticAspectChange::from_authoritative_publication(
        AspectKey::new("risk").unwrap(),
        AspectIdentity(7),
        AspectContractRevision(1),
        AspectBinding::EntityField {
            field: FieldKey::new("value").unwrap(),
        },
        AuthoritativeAspectChangeKind::FieldSet,
        Some(path(field)),
    )
}

fn path(field: &str) -> CanonicalFieldPath {
    CanonicalFieldPath::single(FieldKey::new(field).unwrap())
}
