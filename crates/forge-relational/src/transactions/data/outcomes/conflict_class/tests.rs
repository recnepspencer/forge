use crate::identity::data::{EntityId, KindId, PartitionId, RelationId};
use crate::schema::data::{DescriptorSemanticsVersion, SchemaId, SchemaVersionId};
use crate::transactions::data::{
    BulkImportRowDomain, BulkImportStage, ConflictClass, EntityAuthoritativeAspectStateDenial,
    EntityFieldAspectPatchDenial, EntityUpdateMissingState, RelationAuthoritativeAspectStateDenial,
    RelationEndpointUpdateMissingState,
};
use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, AuthoritativePatchApplicationDenial,
    AuthoritativePatchConstructionDenial, BoundarySourceLocator, CanonicalFieldPath, FieldKey,
    LocatorAuthority, ScalarAspectType,
};

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
fn entity_field_aspect_patch_conflict_carries_foundational_denial() {
    let field = FieldKey::new("count").expect("valid test field key");
    let conflict = ConflictClass::EntityFieldAspectPatchDenied {
        entity_id: EntityId::new(PartitionId::main(), 9, 0),
        denial: EntityFieldAspectPatchDenial::PatchConstructionDenied {
            field_locator: Some(AspectFieldLocator::new(
                LocatorAuthority::Planned,
                AspectKey::new("counter").expect("valid aspect key"),
                CanonicalFieldPath::single(field.clone()),
            )),
            denial: AuthoritativePatchConstructionDenial::FieldTypeMismatch {
                field: field.clone(),
                expected: ScalarAspectType::Int64,
                found: ScalarAspectType::String,
            },
        },
    };

    let ConflictClass::EntityFieldAspectPatchDenied {
        denial:
            EntityFieldAspectPatchDenial::PatchConstructionDenied {
                field_locator,
                denial:
                    AuthoritativePatchConstructionDenial::FieldTypeMismatch {
                        field: actual_field,
                        expected,
                        found,
                    },
            },
        ..
    } = conflict
    else {
        panic!("expected foundational field-type denial");
    };

    let field_locator = field_locator.expect("field-level construction target");
    assert_eq!(
        field_locator.aspect().aspect_key(),
        &AspectKey::new("counter").expect("valid aspect key")
    );
    assert_eq!(field_locator.field_path().fields().first(), Some(&field));
    assert_eq!(actual_field, field);
    assert_eq!(expected, ScalarAspectType::Int64);
    assert_eq!(found, ScalarAspectType::String);
}

#[test]
fn entity_field_patch_application_denial_carries_contract_field_locator() {
    let field = FieldKey::new("count").expect("valid test field key");
    let field_locator = AspectFieldLocator::new(
        LocatorAuthority::Planned,
        AspectKey::new("counter").expect("valid aspect key"),
        CanonicalFieldPath::single(field.clone()),
    );
    let conflict = ConflictClass::EntityFieldAspectPatchDenied {
        entity_id: EntityId::new(PartitionId::main(), 10, 0),
        denial: EntityFieldAspectPatchDenial::FieldPatchApplicationDenied {
            field_locator: field_locator.clone(),
            denial: AuthoritativePatchApplicationDenial::MissingAspectForFieldPatch(
                AspectKey::new("counter").expect("valid aspect key"),
            ),
        },
    };

    let ConflictClass::EntityFieldAspectPatchDenied {
        denial:
            EntityFieldAspectPatchDenial::FieldPatchApplicationDenied {
                field_locator: actual_locator,
                denial:
                    AuthoritativePatchApplicationDenial::MissingAspectForFieldPatch(actual_aspect_key),
            },
        ..
    } = conflict
    else {
        panic!("expected field patch application denial with locator");
    };

    assert_eq!(actual_locator, field_locator);
    assert_eq!(
        actual_aspect_key,
        AspectKey::new("counter").expect("valid aspect key")
    );
}

#[test]
fn authoritative_aspect_state_conflicts_carry_boundary_source_locators() {
    let entity_denial = EntityAuthoritativeAspectStateDenial::UnsupportedAspectValue {
        source_locator: authoritative_field_source_locator(
            crate::tests::support::aspect_key("profile.summary"),
            crate::tests::support::field_key("summary"),
        ),
        value_family: "non-scalar-compatibility-input".to_string(),
    };
    let relation_denial = RelationAuthoritativeAspectStateDenial::UnsupportedAspectValue {
        source_locator: authoritative_field_source_locator(
            crate::tests::support::aspect_key("edge.label"),
            crate::tests::support::field_key("label"),
        ),
        value_family: "non-scalar-compatibility-input".to_string(),
    };

    assert_authoritative_field_source_locator(
        entity_denial_source_locator(&entity_denial),
        "profile.summary",
        "summary",
    );
    assert_authoritative_field_source_locator(
        relation_denial_source_locator(&relation_denial),
        "edge.label",
        "label",
    );

    let entity_conflict = ConflictClass::EntityAuthoritativeAspectStateDenied {
        kind_id: KindId(1),
        denial: entity_denial,
    };
    let relation_conflict = ConflictClass::RelationAuthoritativeAspectStateDenied {
        kind_id: KindId(2),
        denial: relation_denial,
    };

    assert!(entity_conflict.detail().contains("profile.summary"));
    assert!(entity_conflict.detail().contains("summary"));
    assert!(relation_conflict.detail().contains("edge.label"));
    assert!(relation_conflict.detail().contains("label"));
}

fn authoritative_field_source_locator(
    aspect_key: AspectKey,
    field_key: FieldKey,
) -> BoundarySourceLocator {
    BoundarySourceLocator::aspect_field(AspectFieldLocator::new(
        LocatorAuthority::SupportOnly,
        aspect_key,
        CanonicalFieldPath::single(field_key),
    ))
}

fn entity_denial_source_locator(
    denial: &EntityAuthoritativeAspectStateDenial,
) -> &forge_foundational::facade::BoundarySourceLocator {
    match denial {
        EntityAuthoritativeAspectStateDenial::UnsupportedAspectValue { source_locator, .. } => {
            source_locator
        }
        other => panic!("expected unsupported entity authoritative aspect denial, got {other:?}"),
    }
}

fn relation_denial_source_locator(
    denial: &RelationAuthoritativeAspectStateDenial,
) -> &forge_foundational::facade::BoundarySourceLocator {
    match denial {
        RelationAuthoritativeAspectStateDenial::UnsupportedAspectValue {
            source_locator, ..
        } => source_locator,
        other => panic!("expected unsupported relation authoritative aspect denial, got {other:?}"),
    }
}

fn assert_authoritative_field_source_locator(
    locator: &forge_foundational::facade::BoundarySourceLocator,
    expected_aspect_key: &str,
    expected_field: &str,
) {
    let forge_foundational::facade::BoundarySourceLocator::AspectField(field_locator) = locator
    else {
        panic!("expected aspect field source locator, got {locator:?}");
    };
    assert_eq!(
        field_locator.aspect().aspect_key().as_str(),
        expected_aspect_key
    );
    assert_eq!(
        field_locator.field_path().fields(),
        &[crate::tests::support::field_key(expected_field)]
    );
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
        ConflictClass::DescriptorVersionIncompatibility {
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
        ConflictClass::TypeIncompatibleSchemaTransition {
            detail: "type mismatch".to_string(),
        },
        ConflictClass::StructuralIncompatibleSchemaTransition {
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
