use super::super::support::*;

fn relation_integrity_registry(
    relation_integrity: crate::schema::data::RelationIntegrityDeclarations,
) -> RelationalSchemaRegistry {
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::default(),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity,
            })
        })
        .unwrap()
}

#[test]
fn relation_integrity_declaration_lowering_is_stable() {
    let registry = relation_integrity_registry(crate::schema::data::RelationIntegrityDeclarations::new(
        vec![crate::schema::data::EndpointKindContractDeclaration {
            contract_id: "endpoint".to_string(),
            allowed_source_kinds: vec![KindId(1)],
            allowed_target_kinds: vec![KindId(1)],
            self_edges_allowed: false,
            cross_context_policy: CrossContextPolicy::AllowExplicit,
        }],
        vec![crate::schema::data::CardinalityContractDeclaration {
            contract_id: "source_cardinality".to_string(),
            source_max: Some(1),
            target_max: None,
            pair_max: None,
        }],
        vec![crate::schema::data::UniquenessContractDeclaration {
            contract_id: "uniq".to_string(),
            scope: crate::schema::data::UniquenessScope::DirectedSemanticEdge,
        }],
        vec![crate::schema::data::SymmetryContractDeclaration {
            contract_id: "symmetry".to_string(),
            mode: crate::schema::data::SymmetryMode::InverseProhibited,
        }],
        vec![crate::schema::data::EndpointDeletionIntegrityDeclaration {
            contract_id: "delete_guard".to_string(),
            mode: crate::schema::data::EndpointDeletionIntegrityMode::RejectDeleteWithLiveRelations,
        }],
    ));

    let runtime = RelationalRuntimeApi::builder()
        .schema_registry(registry)
        .build();
    let plan = runtime.relation_integrity_plan(KindId(2)).unwrap();

    assert_eq!(plan.kind_id, KindId(2));
    assert_eq!(plan.contract_count(), 5);
    assert_ne!(plan.plan_revision.0, 0);
    assert_eq!(plan.endpoint_kind_contracts[0].contract_id, "endpoint");
    assert_eq!(plan.cardinality_contracts[0].source_max, Some(1));
}

#[test]
fn duplicate_relation_contract_ids_are_rejected() {
    let error = RelationalSchemaRegistry::new()
        .register_relation_kind(RelationKindRegistration {
            kind_id: KindId(2),
            kind_name: "test.relation".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            payload_class: RelationPayloadClass::PayloadBearingRelation,
            cross_context_policy: CrossContextPolicy::AllowExplicit,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            aspect_declarations: KindAspectDeclarations::default(),
            relation_integrity: crate::schema::data::RelationIntegrityDeclarations::new(
                vec![crate::schema::data::EndpointKindContractDeclaration {
                    contract_id: "dup".to_string(),
                    allowed_source_kinds: vec![KindId(1)],
                    allowed_target_kinds: vec![KindId(1)],
                    self_edges_allowed: true,
                    cross_context_policy: CrossContextPolicy::AllowExplicit,
                }],
                vec![crate::schema::data::CardinalityContractDeclaration {
                    contract_id: "dup".to_string(),
                    source_max: Some(1),
                    target_max: None,
                    pair_max: None,
                }],
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
        })
        .unwrap_err();

    assert!(matches!(
        error.class,
        crate::schema::data::SchemaRegistryErrorClass::DuplicateRelationContractId {
            kind_id: KindId(2),
            ..
        }
    ));
}
