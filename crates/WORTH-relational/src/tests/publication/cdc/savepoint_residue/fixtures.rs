use crate::tests::support::*;

pub(super) fn relation_integrity_schema(
    cascade_delete_policy: CascadeDeletePolicy,
    relation_integrity: crate::schema::data::RelationIntegrityDeclarations,
) -> RelationalSchemaRegistry {
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_contract_declarations: KindAspectContractDeclarations::new(vec![
                entity_field_aspect(aspect_key("name"), field_key("name")),
                lifecycle_aspect(),
            ]),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy,
                aspect_contract_declarations: KindAspectContractDeclarations::default(),
                relation_integrity,
            })
        })
        .unwrap()
}

pub(super) fn endpoint_kind_integrity_declarations(
) -> crate::schema::data::RelationIntegrityDeclarations {
    crate::schema::data::RelationIntegrityDeclarations::new(
        vec![crate::schema::data::EndpointKindContractDeclaration {
            contract_id: "no_self".into(),
            allowed_source_kinds: vec![KindId(1)],
            allowed_target_kinds: vec![KindId(1)],
            self_edges_allowed: false,
            cross_context_policy: CrossContextPolicy::AllowExplicit,
        }],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
}

pub(super) fn endpoint_deletion_integrity_declarations(
) -> crate::schema::data::RelationIntegrityDeclarations {
    crate::schema::data::RelationIntegrityDeclarations::new(
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        vec![crate::schema::data::EndpointDeletionIntegrityDeclaration {
            contract_id: "require_retirement".into(),
            mode: crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationRetirement,
        }],
    )
}
