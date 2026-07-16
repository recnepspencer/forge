use crate::identity::data::{EntityId, KindId, PartitionId, RelationId};
use crate::schema::data::{DescriptorSemanticsVersion, SchemaId, SchemaVersionId};
use crate::transactions::data::{
    AspectFieldTargetRejectionReason, BulkImportRowDomain, BulkImportStage, ConflictClass,
    EntityAuthoritativeAspectStateDenial, EntityUpdateMissingState,
    RelationEndpointUpdateMissingState,
};
use worth_foundational::facade::AspectFieldLocator;

#[test]
fn entity_update_state_conflict_is_typed_not_json() {
    let conflict = ConflictClass::EntityUpdateStateInconsistency {
        entity_id: EntityId::new(PartitionId::main(), 7, 0),
        missing: EntityUpdateMissingState::AuthoritativeAspectState,
    };

    assert!(conflict
        .detail()
        .contains("retained authoritative aspect state after stale-target validation"));
}

#[test]
fn relation_endpoint_state_conflicts_are_typed_not_json() {
    let relation_id = RelationId::new(PartitionId::main(), 8, 0);
    let missing_state = ConflictClass::RelationEndpointUpdateStateInconsistency {
        relation_id,
        missing: RelationEndpointUpdateMissingState::Endpoints,
    };
    let kind_mismatch = ConflictClass::RelationEndpointUpdateKindMismatch {
        relation_id,
        intent_kind_id: KindId(1),
        authoritative_kind_id: KindId(2),
    };

    assert!(missing_state.detail().contains("retained endpoints"));
    assert!(kind_mismatch.detail().contains("intent kind 1"));
}

#[test]
fn bulk_import_domain_mismatch_is_typed_not_json() {
    let conflict = ConflictClass::BulkImportDomainMismatch {
        expected: BulkImportRowDomain::Entity,
        actual: BulkImportRowDomain::Relation,
        stage: BulkImportStage::EntityCreate,
    };

    assert!(conflict.detail().contains("expected entity rows"));
}

#[test]
fn authoritative_aspect_state_conflict_carries_typed_target_rejection() {
    let entity_target = crate::transactions::data::planned_single_field_locator(
        crate::tests::support::aspect_key("profile.summary"),
        crate::tests::support::field_key("summary"),
    );
    let entity_denial = EntityAuthoritativeAspectStateDenial::UnsupportedAspectFieldTarget {
        target: entity_target.clone(),
        reason: AspectFieldTargetRejectionReason::UndeclaredAspect,
    };
    assert_eq!(entity_denial_target(&entity_denial), &entity_target);

    let entity_conflict = ConflictClass::EntityAuthoritativeAspectStateDenied {
        kind_id: KindId(1),
        denial: entity_denial,
    };
    assert!(entity_conflict.detail().contains("profile.summary"));
    assert!(entity_conflict.detail().contains("summary"));
    assert!(entity_conflict.detail().contains("undeclared aspect"));
}

fn entity_denial_target(denial: &EntityAuthoritativeAspectStateDenial) -> &AspectFieldLocator {
    match denial {
        EntityAuthoritativeAspectStateDenial::UnsupportedAspectFieldTarget { target, .. } => target,
        other => panic!("expected unsupported entity authoritative aspect denial, got {other:?}"),
    }
}

#[test]
fn schema_continuity_conflicts_are_typed_not_json() {
    let conflicts = [
        ConflictClass::UndeclaredSchemaTransition {
            previous_schema_version: SchemaVersionId(1),
            current_schema_version: SchemaVersionId(2),
            previous_descriptor_semantics_version: DescriptorSemanticsVersion(1),
            current_descriptor_semantics_version: DescriptorSemanticsVersion(2),
        },
        ConflictClass::DescriptorSemanticsVersionUnsupported {
            previous_descriptor_semantics_version: DescriptorSemanticsVersion(1),
            current_descriptor_semantics_version: DescriptorSemanticsVersion(2),
        },
        ConflictClass::InvalidSchemaTransitionSourceBasis {
            declared_schema_id: SchemaId("declared".to_string()),
            declared_schema_version: SchemaVersionId(1),
            expected_schema_id: SchemaId("expected".to_string()),
            expected_schema_version: SchemaVersionId(1),
        },
        ConflictClass::InvalidSchemaTransitionTargetBasis {
            declared_schema_id: SchemaId("declared".to_string()),
            declared_schema_version: SchemaVersionId(2),
            expected_schema_id: SchemaId("expected".to_string()),
            expected_schema_version: SchemaVersionId(2),
        },
        ConflictClass::MissingSchemaBasisForTransition {
            role: "source".to_string(),
        },
        ConflictClass::UnsupportedBridgeDescriptor {
            detail: "unsupported descriptor".to_string(),
        },
        ConflictClass::HistoricalReinterpretationViolation {
            detail: "reinterpretation denied".to_string(),
        },
        ConflictClass::TypeContinuityDeniedSchemaTransition {
            detail: "type mismatch".to_string(),
        },
        ConflictClass::StructuralContinuityDeniedSchemaTransition {
            detail: "shape mismatch".to_string(),
        },
        ConflictClass::DirectionalityMismatchUnderCanonicalReconciliation {
            detail: "direction mismatch".to_string(),
        },
        ConflictClass::InvalidSchemaTransitionShape {
            detail: "invalid transition shape".to_string(),
        },
    ];

    for conflict in conflicts {
        assert!(!conflict.detail().is_empty());
    }
}
