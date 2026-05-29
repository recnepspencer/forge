use crate::identity::data::{EntityId, KindId, PartitionId, RelationId};
use crate::schema::data::{DescriptorSemanticsVersion, SchemaId, SchemaVersionId};
use crate::transactions::data::{
    AspectFieldPatchTarget, AspectFieldTargetRejectionReason, BulkImportRowDomain, BulkImportStage,
    ConflictClass, EntityAuthoritativeAspectStateDenial, EntityFieldAspectPatchDenial,
    EntityUpdateMissingState, RelationAuthoritativeAspectStateDenial,
    RelationEndpointUpdateMissingState,
};
use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, AspectLocator, AuthoritativePatchApplicationDenial,
    AuthoritativePatchConstructionDenial, BoundarySourceLocator, CanonicalFieldPath,
    ContractValidationDenial, FieldKey, LocatorAuthority, ScalarAspectType,
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
fn authoritative_aspect_state_conflicts_carry_typed_target_rejections() {
    let entity_target = AspectFieldPatchTarget::single(
        crate::tests::support::aspect_key("profile.summary"),
        crate::tests::support::field_key("summary"),
    );
    let relation_target = AspectFieldPatchTarget::single(
        crate::tests::support::aspect_key("edge.label"),
        crate::tests::support::field_key("label"),
    );
    let entity_denial = EntityAuthoritativeAspectStateDenial::UnsupportedAspectFieldTarget {
        target: entity_target.clone(),
        reason: AspectFieldTargetRejectionReason::UndeclaredAspect,
    };
    let relation_denial = RelationAuthoritativeAspectStateDenial::UnsupportedAspectFieldTarget {
        target: relation_target.clone(),
        reason: AspectFieldTargetRejectionReason::FieldPathNotAdmittedByAspectBinding,
    };

    assert_eq!(entity_denial_target(&entity_denial), &entity_target);
    assert_eq!(relation_denial_target(&relation_denial), &relation_target);

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
    assert!(entity_conflict.detail().contains("undeclared aspect"));
    assert!(relation_conflict.detail().contains("edge.label"));
    assert!(relation_conflict.detail().contains("label"));
    assert!(relation_conflict
        .detail()
        .contains("field path not admitted by aspect binding"));
}

#[test]
fn relation_endpoint_contract_denial_carries_whole_aspect_locator() {
    let aspect_key = crate::tests::support::aspect_key("edge.source");
    let source_locator =
        BoundarySourceLocator::aspect(AspectLocator::new(LocatorAuthority::Planned, aspect_key));
    let denial = RelationAuthoritativeAspectStateDenial::ContractValidationDenied {
        source_locator: source_locator.clone(),
        denial: ContractValidationDenial::ScalarTypeMismatch {
            expected: ScalarAspectType::String,
            found: ScalarAspectType::Bool,
        },
    };
    let conflict = ConflictClass::RelationAuthoritativeAspectStateDenied {
        kind_id: KindId(4),
        denial,
    };

    let ConflictClass::RelationAuthoritativeAspectStateDenied {
        denial:
            RelationAuthoritativeAspectStateDenial::ContractValidationDenied {
                source_locator: actual_source_locator,
                ..
            },
        ..
    } = conflict.clone()
    else {
        panic!("expected relation authoritative aspect contract denial");
    };

    assert_eq!(actual_source_locator, source_locator);
    assert!(conflict.detail().contains("edge.source"));
    assert!(conflict.detail().contains("whole_aspect"));
    assert!(!conflict.detail().contains("source_endpoint"));
}

fn entity_denial_target(denial: &EntityAuthoritativeAspectStateDenial) -> &AspectFieldPatchTarget {
    match denial {
        EntityAuthoritativeAspectStateDenial::UnsupportedAspectFieldTarget { target, .. } => target,
        other => panic!("expected unsupported entity authoritative aspect denial, got {other:?}"),
    }
}

fn relation_denial_target(
    denial: &RelationAuthoritativeAspectStateDenial,
) -> &AspectFieldPatchTarget {
    match denial {
        RelationAuthoritativeAspectStateDenial::UnsupportedAspectFieldTarget { target, .. } => {
            target
        }
        other => panic!("expected unsupported relation authoritative aspect denial, got {other:?}"),
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
