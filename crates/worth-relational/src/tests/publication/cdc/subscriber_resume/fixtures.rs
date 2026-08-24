use crate::schema::data::{
    HistoricalInterpretationSensitivity, ProposedSchemaTransition, SchemaDiffAtom,
    SchemaDiffDetail, SchemaElementKind, SchemaElementRef, SchemaId, SchemaPublicationImpact,
    SchemaReconciliationPolicy, SchemaStratum, SchemaSubscriberImpact, SchemaVersionId,
};
use crate::tests::support::{
    field_key, AspectSchemaFixture, KindId, RelationalTransactionValidationInput,
};

pub(super) fn schema_transition_for_subscriber_impact(
    target_schema_version_id: SchemaVersionId,
    subscriber_impact: SchemaSubscriberImpact,
) -> ProposedSchemaTransition {
    ProposedSchemaTransition {
        source_schema_id: SchemaId("test".to_string()),
        source_schema_version_id: SchemaVersionId(target_schema_version_id.0 - 1),
        target_schema_id: SchemaId("test".to_string()),
        target_schema_version_id,
        diff_atoms: vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("test".to_string()),
                target_schema_version_id,
                Some(KindId(1)),
                "tag",
            ),
            vec![
                SchemaStratum::StructuralShape,
                SchemaStratum::PublicationContract,
            ],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            subscriber_impact,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field: field_key("tag"),
                required: false,
                default_expression: Some("null".into()),
            },
        )
        .with_boundary_visibility_proof(match subscriber_impact {
            SchemaSubscriberImpact::ConsumableSurfaceChanged => {
                crate::schema::data::SubscriberBoundaryVisibility::VisibleSemanticallyIgnorable
            }
            SchemaSubscriberImpact::ContractUpgradeRequired => {
                crate::schema::data::SubscriberBoundaryVisibility::VisibleRequiresContractUptake
            }
            _ => crate::schema::data::SubscriberBoundaryVisibility::NotVisible,
        })],
    }
}

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

pub(super) fn transaction_validation_input_for_subscriber_impact(
    runtime: &crate::facade::runtime::RelationalRuntime,
    schema_version_id: SchemaVersionId,
    subscriber_impact: SchemaSubscriberImpact,
) -> RelationalTransactionValidationInput {
    crate::tests::support::test_owner_transaction_validation_input_for_main(runtime)
        .with_schema_transition(
            schema_transition_for_subscriber_impact(schema_version_id, subscriber_impact),
            Some(SchemaReconciliationPolicy::PreserveInformation),
        )
}
