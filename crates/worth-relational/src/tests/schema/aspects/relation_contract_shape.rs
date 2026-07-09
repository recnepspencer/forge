use crate::facade::schema::{
    KindAspectContractDeclarations, RelationKindRegistration, RelationalSchemaRegistry, SchemaId,
    SchemaRegistryErrorClass, SchemaVersionId,
};
use crate::tests::schema::aspects::fixtures::relation_field_with_entity_reference_contract;
use crate::tests::support::*;

#[test]
fn relation_field_aspects_are_governed_by_foundational_contract_shape() {
    let registry = RelationalSchemaRegistry::new()
        .register_relation_kind(RelationKindRegistration {
            kind_id: KindId(2),
            kind_name: "test.relation".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            cross_context_policy: CrossContextPolicy::AllowExplicit,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            aspect_contract_declarations: KindAspectContractDeclarations::new(vec![
                relation_field_aspect(
                    crate::tests::support::aspect_key("relation_label"),
                    crate::tests::support::field_key("label"),
                ),
            ]),
            relation_integrity: crate::schema::data::RelationIntegrityDeclarations::default(),
        })
        .expect("relation field aspect with scalar contract registers");

    let relation_trace = registry
        .relation_aspect_declaration_trace(KindId(2))
        .expect("relation declaration trace");
    assert_eq!(
        relation_trace.declarations[0].aspect_key,
        aspect_key("relation_label")
    );

    let error = RelationalSchemaRegistry::new()
        .register_relation_kind(RelationKindRegistration {
            kind_id: KindId(3),
            kind_name: "test.invalid_relation".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            cross_context_policy: CrossContextPolicy::AllowExplicit,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            aspect_contract_declarations: KindAspectContractDeclarations::new(vec![
                relation_field_with_entity_reference_contract(
                    crate::tests::support::aspect_key("target_as_field"),
                    crate::tests::support::field_key("label"),
                ),
            ]),
            relation_integrity: crate::schema::data::RelationIntegrityDeclarations::default(),
        })
        .unwrap_err();

    assert!(matches!(
        error.class,
        SchemaRegistryErrorClass::InvalidAspectDeclaration {
            kind_id: KindId(3),
            ..
        }
    ));
}
