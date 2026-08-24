use crate::schema::data::{
    HistoricalInterpretationSensitivity, ProposedSchemaTransition, SchemaDiffAtom,
    SchemaDiffDetail, SchemaElementKind, SchemaElementRef, SchemaId, SchemaPublicationImpact,
    SchemaReconciliationPolicy, SchemaStratum, SchemaSubscriberImpact, SchemaVersionId,
};
use crate::tests::support::{
    field_key, AspectSchemaFixture, KindId, RelationalTransactionValidationInput,
};

pub(super) fn install_schema_version(
    runtime: &mut crate::facade::runtime::RelationalRuntime,
    schema_version_id: SchemaVersionId,
) {
    runtime.config.schema.registry = AspectSchemaFixture {
        schema_version_id,
        ..AspectSchemaFixture::with_default_declared_aspects(
            crate::tests::support::CascadeDeletePolicy::CascadeDeleteRelations,
        )
    }
    .build_registry();
}

pub(super) fn visible_bridge_transition_options(
    runtime: &crate::facade::runtime::RelationalRuntime,
    schema_version_id: SchemaVersionId,
) -> RelationalTransactionValidationInput {
    crate::tests::support::test_owner_transaction_validation_input_for_main(runtime)
        .with_schema_transition(
            ProposedSchemaTransition {
                source_schema_id: SchemaId("test".to_string()),
                source_schema_version_id: SchemaVersionId(schema_version_id.0 - 1),
                target_schema_id: SchemaId("test".to_string()),
                target_schema_version_id: schema_version_id,
                diff_atoms: vec![SchemaDiffAtom::new(
                    SchemaElementRef::new(
                        SchemaElementKind::Field,
                        SchemaId("test".to_string()),
                        schema_version_id,
                        Some(KindId(1)),
                        "tag",
                    ),
                    vec![
                        SchemaStratum::StructuralShape,
                        SchemaStratum::PublicationContract,
                    ],
                    SchemaPublicationImpact::ObservableSurfaceChanged,
                    SchemaSubscriberImpact::ConsumableSurfaceChanged,
                    HistoricalInterpretationSensitivity::NotSensitive,
                    SchemaDiffDetail::AddedField {
                        field: field_key("tag"),
                        required: false,
                        default_expression: Some("null".into()),
                    },
                )
                .with_boundary_visibility_proof(
                    crate::schema::data::SubscriberBoundaryVisibility::VisibleSemanticallyIgnorable,
                )],
            },
            Some(SchemaReconciliationPolicy::PreserveInformation),
        )
}
